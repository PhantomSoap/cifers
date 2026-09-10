use crate::Cipher;

pub struct Redefence {
    code : String,
}

impl Redefence {
    pub fn new() -> Self {
        Self {
            code : String::new(),
        }
    }

    pub fn set_code(mut self,code : String,) -> Self {
        self.code = code.to_ascii_uppercase();
        self
    }

    pub fn get_rail_indices(&self, len : usize) -> Vec<usize> {
        if self.code.len() <= 1 {
            return vec![0; len]
        };

        let cycle = (self.code.len() as usize-1) * 2;
        (0..len).map( |i|{
            let rem = i % cycle;
            if rem < self.code.len() as usize {rem} else {cycle-rem}
        }
        ).collect()
    }

}

impl Cipher for Redefence {
    fn encipher(&self,text : &str) -> String {
        
        let mut letters = self.code
            .chars()
            .enumerate()
            .map(|(index,value)| (value as u8 - b'A',index))
            .collect::<Vec<(u8,usize)>>();
        letters.sort_by_key(|&(value,_index)| value);
        let orders = letters.iter().map(|(_value,index)| *index).collect::<Vec<usize>>();
        

        let mut ciphertext = String::with_capacity(text.len());
        let indices = self.get_rail_indices(text.len());
        let chars = text.chars().collect::<Vec<char>>();
        for &rail in orders.iter() {
            for (index,&char_rail) in indices.iter().enumerate() {
                if rail == char_rail {
                    ciphertext.push(chars[index])
                }
            }
        }
        ciphertext
        


    }

    fn decipher(&self,text : &str) -> String {
        let mut letters = self.code
            .chars()
            .enumerate()
            .map(|(index,value)| (value as u8 - b'A',index))
            .collect::<Vec<(u8,usize)>>();
        letters.sort_by_key(|&(value,_index)| value);
        let orders = letters.iter().map(|(_value,index)| *index).collect::<Vec<usize>>();
        let indices = self.get_rail_indices(text.len());
        let mut plaintext = vec![' '; text.len()];
        let mut text_chars = text.chars();
        for &rail in orders.iter() {
            for (index,_chr_rail) in indices.iter().enumerate().filter(|(_index,chr_rail)| **chr_rail == rail) {
                plaintext[index] = text_chars.next().unwrap()
            }
        }
        
        

        plaintext.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn encipher() {
        assert_eq!(Redefence::new().set_code(String::from("abcd")).encipher("exampletext"),String::from("eexltapetmx"))
    }
    #[test]
    fn decipher() {
        assert_eq!(Redefence::new().set_code(String::from("abcd")).decipher("eexltapetmx"),String::from("exampletext"))
    }

    #[test]
    fn encipher_mix_symbols() {
        assert_eq!(Redefence::new().set_code(String::from("code")).encipher("3x@mplEtexT"),String::from("3Emxxlt@peT"))
    }
    #[test]
    fn decipher_mix_symbols() {
        assert_eq!(Redefence::new().set_code(String::from("code")).decipher("3Emxxlt@peT"),String::from("3x@mplEtexT"))
    }
}