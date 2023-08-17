//! A simple representation of a k-mer as an array.

use crate::implementation::vec_sequence::SliceSubGenome;
use crate::interface::alphabet::Alphabet;
use crate::interface::k_mer::{Kmer, OwnedKmer};
use crate::interface::sequence::{GenomeSequence, GenomeSequenceMut, OwnedGenomeSequence};
use ref_cast::RefCast;
use std::ops::{Index, IndexMut, Range};
use traitsequence::interface::{OwnedSequence, Sequence, SequenceMut};

/// A k-mer stored as array of plain characters.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct ArrayKmer<const K: usize, AlphabetType: Alphabet> {
    array: [AlphabetType::CharacterType; K],
}

impl<const K: usize, AlphabetType: Alphabet> Kmer<K, AlphabetType, SliceSubGenome<AlphabetType>>
    for SliceSubGenome<AlphabetType>
{
}

impl<const K: usize, AlphabetType: Alphabet>
    OwnedKmer<K, AlphabetType, SliceSubGenome<AlphabetType>> for ArrayKmer<K, AlphabetType>
{
}

impl<const K: usize, AlphabetType: Alphabet>
    GenomeSequence<AlphabetType, SliceSubGenome<AlphabetType>> for ArrayKmer<K, AlphabetType>
{
    fn as_genome_subsequence(&self) -> &SliceSubGenome<AlphabetType> {
        SliceSubGenome::ref_cast(&self.array[..])
    }
}

impl<const K: usize, AlphabetType: Alphabet>
    Sequence<AlphabetType::CharacterType, SliceSubGenome<AlphabetType>>
    for ArrayKmer<K, AlphabetType>
{
    type Iterator<'a> = std::slice::Iter<'a, AlphabetType::CharacterType> where Self: 'a, AlphabetType::CharacterType: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        self.array.iter()
    }

    fn len(&self) -> usize {
        self.array.len()
    }
}

impl<const K: usize, AlphabetType: Alphabet>
    OwnedGenomeSequence<AlphabetType, SliceSubGenome<AlphabetType>> for ArrayKmer<K, AlphabetType>
{
}

impl<const K: usize, AlphabetType: Alphabet>
    OwnedSequence<AlphabetType::CharacterType, SliceSubGenome<AlphabetType>>
    for ArrayKmer<K, AlphabetType>
{
}

impl<const K: usize, AlphabetType: Alphabet>
    GenomeSequenceMut<AlphabetType, SliceSubGenome<AlphabetType>> for ArrayKmer<K, AlphabetType>
{
    fn as_genome_subsequence_mut(&mut self) -> &mut SliceSubGenome<AlphabetType> {
        SliceSubGenome::ref_cast_mut(&mut self.array[..])
    }
}

impl<const K: usize, AlphabetType: Alphabet>
    SequenceMut<AlphabetType::CharacterType, SliceSubGenome<AlphabetType>>
    for ArrayKmer<K, AlphabetType>
{
    type IteratorMut<'a> = std::slice::IterMut<'a, AlphabetType::CharacterType>  where Self: 'a, AlphabetType::CharacterType: 'a;

    fn iter_mut(&mut self) -> Self::IteratorMut<'_> {
        self.array.iter_mut()
    }
}

impl<const K: usize, AlphabetType: Alphabet> FromIterator<AlphabetType::CharacterType>
    for ArrayKmer<K, AlphabetType>
{
    fn from_iter<T: IntoIterator<Item = AlphabetType::CharacterType>>(iter: T) -> Self {
        Self {
            array: iter
                .into_iter()
                .collect::<Vec<_>>()
                .try_into()
                .unwrap_or_else(|error: Vec<_>| {
                    panic!("iterator is not of length k = {K}, but {}", error.len())
                }),
        }
    }
}

impl<const K: usize, AlphabetType: Alphabet> Index<Range<usize>> for ArrayKmer<K, AlphabetType> {
    type Output = SliceSubGenome<AlphabetType>;

    fn index(&self, index: Range<usize>) -> &Self::Output {
        self.as_genome_subsequence().index(index)
    }
}

impl<const K: usize, AlphabetType: Alphabet> Index<usize> for ArrayKmer<K, AlphabetType> {
    type Output = AlphabetType::CharacterType;

    fn index(&self, index: usize) -> &Self::Output {
        self.as_genome_subsequence().index(index)
    }
}

impl<const K: usize, AlphabetType: Alphabet> IndexMut<Range<usize>> for ArrayKmer<K, AlphabetType> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        self.as_genome_subsequence_mut().index_mut(index)
    }
}

impl<const K: usize, AlphabetType: Alphabet> IndexMut<usize> for ArrayKmer<K, AlphabetType> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.as_genome_subsequence_mut().index_mut(index)
    }
}
