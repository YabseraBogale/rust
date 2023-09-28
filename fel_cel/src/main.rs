use std::io;
fn main() {
    println!("Temprture Converter");
    println!("press '1' for Fahrenheit to Celsius");
    println!("press '2' for Celsius to Fahrenheit: ");
    let mut input=String::new();
    io::stdin().read_line(&mut input).expect("Try again");
    println!("you have entered {}",input);
    let input:i32=input.trim().parse().expect("Enter Number");
    if input==1{
        let mut fel=String::new();
        println!("Enter Fahrenheit to Celsius");
        io::stdin().read_line(&mut fel).expect("Try again");
        let fel:f32=fel.trim().parse().expect("Enter number");
        println!("Enterd Fahrenheit is {}",fel);
    } else {
        println!("ok");
    }
}
