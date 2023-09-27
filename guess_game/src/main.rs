extern crate rand;
use rand::Rng;
use std::io;
fn main() {
    let secret_number=rand::thread_rng().gen_range(0, 101);
    println!("the secrete number is {}",secret_number);
    println!("Guesing Game");
    println!("Enter Number: ");
    let mut input=String::new();
    io::stdin().read_line(&mut input)
                .expect("Falied to get number");
    println!("You guessed: {}", input);
}
