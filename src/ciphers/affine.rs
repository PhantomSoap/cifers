//! Affine cipher implementation.
//!
//! The Affine cipher is a monoalphabetic substitution defined by
//! `E(x) = (a * x + b) mod 26`. This module exposes `Affine` with
//! setters for the multiplicative `a` and additive `b` parameters. The
//! `atbash` helper returns a commonly used parameter pair that implements
//! the Atbash transformation.

use crate::{Cipher, custom_alphabet_ciphers::custom_affine::CustomAffine};

/// Affine cipher parameters `a` (multiplier) and `b` (shift).
///
/// `a` must be chosen so that it is invertible modulo 26 (i.e., gcd(a,26)=1).
///
/// # Examples
///
/// ```rust
/// use cifers::Affine;
/// let c = Affine::new().set_a(7).set_b(12);
/// assert_eq!(c.encipher("exampletext"), "ormsnloporp");
/// ```
pub struct Affine {
    a: i32,
    b: i32,
}

impl Affine {
    /// Create a default Affine cipher with `a=1, b=0` (identity transformation).
    pub fn new() -> Self {
        Self { a: 1, b: 0 }
    }

    /// Create a custom-alphabet variant.
    pub fn set_alphabet(self, alphabet: String) -> CustomAffine {
        assert!(super::has_duplicate(&alphabet));
        CustomAffine::new(alphabet)
    }

    /// Set the multiplicative parameter `a`.
    ///
    /// Panics if `a` is not invertible modulo 26 (i.e., if `a` is even or
    /// a multiple of 13 in this implementation).
    pub fn set_a(mut self, a: i32) -> Self {
        assert!(a % 13 != 0 && a % 2 != 0);
        self.a = a;
        self
    }

    /// Set the additive parameter `b` (0..=25).
    pub fn set_b(mut self, b: i32) -> Self {
        assert!(b >= 0 && b <= 25);
        self.b = b;
        self
    }

    /// Return an `Affine` configured to perform an Atbash substitution.
    pub fn atbash() -> Self {
        Self { a: 25, b: 25 }
    }

    /// Shift a character according to the affine transformation.
    ///
    /// If `decrypt` is true the multiplicative inverse of `a` is used.
    pub fn shift_char(&self, chr: char, decrypt: bool) -> char {
        if !chr.is_alphabetic() {
            return chr;
        }
        let base = if chr.is_ascii_uppercase() { b'A' } else { b'a' };
        if !decrypt {
            (((self.a as u32 * (chr as u32 - base as u32) + self.b as u32)) % 26 + base as u32) as u8 as char
        } else {
            let modinverse = (0..26).find(|&x| (self.a * x) % 26 == 1).unwrap();
            let shifted = (chr as i32 - base as i32 - self.b as i32).rem_euclid(26);
            (((modinverse as i32 * shifted as i32)) % 26 + base as i32) as u8 as char
        }
    }
}

impl Cipher for Affine {
    fn encipher(&self,text : &str) -> String {
        text.chars().map(|chr| self.shift_char(chr, false)).collect()
    }

    fn decipher(&self,text : &str) -> String {
        text.chars().map(|chr| self.shift_char(chr, true)).collect()
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn identity_cipher() {
        assert_eq!(Affine::new().set_a(1).set_b(0).encipher("3x@mplEtexT"),"3x@mplEtexT");
    }

    #[test] 
    fn shift_char() {
        assert_eq!(Affine::new().set_a(25).set_b(25).shift_char('A', false),'Z');
    }

    #[test]
    fn encipher() {
        assert_eq!(Affine::new().set_a(7).set_b(12).encipher("exampletext"),"ormsnloporp");
    }

    

    #[test]
    fn decipher() {
        assert_eq!(Affine::new().set_a(7).set_b(12).decipher("ormsnloporp"),"exampletext");
    }

    #[test]
    fn decipher_symbols() {
        assert_eq!(Affine::new().set_a(7).set_b(12).decipher("6orm%snloporp$"),"6exa%mpletext$");
    }
    #[test]
    fn encipher_symbols() {
        assert_eq!(Affine::new().set_a(7).set_b(12).encipher("6exa%mpletext"),"6orm%snloporp");
    }

    #[test]
    fn encipher_large_ab() {
        assert_eq!(Affine::new().set_a(17).set_b(24).encipher("exampletext"),"ozyutdojozj");
    }

    #[test]
    fn decipher_large_ab() {
        assert_eq!(Affine::new().set_a(17).set_b(24).decipher("ozyutdojozj"),"exampletext");
    }

    #[test]
    fn boundry_a_b_encipher() {
        assert_eq!(Affine::new().set_a(1).set_b(0).encipher("3x@mplEtexT"),"3x@mplEtexT");
        assert_eq!(Affine::new().set_a(25).set_b(0).encipher("3x@mplEtexT"),"3d@olpWhwdH");
        assert_eq!(Affine::new().set_a(1).set_b(25).encipher("3x@mplEtexT"),"3w@lokDsdwS");
        assert_eq!(Affine::new().set_a(25).set_b(25).encipher("3x@mplEtexT"),"3c@nkoVgvcG");
    }

    #[test]
    fn boundry_a_b_decipher() {
        assert_eq!(Affine::new().set_a(1).set_b(0).decipher("3x@mplEtexT"),"3x@mplEtexT");
        assert_eq!(Affine::new().set_a(25).set_b(0).decipher("3d@olpWhwdH"),"3x@mplEtexT");
        assert_eq!(Affine::new().set_a(1).set_b(25).decipher("3w@lokDsdwS"),"3x@mplEtexT");
        assert_eq!(Affine::new().set_a(25).set_b(25).decipher("3c@nkoVgvcG"),"3x@mplEtexT");
    }
    #[test]
    fn atbash_test() {
        assert_eq!(Affine::new().set_a(25).set_b(25).encipher("3x@mplEtexT"),Affine::atbash().encipher("3x@mplEtexT"));
        assert_eq!(Affine::atbash().encipher("3x@mplEtexT"),"3c@nkoVgvcG");
    
    }

    #[test]
    #[should_panic]
    fn invalid_a() {
        let _ = Affine::new().set_a(2).set_b(1);
    }

    #[test]
    #[should_panic]
    fn over_value_b() {
        let _ = Affine::new().set_a(17).set_b(27);
    }
    #[test]
    #[should_panic]
    fn under_value_b() {
        let _ = Affine::new().set_a(17).set_b(-1);
    }
    
}

