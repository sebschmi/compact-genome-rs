//! The DNA alphabet, consisting of characters A, C, G and T.

use crate::impl_generic_alphabet;

impl_generic_alphabet!("DNA alphabet", DnaAlphabet, DnaCharacter, b"ACGT", b"TGCA");

#[cfg(test)]
mod tests {
    use crate::implementation::alphabets::dna_alphabet::DnaCharacter;
    use std::convert::TryFrom;

    #[test]
    fn test_dna_alphabet_conversion() {
        for ascii in 0u8..=255u8 {
            if ascii == b'A' || ascii == b'C' || ascii == b'G' || ascii == b'T' {
                assert_eq!(
                    u8::from(DnaCharacter::try_from(ascii).unwrap_or_else(|_| panic!(
                        "character {ascii} was expected to be valid, but is not"
                    ))),
                    ascii
                );
            } else {
                assert!(DnaCharacter::try_from(ascii).is_err());
            }
        }
    }

    #[test]
    fn test_display() {
        for character in ['A', 'C', 'G', 'T'] {
            let dna_character = DnaCharacter::try_from(character).unwrap();
            assert_eq!(format!("{character}"), format!("{dna_character}"));
        }
    }
}
