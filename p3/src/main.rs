use std::fs;
use p3::part_a::part_a;
use p3::part_b::part_b;
use std::convert::Infallible;

pub fn part_c<E>(text:String) -> i32 {
    let out_b:Result<(i32,i32,i32,i32),Infallible> = part_b::<std::convert::Infallible>(text.clone());
    let v = out_b.unwrap();
    let (r, g, b, s) = (v.0*5, v.1*2, v.2*4, v.3*10);
    r + g + b + s
}


fn main() {
    // println!("Hello, world!");
    let text = fs::read_to_string("./src/input.txt").expect("File Not Found");
    let out_a:Result<String,Infallible> = part_a::<std::convert::Infallible>(text.clone());
    let out_b:Result<(i32,i32,i32,i32),Infallible> = part_b::<std::convert::Infallible>(text.clone());    
    let out_c:i32 = part_c::<std::convert::Infallible>(text.clone());
    println!("{:?} {:?} {:?}", out_a, out_b, out_c);
}
