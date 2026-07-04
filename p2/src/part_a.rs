use std::fs;
use std::collections::HashMap;

pub fn part_a() -> i32{
    let mut hmap:HashMap<char, i32> = HashMap::new();
    let lines = fs::read_to_string("./src/input.txt").expect("File Not Found");
    hmap.insert('^', 1);
    hmap.insert('v', -1);
    let mut maxv = 0;
    let mut tot = 0;
    for c in lines.chars(){
        tot += hmap.get(&c).unwrap();
        if tot > maxv{
            maxv = tot;
        }
    }
    maxv
    // println!("{:?}", lines);
}