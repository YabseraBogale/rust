use std::{io, num::ParseIntError};
fn main() {
    loop {
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
            let fel:f64=fel.trim().parse().expect("Enter number");
            println!("Enterd Fahrenheit is {}",fel);
            let cel:f64=(fel-32.0)*5.0/9.0;
            println!("Celsius is {}",cel);
        } else if input==2 {
            let mut cel=String::new();
            println!("Enter Celsius to Fahrenheit");
            io::stdin().read_line(&mut cel).expect("Try again");
            let cel:f64=cel.trim().parse().expect("Enter number");
            let fel:f64=(cel*(9.0/5.0)) + 32.0;
            println!("Enterd Celsius is {}",cel);
            println!("Fahrenheit is {}",fel);
        } else{
            println!("Enter thr right number");
        }
        let mut go=String::new();
        println!("Enter 'q' to exit else to contiune");
        io::stdin().read_line(&mut go).expect("Try again");
        let go=go.trim();
        if go=="q"{
            println!("Exiting");
            break;
        }
    }
}
