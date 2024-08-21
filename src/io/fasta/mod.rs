//! Sequence IO in fasta format.

use std::{fs::File, io::Read, path::Path};

use crate::{
    interface::{alphabet::Alphabet, sequence_store::SequenceStore},
    io::{peekable_reader::PeekableReader, unzip_if_zipped},
};

use super::{error::IOError, ZipFormat};

/// A fasta record.
pub struct FastaRecord<Handle> {
    /// The id of the fasta record.
    pub id: String,
    /// The handle to the sequence of the fasta record.
    pub sequence_handle: Handle,
}

/// Read a fasta file into the given sequence store.
pub fn read_fasta_file<AlphabetType: Alphabet, SequenceStoreType: SequenceStore<AlphabetType>>(
    path: impl AsRef<Path>,
    store: impl AsMut<SequenceStoreType>,
) -> Result<(), IOError> {
    let zip_format_hint = ZipFormat::from_path_name(&path);
    let file = File::open(path)?;

    unzip_if_zipped(file, zip_format_hint, |reader| read_fasta(reader, store))
}

/// Read fasta data into the given sequence store.
/// The reader should be buffered for performance.
pub fn read_fasta<AlphabetType: Alphabet, SequenceStoreType: SequenceStore<AlphabetType>>(
    reader: impl Read,
    _store: impl AsMut<SequenceStoreType>,
) -> Result<(), IOError> {
    enum State {
        Init,
        RecordId,
    }

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
                            return Ok(());
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
            State::RecordId => todo!(),
        }
    }
}
