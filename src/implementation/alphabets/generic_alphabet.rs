//! A generic alphabet that can be configured to support any subset of ASCII codes from 0 to including 127.

use ref_cast::RefCast;

use crate::interface::alphabet::{Alphabet, AlphabetCharacter, AlphabetError};

use std::convert::TryFrom;
use std::fmt::{Debug, Display, Formatter};
use std::hash::Hash;
use std::marker::PhantomData;

const fn generate_ascii_to_character_lookup_table(character_to_ascii: &[u8]) -> [u8; 256] {
    assert!(character_to_ascii.len() < (u8::MAX as usize));
    let mut result = [u8::MAX; 256];

    let mut character = 0;
    while character < character_to_ascii.len() {
        let ascii = character_to_ascii[character];
        assert!(result[ascii as usize] == u8::MAX);
        result[ascii as usize] = character as u8;
        character += 1;
    }

    result
}

const fn generate_character_to_complement_character_lookup_table(
    character_to_comp_ascii: &[u8],
    ascii_to_character: &[u8; 256],
) -> [u8; 256] {
    assert!(character_to_comp_ascii.len() < (u8::MAX as usize));

    let mut result = [u8::MAX; 256];

    let mut character = 0;
    while character < character_to_comp_ascii.len() {
        let comp_ascii = character_to_comp_ascii[character];
        let comp_character = ascii_to_character[comp_ascii as usize];
        result[character] = comp_character;
        character += 1;
    }

    result
}

const U8_TABLE: [u8; 256] = const {
    let mut result = [0; 256];

    let mut index = 0;
    while index < 256 {
        result[index] = index as u8;
        index += 1;
    }

    result
};

/// The translation table between internal character indices and ASCII characters.
///
/// This trait should be implemented by specifying only the required constants.
///
/// ## Example
///
/// Note how the each character has its complement written directly below it.
///
/// ```rust
/// use compact_genome::implementation::alphabets::generic_alphabet::CharacterFromToAsciiTable;
///
/// struct DnaCharacterFromToAsciiTable;
///
/// impl CharacterFromToAsciiTable for DnaCharacterFromToAsciiTable {
///     const CHARACTER_TO_ASCII: &[u8] = b"ACGT";
///     const CHAR_TO_COMP_ASCII: &[u8] = b"TGCA";
/// }
/// ```
pub trait CharacterFromToAsciiTable: 'static {
    /// Convert internal character indices into ASCII characters.
    const CHARACTER_TO_ASCII: &[u8];
    /// Convert internal character indices into their complement ASCII characters.
    const CHAR_TO_COMP_ASCII: &[u8];

    /// The size of the alphabet.
    /// Automatically determined by the required constants.
    const ALPHABET_SIZE: u8 = Self::CHARACTER_TO_ASCII.len() as u8;

    /// Convert ASCII characters into internal character indices.
    /// Automatically determined by the required constants.
    const ASCII_TO_CHARACTER: [u8; 256] =
        generate_ascii_to_character_lookup_table(Self::CHARACTER_TO_ASCII);

    /// Convert internal character indices into their complement indices.
    /// Automatically determined by the required constants.
    // We cannot choose the length of this based on the alphabet size yet.
    const CHARACTER_TO_COMPLEMENT_CHARACTER: [u8; 256] =
        generate_character_to_complement_character_lookup_table(
            Self::CHAR_TO_COMP_ASCII,
            &Self::ASCII_TO_CHARACTER,
        );

    /// Convert an ASCII character into an internal character index.
    fn ascii_to_character(ascii: u8) -> Option<u8> {
        Some(Self::ASCII_TO_CHARACTER[usize::from(ascii)]).filter(|&character| character != u8::MAX)
    }

    /// Convert an internal character index into its borrowed representation.
    fn character_to_ref(character: u8) -> &'static u8 {
        &U8_TABLE[usize::from(character)]
    }

    /// Convert an internal character index into an ASCII character.
    fn character_to_ascii(character: u8) -> u8 {
        Self::CHARACTER_TO_ASCII[usize::from(character)]
    }

    /// Convert an internal character index into its complement index.
    fn character_to_complement(character: u8) -> u8 {
        Self::CHARACTER_TO_COMPLEMENT_CHARACTER[usize::from(character)]
    }
}

/// A character of a generic alphabet.
#[derive(RefCast)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(transparent)]
pub struct GenericCharacter<Table: CharacterFromToAsciiTable> {
    character: u8,
    phantom_data: PhantomData<Table>,
}

/// A generic alphabet.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GenericAlphabet<Table: CharacterFromToAsciiTable> {
    phantom_data: PhantomData<Table>,
}

impl<Table: CharacterFromToAsciiTable> From<GenericCharacter<Table>> for u8 {
    fn from(character: GenericCharacter<Table>) -> u8 {
        Table::character_to_ascii(character.character)
    }
}

impl<Table: CharacterFromToAsciiTable> From<GenericCharacter<Table>> for char {
    fn from(character: GenericCharacter<Table>) -> Self {
        u8::from(character).into()
    }
}

impl<Table: CharacterFromToAsciiTable> TryFrom<u8> for GenericCharacter<Table> {
    type Error = ();

    fn try_from(ascii: u8) -> Result<Self, Self::Error> {
        if let Some(character) = Table::ascii_to_character(ascii) {
            Ok(Self {
                character,
                phantom_data: PhantomData,
            })
        } else {
            Err(())
        }
    }
}

impl<Table: CharacterFromToAsciiTable> TryFrom<char> for GenericCharacter<Table> {
    type Error = ();

    fn try_from(character: char) -> Result<Self, Self::Error> {
        u8::try_from(character).map_err(|_| ())?.try_into()
    }
}

