use std::io;
fn main() {
    println!("Guesing Game");
    println!("Enter Number: ");
    let mut input=String::new();
    io::stdin().read_line(&mut input)
                .expect("Falied to get number");
    println!("You guessed: {}", input);
}
