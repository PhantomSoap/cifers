use crate::cipher::Cipher;

pub struct RailFence {
    key : u8,
}

impl RailFence {
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
}

impl Cipher for RailFence {
    fn encipher(&self,text : &str) -> String {
        let rails = self.key as usize; 
        let fences = text.len(); 
        let mut fenced_rails: Vec<char> = vec![' '; rails * fences];
        let letters: Vec<char> = text.chars().collect();
        let mut rows = 0;
        let mut down = false;
        for (index, &chr) in letters.iter().enumerate() {
            if rows == rails - 1 || rows == 0 {
                down = !down;
            }
            fenced_rails[(fences * (rows)) + index] = chr;
            if rails != 1 {
                if down {
                    rows += 1;
                } else {
                    rows -= 1;
                }
            }
        }
        let mut ciphertext = String::new();
        for chr in fenced_rails {
            if chr != ' ' {
                ciphertext.push(chr);
            }
        }

        ciphertext
    }

    fn decipher(&self,text : &str) -> String {
        let rails = self.key as usize; 
        let fences = text.len(); 
        let mut fenced_rails: Vec<char> = vec![' '; rails * fences];
        let letters: Vec<char> = text.chars().collect();
        let mut rows = 0;
        let mut down = false;
        for (index, &chr) in letters.iter().enumerate() {
            if rows == rails - 1 || rows == 0 {
                down = !down;
            }
            fenced_rails[(fences * (rows)) + index] = '?';
            if rails != 1 {
                if down {
                    rows += 1;
                } else {
                    rows -= 1;
                }
            }
        }
        
        let mut text_chars = text.chars();
        for chr in &mut fenced_rails {
            if *chr == '?' {
                *chr = text_chars.next().unwrap();
            }
        }

        let mut plaintext = String::new();

        let mut rows = 0;
        let mut down = false;
        for (index, &chr) in letters.iter().enumerate() {
            if rows == rails - 1 || rows == 0 {
                down = !down;
            }
            plaintext.push(fenced_rails[(fences * rows) + index]);
            if rails != 1 {
                if down {
                    rows += 1;
                } else {
                    rows -= 1;
                }
            }
        }
        plaintext
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn encipher() {
        assert_eq!(RailFence::new(4).encipher("exampletext"),String::from("eexltapetmx"))
    }
    #[test]
    fn decipher() {
        assert_eq!(RailFence::new(4).decipher("eexltapetmx"),String::from("exampletext"))
    }
}