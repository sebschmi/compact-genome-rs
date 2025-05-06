//! The [IUPAC amino acid alphabet][1].
//!
//! [1]: https://web.archive.org/web/20250221074139/https://iupac.qmul.ac.uk/AminoAcid/AA1n2.html

use crate::impl_generic_alphabet;

impl_generic_alphabet!(
    "IUPAC amino acid alphabet",
    IupacAminoAcidAlphabet,
    IupacAminoAcidCharacter,
    b"ARNDCQEGHILKMFPSTWYVX",
    b"ARNDCQEGHILKMFPSTWYVX",
);

#[cfg(test)]
mod tests {
    use crate::implementation::alphabets::iupac_amino_acid_alphabet::IupacAminoAcidCharacter;
    use std::convert::TryFrom;

    #[test]
    fn test_alphabet_conversion() {
        let characters = b"ARNDCQEGHILKMFPSTWYVX";

        for ascii in 0u8..=255u8 {
            if characters.contains(&ascii) {
                assert_eq!(
                    u8::from(
                        IupacAminoAcidCharacter::try_from(ascii).unwrap_or_else(|_| panic!(
                            "character {ascii} was expected to be valid, but is not"
                        ))
                    ),
                    ascii
                );
            } else {
                assert!(IupacAminoAcidCharacter::try_from(ascii).is_err());
            }
        }
    }

    #[test]
    fn test_display() {
        for &character in b"ARNDCQEGHILKMFPSTWYVX" {
            let character = character as char;
            let dna_character = IupacAminoAcidCharacter::try_from(character).unwrap();
            assert_eq!(format!("{character}"), format!("{dna_character}"));
        }
    }
}
