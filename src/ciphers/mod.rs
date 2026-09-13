//! Common cipher helpers and the included cipher modules.
//!
//! The modules in this file provide implementations for several classic
//! ciphers that operate on the (ASCII) Latin alphabet. The helper
//! utilities are used by the custom-alphabet variants as well.

use std::collections::HashSet;

pub mod affine;
pub mod beaufort;
pub mod caesar;
pub mod railfence;
pub mod vigenere;
pub mod redefence;

/// Returns `true` when `alphabet` contains duplicate characters.
///
/// This is a small helper used by the custom-alphabet ciphers to validate
/// that the provided alphabet is suitable for substitution ciphers.
///
/// # Examples
///
/// ```rust
/// assert!(cifers::has_duplicate("aba"));
/// assert!(!cifers::has_duplicate("abc"));
/// ```
pub fn has_duplicate(alphabet: &str) -> bool {
    let mut chars: HashSet<char> = HashSet::new();
    for c in alphabet.chars() {
        if !chars.insert(c) {
            return true;
        }
    }
    false
}

/// Checks that every character in `text` appears in `alphabet`.
///
/// Returns `true` when `text` is composed only of characters from `alphabet`.
/// This is useful for validating inputs to ciphers that rely on a custom
/// alphabet ordering.
pub fn is_alphabet(alphabet: &str, text: &str) -> bool {
    for c in text.chars() {
        if !alphabet.contains(c) {
            return false;
        }
    }
    true
}