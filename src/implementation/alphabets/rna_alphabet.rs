//! The RNA alphabet, consisting of characters A, C, G and U.

use crate::impl_generic_alphabet;

impl_generic_alphabet!("RNA alphabet", RnaAlphabet, RnaCharacter, b"ACGU", b"UGCA");

#[cfg(test)]
mod tests {
    use crate::implementation::alphabets::rna_alphabet::RnaCharacter;
    use std::convert::TryFrom;

    #[test]
    fn test_rna_alphabet_conversion() {
        for ascii in 0u8..=255u8 {
            if ascii == b'A' || ascii == b'C' || ascii == b'G' || ascii == b'U' {
                assert_eq!(
                    u8::from(RnaCharacter::try_from(ascii).unwrap_or_else(|_| panic!(
                        "character {ascii} was expected to be valid, but is not"
                    ))),
                    ascii
                );
            } else {
                assert!(RnaCharacter::try_from(ascii).is_err());
            }
        }
    }

    #[test]
    fn test_display() {
        for character in ['A', 'C', 'G', 'U'] {
            let rna_character = RnaCharacter::try_from(character).unwrap();
            assert_eq!(format!("{character}"), format!("{rna_character}"));
        }
    }
}
