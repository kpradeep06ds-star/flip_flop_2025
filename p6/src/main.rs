use std::fs;
use p6::part_a::move_score;

fn main() {
    let text = fs::read_to_string(r"./src/input.txt").expect("FileNotFound");
    println!("{}", move_score(text.clone()));
}
