use crate::cipher::Cipher;

pub struct Affine {
    a : i32,
    b : i32,
}

impl Affine {
    pub fn new(a : i32, b : i32) -> Self {
        assert!(a % 13 !=0 && a % 2 !=0);
        assert!(b >= 0 && b <=25);

        Self {
            a,
            b,
        }
    }

    pub fn affine() -> Self {
        Self {
            a : 25,
            b : 25,
        }
    }

    pub fn shift_char(&self,chr : char,decrypt : bool) -> char {
        let base = if chr.is_ascii_uppercase() {b'A'} else {b'a'};
        if !decrypt {
            (((self.a as u32 * (chr as u32 - base as u32) + self.b as u32)) % 26 + base as u32) as u8 as char
        } else {
            let modinverse = (0..26).find(|&x| (self.a * x) % 26 == 1).unwrap();
            let shifted = (chr as i32 - base as i32 - self.b as i32).rem_euclid(26);
            (((modinverse as i32 * shifted as i32)) % 26 + base as i32) as u8  as char
        }
    }
}

impl Cipher for Affine {
    fn encipher(&self,text : &str) -> String {
        text.chars().map(|chr| self.shift_char(chr, false)).collect()
    }

    fn decipher(&self,text : &str) -> String {
        text.chars().map(|chr| self.shift_char(chr, true)).collect()
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn shift_char() {
        assert_eq!(Affine::new(25,25).shift_char('A', false),'Z');
    }

    #[test]
    fn encipher() {
        assert_eq!(Affine::new(7,12).encipher("exampletext"),"ormsnloporp");
    }

    #[test]
    fn decipher() {
        assert_eq!(Affine::new(7,12).decipher("ormsnloporp"),"exampletext");
    }

    #[test]
    fn encipher_large_ab() {
        assert_eq!(Affine::new(17,24).encipher("exampletext"),"ozyutdojozj");
    }

    #[test]
    fn decipher_large_ab() {
        assert_eq!(Affine::new(17,24).decipher("ozyutdojozj"),"exampletext");
    }

    #[test]
    #[should_panic]
    fn invalid_a() {
        let _ = Affine::new(2,1);
    }

    #[test]
    #[should_panic]
    fn invalid_b() {
        let _ = Affine::new(17,27);
        
    }
    
}

