use crate::cipher::Cipher;

pub struct Redefence {
    code : String,
}

impl Redefence {
    pub fn new(code : String,) -> Self {
        Self {
            code : code.to_uppercase(),
        }
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
        let orders = letters.iter().map(|(value,index)| *index).collect::<Vec<usize>>();
        

        let rails = orders.len(); 
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
        for row in orders {
            for &chr in fenced_rails[(fences * row)..(fences * (row+1))].iter() {
                if chr != ' ' {
                    ciphertext.push(chr);
                }
            }
        }
        

        ciphertext
    }

    fn decipher(&self,text : &str) -> String {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn encipher() {
        assert_eq!(Redefence::new(String::from("abcd")).encipher("exampletext"),String::from("eexltapetmx"))
    }
    #[test]
    fn decipher() {
        //assert_eq!(Redefence::new(String::from("abcd")).decipher("eexltapetmx"),String::from("exampletext"))
    }
}