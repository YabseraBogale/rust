#[derive(Debug)]
struct Rect{
    w:f64,
    h:f64,
    area:f64,
}

impl Rect{
    fn Area(&self)-> f64{
        self.w*self.h
    }
}


fn main() {
    let rect_1=Rect{
        w:5.6,
        h:5.9,
        area:0.0,
    };
    println!("Rectangle: {:#?}",rect_1);
    let rect_1=Rect{
        area:rect_1.Area(),
       ..rect_1
        };
    println!("Rectangle: {:#?}",rect_1);
    println!("Rectangle area : {:?}",rect_1.area);
}
