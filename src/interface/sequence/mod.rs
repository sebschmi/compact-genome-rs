//! Traits for genome sequences.

use crate::interface::alphabet::{Alphabet, AlphabetCharacter, AlphabetError};
use crate::interface::k_mer::OwnedKmer;
use std::cmp::Ordering;
use std::iter;
use std::iter::{FromIterator, Map, Repeat, Rev, Zip};
use std::ops::Range;
use traitsequence::interface::{EditableSequence, OwnedSequence, Sequence, SequenceMut};

pub mod neighbor_iterators;

/// An iterator over the reverse complement of a genome sequence.
pub type ReverseComplementIterator<I, AlphabetType> = Map<
    Rev<I>,
    for<'c> fn(
        &'c <AlphabetType as Alphabet>::CharacterType,
    ) -> <AlphabetType as Alphabet>::CharacterType,
>;

/// An iterator over the cloned k-mers of a genome sequence.
pub type OwnedKmerIterator<'a, GenomeSequenceType, KmerType> = Map<
    Zip<Range<usize>, Repeat<&'a GenomeSequenceType>>,
    fn((usize, &'a GenomeSequenceType)) -> KmerType,
>;

/// A genome sequence.
pub trait GenomeSequence<
    AlphabetType: Alphabet,
    GenomeSubsequence: GenomeSequence<AlphabetType, GenomeSubsequence> + ?Sized,
>: Sequence<AlphabetType::CharacterType, GenomeSubsequence>
{
    /// Returns true if this genome is valid, i.e. it contains no invalid characters.
    fn is_valid(&self) -> bool {
        true
    }

    /// Copies this genome string into a `Vec`.
    fn clone_as_vec(&self) -> Vec<u8> {
        self.iter()
            .cloned()
            .map(AlphabetType::character_to_ascii)
            .collect()
    }

    /// Get a reference to this genome as its subsequence type.
    fn as_genome_subsequence(&self) -> &GenomeSubsequence {
        self.index(0..self.len())
    }

    /// Returns the genome as nucleotide string.
    fn as_string(&self) -> String {
        String::from_utf8(self.clone_as_vec())
            .expect("Genome contains non-utf8 characters (It should be ASCII only).")
    }

    /// Returns an iterator over the reverse complement of this genome.
    /// Panics if the iterator his an invalid character (see [not valid](GenomeSequence::is_valid)).
    fn reverse_complement_iter(
        &self,
    ) -> ReverseComplementIterator<Self::Iterator<'_>, AlphabetType> {
        self.iter()
            .rev()
            .map(AlphabetType::CharacterType::complement)
    }

    /// Returns an iterator over the k-mers of this genome.
    /// The k-mers are cloned from this genome.
    fn cloned_k_mer_iter<
        const K: usize,
        KmerType: OwnedKmer<K, AlphabetType, GenomeSubsequence>,
    >(
        &self,
    ) -> OwnedKmerIterator<'_, Self, KmerType> {
        (0..self.len() - K + 1)
            .zip(iter::repeat(self))
            .map(|(offset, source_genome)| {
                source_genome.iter().skip(offset).take(K).cloned().collect()
            })
    }

    /// Returns an owned copy of the reverse complement of this genome.
    /// Panics if this genome is [not valid](GenomeSequence::is_valid).
    fn convert_with_reverse_complement<
        ReverseComplementSequence: OwnedGenomeSequence<AlphabetType, ReverseComplementSubsequence>,
        ReverseComplementSubsequence: GenomeSequence<AlphabetType, ReverseComplementSubsequence> + ?Sized,
    >(
        &self,
    ) -> ReverseComplementSequence {
        self.reverse_complement_iter().collect()
    }

    /// Returns an owned copy of this genome.
    fn convert<
        ResultSequence: OwnedGenomeSequence<AlphabetType, ResultSubsequence>,
        ResultSubsequence: GenomeSequence<AlphabetType, ResultSubsequence> + ?Sized,
    >(
        &self,
    ) -> ResultSequence {
        self.iter().cloned().collect()
    }

    /// Returns true if the genome is canonical.
    /// A canonical genome is lexicographically smaller or equal to its reverse complement.
    fn is_canonical(&self) -> bool {
        for (forward_character, reverse_character) in
            self.iter().cloned().zip(self.reverse_complement_iter())
        {
            match forward_character.cmp(&reverse_character) {
                Ordering::Less => return true,
                Ordering::Greater => return false,
                _ => {}
            }
        }
        true
    }

    /// Returns true if the genome is self-complemental.
    /// A self-complemental genome is equivalent to its reverse complement.
    fn is_self_complemental(&self) -> bool {
        self.iter().cloned().eq(self.reverse_complement_iter())
    }
}

/// A genome sequence that is owned, i.e. not a reference.
pub trait OwnedGenomeSequence<
    AlphabetType: Alphabet,
    GenomeSubsequence: GenomeSequence<AlphabetType, GenomeSubsequence> + ?Sized,
