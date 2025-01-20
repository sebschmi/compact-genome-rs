//! The RNA alphabet including N, consisting of characters A, C, G, U, and N.

use crate::impl_generic_alphabet;

impl_generic_alphabet!(
    "RNA alphabet including N",
    RnaAlphabetOrN,
    RnaCharacterOrN,
    b"ACGNU",
    b"UGCNA"
);

#[cfg(test)]
mod tests {
    use crate::implementation::alphabets::rna_alphabet_or_n::RnaCharacterOrN;
    use std::convert::TryFrom;

    #[test]
    fn test_rna_alphabet_conversion() {
        for ascii in 0u8..=255u8 {
            if ascii == b'A' || ascii == b'C' || ascii == b'G' || ascii == b'N' || ascii == b'U' {
                assert_eq!(
                    u8::from(RnaCharacterOrN::try_from(ascii).unwrap_or_else(|_| panic!(
                        "character {} was expected to be valid, but is not",
                        ascii
                    ))),
                    ascii
                );
            } else {
                assert!(RnaCharacterOrN::try_from(ascii).is_err());
            }
        }
    }

    #[test]
    fn test_display() {
        for character in ['A', 'C', 'G', 'N', 'U'] {
            let rna_character = RnaCharacterOrN::try_from(character).unwrap();
            assert_eq!(format!("{character}"), format!("{rna_character}"));
        }
    }
}
