pub mod ciphers;
pub mod custom_alphabet_ciphers;

pub use ciphers::caesar::Caeser;
pub use ciphers::vigenere::Vigenere;
pub use ciphers::beaufort::Beaufort;
pub use ciphers::redefence::Redefence;
pub use ciphers::affine::Affine;
pub use ciphers::railfence::Railfence;

pub trait Cipher {
    fn encipher(&self,text : &str) -> String;
    fn decipher(&self,text : &str) -> String; 
}


fn index_of_coincidence(text : &str) -> f64 {
    let mut counts = [0u32; 26];
    let mut total_letters = 0;     

    for byte in text.bytes() {
        if byte.is_ascii_alphabetic() {
            counts[(byte.to_ascii_lowercase() - b'a') as usize] +=1;
            total_letters+=1;
        }
    }
    if total_letters < 2 {
        return 0.0
    }

    let numerator : u32  = counts.iter().map(|&x| x * (x-1)).sum();
    numerator as f64 / (total_letters as f64 * (total_letters as f64-1.0))

}

