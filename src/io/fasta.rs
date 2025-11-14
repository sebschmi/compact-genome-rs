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
///
/// If `skip_invalid_characters` is set, then invalid characters are skipped.
/// If `capitalise_characters` is set, then lower-case characters are parsed as upper-case.
/// If an ASCII index in `skip_characters` contains true, then that character will always be skipped (after capitalisation).
/// If the index does not exist (i.e. `skip_characters` is too short), the character will not be skipped.
pub fn read_fasta_file<AlphabetType: Alphabet, SequenceStoreType: SequenceStore<AlphabetType>>(
    path: impl AsRef<Path>,
    store: &mut SequenceStoreType,
    skip_invalid_characters: bool,
    capitalise_characters: bool,
    skip_characters: &[bool],
) -> Result<Vec<FastaRecord<SequenceStoreType::Handle>>, IOError> {
    let zip_format_hint = ZipFormat::from_path_name(&path);
    let file = File::open(path)?;

    unzip_if_zipped(file, zip_format_hint, |reader| {
        read_fasta(
            reader,
            store,
            skip_invalid_characters,
            capitalise_characters,
            skip_characters,
        )
    })
}

/// Read fasta data into the given sequence store.
///
/// The reader should be buffered for performance.
/// If an ASCII index in `skip_characters` contains true, then that character will always be skipped (after capitalisation).
/// If the index does not exist (i.e. `skip_characters` is too short), the character will not be skipped.
pub fn read_fasta<AlphabetType: Alphabet, SequenceStoreType: SequenceStore<AlphabetType>>(
    reader: impl Read,
    store: &mut SequenceStoreType,
    skip_invalid_characters: bool,
    capitalise_characters: bool,
    skip_characters: &[bool],
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
                    capitalise_characters,
                    skip_characters,
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

struct FastaSequenceIterator<'reader, 'skip_characters, AlphabetType, Reader> {
    reader: &'reader mut Reader,
    buffer: [u8; 1],
    /// Holds Some() on termination.
    /// Is Err() if an error occurred, and Ok() otherwise.
    /// The bool is true if there are more records.
    result: Option<Result<bool, IOError>>,
    newline: bool,
    skip_invalid_characters: bool,
    capitalise_characters: bool,
    skip_characters: &'skip_characters [bool],
    phantom_data: PhantomData<AlphabetType>,
}

impl<AlphabetType: Alphabet, Reader: Read> Iterator
    for FastaSequenceIterator<'_, '_, AlphabetType, Reader>
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

                let ascii = if self.capitalise_characters {
                    self.buffer[0].to_ascii_uppercase()
                } else {
                    self.buffer[0]
                };

                if !self
                    .skip_characters
                    .get(usize::from(ascii))
                    .copied()
                    .unwrap_or(false)
                {
                    match AlphabetType::CharacterType::try_from(ascii) {
                        Ok(character) => return Some(character),
                        Err(_) => {
                            if !self.skip_invalid_characters {
                                self.result = Some(Err(IOError::AlphabetError(
                                    AlphabetError::AsciiNotPartOfAlphabet {
                                        ascii: char::from(ascii),
                                    },
                                )));
                                return None;
                            }
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
pub fn write_fasta_file<
    'records,
    AlphabetType: Alphabet,
    SequenceStoreType: SequenceStore<AlphabetType>,
>(
    path: impl AsRef<Path>,
    records: impl IntoIterator<Item = &'records FastaRecord<SequenceStoreType::Handle>>,
    store: &SequenceStoreType,
) -> Result<(), IOError>
where
    SequenceStoreType::Handle: 'records,
{
    let zip_format = ZipFormat::from_path_name(&path);
    let file = File::create(path)?;

    zip(file, zip_format, |writer| {
        write_fasta(writer, records, store)
    })
}

/// Write fasta data into the given sequence store.
/// The writer should be buffered for performance.
pub fn write_fasta<
    'records,
    AlphabetType: Alphabet,
    SequenceStoreType: SequenceStore<AlphabetType>,
>(
    mut writer: impl Write,
    records: impl IntoIterator<Item = &'records FastaRecord<SequenceStoreType::Handle>>,
    store: &SequenceStoreType,
) -> Result<(), IOError>
where
    SequenceStoreType::Handle: 'records,
{
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

impl<Handle> FastaRecord<Handle> {
    /// Transforms the handle into a new type.
    pub fn transform_handle<NewHandle>(
        self,
        transformation: impl FnOnce(Handle) -> NewHandle,
    ) -> FastaRecord<NewHandle> {
        FastaRecord {
            id: self.id,
            comment: self.comment,
            sequence_handle: transformation(self.sequence_handle),
        }
    }
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
        let records = read_fasta(input_file, &mut store, false, false, &[]).unwrap();
        let mut output_file = Vec::new();
        write_fasta(&mut output_file, &records, &store).unwrap();

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
            b">alt1 comment1\nGGTTZGGCCT\n>f2\nACCUTG\n>f3 \nAA\n>seq c2  \ngxT".as_slice();
        let expected_output_file =
            b">alt1 comment1\nGGTTGGCCT\n>f2\nACCTG\n>f3\nAA\n>seq c2\nGT\n".as_slice();

        let mut store = DefaultSequenceStore::<DnaAlphabet>::new();
        let records = read_fasta(input_file, &mut store, true, true, &[]).unwrap();
        let mut output_file = Vec::new();
        write_fasta(&mut output_file, &records, &store).unwrap();

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
