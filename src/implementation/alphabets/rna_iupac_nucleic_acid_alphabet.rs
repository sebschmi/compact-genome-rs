//! The RNA [IUPAC nucleic acid alphabet][1].
//!
//! This version omits the character T, to avoid the complement of A being ambiguous (T or U).
//!
//! [1]: https://web.archive.org/web/20110811073845/http://www.dna.affrc.go.jp/misc/MPsrch/InfoIUPAC.html

use crate::impl_generic_alphabet;

impl_generic_alphabet!(
    "RNA IUPAC nucleic acid alphabet",
    RnaIupacNucleicAcidAlphabet,
    RnaIupacNucleicAcidCharacter,
    b"ABCDGHKMNRSUVWY",
    b"UVGHCDMKNYWABSR",
);
