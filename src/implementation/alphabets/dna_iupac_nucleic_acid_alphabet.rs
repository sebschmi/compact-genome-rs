//! The DNA [IUPAC nucleic acid alphabet][1].
//!
//! This version omits the character U, to avoid the complement of A being ambiguous (T or U).
//!
//! [1]: https://web.archive.org/web/20110811073845/http://www.dna.affrc.go.jp/misc/MPsrch/InfoIUPAC.html

use crate::impl_generic_alphabet;

impl_generic_alphabet!(
    "DNA IUPAC nucleic acid alphabet",
    DnaIupacNucleicAcidAlphabet,
    DnaIupacNucleicAcidCharacter,
    b"ABCDGHKMNRSTVWY",
    b"TVGHCDMKNYWABSR",
);
