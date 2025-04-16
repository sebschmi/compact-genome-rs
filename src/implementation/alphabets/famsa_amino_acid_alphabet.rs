//! The [FAMSA amino acid alphabet][1].
//!
//! This is a small extension of the [IUPAC amino acid alphabet][2].
//! According to [a github issue in the FAMSA repository][3], the meaning of the additional symbols is the following:
//!
//! * B: Represents asparagine (Asn) or aspartic acid (Asp)
//! * Z: Represents glutamic acid (Glu) or glutamine (Gln)
//! * X, *: Represents any or unknown amino acid
//!
//! However, FAMSA treats all these symbols (and any other symbols not in the list above) as unknown.
//!
//! [1]: https://github.com/refresh-bio/FAMSA/blob/1669fc1444c8bc4000d71121ec2a7aa62d848b57/src/msa.cpp#L42
//! [2]: https://web.archive.org/web/20250221074139/https://iupac.qmul.ac.uk/AminoAcid/AA1n2.html
//! [3]: https://github.com/refresh-bio/FAMSA/issues/53

use crate::impl_generic_alphabet;

impl_generic_alphabet!(
    "FAMSA amino acid alphabet",
    FamsaAminoAcidAlphabet,
    FamsaAminoAcidCharacter,
    b"ARNDCQEGHILKMFPSTWYVBZX*",
    b"ARNDCQEGHILKMFPSTWYVBZX*",
);

#[cfg(test)]
mod tests {
    use crate::implementation::alphabets::famsa_amino_acid_alphabet::FamsaAminoAcidCharacter;
    use std::convert::TryFrom;

    #[test]
    fn test_alphabet_conversion() {
        let characters = b"ARNDCQEGHILKMFPSTWYVBZX*";

        for ascii in 0u8..=255u8 {
            if characters.contains(&ascii) {
                assert_eq!(
                    u8::from(
                        FamsaAminoAcidCharacter::try_from(ascii).unwrap_or_else(|_| panic!(
                            "character {} was expected to be valid, but is not",
                            ascii
                        ))
                    ),
                    ascii
                );
            } else {
                assert!(FamsaAminoAcidCharacter::try_from(ascii).is_err());
            }
        }
    }

    #[test]
    fn test_display() {
        for &character in b"ARNDCQEGHILKMFPSTWYVBZX*" {
            let character = character as char;
            let dna_character = FamsaAminoAcidCharacter::try_from(character).unwrap();
            assert_eq!(format!("{character}"), format!("{dna_character}"));
        }
    }
}
