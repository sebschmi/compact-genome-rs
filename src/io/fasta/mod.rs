//! Sequence IO in fasta format.

use std::{fs::File, io::Read, mem, path::Path};

use crate::{
    interface::{alphabet::Alphabet, sequence_store::SequenceStore},
    io::{peekable_reader::PeekableReader, unzip_if_zipped},
};

use super::{error::IOError, ZipFormat};

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
    store: impl AsMut<SequenceStoreType>,
) -> Result<Vec<FastaRecord<SequenceStoreType::Handle>>, IOError> {
    let zip_format_hint = ZipFormat::from_path_name(&path);
    let file = File::open(path)?;

    unzip_if_zipped(file, zip_format_hint, |reader| read_fasta(reader, store))
}

/// Read fasta data into the given sequence store.
/// The reader should be buffered for performance.
pub fn read_fasta<
    Reader: Read,
    AlphabetType: Alphabet,
    SequenceStoreType: SequenceStore<AlphabetType>,
>(
    reader: Reader,
    mut store: impl AsMut<SequenceStoreType>,
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

    let store = store.as_mut();
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

                    if buffer[0].is_ascii_whitespace() {
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

                    if !buffer[0].is_ascii_whitespace() {
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
                };

                record_sequence_handle = Some(store.add_from_iter_u8(&mut iterator)?);
                state = State::EndOfRecord {
                    has_more_records: iterator.result.unwrap()?,
                };
            }
            State::EndOfRecord { has_more_records } => {
                let sequence_handle = record_sequence_handle
                    .take()
                    .unwrap_or_else(|| store.add_from_slice_u8(&[]).unwrap());
                records.push(FastaRecord {
                    id: mem::take(&mut record_id),
                    comment: mem::take(&mut record_comment),
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

struct FastaSequenceIterator<'a, Reader> {
    reader: &'a mut Reader,
    buffer: [u8; 1],
    /// Holds Some() on termination.
    /// Is Err() if an error occurred, and Ok() otherwise.
    /// The bool is true if there are more records.
    result: Option<Result<bool, IOError>>,
    newline: bool,
}

impl<'a, Reader: Read> Iterator for FastaSequenceIterator<'a, Reader> {
    type Item = u8;

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
                return Some(self.buffer[0]);
            } else {
                self.newline = true;
            }
        }
    }
}
