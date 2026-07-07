use std::fs;
use p5::part_a::part_a;
use p5::part_b::part_b;
use p5::part_c::part_c;

fn main() {
    let text = fs::read_to_string("./src/input.txt").expect("File Not Found");
    let out_a = part_a(text.clone());
    let out_b = part_b(text.clone());
    let out_c= part_c(text.clone());
    println!("{out_a} {:?} {out_c}", out_b);
}
