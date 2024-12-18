//! Alphabets for genome sequences.

use std::{
    convert::{TryFrom, TryInto},
    fmt::Display,
};
use thiserror::Error;

/// A character in an alphabet.
pub trait AlphabetCharacter: Into<u8> + Into<char> + TryFrom<u8> + TryFrom<char> + Display {
    /// The amount of characters in the alphabet.
    const ALPHABET_SIZE: usize;

    /// The index of this character in the alphabet.
    fn index(&self) -> usize;

    /// Constructs the character from the given index, returning `Err` if it is invalid.
    fn from_index(index: usize) -> Result<Self, AlphabetError>;

    /// Constructs the character from the given index, returning `Err` if it is invalid.
    /// This method returns a static reference to the character type, so it can only be implemented via lookup in a static table.
    /// It is required to create an implementation of [std::ops::Index] for genome sequence types that do not store the characters in plain format.
    fn from_index_ref(index: usize) -> Result<&'static Self, AlphabetError>;

    /// Constructs the complement of this character.
    fn complement(&self) -> Self;
}

/// An alphabet as a subset of the ASCII alphabet.
pub trait Alphabet: Sized {
    /// The amount of characters in the alphabet.
    const SIZE: usize = Self::CharacterType::ALPHABET_SIZE;

    /// The internal character type used by the alphabet.
    type CharacterType: AlphabetCharacter + Eq + Ord + Clone + 'static;

    /// Converts the given ASCII character into an alphabet character.
    /// If the ASCII character is not mapped to an alphabet character, then `Err` is returned.
    fn ascii_to_character(ascii: u8) -> Result<Self::CharacterType, AlphabetError> {
        ascii
            .try_into()
            .map_err(|_| AlphabetError::AsciiNotPartOfAlphabet { ascii })
    }

    /// Converts this alphabet character into an ASCII character.
    fn character_to_ascii(character: Self::CharacterType) -> u8 {
        character.into()
    }

    /// Returns an iterator over the characters in this alphabet.
    fn iter() -> impl Iterator<Item = Self::CharacterType> {
        (0..Self::SIZE).map(|index| Self::CharacterType::from_index(index).unwrap())
    }
}

/// An error when dealing with alphabets.
#[derive(Debug, Clone, Eq, PartialEq, Error)]
pub enum AlphabetError {
    #[error("found an ASCII character that is not part of the alphabet: {ascii}")]
    /// An ascii character was attempted to convert to an alphabet character, but it is not part of the alphabet.
    AsciiNotPartOfAlphabet {
        /// The offending ascii character.
        ascii: u8,
    },

    #[error("found an index that is not part of the alphabet: {index}")]
    /// An index was attempted to convert to an alphabet character, but it is not part of the alphabet.
    IndexNotPartOfAlphabet {
        /// The offending index.
        index: usize,
    },
}

/// Random distributions over alphabets.
#[cfg(feature = "rand")]
pub mod rand {
    use std::marker::PhantomData;

    use rand::{distributions::Uniform, prelude::Distribution};

    use super::{Alphabet, AlphabetCharacter};

    /// A uniform distribution over a generic alphabet.
    ///
    /// Sampling this distribution yields a random alphabet character, where each character is equally likely.
    pub struct UniformAlphabetDistribution<AlphabetType: Alphabet> {
        phantom_data: PhantomData<AlphabetType>,
    }

    impl<AlphabetType: Alphabet> Distribution<AlphabetType::CharacterType>
        for UniformAlphabetDistribution<AlphabetType>
    {
        fn sample<R: rand::Rng + ?Sized>(&self, rng: &mut R) -> AlphabetType::CharacterType {
            let index_distribution = Uniform::new(0, AlphabetType::SIZE);
            let index = index_distribution.sample(rng);
            AlphabetType::CharacterType::from_index(index).unwrap()
        }
    }

    impl<AlphabetType: Alphabet> Default for UniformAlphabetDistribution<AlphabetType> {
        fn default() -> Self {
            Self {
                phantom_data: Default::default(),
            }
        }
    }
}
