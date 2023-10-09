fn main() {
    let numbers=vec![1,50,-5,9];
    let large=largest(&numbers);
    println!("largest number in the Vector {:?} is {}",numbers,large);
}


fn largest(arr:&[i32])-> i32{
    let mut large=arr[0];
    for &number in arr.iter(){
        if number>large {
                large=number;
        }
    }
    large
}