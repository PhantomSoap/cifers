use std::collections::HashSet;

pub mod affine;
pub mod beaufort;
pub mod caesar;
pub mod railfence;
pub mod vigenere;
pub mod redefence;

pub fn has_duplicate(alphabet : &str) -> bool {
    let mut chars : HashSet<char> = HashSet::new();
    for c in alphabet.chars() {
         if !chars.insert(c) {
            return true
         }
    } 
    false

}