impl<Table: CharacterFromToAsciiTable> AlphabetCharacter for GenericCharacter<Table> {
    const ALPHABET_SIZE: u8 = Table::ALPHABET_SIZE;

    fn index(&self) -> u8 {
        self.character
    }

    fn from_index(index: u8) -> Result<Self, AlphabetError> {
        if index < Self::ALPHABET_SIZE {
            Ok(Self {
                character: index,
                phantom_data: PhantomData,
            })
        } else {
            Err(AlphabetError::IndexNotPartOfAlphabet { index })
        }
    }

    fn from_index_ref(index: u8) -> Result<&'static Self, AlphabetError> {
        if index < Self::ALPHABET_SIZE {
            Ok(Self::ref_cast(Table::character_to_ref(index)))
        } else {
            Err(AlphabetError::IndexNotPartOfAlphabet { index })
        }
    }

    fn complement(&self) -> Self {
        Self {
            character: Table::character_to_complement(self.character),
            phantom_data: PhantomData,
        }
    }
}

impl<Table: CharacterFromToAsciiTable> Alphabet for GenericAlphabet<Table> {
    type CharacterType = GenericCharacter<Table>;
}

impl<Table: CharacterFromToAsciiTable> Display for GenericCharacter<Table> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", char::from(*self))
    }
}

impl<Table: CharacterFromToAsciiTable> Debug for GenericCharacter<Table> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GenericCharacter")
            .field("character", &self.character)
            .finish()
    }
}

impl<Table: CharacterFromToAsciiTable> Clone for GenericCharacter<Table> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<Table: CharacterFromToAsciiTable> Copy for GenericCharacter<Table> {}

impl<Table: CharacterFromToAsciiTable> Eq for GenericCharacter<Table> {}

impl<Table: CharacterFromToAsciiTable> PartialEq for GenericCharacter<Table> {
    fn eq(&self, other: &Self) -> bool {
        self.character == other.character
    }
}

impl<Table: CharacterFromToAsciiTable> Ord for GenericCharacter<Table> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.character.cmp(&other.character)
    }
}

impl<Table: CharacterFromToAsciiTable> PartialOrd for GenericCharacter<Table> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<Table: CharacterFromToAsciiTable> Hash for GenericCharacter<Table> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.character.hash(state);
        self.phantom_data.hash(state);
    }
}

/// Generate a custom alphabet implementation.
///
/// ## Example
///
/// We want to implement a custom alphabet that is commonly referred to as "DNA".
/// The name of the alphabet type shall be `DnaAlphabet`, and the name of its character type shall be `DnaCharacter`.
/// The characters are `ACGT` and their respecive complements are `TGCA` (A and T are complements and C and G are complements).
///
/// ```rust
/// compact_genome::impl_generic_alphabet!("DNA alphabet", DnaAlphabet, DnaCharacter, b"ACGT", b"TGCA");
/// ```
#[macro_export]
macro_rules! impl_generic_alphabet {
    ($name:literal, $alphabet:ident, $character:ident, $character_to_ascii:literal, $char_to_comp_ascii:literal $(,)?) => {
        #[doc = concat!("The translation table between internal ", $name, " character indices and ASCII characters.")]
        pub struct DnaCharacterFromToAsciiTable;

        impl $crate::implementation::alphabets::generic_alphabet::CharacterFromToAsciiTable
            for DnaCharacterFromToAsciiTable
        {
            const CHARACTER_TO_ASCII: &[u8] = $character_to_ascii;
            const CHAR_TO_COMP_ASCII: &[u8] = $char_to_comp_ascii;
        }

        #[doc = concat!("A character of a ", $name, ": ", stringify!($character_to_ascii), ".")]
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, ref_cast::RefCast)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[repr(transparent)]
        pub struct $character(
            $crate::implementation::alphabets::generic_alphabet::GenericCharacter<
                DnaCharacterFromToAsciiTable,
            >,
        );

        #[doc = concat!("The ", $name, ", consisting of characters ", stringify!($character_to_ascii), ".")]
        #[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Default)]
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        pub struct $alphabet;

        impl From<$character> for u8 {
            fn from(character: $character) -> u8 {
                character.0.into()
            }
        }

        impl From<$character> for char {
            fn from(character: $character) -> Self {
                u8::from(character).into()
            }
        }

        impl TryFrom<u8> for $character {
            type Error = ();

            fn try_from(ascii: u8) -> Result<Self, Self::Error> {
                Ok($character(ascii.try_into()?))
            }
        }

        impl TryFrom<char> for $character {
            type Error = ();

            fn try_from(character: char) -> Result<Self, Self::Error> {
                Ok($character(character.try_into()?))
            }
        }

        impl $crate::interface::alphabet::AlphabetCharacter for $character {
            const ALPHABET_SIZE: u8 = 4;

            fn index(&self) -> u8 {
                self.0.index()
            }

            fn from_index(index: u8) -> Result<Self, $crate::interface::alphabet::AlphabetError> {
                Ok(Self(
                    $crate::interface::alphabet::AlphabetCharacter::from_index(index)?,
                ))
            }

            fn from_index_ref(
                index: u8,
            ) -> Result<&'static Self, $crate::interface::alphabet::AlphabetError> {
                Ok(ref_cast::RefCast::ref_cast(
                    $crate::interface::alphabet::AlphabetCharacter::from_index_ref(index)?,
                ))
            }

            fn complement(&self) -> Self {
                Self(self.0.complement())
            }
        }

        impl $crate::interface::alphabet::Alphabet for $alphabet {
            type CharacterType = $character;
        }

        impl std::fmt::Display for $character {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }
    };
}
