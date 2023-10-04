#[derive(Debug)]
enum IpAdrr{
    V4(u8,u8,u8,u8),
    V6(String),
}
fn main() {
    let v4=IpAdrr::V4(127,0,0,1);
    let v6=IpAdrr::V6(String::from("google.com"));
    println!("Hello, Enum Of ipv4 type {:?}",v4);
    println!("Hello, Enum Of ipv4 type {:?}",v6);
}
