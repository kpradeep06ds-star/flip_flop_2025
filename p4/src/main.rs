use std::fs;
use p4::part_a::part_a;
use p4::part_b::part_b;
use p4::part_c::part_c;
// use std::convert::Infallible;

fn main() {
    let text = fs::read_to_string("./src/input.txt").expect("File Not Found");
    let out_a:i32 = part_a(text.clone());
    let out_b:i32 = part_b(text.clone());
    let out_c:i32 = part_c(text.clone());
    println!("{out_a} {out_b} {out_c}");
 
}
