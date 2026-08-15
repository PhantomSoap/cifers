use crate::cipher::Cipher;

pub struct Caeser {
    shift : i32,
}

    

impl Caeser {
    pub fn new(shift : i32) -> Self {
        Self {
            shift,
        }
    }

    pub fn rot_13() -> Self {
        Self { shift : 13 }
    }

    pub fn brute_force(text : &str) -> Vec<String> {
        let mut shifts : Vec<String> = Vec::new();
        for i in 0..25 {
            shifts.push(Self::new(i).encipher(text))
        }
        shifts
    }

    pub fn shift_char(&self,chr : char,shift : i32) -> char{
        if chr.is_alphabetic() {
            let base = if chr.is_ascii_uppercase() {b'A'}  else {b'a'};
            (base + ((chr as u8 - base) as i32 + shift).rem_euclid(26) as u8 ) as char
        } else {
            chr
        }
    }
}

impl Cipher for Caeser {
    fn encipher(&self,text : &str) -> String {
        text.chars().map(|chr| self.shift_char(chr,self.shift)).collect()
    }

    fn decipher(&self,text : &str) -> String {
        text.chars().map(|chr| self.shift_char(chr, -self.shift)).collect()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn shift_char() {
        assert_eq!(Caeser::new(1).shift_char('a',1),'b');
    }

    #[test]
    fn uppercase_shift() {
        assert_eq!(Caeser::new(1).shift_char('A',1),'B');
    }
    #[test]
    fn shift_word() {
        assert_eq!(Caeser::new(3).encipher("exampletext"),String::from("hadpsohwhaw"));
    }
    #[test]
    fn shift_mix_case() {
        assert_eq!(Caeser::new(3).encipher("ExaMpletexT"),String::from("HadPsohwhaW"));

    }
    #[test]
    fn shift_mix_symbols() {
        assert_eq!(Caeser::new(3).encipher("Ex@MpletexT!"),String::from("Ha@PsohwhaW!"));

    }

}