extern crate rand;
use rand::Rng;
fn main() {
    let mut i=0;
    loop{
        println!("Random number {}",rand::thread_rng().gen_range(0, 6));
        i+=1;
        if i==6{
            break;
        }
    }
}
