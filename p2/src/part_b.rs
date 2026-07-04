use std::fs;

pub fn part_b() -> i32{

    let lines = fs::read_to_string("./src/input.txt").expect("File Not Found");

    let mut prev:char = 'k';
    let mut i = 0;
    let mut tot = 0;
    let mut maxv: i32 = 0;

    for c in lines.chars(){
        if c == '^' && prev == c{
            i += 1;
        } else if c == 'v' && prev == c{
            i -= 1;
        } else if c == '^' && prev != c{
            i = 1;
        } else if c == 'v' && prev != c{
            i = -1;
        }
        tot += i;
        prev = c;
        if maxv < tot{
            maxv = tot;
        }
    }

    maxv

}