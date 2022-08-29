pub mod encrypter;
use std::io;
use encrypter::Encryptable;
fn main() {
    println!("Input the String you want to encrypt");
    let mut user_input=String::new();

    io::stdin().read_line(&mut user_input).expect("cannot read input");
    println!("Your encrypted String : {}",  encrypter::rot13::Rot13(user_input).encrypt());
}
