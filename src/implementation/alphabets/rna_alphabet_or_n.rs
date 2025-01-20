//! The RNA alphabet including N, consisting of characters A, C, G, U, and N.

use crate::interface::alphabet::{Alphabet, AlphabetCharacter, AlphabetError};
use lazy_static::lazy_static;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

/// A character of a RNA alphabet or N: A, C, G, U or N.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RnaCharacterOrN {
    character: u8,
}

/// The RNA alphabet, consisting of characters A, C, G and U, or N.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RnaAlphabetOrN;

static RNA_CHARACTER_OR_N_TO_ASCII_TABLE: [u8; RnaAlphabetOrN::SIZE] =
    [b'A', b'C', b'G', b'N', b'U'];

impl From<RnaCharacterOrN> for u8 {
    fn from(character: RnaCharacterOrN) -> u8 {
        // Safety: character is private and cannot be constructed out of range.
        unsafe { *RNA_CHARACTER_OR_N_TO_ASCII_TABLE.get_unchecked(character.character as usize) }
    }
}

impl From<RnaCharacterOrN> for char {
    fn from(character: RnaCharacterOrN) -> Self {
        u8::from(character).into()
    }
}

static ASCII_TO_RNA_CHARACTER_OR_N_TABLE: [u8; 256] = [
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 0, 5, 1, 5, 5, 5, 2, 5, 5, 5, 5, 5, 5, 3, 5, 5, 5, 5, 5, 5, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
];

impl TryFrom<u8> for RnaCharacterOrN {
    type Error = ();

    fn try_from(ascii: u8) -> Result<Self, Self::Error> {
        // Safety: array covers the whole range of u8.
        let character = unsafe { *ASCII_TO_RNA_CHARACTER_OR_N_TABLE.get_unchecked(ascii as usize) };
        if character >= Self::ALPHABET_SIZE.try_into().unwrap() {
            Err(())
        } else {
            Ok(Self { character })
        }
    }
}

impl TryFrom<char> for RnaCharacterOrN {
    type Error = ();

    fn try_from(character: char) -> Result<Self, Self::Error> {
        if character.is_ascii() {
            u8::try_from(character).map_err(|_| ())?.try_into()
        } else {
            Err(())
        }
    }
}

static RNA_CHARACTER_OR_N_COMPLEMENT_TABLE: [u8; RnaCharacterOrN::ALPHABET_SIZE] = [4, 2, 1, 3, 0];

lazy_static! {
    static ref RNA_CHARACTER_OR_N_TABLE: Vec<RnaCharacterOrN> = {
        (0..RnaCharacterOrN::ALPHABET_SIZE)
            .map(RnaCharacterOrN::from_index)
            .map(Result::unwrap)
            .collect()
    };
}

impl AlphabetCharacter for RnaCharacterOrN {
    const ALPHABET_SIZE: usize = 5;

    fn index(&self) -> usize {
        self.character as usize
    }

    fn from_index(index: usize) -> Result<Self, AlphabetError> {
        if index < Self::ALPHABET_SIZE {
            Ok(Self {
                character: index as u8,
            })
        } else {
            Err(AlphabetError::IndexNotPartOfAlphabet { index })
        }
    }

    fn from_index_ref(index: usize) -> Result<&'static Self, AlphabetError> {
        if index < Self::ALPHABET_SIZE {
            Ok(&RNA_CHARACTER_OR_N_TABLE[index])
        } else {
            Err(AlphabetError::IndexNotPartOfAlphabet { index })
        }
    }

    fn complement(&self) -> Self {
        Self {
            // Safety: character is private and cannot be constructed out of range.
            character: unsafe {
                *RNA_CHARACTER_OR_N_COMPLEMENT_TABLE.get_unchecked(self.character as usize)
            },
        }
    }
}

impl Alphabet for RnaAlphabetOrN {
    type CharacterType = RnaCharacterOrN;
}

impl Display for RnaCharacterOrN {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.character {
                0 => 'A',
                1 => 'C',
                2 => 'G',
                3 => 'N',
                4 => 'U',
                _ => unreachable!("Character is private and cannot be constructed out of range."),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::implementation::alphabets::rna_alphabet_or_n::RnaCharacterOrN;
    use std::convert::TryFrom;

    #[test]
    fn test_rna_alphabet_conversion() {
        for ascii in 0u8..=255u8 {
            if ascii == b'A' || ascii == b'C' || ascii == b'G' || ascii == b'N' || ascii == b'U' {
                assert_eq!(
                    u8::from(RnaCharacterOrN::try_from(ascii).unwrap_or_else(|_| panic!(
                        "character {} was expected to be valid, but is not",
                        ascii
                    ))),
                    ascii
                );
            } else {
                assert!(RnaCharacterOrN::try_from(ascii).is_err());
            }
        }
    }

    #[test]
    fn test_display() {
        for character in ['A', 'C', 'G', 'N', 'U'] {
            let rna_character =
                RnaCharacterOrN::try_from(u8::try_from(character).unwrap()).unwrap();
            assert_eq!(format!("{character}"), format!("{rna_character}"));
        }
    }
}
