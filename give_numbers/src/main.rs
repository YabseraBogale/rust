use std::io;
//use std::cmp::Ordering;
fn main() {
    println!("Give there Numbers to order");
    let mut arr=[0,0,0];
    let mut input=String::new();
    let i=0;
    loop{
        io::stdin().read_line(&mut input).expect("try again.");
        arr[i]=input.trim().parse().expect("Enter Number");
        if i==3 {
            break;
        }
        println!("Array Values: {}",arr[i]);
        let _i=i+1;
    }
}
