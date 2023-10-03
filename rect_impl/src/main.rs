#[derive(Debug)]
struct Rect{
    w:f64,
    h:f64,
}

impl Rect{
    fn area(&self)-> f64{
        self.w*self.h
    }
}


fn main() {
    let rect_1=Rect{
        w:5.6,
        h:5.9,
    };
    println!("Rectangle: {:#?}",rect_1);
    println!("Rectangle area : {:?}",rect_1.area());
}
