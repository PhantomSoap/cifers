pub mod cipher;
pub mod caeser;
pub mod vigenere;
pub mod railfence;
pub mod affine;
pub mod beaufort;
pub mod redefence;


pub use caeser::Caeser;
pub use vigenere::Vigenere;
pub use beaufort::Beaufort;
pub use redefence::Redefence;
pub use affine::Affine;


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

