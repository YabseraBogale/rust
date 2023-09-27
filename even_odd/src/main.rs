use std::cmp::Ordering;
fn main() {
    let mut i:i32=0;
    loop{
        match i.cmp(&{2}){
            Ordering::Equal=> {
                println!("number is {}",2);
                break;
            },
            Ordering::Greater=>println!("greater"),
            Ordering::Less=>println!("Less"),
        }
        i+=1;
    }
}
