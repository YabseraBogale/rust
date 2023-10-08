use std::io;
use  std::io::Read;
use std::fs::File;


fn read_file()->Result<String,io::Error>{
    let mut f=File::open("hello.txt")?;
    let mut s=String::new();
    f.read_to_string(&mut s)?;
    return Ok(s);
}
fn main() {
   read_file();

}