>:
    GenomeSequence<AlphabetType, GenomeSubsequence>
    + FromIterator<AlphabetType::CharacterType>
    + OwnedSequence<AlphabetType::CharacterType, GenomeSubsequence>
{
    /// Returns the reverse complement of this genome.
    /// Panics if this genome is [not valid](GenomeSequence::is_valid).
    fn clone_as_reverse_complement(&self) -> Self {
        self.reverse_complement_iter().collect()
    }

    /// Constructs an owned genome sequence from an `IntoIter` over ASCII characters.
    /// If any character is not part of the alphabet, then `None` is returned.
    fn from_iter_u8<T: IntoIterator<Item = u8>>(iter: T) -> Result<Self, AlphabetError> {
        iter.into_iter()
            .map(AlphabetType::ascii_to_character)
            .collect()
    }

    /// Constructs an owned genome sequence from a slice of ASCII characters.
    /// If any character is not part of the alphabet, then `None` is returned.
    fn from_slice_u8(slice: &[u8]) -> Result<Self, AlphabetError> {
        Self::from_iter_u8(slice.iter().copied())
    }
}

/// A mutable genome sequence.
pub trait GenomeSequenceMut<
    AlphabetType: Alphabet,
    GenomeSubsequenceMut: GenomeSequenceMut<AlphabetType, GenomeSubsequenceMut> + ?Sized,
>:
    SequenceMut<AlphabetType::CharacterType, GenomeSubsequenceMut>
    + GenomeSequence<AlphabetType, GenomeSubsequenceMut>
{
    /// Get a reference to this genome as its subsequence type.
    fn as_genome_subsequence_mut(&mut self) -> &mut GenomeSubsequenceMut {
        self.index_mut(0..self.len())
    }
}

type IntoIterU8<SourceType, AlphabetType> = Map<
    <SourceType as IntoIterator>::IntoIter,
    fn(<AlphabetType as Alphabet>::CharacterType) -> u8,
>;

/// An editable genome sequence.
pub trait EditableGenomeSequence<
    AlphabetType: Alphabet,
    GenomeSubsequence: GenomeSequence<AlphabetType, GenomeSubsequence> + ?Sized,
>:
    EditableSequence<AlphabetType::CharacterType, GenomeSubsequence>
    + GenomeSequence<AlphabetType, GenomeSubsequence>
{
    /// Replace the character at the given index with the given character.
    fn set(&mut self, index: usize, character: AlphabetType::CharacterType);

    /// Converts this genome sequence into an iterator over ASCII characters.
    fn into_iter_u8(self) -> IntoIterU8<Self, AlphabetType> {
        self.into_iter().map(AlphabetType::character_to_ascii)
    }

    /// Extends this genome from a sequence of ASCII characters.
    fn extend_from_iter_u8<IteratorType: IntoIterator<Item = u8>>(
        &mut self,
        iter: IteratorType,
    ) -> Result<(), AlphabetError> {
        let original_len = self.len();
        let iter = iter.into_iter();
        let (size, _) = iter.size_hint();
        self.reserve(size);
        for item in iter {
            match AlphabetType::ascii_to_character(item) {
                Ok(character) => self.push(character),
                Err(error) => {
                    self.resize(
                        original_len,
                        AlphabetType::CharacterType::from_index(0).unwrap(),
                    );
                    return Err(error);
                }
            }
        }

        Ok(())
    }

    /// Extends this genome from a sequence of ASCII characters.
    fn extend_from_slice_u8(&mut self, slice: &[u8]) -> Result<(), AlphabetError> {
        self.extend_from_iter_u8(slice.iter().copied())
    }

    /// Reserve memory for at least `additional` items.
    fn reserve(&mut self, additional: usize);

    /// Resize to contain the given number of items.
    /// Empty spaces are filled with the given default item.
    fn resize(&mut self, len: usize, default: AlphabetType::CharacterType);

    /// Insert the given character at the end of the genome sequence.
    fn push(&mut self, character: AlphabetType::CharacterType);

    /// Delete the characters in the specified sequence index range.
    fn delete(&mut self, range: Range<usize>) {
        assert!(range.end <= self.len());
        if range.start >= range.end {
            assert_eq!(range.start, range.end);
        } else {
            for index in range.start..self.len() - range.len() {
                self.set(index, self[index + range.len()].clone());
            }
            self.resize(
                self.len() - range.len(),
                AlphabetType::iter().next().unwrap(),
            );
        }
    }

    /// Insert a repeat at `target` that consists of the characters in `source_range`.
    fn insert_repeat(&mut self, source_range: Range<usize>, target: usize) {
        assert!(source_range.end <= self.len());
        if source_range.start >= source_range.end {
            assert_eq!(source_range.start, source_range.end);
        } else {
            self.resize(
                self.len() + source_range.len(),
                AlphabetType::CharacterType::from_index(0).unwrap(),
            );
            for index in (target + source_range.len()..self.len() + source_range.len()).rev() {
                self.set(index, self[index - source_range.len()].clone());
            }
            for index in 0..source_range.len() {
                if index + source_range.start < target {
                    self.set(index + target, self[index + source_range.start].clone());
                } else {
                    self.set(index + target, self[index + source_range.end].clone());
                }
            }
        }
    }
}
