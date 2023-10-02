#[derive(Debug)]
struct Color(i32,i32,f32);

fn main() {
    let white=Color(0,1,2.5);
    //println!("White at 0 {}",white[0]) doesn't work;
    println!("White at 0 {}",white.0);
    println!("White at 1 {}",white.1);
    println!("White at 2 {}",white.2);
}
