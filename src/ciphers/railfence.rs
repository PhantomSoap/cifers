//! Railfence (zig-zag) transposition cipher implementation.
//!
//! This module implements the Railfence cipher: characters are written in a
//! zig-zag pattern across a number of rails (the `key`) and then read off
//! row-by-row. The implementation provides helpers to compute the rail
//! indices and to brute-force small keys.

use crate::Cipher;

/// Railfence cipher with a numeric `key` indicating the number of rails.
///
/// # Examples
///
/// ```rust
/// use cifers::Railfence;
/// let c = Railfence::new().set_key(4);
/// assert_eq!(c.encipher("exampletext"), "eexltapetmx");
/// ```
pub struct Railfence {
    key: u8,
}

impl Railfence {
    /// Create a new `Railfence` with a default key of 1 (no transposition).
    pub fn new() -> Self {
        Self { key: 1 }
    }

    /// Set the rail `key` (number of rails).
    pub fn set_key(mut self, key: u8) -> Self {
        self.key = key;
        self
    }

    /// Brute-force deciphering by trying keys from 2..text.len().
    ///
    /// Returns a vector with each candidate plaintext for inspection.
    pub fn brute_force(text: &str) -> Vec<String> {
        let mut vector: Vec<String> = Vec::new();
        for i in 2..text.len() {
            vector.push(Self::new().set_key(i as u8).decipher(text));
        }
        vector
    }

    /// Compute the rail index for every character position for the given
    /// message length.
    ///
    /// Returns a `Vec<usize>` where each element is the rail number (0..key-1)
    /// for the corresponding character position in the message.
    pub fn get_rail_indices(&self, len: usize) -> Vec<usize> {
        if self.key <= 1 {
            return vec![0; len];
        }

        let cycle = (self.key as usize - 1) * 2;
        (0..len)
            .map(|i| {
                let rem = i % cycle;
                if rem < self.key as usize { rem } else { cycle - rem }
            })
            .collect()
    }
}

impl Cipher for Railfence {
    fn encipher(&self,text : &str) -> String {
        let rails = self.key as usize;
        let mut ciphertext = String::with_capacity(text.len());
        let indices = self.get_rail_indices(text.len());
        let chars = text.chars().collect::<Vec<char>>();
        for rail in 0..rails {
            for (index,&char_rail) in indices.iter().enumerate() {
                if rail == char_rail {
                    ciphertext.push(chars[index])
                }
            }
        }
        ciphertext

        
    }

    fn decipher(&self,text : &str) -> String {
        let rails = self.key;
        let indices = self.get_rail_indices(text.len());
        let mut plaintext = vec![' '; text.len()];
        let mut text_chars = text.chars();
        for rail in 0..rails as usize {
            for (index,_chr_rail) in indices.iter().enumerate().filter(|(_index,chr_rail)| **chr_rail == rail) {
                plaintext[index] = text_chars.next().unwrap()
            }
        }
        
        

        plaintext.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn encipher() {
        assert_eq!(Railfence::new().set_key(4).encipher("exampletext"),String::from("eexltapetmx"))
    }
    #[test]
    fn decipher() {
        assert_eq!(Railfence::new().set_key(4).decipher("eexltapetmx"),String::from("exampletext"))
    }

    #[test]
    fn encipher_mix_symbols() {
        assert_eq!(Railfence::new().set_key(4).encipher("jNhdN&jod*"),String::from("jjN&ohNdd*"))
    }
    #[test]
    fn decipher_mix_symbols() {
        assert_eq!(Railfence::new().set_key(4   ).decipher("jjN&ohNdd*"),String::from("jNhdN&jod*"))
    }

    #[test]
    fn encipher_larger_key() {
        assert_eq!(Railfence::new().set_key(15).encipher("3x@mplEtexT"),String::from("3x@mplEtexT"))
    }
    #[test]
    fn decipher_larger_key() {
        assert_eq!(Railfence::new().set_key(15).decipher("3x@mplEtexT"),String::from("3x@mplEtexT"))
    }

    #[test]
    fn get_rail_indices() {
        assert_eq!(Railfence::new().set_key(4).get_rail_indices("exampletext".len()),vec![0, 1, 2, 3, 2, 1, 0, 1, 2, 3, 2])
    }
    

}