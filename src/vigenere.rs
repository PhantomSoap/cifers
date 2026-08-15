use crate::cipher::Cipher;

pub struct Vigenere {
    code : String,
}

impl Vigenere {
    pub fn new(code : String) -> Self {
        Self {
            code : code.to_uppercase().chars().filter(|chr| chr.is_alphabetic()).collect()
        }
    }

    pub fn shift_char(&self,chr : char,code_chr : char,decrypt : bool) -> char {
        let base = if chr.is_ascii_uppercase() {b'A' as i16}  else {b'a' as i16};
        if !decrypt {
            (base  + (((chr as i16 - base) + (code_chr as i16 - b'A' as i16)) % 26)) as u8 as char
        } else {
            (base  + (((chr as i16 - base) - (code_chr as i16 - b'A' as i16)+26) % 26)) as u8 as char
        }
    }



}

impl Cipher for Vigenere {
    fn encipher(&self,text : &str) -> String {
        let mut code_index = 0;
        text.chars().map(|chr| {
            if chr.is_alphabetic() {
            let chr = self.shift_char(chr, self.code.chars().nth(code_index).unwrap(),false);
                if code_index != self.code.len()-1 {
                    code_index+=1
                } else {
                    code_index = 0
                }
                chr
            } else {
                chr
            }
            }
        ).collect()
    }

    fn decipher(&self,text : &str) -> String {
        let mut code = self.code.chars().cycle();
        text.chars().map(|chr| {
            if chr.is_alphabetic() {
                self.shift_char(chr, code.next().unwrap(),true)
            } else {
                chr
            }
        }
        ).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shift_char() {
        assert_eq!(Vigenere::new(String::from("apple")).shift_char('b','A',false),'b');
    }

    #[test]
    fn encipher() {
        assert_eq!(Vigenere::new(String::from("acrylic")).encipher("exampletext"),String::from("ezrkatgtgor"));
    }
    #[test]
    fn decipher() {
        assert_eq!(Vigenere::new(String::from("acrylic")).decipher("ezrkatgtgor"),String::from("exampletext"));
    }

    #[test]
    fn encipher_mix_case_symbols() {
        assert_eq!(Vigenere::new(String::from("acrylic")).encipher("Examp0let@ext0"),String::from("Ezrka0tgt@gor0"));
    }

    #[test]
    fn decipher_mix_case_symbols() {
        assert_eq!(Vigenere::new(String::from("acrylic")).decipher("Ezrka0tgt@gor0"),String::from("Examp0let@ext0"));
    }


}