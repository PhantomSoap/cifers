
use crate::Cipher;

/// Caesar cipher that uses a custom alphabet ordering.
///
/// `CustomCaesar` allows enciphering/deciphering using any provided
/// alphabet string. The alphabet must contain unique characters and the
/// cipher will shift characters according to their position within the
/// supplied alphabet. Uppercase characters are preserved when present in
/// the input.
///
/// # Examples
///
/// ```rust
/// use cifers::custom_alphabet_ciphers::custom_caesar::CustomCaesar;
/// let c = CustomCaesar::new(String::from("abcdefghijklmnopqrstuvwxyz")).set_shift(3);
/// assert_eq!(c.encipher("abc"), "def");
/// ```
pub struct CustomCaesar {
    shift: i32,
    alphabet: String,
}

impl CustomCaesar {
    pub fn new(alphabet : String) -> Self {
        Self {
            shift : 0,
            alphabet,
        }
    }
    pub fn set_shift(mut self,shift : i32) -> Self {
        self.shift = shift;
        self
    }

    
    fn shift_char(&self,c : char,shift : i32) -> char{
        if let Some(index) = self.alphabet.chars().position(|x|x==c.to_ascii_lowercase()) {
            if c.is_ascii_uppercase() {
                self.alphabet.chars().nth((index + shift as usize).rem_euclid(self.alphabet.len())).unwrap_or(c).to_ascii_uppercase()
            } else {
                self.alphabet.chars().nth((index + shift as usize).rem_euclid(self.alphabet.len())).unwrap_or(c)

            }
        } else {
            c
        }
        
    }
}

impl Cipher for CustomCaesar {
    fn encipher(&self,text : &str) -> String {
        text.chars().map(|c| self.shift_char(c, self.shift)).collect()
    }

    fn decipher(&self,text : &str) -> String {
        text.chars().map(|c| self.shift_char(c, -self.shift)).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::{Cipher, CustomCaesar};
    #[test]
    fn shift_word() {
        assert_eq!(CustomCaesar{shift : 3, alphabet : String::from("abcdefghijklmnopqrstuvwxyz")}.encipher("exampletext"),String::from("hadpsohwhaw"));
    }

    #[test]
    fn shift_word_mix_case() {
        assert_eq!(CustomCaesar{shift : 3, alphabet : String::from("abcdefghijklmnopqrstuvwxyz")}.encipher("examPletexT"),String::from("hadpSohwhaW"));
    }
}
