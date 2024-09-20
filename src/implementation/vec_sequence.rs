//! A simple representation of a genome as `Vec<u8>`.

use std::borrow::Borrow;
use std::ops::{
    Index, IndexMut, Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive,
};
//use std::marker::PhantomData;
use crate::interface::alphabet::Alphabet;
use crate::interface::sequence::{
    EditableGenomeSequence, GenomeSequence, GenomeSequenceMut, OwnedGenomeSequence,
};
use ref_cast::RefCast;
use traitsequence::interface::{EditableSequence, OwnedSequence, Sequence, SequenceMut};

/// A genome sequence stored as vector of plain characters.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct VectorGenome<AlphabetType: Alphabet> {
    vector: Vec<AlphabetType::CharacterType>,
}

/// The subsequence of a genome sequence stored as slice of plain characters.
#[derive(RefCast, Debug, Eq, PartialEq, Hash)]
#[repr(transparent)]
pub struct SliceSubGenome<AlphabetType: Alphabet> {
    //phantom_data: PhantomData<AlphabetType>,
    pub(crate) slice: [AlphabetType::CharacterType],
}

impl<AlphabetType: Alphabet> GenomeSequence<AlphabetType, SliceSubGenome<AlphabetType>>
    for VectorGenome<AlphabetType>
{
    fn as_genome_subsequence(&self) -> &SliceSubGenome<AlphabetType> {
        SliceSubGenome::ref_cast(&self.vector[..])
    }
}

impl<AlphabetType: Alphabet> GenomeSequenceMut<AlphabetType, SliceSubGenome<AlphabetType>>
    for VectorGenome<AlphabetType>
{
}

impl<AlphabetType: Alphabet> OwnedGenomeSequence<AlphabetType, SliceSubGenome<AlphabetType>>
    for VectorGenome<AlphabetType>
{
}

impl<AlphabetType: Alphabet> EditableGenomeSequence<AlphabetType, SliceSubGenome<AlphabetType>>
    for VectorGenome<AlphabetType>
{
}

impl<AlphabetType: Alphabet> GenomeSequence<AlphabetType, SliceSubGenome<AlphabetType>>
    for SliceSubGenome<AlphabetType>
{
}

impl<AlphabetType: Alphabet> GenomeSequenceMut<AlphabetType, SliceSubGenome<AlphabetType>>
    for SliceSubGenome<AlphabetType>
{
}

impl<AlphabetType: Alphabet> Sequence<AlphabetType::CharacterType, SliceSubGenome<AlphabetType>>
    for VectorGenome<AlphabetType>
{
    type Iterator <'a>= std::slice::Iter<'a, AlphabetType::CharacterType> where AlphabetType: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        self.as_genome_subsequence().iter()
    }

    fn len(&self) -> usize {
        self.as_genome_subsequence().len()
    }
}

impl<AlphabetType: Alphabet> Sequence<AlphabetType::CharacterType, SliceSubGenome<AlphabetType>>
    for SliceSubGenome<AlphabetType>
{
    type Iterator<'a> = std::slice::Iter<'a, AlphabetType::CharacterType> where AlphabetType: 'a;

    fn iter(&self) -> Self::Iterator<'_> {
        self.slice.iter()
    }

    fn len(&self) -> usize {
        self.slice.len()
    }
}

impl<AlphabetType: Alphabet>
    EditableSequence<AlphabetType::CharacterType, SliceSubGenome<AlphabetType>>
    for VectorGenome<AlphabetType>
{
    fn split_off(&mut self, at: usize) -> Self {
        Self {
            vector: self.vector.split_off(at),
        }
    }

    fn set(&mut self, index: usize, item: AlphabetType::CharacterType) {
        self.vector.set(index, item)
    }

    fn reserve(&mut self, additional: usize) {
        self.vector.reserve(additional)
    }

    fn resize(&mut self, new_len: usize, value: AlphabetType::CharacterType)
    where
        AlphabetType::CharacterType: Clone,
    {
        self.vector.resize(new_len, value)
    }

    fn resize_with(
        &mut self,
        new_len: usize,
        generator: impl FnMut() -> AlphabetType::CharacterType,
    ) {
        self.vector.resize_with(new_len, generator);
    }

    fn push(&mut self, item: AlphabetType::CharacterType) {
        self.vector.push(item)
    }

    fn splice(
        &mut self,
        range: Range<usize>,
        replace_with: impl IntoIterator<Item = AlphabetType::CharacterType>,
    ) {
        self.vector.splice(range, replace_with);
    }
}

