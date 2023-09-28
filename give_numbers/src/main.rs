use std::io;
use std::cmp::Ordering;
fn main() {
    println!("Give there Numbers to order");
    let mut input;
    io::stdin.read_line(&input).execpet("try agin.");
    println!("Input is {}",input);
}
