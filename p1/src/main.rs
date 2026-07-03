// use std::fs;

use p1::part_a::part_a;
use p1::part_b::part_b;
use p1::part_c::part_c;

fn main() {
    let out_a = part_a();
    let out_b = part_b();
    let out_c = part_c();
    println!("{:?} {:?} {:?}", out_a, out_b, out_c);
}