impl<AlphabetType: Alphabet> Index<Range<usize>> for VectorGenome<AlphabetType> {
    type Output = SliceSubGenome<AlphabetType>;

    fn index(&self, index: Range<usize>) -> &Self::Output {
        self.as_genome_subsequence().index(index)
    }
}

impl<AlphabetType: Alphabet> Index<RangeFrom<usize>> for VectorGenome<AlphabetType> {
    type Output = SliceSubGenome<AlphabetType>;

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        self.as_genome_subsequence().index(index)
    }
}

impl<AlphabetType: Alphabet> Index<RangeTo<usize>> for VectorGenome<AlphabetType> {
    type Output = SliceSubGenome<AlphabetType>;

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        self.as_genome_subsequence().index(index)
    }
}

impl<AlphabetType: Alphabet> Index<RangeFull> for VectorGenome<AlphabetType> {
    type Output = SliceSubGenome<AlphabetType>;

    fn index(&self, index: RangeFull) -> &Self::Output {
        self.as_genome_subsequence().index(index)
    }
}

impl<AlphabetType: Alphabet> Index<RangeInclusive<usize>> for VectorGenome<AlphabetType> {
    type Output = SliceSubGenome<AlphabetType>;

    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        self.as_genome_subsequence().index(index)
    }
}

impl<AlphabetType: Alphabet> Index<RangeToInclusive<usize>> for VectorGenome<AlphabetType> {
    type Output = SliceSubGenome<AlphabetType>;

    fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
        self.as_genome_subsequence().index(index)
    }
}

impl<AlphabetType: Alphabet> Index<usize> for VectorGenome<AlphabetType> {
    type Output = AlphabetType::CharacterType;

    fn index(&self, index: usize) -> &Self::Output {
        self.as_genome_subsequence().index(index)
    }
}

impl<AlphabetType: Alphabet> Index<Range<usize>> for SliceSubGenome<AlphabetType> {
    type Output = SliceSubGenome<AlphabetType>;

    fn index(&self, index: Range<usize>) -> &Self::Output {
        SliceSubGenome::ref_cast(&self.slice[index.start..index.end])
    }
}

impl<AlphabetType: Alphabet> Index<RangeFrom<usize>> for SliceSubGenome<AlphabetType> {
    type Output = SliceSubGenome<AlphabetType>;

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        self.index(index.start..self.len())
    }
}

impl<AlphabetType: Alphabet> Index<RangeTo<usize>> for SliceSubGenome<AlphabetType> {
    type Output = SliceSubGenome<AlphabetType>;

    fn index(&self, index: RangeTo<usize>) -> &Self::Output {
        self.index(0..index.end)
    }
}

impl<AlphabetType: Alphabet> Index<RangeFull> for SliceSubGenome<AlphabetType> {
    type Output = SliceSubGenome<AlphabetType>;

    fn index(&self, _index: RangeFull) -> &Self::Output {
        self.index(0..self.len())
    }
}

impl<AlphabetType: Alphabet> Index<RangeInclusive<usize>> for SliceSubGenome<AlphabetType> {
    type Output = SliceSubGenome<AlphabetType>;

    fn index(&self, index: RangeInclusive<usize>) -> &Self::Output {
        SliceSubGenome::ref_cast(&self.slice[*index.start()..=*index.end()])
    }
}

impl<AlphabetType: Alphabet> Index<RangeToInclusive<usize>> for SliceSubGenome<AlphabetType> {
    type Output = SliceSubGenome<AlphabetType>;

    fn index(&self, index: RangeToInclusive<usize>) -> &Self::Output {
        self.index(0..=index.end)
    }
}

impl<AlphabetType: Alphabet> Index<usize> for SliceSubGenome<AlphabetType> {
    type Output = AlphabetType::CharacterType;

    fn index(&self, index: usize) -> &Self::Output {
        self.slice.index(index)
    }
}

impl<AlphabetType: Alphabet> FromIterator<AlphabetType::CharacterType>
    for VectorGenome<AlphabetType>
{
    fn from_iter<T: IntoIterator<Item = AlphabetType::CharacterType>>(iter: T) -> Self {
        let mut result = Self::default();
        result.extend(iter);
        result
    }
}

