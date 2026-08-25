use crate::cipher::Cipher;

pub struct Railfence {
    key : u8,
}

impl Railfence {
    pub fn new(key : u8) -> Self{
        
        Self {
            key,
        }
    }

    pub fn brute_force(text : &str) -> Vec<String> {
        let mut vector : Vec<String> = Vec::new();
        for i in 2..text.len()   {
            vector.push(Self::new(i as u8).decipher(text));
        };
        vector
    }

    pub fn get_rail_indices(&self, len : usize) -> Vec<usize> {
        if self.key <= 1 {
            return vec![0; len]
        };

        let cycle = (self.key as usize-1) * 2;
        (0..len).map( |i|{
            let rem = i % cycle;
            if rem < self.key as usize {rem} else {cycle-rem}
        }
        ).collect()
    }
}

impl Cipher for Railfence {
    fn encipher(&self,text : &str) -> String {
        let rails = self.key as usize;
        let mut ciphertext = String::with_capacity(text.len());
        let indices = self.get_rail_indices(text.len());
        let mut chars = text.chars().collect::<Vec<char>>();
        for rail in 0..rails {
            for (index,&char_rail) in indices.iter().enumerate() {
                if rail == char_rail {
                    ciphertext.push(chars[index])
                }
            }
        }
        ciphertext

        
    }

    fn decipher(&self,text : &str) -> String {
        let indices= self.get_rail_indices(text.len());

        let mut plaintext = String::new();

        
        plaintext
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn encipher() {
        assert_eq!(Railfence::new(4).encipher("exampletext"),String::from("eexltapetmx"))
    }
    #[test]
    fn decipher() {
        assert_eq!(Railfence::new(4).decipher("eexltapetmx"),String::from("exampletext"))
    }

    #[test]
    fn encipher_mix_symbols() {
        assert_eq!(Railfence::new(4).encipher("3x@mplEtexT"),String::from("3Exlt@peTmx"))
    }
    #[test]
    fn decipher_mix_symbols() {
        assert_eq!(Railfence::new(4).decipher("3Exlt@peTmx"),String::from("3x@mplEtexT"))
    }

    #[test]
    fn encipher_larger_key() {
        assert_eq!(Railfence::new(15).encipher("3x@mplEtexT"),String::from("3x@mplEtexT"))
    }
    #[test]
    fn decipher_larger_key() {
        assert_eq!(Railfence::new(15).decipher("3x@mplEtexT"),String::from("3x@mplEtexT"))
    }
    

}