//! Sequence IO in fasta format.

use std::{
    fs::File,
    io::{Read, Write},
    marker::PhantomData,
    mem,
    path::Path,
};

use traitsequence::interface::Sequence;

use crate::{
    interface::{
        alphabet::{Alphabet, AlphabetError},
        sequence_store::SequenceStore,
    },
    io::{peekable_reader::PeekableReader, unzip_if_zipped},
};

use super::{error::IOError, zip, ZipFormat};

/// A fasta record.
pub struct FastaRecord<Handle> {
    /// The id of the fasta record.
    pub id: String,
    /// Anything after the id of the fasta record.
    pub comment: String,
    /// The handle to the sequence of the fasta record.
    pub sequence_handle: Handle,
}

/// Read a fasta file into the given sequence store.
pub fn read_fasta_file<AlphabetType: Alphabet, SequenceStoreType: SequenceStore<AlphabetType>>(
    path: impl AsRef<Path>,
    store: &mut SequenceStoreType,
) -> Result<Vec<FastaRecord<SequenceStoreType::Handle>>, IOError> {
    let zip_format_hint = ZipFormat::from_path_name(&path);
    let file = File::open(path)?;

    unzip_if_zipped(file, zip_format_hint, |reader| {
        read_fasta(reader, store, false)
    })
}

/// Read a fasta file into the given sequence store.
///
/// Invalid characters are skipped.
pub fn read_fuzzy_fasta_file<
    AlphabetType: Alphabet,
    SequenceStoreType: SequenceStore<AlphabetType>,
>(
    path: impl AsRef<Path>,
    store: &mut SequenceStoreType,
) -> Result<Vec<FastaRecord<SequenceStoreType::Handle>>, IOError> {
    let zip_format_hint = ZipFormat::from_path_name(&path);
    let file = File::open(path)?;

    unzip_if_zipped(file, zip_format_hint, |reader| {
        read_fasta(reader, store, true)
    })
}

/// Read fasta data into the given sequence store.
/// The reader should be buffered for performance.
pub fn read_fasta<AlphabetType: Alphabet, SequenceStoreType: SequenceStore<AlphabetType>>(
    reader: impl Read,
    store: &mut SequenceStoreType,
    skip_invalid_characters: bool,
) -> Result<Vec<FastaRecord<SequenceStoreType::Handle>>, IOError> {
    enum State {
        Init,
        RecordId,
        RecordWhitespace,
        RecordComment,
        RecordSequence,
        EndOfRecord { has_more_records: bool },
    }

    let mut records = Vec::new();
    let mut record_id = String::new();
    let mut record_comment = String::new();
    let mut record_sequence_handle: Option<SequenceStoreType::Handle> = None;

    let mut reader = PeekableReader::new(reader);
    let mut state = State::Init;
    let mut buffer = Vec::<u8>::new();

    loop {
        match state {
            State::Init => {
                buffer.resize(1, 0);
                let mut is_newline = true;

                state = loop {
                    let result = reader.read_exact(&mut buffer);
                    if let Err(error) = result {
                        if matches!(error.kind(), std::io::ErrorKind::UnexpectedEof) {
                            return Ok(records);
                        } else {
                            return Err(error.into());
                        }
                    }

                    match buffer[0] {
                        b'\r' | b'\n' => is_newline = true,
                        b'>' => {
                            if is_newline {
                                break State::RecordId;
                            }
                        }
                        _ => is_newline = false,
                    }
                };
            }
            State::RecordId => {
                buffer.resize(1, 0);

                state = loop {
                    let result = reader.read_exact(&mut buffer);
                    if let Err(error) = result {
                        if matches!(error.kind(), std::io::ErrorKind::UnexpectedEof) {
                            break State::EndOfRecord {
                                has_more_records: false,
                            };
                        } else {
                            return Err(error.into());
                        }
                    }

                    if buffer[0] == b'\n' || buffer[0] == b'\r' {
                        break State::RecordSequence;
                    } else if buffer[0].is_ascii_whitespace() {
                        break State::RecordWhitespace;
                    } else {
                        record_id.push(buffer[0].into());
                    }
                };
            }
            State::RecordWhitespace => {
                buffer.resize(1, 0);

                state = loop {
                    let result = reader.read_exact(&mut buffer);
                    if let Err(error) = result {
                        if matches!(error.kind(), std::io::ErrorKind::UnexpectedEof) {
                            break State::EndOfRecord {
                                has_more_records: false,
                            };
                        } else {
                            return Err(error.into());
                        }
                    }

                    if buffer[0] == b'\n' || buffer[0] == b'\r' {
                        break State::RecordSequence;
                    } else if !buffer[0].is_ascii_whitespace() {
                        record_comment.push(buffer[0].into());
                        break State::RecordComment;
                    }
                };
            }
            State::RecordComment => {
                buffer.resize(1, 0);

                state = loop {
                    let result = reader.read_exact(&mut buffer);
                    if let Err(error) = result {
                        if matches!(error.kind(), std::io::ErrorKind::UnexpectedEof) {
                            break State::EndOfRecord {
                                has_more_records: false,
                            };
                        } else {
                            return Err(error.into());
                        }
                    }

                    if buffer[0] == b'\n' || buffer[0] == b'\r' {
                        break State::RecordSequence;
                    } else {
                        record_comment.push(buffer[0].into());
                    }
                };
            }
            State::RecordSequence => {
                let mut iterator = FastaSequenceIterator {
                    reader: &mut reader,
                    buffer: Default::default(),
                    result: None,
                    newline: true,
                    skip_invalid_characters,
                    phantom_data: PhantomData::<AlphabetType>,
                };

                record_sequence_handle = Some(store.add_from_iter(&mut iterator));
                state = State::EndOfRecord {
                    has_more_records: iterator.result.unwrap()?,
                };
            }
            State::EndOfRecord { has_more_records } => {
                let comment = record_comment.trim_end().to_string();
                record_comment.clear();

                let sequence_handle = record_sequence_handle
                    .take()
                    .unwrap_or_else(|| store.add_from_slice_u8(&[]).unwrap());

                records.push(FastaRecord {
                    id: mem::take(&mut record_id),
                    comment,
                    sequence_handle,
                });

                if has_more_records {
                    state = State::RecordId;
                } else {
                    break;
                }
            }
        }
    }

    Ok(records)
}

