//! A simple representation of a k-mer as an array.

use crate::implementation::bit_vec_sequence::alphabet_character_bit_width;
use crate::implementation::bit_vec_sequence::{BitVectorSubGenome, BitVectorSubGenomeIterator};
use crate::interface::alphabet::Alphabet;
use crate::interface::alphabet::AlphabetCharacter;
use crate::interface::k_mer::{Kmer, OwnedKmer};
use crate::interface::sequence::{GenomeSequence, OwnedGenomeSequence};
use bitvec::array::BitArray;
use bitvec::field::BitField;
use bitvec::store::BitStore;
use bitvec::view::BitView;
pub use bitvec::view::BitViewSized;
use ref_cast::RefCast;
use std::marker::PhantomData;
use std::ops::{Index, Range};
use traitsequence::interface::{OwnedSequence, Sequence};

/// A k-mer stored as array of minimum-bit characters.
#[derive(Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct BitArrayKmer<const K: usize, AlphabetType: Alphabet, BitArrayType = usize>
where
    BitArrayType: BitViewSized + BitStore,
{
    phantom_data: PhantomData<AlphabetType>,
    array: BitArray<BitArrayType>,
}

impl<const K: usize, AlphabetType: Alphabet, BitArrayType: BitViewSized + BitStore>
    Kmer<K, AlphabetType, BitVectorSubGenome<AlphabetType, BitArrayType>>
    for BitVectorSubGenome<AlphabetType, BitArrayType>
{
}

impl<
        const K: usize,
        AlphabetType: Alphabet,
        BitArrayType: BitViewSized + BitStore + BitView<Store = BitArrayType>,
    > OwnedKmer<K, AlphabetType, BitVectorSubGenome<AlphabetType, BitArrayType>>
    for BitArrayKmer<K, AlphabetType, BitArrayType>
{
}

impl<
        const K: usize,
        AlphabetType: Alphabet,
        BitArrayType: BitViewSized + BitStore + BitView<Store = BitArrayType>,
    > GenomeSequence<AlphabetType, BitVectorSubGenome<AlphabetType, BitArrayType>>
    for BitArrayKmer<K, AlphabetType, BitArrayType>
{
    fn as_genome_subsequence(&self) -> &BitVectorSubGenome<AlphabetType, BitArrayType> {
        BitVectorSubGenome::ref_cast(self.array.as_bitslice())
    }
}

impl<
        const K: usize,
        AlphabetType: Alphabet,
        BitArrayType: BitViewSized + BitStore + BitView<Store = BitArrayType>,
    > Sequence<AlphabetType::CharacterType, BitVectorSubGenome<AlphabetType, BitArrayType>>
    for BitArrayKmer<K, AlphabetType, BitArrayType>
{
    type Iterator<'a> = BitVectorSubGenomeIterator<'a, AlphabetType, BitArrayType> where Self: 'a, AlphabetType::CharacterType: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        self.as_genome_subsequence().iter()
    }

    fn len(&self) -> usize {
        K
    }
}

impl<
        const K: usize,
        AlphabetType: Alphabet,
        BitArrayType: BitViewSized + BitStore + BitView<Store = BitArrayType>,
    > OwnedGenomeSequence<AlphabetType, BitVectorSubGenome<AlphabetType, BitArrayType>>
    for BitArrayKmer<K, AlphabetType, BitArrayType>
{
}

impl<
        const K: usize,
        AlphabetType: Alphabet,
        BitArrayType: BitViewSized + BitStore + BitView<Store = BitArrayType>,
    > OwnedSequence<AlphabetType::CharacterType, BitVectorSubGenome<AlphabetType, BitArrayType>>
    for BitArrayKmer<K, AlphabetType, BitArrayType>
{
}

impl<const K: usize, AlphabetType: Alphabet, BitArrayType: BitViewSized + BitStore>
    FromIterator<AlphabetType::CharacterType> for BitArrayKmer<K, AlphabetType, BitArrayType>
{
    fn from_iter<T: IntoIterator<Item = AlphabetType::CharacterType>>(iter: T) -> Self {
        let mut array: BitArray<BitArrayType> =
            <BitArrayType as BitViewSized>::ZERO.into_bitarray();
        let mut iter = iter.into_iter();

        for index in 0..K {
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

impl<
        const K: usize,
        AlphabetType: Alphabet,
        BitArrayType: BitViewSized + BitStore + BitView<Store = BitArrayType>,
    > Index<Range<usize>> for BitArrayKmer<K, AlphabetType, BitArrayType>
{
    type Output = BitVectorSubGenome<AlphabetType, BitArrayType>;

    fn index(&self, index: Range<usize>) -> &Self::Output {
        self.as_genome_subsequence().index(index)
    }
}

impl<
        const K: usize,
        AlphabetType: Alphabet,
        BitArrayType: BitViewSized + BitStore + BitView<Store = BitArrayType>,
    > Index<usize> for BitArrayKmer<K, AlphabetType, BitArrayType>
{
    type Output = AlphabetType::CharacterType;

    fn index(&self, index: usize) -> &Self::Output {
        self.as_genome_subsequence().index(index)
    }
}

impl<const K: usize, AlphabetType: Alphabet, BitArrayType: BitViewSized + BitStore + Clone> Clone
    for BitArrayKmer<K, AlphabetType, BitArrayType>
{
    fn clone(&self) -> Self {
        Self {
            phantom_data: PhantomData,
            array: self.array.clone(),
        }
    }
}

impl<const K: usize, AlphabetType: Alphabet, BitArrayType: BitViewSized + BitStore + Copy> Copy
    for BitArrayKmer<K, AlphabetType, BitArrayType>
{
}

#[cfg(feature = "serde")]
mod serde {
    use bitvec::{array::BitArray, store::BitStore, view::BitViewSized};
    use serde::{Deserialize, Serialize};

    use crate::interface::alphabet::Alphabet;

    use super::BitArrayKmer;

    impl<
            const K: usize,
            AlphabetType: Alphabet,
            BitArrayType: BitViewSized + BitStore + Serialize,
        > Serialize for BitArrayKmer<K, AlphabetType, BitArrayType>
    {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            self.array.data.serialize(serializer)
        }
    }

    impl<
            'a,
            const K: usize,
            AlphabetType: Alphabet,
            BitArrayType: BitViewSized + BitStore + Deserialize<'a>,
        > Deserialize<'a> for BitArrayKmer<K, AlphabetType, BitArrayType>
    {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'a>,
        {
            Ok(Self {
                phantom_data: Default::default(),
                array: BitArray {
                    _ord: Default::default(),
                    data: BitArrayType::deserialize(deserializer)?,
                },
            })
        }

        fn deserialize_in_place<D>(deserializer: D, place: &mut Self) -> Result<(), D::Error>
        where
            D: serde::Deserializer<'a>,
        {
            BitArrayType::deserialize_in_place(deserializer, &mut place.array.data)
        }
    }
}
