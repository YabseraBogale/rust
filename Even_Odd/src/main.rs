extern crate rand;
use rand::Rng;
fn main() {
    
    loop{
        let num=rand::thread_rng().gen_range(0, 7);
        if num%2==0{
            break;
        }
        println!("Random number {}",num);
    }
}