struct FastaSequenceIterator<'a, AlphabetType, Reader> {
    reader: &'a mut Reader,
    buffer: [u8; 1],
    /// Holds Some() on termination.
    /// Is Err() if an error occurred, and Ok() otherwise.
    /// The bool is true if there are more records.
    result: Option<Result<bool, IOError>>,
    newline: bool,
    skip_invalid_characters: bool,
    phantom_data: PhantomData<AlphabetType>,
}

impl<'a, AlphabetType: Alphabet, Reader: Read> Iterator
    for FastaSequenceIterator<'a, AlphabetType, Reader>
{
    type Item = AlphabetType::CharacterType;

    fn next(&mut self) -> Option<Self::Item> {
        if self.result.is_some() {
            return None;
        }

        loop {
            let result = self.reader.read_exact(&mut self.buffer);
            if let Err(error) = result {
                if matches!(error.kind(), std::io::ErrorKind::UnexpectedEof) {
                    self.result = Some(Ok(false));
                    return None;
                } else {
                    self.result = Some(Err(error.into()));
                    return None;
                }
            }

            if self.buffer[0] == b'>' && self.newline {
                self.result = Some(Ok(true));
                self.newline = false;
                return None;
            } else if self.buffer[0] != b'\n' && self.buffer[0] != b'\r' {
                self.newline = false;
                match AlphabetType::CharacterType::try_from(self.buffer[0]) {
                    Ok(character) => return Some(character),
                    Err(_) => {
                        if !self.skip_invalid_characters {
                            self.result = Some(Err(IOError::AlphabetError(
                                AlphabetError::AsciiNotPartOfAlphabet {
                                    ascii: self.buffer[0],
                                },
                            )));
                            return None;
                        }
                    }
                }
            } else {
                self.newline = true;
            }
        }
    }
}

/// Write a fasta file from the given records.
pub fn write_fasta_file<AlphabetType: Alphabet, SequenceStoreType: SequenceStore<AlphabetType>>(
    path: impl AsRef<Path>,
    records: impl IntoIterator<Item = FastaRecord<SequenceStoreType::Handle>>,
    store: &SequenceStoreType,
) -> Result<(), IOError> {
    let zip_format = ZipFormat::from_path_name(&path);
    let file = File::create(path)?;

    zip(file, zip_format, |writer| {
        write_fasta(writer, records, store)
    })
}

/// Write fasta data into the given sequence store.
/// The writer should be buffered for performance.
pub fn write_fasta<AlphabetType: Alphabet, SequenceStoreType: SequenceStore<AlphabetType>>(
    mut writer: impl Write,
    records: impl IntoIterator<Item = FastaRecord<SequenceStoreType::Handle>>,
    store: &SequenceStoreType,
) -> Result<(), IOError> {
    for record in records {
        writeln!(
            writer,
            ">{id}{space}{comment}",
            id = record.id,
            space = if record.comment.is_empty() { "" } else { " " },
            comment = record.comment
        )?;
        let sequence = store.get(&record.sequence_handle);
        for character in sequence.iter() {
            write!(writer, "{character}")?;
        }
        writeln!(writer)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use core::str;

    use crate::{
        implementation::alphabets::dna_alphabet::DnaAlphabet, implementation::DefaultSequenceStore,
    };

    use super::{read_fasta, write_fasta};

    #[test]
    fn test_read_write() {
        let input_file =
            b">alt1 comment1\nGGTTGGCCT\n>f2\nACCTG\n>f3 \nAA\n>seq c2  \nGT".as_slice();
        let expected_output_file =
            b">alt1 comment1\nGGTTGGCCT\n>f2\nACCTG\n>f3\nAA\n>seq c2\nGT\n".as_slice();

        let mut store = DefaultSequenceStore::<DnaAlphabet>::new();
        let records = read_fasta(input_file, &mut store, false).unwrap();
        let mut output_file = Vec::new();
        write_fasta(&mut output_file, records, &store).unwrap();

        assert_eq!(
            expected_output_file,
            output_file,
            "expected output:\n{}\n\noutput:\n{}",
            str::from_utf8(expected_output_file)
                .unwrap()
                .replace(' ', "_"),
            str::from_utf8(&output_file).unwrap().replace(' ', "_"),
        );
    }

    #[test]
    fn test_invalid_characters() {
        let input_file =
            b">alt1 comment1\nGGTTZGGCCT\n>f2\nACCUTG\n>f3 \nAA\n>seq c2  \nGaT".as_slice();
        let expected_output_file =
            b">alt1 comment1\nGGTTGGCCT\n>f2\nACCTG\n>f3\nAA\n>seq c2\nGT\n".as_slice();

        let mut store = DefaultSequenceStore::<DnaAlphabet>::new();
        let records = read_fasta(input_file, &mut store, true).unwrap();
        let mut output_file = Vec::new();
        write_fasta(&mut output_file, records, &store).unwrap();

        assert_eq!(
            expected_output_file,
            output_file,
            "expected output:\n{}\n\noutput:\n{}",
            str::from_utf8(expected_output_file)
                .unwrap()
                .replace(' ', "_"),
            str::from_utf8(&output_file).unwrap().replace(' ', "_"),
        );
    }
}
