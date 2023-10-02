#[derive(Debug)]

struct Rect{
	w: f64,
	h: f64,
}

fn main() {

	let rect=Rect{w:5_f64,h:9_f64}; 
    	println!("Rect is{:?}",rect);
	let area={
	rect.w*rect.h
};
	println!("Area of Rect is {}",area);
	let area=rect_area(&rect);
	println!("Area of Rect with pointer {}",area);
}

fn rect_area(rr: &Rect)->f64{
	rr.w*rr.h
}
