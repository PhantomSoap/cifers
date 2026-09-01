use crate::Cipher;

pub struct CustomCaesar {
    shift : i32,
    alphabet : String,
}

impl CustomCaesar {
    fn shift_char(&self,c : char,shift : i32) -> char{
        if let Some(index) = self.alphabet.chars().position(|x|x==c) {
            self.alphabet.chars().nth((index + shift as usize).rem_euclid(self.alphabet.len())).unwrap_or(c)
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
}
