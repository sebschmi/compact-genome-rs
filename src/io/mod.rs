//! Various methods of inputting and outputting sequences.

use std::io::{BufReader, Read, Seek};

use error::IOError;
use flate2::bufread::GzDecoder;
use itertools::Itertools;

pub mod error;
pub mod fasta;
pub mod peekable_reader;

#[derive(Debug, Clone, Copy, PartialEq, Eq, enum_iterator::Sequence)]
enum ZipFormat {
    None,
    Gzip,
}

/// Wrapper around a parsing function to handle a zipped stream.
///
/// The `reader` should not be buffered, as buffering will be added by this method.
fn unzip_if_zipped(
    mut reader: impl Read + Seek,
    zip_format_hint: ZipFormat,
    parse_function: impl FnOnce(&mut dyn Read) -> Result<(), IOError>,
) -> Result<(), IOError> {
    // Try the formats in the following order:
    // * the hinted format first
    // * then all the formats that have headers, i.e. all zip formats
    // * finally, try plain text
    let formats = [zip_format_hint]
        .into_iter()
        .chain(
            enum_iterator::all::<ZipFormat>()
                .filter(|zip_format| {
                    *zip_format != zip_format_hint && *zip_format != ZipFormat::None
                })
                .chain([ZipFormat::None]),
        )
        .collect_vec();

    for format in formats {
        match format {
            ZipFormat::None => return parse_function(&mut BufReader::new(reader)),
            ZipFormat::Gzip => {
                let mut decoder = GzDecoder::new(BufReader::new(reader));
                // Check if this file can be parsed as gz.
                // TODO this method of checking falsely returns None if the given reader blocks.
                if decoder.header().is_some() {
                    return parse_function(&mut decoder);
                } else {
                    reader = decoder.into_inner().into_inner();
                    reader.seek(std::io::SeekFrom::Start(0))?;
                }
            }
        }
    }

    unreachable!("formats vec always contains the None format")
}

impl ZipFormat {
    fn from_path_name(path: impl AsRef<std::path::Path>) -> Self {
        let Some(extension) = path
            .as_ref()
            .extension()
            .map(|extension| extension.to_string_lossy())
        else {
            return Self::None;
        };

        match extension.as_ref() {
            "gz" | "gzip" => Self::Gzip,
            _ => Self::None,
        }
    }
}