impl<AlphabetType: Alphabet> Extend<AlphabetType::CharacterType> for VectorGenome<AlphabetType> {
    fn extend<T: IntoIterator<Item = AlphabetType::CharacterType>>(&mut self, iter: T) {
        self.vector.extend(iter)
    }
}

impl<AlphabetType: Alphabet> IntoIterator for VectorGenome<AlphabetType> {
    type Item = AlphabetType::CharacterType;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.vector.into_iter()
    }
}

impl<AlphabetType: Alphabet> Borrow<SliceSubGenome<AlphabetType>> for VectorGenome<AlphabetType> {
    fn borrow(&self) -> &SliceSubGenome<AlphabetType> {
        self.as_genome_subsequence()
    }
}

impl<AlphabetType: Alphabet> ToOwned for SliceSubGenome<AlphabetType> {
    type Owned = VectorGenome<AlphabetType>;

    fn to_owned(&self) -> Self::Owned {
        self.iter().cloned().collect()
    }
}

impl<AlphabetType: Alphabet> SequenceMut<AlphabetType::CharacterType, SliceSubGenome<AlphabetType>>
    for VectorGenome<AlphabetType>
{
    type IteratorMut<'a> = std::slice::IterMut<'a, AlphabetType::CharacterType> where AlphabetType: 'a;

    fn iter_mut(&mut self) -> Self::IteratorMut<'_> {
        self.vector.iter_mut()
    }
}

impl<AlphabetType: Alphabet> IndexMut<usize> for VectorGenome<AlphabetType> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.vector.index_mut(index)
    }
}

impl<AlphabetType: Alphabet> IndexMut<Range<usize>> for VectorGenome<AlphabetType> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        SliceSubGenome::ref_cast_mut(&mut self.vector[index])
    }
}

impl<AlphabetType: Alphabet> SequenceMut<AlphabetType::CharacterType, SliceSubGenome<AlphabetType>>
    for SliceSubGenome<AlphabetType>
{
    type IteratorMut<'a> = std::slice::IterMut<'a, AlphabetType::CharacterType> where AlphabetType: 'a;

    fn iter_mut(&mut self) -> Self::IteratorMut<'_> {
        self.slice.iter_mut()
    }
}

impl<AlphabetType: Alphabet> IndexMut<usize> for SliceSubGenome<AlphabetType> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.slice.index_mut(index)
    }
}

impl<AlphabetType: Alphabet> IndexMut<Range<usize>> for SliceSubGenome<AlphabetType> {
    fn index_mut(&mut self, index: Range<usize>) -> &mut Self::Output {
        SliceSubGenome::ref_cast_mut(&mut self.slice[index])
    }
}

impl<AlphabetType: Alphabet> Default for VectorGenome<AlphabetType> {
    fn default() -> Self {
        Self {
            vector: Default::default(),
        }
    }
}

impl<AlphabetType: Alphabet>
    OwnedSequence<AlphabetType::CharacterType, SliceSubGenome<AlphabetType>>
    for VectorGenome<AlphabetType>
{
}

impl<AlphabetType: Alphabet> VectorGenome<AlphabetType> {
    /// Converts the `VectorGenome` to its inner data vector.
    pub fn into_inner(self) -> Vec<AlphabetType::CharacterType> {
        self.vector
    }
}

impl<AlphabetType: Alphabet> From<VectorGenome<AlphabetType>> for Vec<AlphabetType::CharacterType> {
    fn from(genome: VectorGenome<AlphabetType>) -> Self {
        genome.into_inner()
    }
}

#[cfg(test)]
mod tests {
    use crate::implementation::alphabets::dna_alphabet::DnaAlphabet;
    use crate::implementation::vec_sequence::VectorGenome;
    use crate::interface::sequence::{GenomeSequence, OwnedGenomeSequence};

    #[test]
    fn test_reverse_complement() {
        let genome = VectorGenome::<DnaAlphabet>::from_slice_u8(b"ATTCGGT").unwrap();
        let reverse_complement = VectorGenome::<DnaAlphabet>::from_slice_u8(b"ACCGAAT").unwrap();
        debug_assert_eq!(genome.clone_as_reverse_complement(), reverse_complement);
        debug_assert_eq!(genome, reverse_complement.clone_as_reverse_complement());
    }

    #[test]
    fn test_display() {
        let genome = VectorGenome::<DnaAlphabet>::from_slice_u8(b"ATTCGGT").unwrap();
        let display_string = genome.as_string();
        let expected_string = "ATTCGGT";
        debug_assert_eq!(display_string, expected_string);
    }
}
