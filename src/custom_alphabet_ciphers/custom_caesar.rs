pub struct CustomCaesar {
    shift : i32,
    alphabet : String,
}

impl CustomCaesar {
    fn shift_char(&self,c : char,shift : i32) -> char{
        if let Some(index) = self.alphabet.chars().position(|x|x==c) {
            self.alphabet.chars().nth((index + shift as usize).rem_euclid(self.alphabet.len())).unwrap_or(c)
        } else {
            c
        }
        
    }
}

