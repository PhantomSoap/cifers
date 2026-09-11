use crate::Cipher;

pub struct CustomBeaufort {
    alphabet : String,
    code : String,
}

impl CustomBeaufort {
    pub fn new(alphabet : String) -> Self {
        Self {
            code : String::from(""),
            alphabet,

        }
    }

    pub fn set_code(mut self,code : String) -> Self {
        self.code = code.to_uppercase();
        self
    }

    pub fn encode_char(&self,chr : char,code_chr : char) -> char {
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

        self.alphabet.chars().nth((code_indx - chr_indx).rem_euclid(self.alphabet.len())).unwrap()
        
    }
}


impl Cipher for CustomBeaufort {
    fn encipher(&self,text : &str) -> String {
        if self.code.is_empty() {
            return text.to_string()
        }
        let mut code = self.code.chars().cycle();
        text.chars().map(|chr| {
            self.encode_char(chr, code.next().unwrap())
            
        }
        ).collect()
    }

    fn decipher(&self,text : &str) -> String {
        if self.code.is_empty() {
            return text.to_string()
        }
        let mut code = self.code.chars().cycle();
        text.chars().map(|chr| {
            self.encode_char(chr, code.next().unwrap())
        }
        ).collect()
    }
}