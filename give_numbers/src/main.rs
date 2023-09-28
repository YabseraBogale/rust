use std::io;
use std::cmp::Ordering;
fn main() {
    println!("Give there Numbers to order");
    let mut arr=[0,0,0];
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("try again.");
    arr[0]=input.trim().parse().expect("Enter Number");
    io::stdin().read_line(&mut input).expect("try again.");
    arr[1]=input.trim().parse().expect("Enter Number");
    io::stdin().read_line(&mut input).expect("try again.");
    arr[2]=input.trim().parse().expect("Enter Number");
    let i=0;
    loop{
        if i==3 {
            break;
        }
        println!("Array Values: {}",arr[i]);
        let i=i+1;
    }
}
