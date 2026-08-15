pub trait Cipher {
    fn encipher(&self,text : &str) -> String;
    fn decipher(&self,text : &str) -> String; 
}