use crate::Cipher;
/// Custom Affine cipher that operates over an arbitrary alphabet string.
        ///
        /// The alphabet must contain unique characters. The multiplicative parameter
        /// `a` must be invertible modulo the alphabet length (gcd(a, len) == 1).
        ///
        /// # Examples
        ///
        /// ```rust
        /// use cifers::custom_alphabet_ciphers::custom_affine::CustomAffine;
        /// let c = CustomAffine::new(String::from("abcdefghijklmnopqrstuvwxyz")).set_a(7).set_b(12);
        /// assert_eq!(c.encipher("exampletext"), "ormsnloporp");
        ///


    fn gcd(mut a: u16, mut b: u16) -> u16 {
        while b != 0 {
            let temp = b;
            b = a % temp;
            a = temp;
        }
        a
    }

        pub struct CustomAffine {
            a: u16,
            b: u16,
            alphabet: String,
        }
        impl CustomAffine {
            pub fn new(alphabet: String) -> Self {
                Self { a: 1, b: 0, alphabet }
            }
            pub fn set_a(mut self, a: u16) -> Self {
                assert!(gcd(a, self.alphabet.len() as u16) == 1);
                self.a = a;
                self
            }

            pub fn set_b(mut self, b: u16) -> Self {
                assert!(b <= self.alphabet.len() as u16 - 1);
                self.b = b;
                self
            }

            pub fn shift_char(&self, chr: char, decrypt: bool) -> char {
                let x = match self.alphabet.find(chr) {
                    Some(i) => i as u16,
                    None => return chr,
                };
                if decrypt {
                    let modinverse = (0..self.alphabet.len()).find(|&x| (self.a * x as u16) % self.alphabet.len() as u16 == 1).unwrap();
                    self.alphabet.chars().nth(((modinverse as i16 * (x as i16 - self.b as i16)).rem_euclid(self.alphabet.len() as i16)) as usize).unwrap()
                } else {
                    self.alphabet.chars().nth(((self.a * x + self.b) % self.alphabet.len() as u16) as usize).unwrap()
                }
            }
        }

        impl Cipher for CustomAffine {
            fn encipher(&self, text: &str) -> String {
                text.chars().map(|c| self.shift_char(c, false)).collect()
            }

            fn decipher(&self, text: &str) -> String {
                text.chars().map(|c| self.shift_char(c, true)).collect()
            }
        }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn encipher() {
        assert_eq!(CustomAffine {a : 7,b : 12, alphabet : String::from("abcdefghijklmnopqrstuvwxyz")}.encipher("exampletext"),"ormsnloporp");
    }

    #[test]
    fn decipher() {
        assert_eq!(CustomAffine {a : 7,b : 12, alphabet : String::from("abcdefghijklmnopqrstuvwxyz")}.decipher("ormsnloporp"),"exampletext");
    }
    
}