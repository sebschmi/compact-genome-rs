//! The DNA alphabet, consisting of characters A, C, G and T.

use crate::interface::alphabet::{Alphabet, AlphabetCharacter, AlphabetError};
use lazy_static::lazy_static;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

/// A character of a DNA alphabet: A, C, G or T.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DnaCharacter {
    character: u8,
}

/// The DNA alphabet, consisting of characters A, C, G and T.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DnaAlphabet;

static DNA_CHARACTER_TO_ASCII_TABLE: [u8; DnaCharacter::ALPHABET_SIZE] = [b'A', b'C', b'G', b'T'];

impl From<DnaCharacter> for u8 {
    fn from(character: DnaCharacter) -> u8 {
        // Safety: character is private and cannot be constructed out of range.
        unsafe { *DNA_CHARACTER_TO_ASCII_TABLE.get_unchecked(character.character as usize) }
    }
}

impl From<DnaCharacter> for char {
    fn from(character: DnaCharacter) -> Self {
        u8::from(character).into()
    }
}

static ASCII_TO_DNA_CHARACTER_TABLE: [u8; 256] = [
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 0, 4, 1, 4, 4, 4, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
    4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4,
];

impl TryFrom<u8> for DnaCharacter {
    type Error = ();

    fn try_from(ascii: u8) -> Result<Self, Self::Error> {
        // Safety: array covers the whole range of u8.
        let character = unsafe { *ASCII_TO_DNA_CHARACTER_TABLE.get_unchecked(ascii as usize) };
        if character >= Self::ALPHABET_SIZE.try_into().unwrap() {
            Err(())
        } else {
            Ok(Self { character })
        }
    }
}

impl TryFrom<char> for DnaCharacter {
    type Error = ();

    fn try_from(character: char) -> Result<Self, Self::Error> {
        if character.is_ascii() {
            u8::try_from(character).map_err(|_| ())?.try_into()
        } else {
            Err(())
        }
    }
}

static DNA_CHARACTER_COMPLEMENT_TABLE: [u8; DnaCharacter::ALPHABET_SIZE] = [3, 2, 1, 0];

lazy_static! {
    static ref DNA_CHARACTER_TABLE: Vec<DnaCharacter> = {
        (0..DnaCharacter::ALPHABET_SIZE)
            .map(DnaCharacter::from_index)
            .map(Result::unwrap)
            .collect()
    };
}

impl AlphabetCharacter for DnaCharacter {
    const ALPHABET_SIZE: usize = 4;

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
            Ok(&DNA_CHARACTER_TABLE[index])
        } else {
            Err(AlphabetError::IndexNotPartOfAlphabet { index })
        }
    }

    fn complement(&self) -> Self {
        Self {
            // Safety: character is private and cannot be constructed out of range.
            character: unsafe {
                *DNA_CHARACTER_COMPLEMENT_TABLE.get_unchecked(self.character as usize)
            },
        }
    }
}

impl Alphabet for DnaAlphabet {
    type CharacterType = DnaCharacter;
}

impl Display for DnaCharacter {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.character {
                0 => 'A',
                1 => 'C',
                2 => 'G',
                3 => 'T',
                _ => unreachable!("Character is private and cannot be constructed out of range."),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::implementation::alphabets::dna_alphabet::DnaCharacter;
    use std::convert::TryFrom;

    #[test]
    fn test_dna_alphabet_conversion() {
        for ascii in 0u8..=255u8 {
            if ascii == b'A' || ascii == b'C' || ascii == b'G' || ascii == b'T' {
                assert_eq!(
                    u8::from(DnaCharacter::try_from(ascii).unwrap_or_else(|_| panic!(
                        "character {} was expected to be valid, but is not",
                        ascii
                    ))),
                    ascii
                );
            } else {
                assert!(DnaCharacter::try_from(ascii).is_err());
            }
        }
    }

    #[test]
    fn test_display() {
        for character in ['A', 'C', 'G', 'T'] {
            let dna_character = DnaCharacter::try_from(u8::try_from(character).unwrap()).unwrap();
            assert_eq!(format!("{character}"), format!("{dna_character}"));
        }
    }
}
