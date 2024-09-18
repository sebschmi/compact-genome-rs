//! A simple representation of a k-mer as an array.

use crate::implementation::bit_vec_sequence::alphabet_character_bit_width;
use crate::implementation::bit_vec_sequence::{BitVectorSubGenome, BitVectorSubGenomeIterator};
use crate::interface::alphabet::Alphabet;
use crate::interface::alphabet::AlphabetCharacter;
use crate::interface::k_mer::{Kmer, OwnedKmer};
use crate::interface::sequence::{GenomeSequence, OwnedGenomeSequence};
use bitvec::array::BitArray;
use bitvec::field::BitField;
pub use bitvec::view::BitViewSized;
use ref_cast::RefCast;
use std::marker::PhantomData;
use std::ops::{Index, Range};
use traitsequence::interface::{OwnedSequence, Sequence};

/// A k-mer stored as array of minimum-bit characters.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct BitArrayKmer<const K: usize, BitArrayType: BitViewSized, AlphabetType: Alphabet> {
    phantom_data: PhantomData<AlphabetType>,
    array: BitArray<BitArrayType>,
}

impl<const K: usize, AlphabetType: Alphabet> Kmer<K, AlphabetType, BitVectorSubGenome<AlphabetType>>
    for BitVectorSubGenome<AlphabetType>
{
}

macro_rules! implement_bit_array_kmer {
    ($K:expr, $BitArrayType:ty) => {
impl<AlphabetType: Alphabet>
OwnedKmer<$K, AlphabetType, BitVectorSubGenome<AlphabetType>> for BitArrayKmer<$K, $BitArrayType, AlphabetType>
{
}

impl<AlphabetType: Alphabet>
GenomeSequence<AlphabetType, BitVectorSubGenome<AlphabetType>> for BitArrayKmer<$K, $BitArrayType, AlphabetType>
{
    fn as_genome_subsequence(&self) -> &BitVectorSubGenome<AlphabetType> {
        BitVectorSubGenome::ref_cast(&self.array[..])
    }
}

impl<AlphabetType: Alphabet>
Sequence<AlphabetType::CharacterType, BitVectorSubGenome<AlphabetType>>
for BitArrayKmer<$K, $BitArrayType, AlphabetType>
{
    type Iterator<'a> = BitVectorSubGenomeIterator<'a, AlphabetType> where Self: 'a, AlphabetType::CharacterType: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        self.as_genome_subsequence().iter()
    }

    fn len(&self) -> usize {
        $K
    }
}

impl<AlphabetType: Alphabet>
OwnedGenomeSequence<AlphabetType, BitVectorSubGenome<AlphabetType>> for BitArrayKmer<$K, $BitArrayType, AlphabetType>
{
}

impl<AlphabetType: Alphabet>
OwnedSequence<AlphabetType::CharacterType, BitVectorSubGenome<AlphabetType>>
for BitArrayKmer<$K, $BitArrayType, AlphabetType>
{
}

impl<AlphabetType: Alphabet> FromIterator<AlphabetType::CharacterType>
for BitArrayKmer<$K, $BitArrayType, AlphabetType>
{
    fn from_iter<T: IntoIterator<Item = AlphabetType::CharacterType>>(iter: T) -> Self {
        let mut array: BitArray<$BitArrayType> = <$BitArrayType>::default().into();
        let mut iter = iter.into_iter();

        for index in 0..$K {
            let bit_width = alphabet_character_bit_width(AlphabetType::SIZE);
        let offset = index * bit_width;
        let limit = (index + 1) * bit_width;

            let character = iter.next().unwrap();
            array[offset..limit].store(character.index());
        }
        assert!(iter.next().is_none());


        Self {
            phantom_data: Default::default(),
            array,
        }
    }
}

impl<AlphabetType: Alphabet> Index<Range<usize>> for BitArrayKmer<$K, $BitArrayType, AlphabetType> {
    type Output = BitVectorSubGenome<AlphabetType>;

    fn index(&self, index: Range<usize>) -> &Self::Output {
        self.as_genome_subsequence().index(index)
    }
}

impl<AlphabetType: Alphabet> Index<usize> for BitArrayKmer<$K, $BitArrayType, AlphabetType> {
    type Output = AlphabetType::CharacterType;

    fn index(&self, index: usize) -> &Self::Output {
        self.as_genome_subsequence().index(index)
    }
}

    }
}

implement_bit_array_kmer!(1, [usize; 1]);
