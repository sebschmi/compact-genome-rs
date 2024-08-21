//! The error type for sequence IO.

use crate::interface::alphabet::AlphabetError;

/// An error when performing sequence IO.
#[derive(Debug, thiserror::Error)]
pub enum IOError {
    /// An error regarding the sequence alphabet.
    #[error("Alphabet error: {0}")]
    AlphabetError(#[from] AlphabetError),

    /// An error during IO.
    #[error("IO error: {0}")]
    IOError(#[from] std::io::Error),
}
