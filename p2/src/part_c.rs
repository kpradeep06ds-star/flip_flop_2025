use std::fs;
// use std::collections::HashMap;

// pub fn fib(n:i32, mut h:HashMap<i32, u128>) -> u128{
//     if h.contains_key(&n){
//         return *h.get(&n).unwrap();
//     } else{
//         for i in 3..=n{
//             if !h.contains_key(&i){
//                 h.insert(i, h.get(&(i - 1)).unwrap() + h.get(&(i - 2)).unwrap());
//             } 

//         }
//     }
//     *h.get(&n).unwrap()
// }



pub fn part_c() -> i64{

    let lines = fs::read_to_string("./src/input.txt").expect("File Not Found");
    let fib:Vec<i64> = vec![
        0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765,
        10946, 17711,28657,46368
    ];
    
    let mut tot:i64 = 0;
    let mut maxv:i64 = 0;
    let mut prev:char = 'k';
    let mut i = 0;
    let mut pos_flag ;
    
    for c in lines.chars(){
        if c == '^' && prev == c{
            i += 1;
            //pos_flag = 1;
        } else if c == 'v' && prev == c{
            i += 1;
            //pos_flag = -1;
        } else if c == '^' && prev != c{
            // i = 1;
            pos_flag = -1;
            //println!("{i}");
            tot += fib.get(i).unwrap()*pos_flag;
            i = 1;
            
        } else if c == 'v' && prev != c{
            pos_flag = 1;
            //println!("{i}");
            tot += fib.get(i).unwrap()*pos_flag;
            i = 1;
            
        }
        
        prev = c;
        if maxv < tot{
            maxv = tot;
        }
    }

    maxv
   
}