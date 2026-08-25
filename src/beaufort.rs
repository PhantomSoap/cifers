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

    #[test]
    pub fn encipher_symbols_mix() {
        assert_eq!(Beaufort::new(String::from("Key")).encipher("3x@mplEtexT"),"3n@sjzAfghF")
    }

    #[test]
    pub fn decipher_symbols_mix() {
        assert_eq!(Beaufort::new(String::from("Key")).decipher("3n@sjzAfghF"),"3x@mplEtexT")
    }

    #[test]
    pub fn encipher_long_key() {
        assert_eq!(Beaufort::new(String::from("snskjfglksjdflkhkasdlhslkjsdflhf")).encipher("3x@mplEtexT"),"3v@bdzFmcoR")
    }

    #[test]
    pub fn decipher_long_key() {
        assert_eq!(Beaufort::new(String::from("snskjfglksjdflkhkasdlhslkjsdflhf")).decipher("3v@bdzFmcoR"),"3x@mplEtexT")
    }

    #[test]
    pub fn encipher_short_key() {
        assert_eq!(Beaufort::new(String::from("sd")).encipher("3x@mplEtexT"),"3v@rdsOkogZ")
    }

    #[test]
    pub fn decipher_short_key() {
        assert_eq!(Beaufort::new(String::from("sd")).decipher("3v@rdsOkogZ"),"3x@mplEtexT")
    }

    #[test]
    pub fn encipher_one_letter_key() {
        assert_eq!(Beaufort::new(String::from("s")).encipher("3x@mplEtexT"),"")
    }

    #[test]
    pub fn decipher_one_letter_key() {
        assert_eq!(Beaufort::new(String::from("s")).decipher(""),"3x@mplEtexT")
    }

    #[test]
    pub fn running_key_encipher() {
        assert_eq!(Beaufort::new(String::from("dkmncuilsyt")).encipher("3x@mplEtexT"),"3g@yxcYbeoZ")
    }

    #[test]
    pub fn running_key_decipher() {
        assert_eq!(Beaufort::new(String::from("dkmncuilsyt")).decipher("3g@yxcYbeoZ"),"3x@mplEtexT")
    }

    #[test]
    pub fn encipher_shorter_text() {
        assert_eq!(Beaufort::new(String::from("CodeWord")).encipher("3x@mplE"),"3f@cotS")
    }

    #[test]
    pub fn decipher_shorter_text() {
        assert_eq!(Beaufort::new(String::from("CodeWord")).decipher("3f@cotS"),"3x@mplE")
    }
}