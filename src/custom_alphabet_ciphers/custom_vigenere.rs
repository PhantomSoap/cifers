/// Custom Vigenère cipher using an arbitrary alphabet ordering.
///
/// Provide an `alphabet` when creating the cipher and a key via `set_code`.
/// Characters not present in the alphabet are preserved.
///
/// # Examples
///
/// ```rust
/// use cifers::custom_alphabet_ciphers::custom_vigenere::CustomVigenere;
/// let c = CustomVigenere::new(String::from("abcdefghijklmnopqrstuvwxyz")).set_code(String::from("ACRYLIC"));
/// assert_eq!(c.encipher("exampletext"), "ezrkatgtgor");
/// ```


use crate::Cipher;
use crate::is_alphabet;

pub struct CustomVigenere {
    alphabet : String,
    code : String,
}


impl CustomVigenere {
    pub fn new(alphabet : String) -> Self {
        Self {
            code : String::from(""),
            alphabet,
        }
    }

    pub fn set_code(mut self,code : String) -> Self {
        
        self.code = code.to_uppercase().chars().filter(|chr| self.alphabet.chars().position(|c| c==*chr).is_some()).collect();
        self
        
    }

    pub fn encode_char(&self,chr : char,code_chr : char,decrypt : bool) -> char {
        let (chr_indx,code_indx) : (usize , usize) = (
            match self.alphabet.chars().position(|c| c == chr) {
                Some(i) => i,
                None => return chr,
            },
            match self.alphabet.chars().position(|c| c == code_chr) {
                Some(i) => i,
                None => return chr,
            }
        );
        if !decrypt {
            self.alphabet.chars().nth((chr_indx+code_indx).rem_euclid(self.alphabet.len())).unwrap()
        } else {
            self.alphabet.chars().nth((chr_indx-code_indx).rem_euclid(self.alphabet.len())).unwrap()

        }
    }

    



}

impl Cipher for CustomVigenere {
    fn encipher(&self,text : &str) -> String {
        if self.code.is_empty() {
            return text.to_string()
        }
        let mut code = self.code.chars().cycle();
        text.chars().map(|chr| {
            self.encode_char(chr, code.next().unwrap(),false)
        }
        ).collect()
    }

    fn decipher(&self,text : &str) -> String {
        if self.code.is_empty() {
            return text.to_string()
        }
        let mut code = self.code.chars().cycle();
        text.chars().map(|chr| {
            self.encode_char(chr, code.next().unwrap(),true)
        }
        ).collect()
    }
}