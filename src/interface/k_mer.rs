//! K-mers are sequences of length k.

use crate::interface::alphabet::Alphabet;
use crate::interface::sequence::{GenomeSequence, GenomeSequenceMut, OwnedGenomeSequence};

/// A sequence of fixed length k.
/// Fixing the length allows for more efficient representations, such as arrays.
pub trait Kmer<
    const K: usize,
    AlphabetType: Alphabet,
    GenomeSubsequence: GenomeSequence<AlphabetType, GenomeSubsequence> + ?Sized,
>: GenomeSequence<AlphabetType, GenomeSubsequence>
{
    /// The length of sequences of this type.
    fn k() -> usize {
        K
    }
}

/// An owned k-mer.
pub trait OwnedKmer<
    const K: usize,
    AlphabetType: Alphabet,
    GenomeSubsequence: GenomeSequence<AlphabetType, GenomeSubsequence> + ?Sized,
>: OwnedGenomeSequence<AlphabetType, GenomeSubsequence>
{
    /// Get the successor of this k-mer with the specified character.
    ///
    /// This works by shifting the k-mer to the left and adding the character at the end.
    fn successor(&self, successor: AlphabetType::CharacterType) -> Self;
}

/// A k-mer whose characters can be mutated.
pub trait KmerMut<
    AlphabetType: Alphabet,
    GenomeSubsequenceMut: GenomeSequenceMut<AlphabetType, GenomeSubsequenceMut> + ?Sized,
>: GenomeSequenceMut<AlphabetType, GenomeSubsequenceMut>
{
}
