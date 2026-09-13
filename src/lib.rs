//! cifers — a small collection of classical substitution and transposition ciphers
//!
//! This crate provides a set of implementations for classic ciphers such as
//! Caesar, Vigenère, Beaufort, Affine, Railfence and a couple of helper
//! utilities for working with alphabets. Each cipher implements the
//! `Cipher` trait which exposes `encipher` and `decipher` methods.
//!
//! # Examples
//!
//! Enciphering with the Caesar cipher:
//!
//! ```rust
//! use cifers::Caeser;
//!
//! let cipher = Caeser::new().set_shift(3);
//! let encrypted = cipher.encipher("hello");
//! let decrypted = cipher.decipher(&encrypted);
//! assert_eq!(decrypted, "hello");
//! ```
//!
//! # Crate layout
//!
//! - `ciphers/` — implementations that assume the Latin alphabet
//! - `custom_alphabet_ciphers/` — implementations that accept a custom alphabet

pub mod ciphers;
pub mod custom_alphabet_ciphers;

pub use ciphers::caesar::Caeser;
pub use ciphers::vigenere::Vigenere;
pub use ciphers::beaufort::Beaufort;
pub use ciphers::redefence::Redefence;
pub use ciphers::affine::Affine;
pub use ciphers::railfence::Railfence;
pub use ciphers::has_duplicate;
pub use ciphers::is_alphabet;

/// A small trait implemented by all cipher types in this crate.
///
/// Implementors should provide `encipher` and `decipher` methods
/// that operate on UTF-8 `&str` text. Implementations typically
/// preserve non-alphabetic characters unchanged.
pub trait Cipher {
    /// Enciphers `text` and returns the ciphertext as `String`.
    fn encipher(&self, text: &str) -> String;

    /// Deciphers `text` and returns the plaintext as `String`.
    fn decipher(&self, text: &str) -> String;
}


/// Compute the index of coincidence for `text` treating only ASCII letters.
///
/// The index of coincidence is useful for statistical analysis of ciphertexts
/// (for example to help determine key lengths for Vigenère-like ciphers).
/// It returns a floating point value; English plain-text typically yields
/// values around ~0.066 while random text is much lower.
fn index_of_coincidence(text: &str) -> f64 {
    let mut counts = [0u32; 26];
    let mut total_letters = 0;

    for byte in text.bytes() {
        if byte.is_ascii_alphabetic() {
            counts[(byte.to_ascii_lowercase() - b'a') as usize] += 1;
            total_letters += 1;
        }
    }
    if total_letters < 2 {
        return 0.0;
    }

    let numerator: u32 = counts.iter().map(|&x| x * (x - 1)).sum();
    numerator as f64 / (total_letters as f64 * (total_letters as f64 - 1.0))
}

