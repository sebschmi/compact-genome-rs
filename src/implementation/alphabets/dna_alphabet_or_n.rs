//! The DNA alphabet including N, consisting of characters A, C, G, N and T.

use crate::interface::alphabet::{Alphabet, AlphabetCharacter, AlphabetError};
use lazy_static::lazy_static;
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};

/// A character of a DNA alphabet or N: A, C, G, N or T.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DnaCharacterOrN {
    character: u8,
}

/// The DNA alphabet, consisting of characters A, C, G and T, or N.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DnaAlphabetOrN;

static DNA_CHARACTER_OR_N_TO_ASCII_TABLE: [u8; DnaAlphabetOrN::SIZE] =
    [b'A', b'C', b'G', b'N', b'T'];

impl From<DnaCharacterOrN> for u8 {
    fn from(character: DnaCharacterOrN) -> u8 {
        // Safety: character is private and cannot be constructed out of range.
        unsafe { *DNA_CHARACTER_OR_N_TO_ASCII_TABLE.get_unchecked(character.character as usize) }
    }
}

impl From<DnaCharacterOrN> for char {
    fn from(character: DnaCharacterOrN) -> Self {
        u8::from(character).into()
    }
}

static ASCII_TO_DNA_CHARACTER_OR_N_TABLE: [u8; 256] = [
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 0, 5, 1, 5, 5, 5, 2, 5, 5, 5, 5, 5, 5, 3, 5, 5, 5, 5, 5, 4, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
];

impl TryFrom<u8> for DnaCharacterOrN {
    type Error = ();

    fn try_from(ascii: u8) -> Result<Self, Self::Error> {
        // Safety: array covers the whole range of u8.
        let character = unsafe { *ASCII_TO_DNA_CHARACTER_OR_N_TABLE.get_unchecked(ascii as usize) };
        if character >= Self::ALPHABET_SIZE.try_into().unwrap() {
            Err(())
        } else {
            Ok(Self { character })
        }
    }
}

impl TryFrom<char> for DnaCharacterOrN {
    type Error = ();

    fn try_from(character: char) -> Result<Self, Self::Error> {
        if character.is_ascii() {
            u8::try_from(character).map_err(|_| ())?.try_into()
        } else {
            Err(())
        }
    }
}

static DNA_CHARACTER_OR_N_COMPLEMENT_TABLE: [u8; DnaCharacterOrN::ALPHABET_SIZE] = [4, 2, 1, 3, 0];

lazy_static! {
    static ref DNA_CHARACTER_OR_N_TABLE: Vec<DnaCharacterOrN> = {
        (0..DnaCharacterOrN::ALPHABET_SIZE)
            .map(DnaCharacterOrN::from_index)
            .map(Result::unwrap)
            .collect()
    };
}

impl AlphabetCharacter for DnaCharacterOrN {
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
            Ok(&DNA_CHARACTER_OR_N_TABLE[index])
        } else {
            Err(AlphabetError::IndexNotPartOfAlphabet { index })
        }
    }

    fn complement(&self) -> Self {
        Self {
            // Safety: character is private and cannot be constructed out of range.
            character: unsafe {
                *DNA_CHARACTER_OR_N_COMPLEMENT_TABLE.get_unchecked(self.character as usize)
            },
        }
    }
}

impl Alphabet for DnaAlphabetOrN {
    type CharacterType = DnaCharacterOrN;
}

impl Display for DnaCharacterOrN {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.character {
                0 => 'A',
                1 => 'C',
                2 => 'G',
                3 => 'N',
                4 => 'T',
                _ => unreachable!("Character is private and cannot be constructed out of range."),
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::implementation::alphabets::dna_alphabet_or_n::DnaCharacterOrN;
    use std::convert::TryFrom;

    #[test]
    fn test_dna_alphabet_conversion() {
        for ascii in 0u8..=255u8 {
            if ascii == b'A' || ascii == b'C' || ascii == b'G' || ascii == b'N' || ascii == b'T' {
                assert_eq!(
                    u8::from(DnaCharacterOrN::try_from(ascii).unwrap_or_else(|_| panic!(
                        "character {} was expected to be valid, but is not",
                        ascii
                    ))),
                    ascii
                );
            } else {
                assert!(DnaCharacterOrN::try_from(ascii).is_err());
            }
        }
    }

    #[test]
    fn test_display() {
        for character in ['A', 'C', 'G', 'N', 'T'] {
            let dna_character =
                DnaCharacterOrN::try_from(u8::try_from(character).unwrap()).unwrap();
            assert_eq!(format!("{character}"), format!("{dna_character}"));
        }
    }
}
