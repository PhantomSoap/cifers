//! Redefence cipher (columnar rail/route cipher variant).
//!
//! `Redefence` mixes a keyed column order with a rail-style index mapping to
//! produce a transposition cipher. Provide a key via `set_code` to determine
//! column ordering. Non-alphabetic characters are preserved in their positions.

use crate::Cipher;

/// Redefence transposition cipher parameterized by `code`.
///
/// # Examples
///
/// ```rust
/// use cifers::Redefence;
/// let c = Redefence::new().set_code(String::from("abcd"));
/// assert_eq!(c.encipher("exampletext"), "eexltapetmx");
/// ```
pub struct Redefence {
    code: String,
}

impl Redefence {
    /// Create a default `Redefence` with an empty code.
    pub fn new() -> Self {
        Self { code: String::new() }
    }

    /// Set the key (`code`) used to order columns; the code is uppercased.
    pub fn set_code(mut self, code: String) -> Self {
        self.code = code.to_ascii_uppercase();
        self
    }

    /// Compute rail indices using the length of the `code` as the number of rails.
    pub fn get_rail_indices(&self, len: usize) -> Vec<usize> {
        if self.code.len() <= 1 {
            return vec![0; len];
        }

        let cycle = (self.code.len() as usize - 1) * 2;
        (0..len)
            .map(|i| {
                let rem = i % cycle;
                if rem < self.code.len() as usize { rem } else { cycle - rem }
            })
            .collect()
    }

}

impl Cipher for Redefence {
    fn encipher(&self,text : &str) -> String {
        
        let mut letters = self.code
            .chars()
            .enumerate()
            .map(|(index,value)| (value as u8 - b'A',index))
            .collect::<Vec<(u8,usize)>>();
        letters.sort_by_key(|&(value,_index)| value);
        let orders = letters.iter().map(|(_value,index)| *index).collect::<Vec<usize>>();
        

        let mut ciphertext = String::with_capacity(text.len());
        let indices = self.get_rail_indices(text.len());
        let chars = text.chars().collect::<Vec<char>>();
        for &rail in orders.iter() {
            for (index,&char_rail) in indices.iter().enumerate() {
                if rail == char_rail {
                    ciphertext.push(chars[index])
                }
            }
        }
        ciphertext
        


    }

    fn decipher(&self,text : &str) -> String {
        let mut letters = self.code
            .chars()
            .enumerate()
            .map(|(index,value)| (value as u8 - b'A',index))
            .collect::<Vec<(u8,usize)>>();
        letters.sort_by_key(|&(value,_index)| value);
        let orders = letters.iter().map(|(_value,index)| *index).collect::<Vec<usize>>();
        let indices = self.get_rail_indices(text.len());
        let mut plaintext = vec![' '; text.len()];
        let mut text_chars = text.chars();
        for &rail in orders.iter() {
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
        assert_eq!(Redefence::new().set_code(String::from("abcd")).encipher("exampletext"),String::from("eexltapetmx"))
    }
    #[test]
    fn decipher() {
        assert_eq!(Redefence::new().set_code(String::from("abcd")).decipher("eexltapetmx"),String::from("exampletext"))
    }

    #[test]
    fn encipher_mix_symbols() {
        assert_eq!(Redefence::new().set_code(String::from("code")).encipher("3x@mplEtexT"),String::from("3E@peTmxxlt"))
    }
    #[test]
    fn decipher_mix_symbols() {
        assert_eq!(Redefence::new().set_code(String::from("code")).decipher("3E@peTmxxlt"),String::from("3x@mplEtexT"))
    }
}