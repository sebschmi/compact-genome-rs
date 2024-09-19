//! An "anti" sequence store that stores the sequences in the handles.
//!
//! This is useful to use methods that require a sequence store when only plain sequences are available.

use std::marker::PhantomData;

use crate::interface::{
    alphabet::{Alphabet, AlphabetError},
    sequence::{GenomeSequence, OwnedGenomeSequence},
    sequence_store::SequenceStore,
};

/// A handle-based sequence store.
///
/// The sequence store stores nothing, all data is in the handles.
#[derive(Default, Clone, Eq, PartialEq, Debug)]
pub struct HandleSequenceStore<AlphabetType, SequenceType, SubsequenceType: ?Sized> {
    phantom_data: PhantomData<(AlphabetType, SequenceType, SubsequenceType)>,
}

impl<AlphabetType, SequenceType, SubsequenceType: ?Sized>
    HandleSequenceStore<AlphabetType, SequenceType, SubsequenceType>
{
    /// Creates a new instance.
    pub fn new() -> Self {
        Self {
            phantom_data: Default::default(),
        }
    }
}

impl<
        AlphabetType: Alphabet,
        SequenceType: OwnedGenomeSequence<AlphabetType, SubsequenceType>,
        SubsequenceType: GenomeSequence<AlphabetType, SubsequenceType> + ?Sized,
    > SequenceStore<AlphabetType>
    for HandleSequenceStore<AlphabetType, SequenceType, SubsequenceType>
{
    type Handle = SequenceType;
    type SequenceRef = SubsequenceType;

    fn add<
        Sequence: GenomeSequence<AlphabetType, Subsequence> + ?Sized,
        Subsequence: GenomeSequence<AlphabetType, Subsequence> + ?Sized,
    >(
        &mut self,
        s: &Sequence,
    ) -> Self::Handle {
        Self::Handle::from_iter(s.iter().cloned())
    }

    fn add_from_iter(
        &mut self,
        iter: impl IntoIterator<Item = <AlphabetType as Alphabet>::CharacterType>,
    ) -> Self::Handle {
        Self::Handle::from_iter(iter)
    }

    fn add_from_iter_u8<IteratorType: IntoIterator<Item = u8>>(
        &mut self,
        iter: IteratorType,
    ) -> Result<Self::Handle, AlphabetError> {
        Self::Handle::from_iter_u8(iter)
    }

    fn get<'this: 'result, 'handle: 'result, 'result>(
        &'this self,
        handle: &'handle Self::Handle,
    ) -> &'result Self::SequenceRef {
        handle.as_genome_subsequence()
    }
}
