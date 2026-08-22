use crate::cipher::Cipher;

pub struct Beaufort {
    code : String,
}

impl Beaufort {
    pub fn new(code : String) -> Self {
        Self {
            code : code.to_uppercase(),
        }
    }

    pub fn encode_char(&self,chr : char,code_chr : char) -> char {
        let base = if chr.is_ascii_uppercase() {b'A' as i16} else {b'a' as i16};
        (base  + (((code_chr as i16 - b'A' as i16) -    (chr as i16 - base) +26) % 26)) as u8 as char
    }
}

impl Cipher for Beaufort {
    fn encipher(&self,text : &str) -> String {
        if self.code.is_empty() {
            return text.to_string()
        }
        let mut code = self.code.chars().cycle();
        text.chars().map(|chr| {
            if chr.is_alphabetic() {
                self.encode_char(chr, code.next().unwrap())
            } else {
                chr
            }
        }
        ).collect()
    }

    fn decipher(&self,text : &str) -> String {
        if self.code.is_empty() {
            return text.to_string()
        }
        let mut code = self.code.chars().cycle();
        text.chars().map(|chr| {
            if chr.is_alphabetic() {
                self.encode_char(chr, code.next().unwrap())
            } else {
                chr
            }
        }
        ).collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn encipher() {
        assert_eq!(Beaufort::new(String::from("Key")).encipher("exampletext"),"ghyypnglunl")
    }

    #[test]
    pub fn decipher() {
        assert_eq!(Beaufort::new(String::from("Key")).decipher("ghyypnglunl"),"exampletext")
    }

    #[test]
    pub fn mix_case_decipher() {
        assert_eq!(Beaufort::new(String::from("Key")).decipher("GhYypnglunl"),"ExAmpletext")
    }
}