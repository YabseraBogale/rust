extern crate rand;
use rand::Rng;
use std::io;
use std::cmp::Ordering;
fn main() {
    let secret_number=rand::thread_rng().gen_range(0, 101);
    println!("Guesing Game");
    loop{
        println!("Enter Number: ");
        let mut input=String::new();
        io::stdin().read_line(&mut input)
                .expect("Falied to get number");
        println!("You guessed: {}", input);
        let guess: i32=match input.trim().parse() {
            Ok(num)=> num,
            Err(_)=>continue,
        };
        match guess.cmp(&secret_number){
            Ordering::Less=> println!("Less"),
            Ordering::Greater=>println!("Greater"),
            Ordering::Equal=>{
                println!("You Win");
                break;
            }
        }
    }
}
