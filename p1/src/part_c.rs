use std::fs;

pub fn part_c() -> usize{ 
    
    let mut countlines = 0;
    let input_data = fs::read_to_string("./src/input.txt").expect("file doesn't present");
    // trim doesn't return a new word it returnsa reference hence the .to_string()
    // replace is different it returns a word not a reference 
    for line in input_data.lines(){
        if !line.contains("ne"){
            let temp = line.replace("ba", " ba ").replace("na", " na ").trim().to_string();
        //countlines += temp.replace("  ", " ").split(" ").filter(|c| *c == "na" || *c == "ne" || *c == "ba" ).collect::<Vec<&str>>().len();
            let tempcount = temp.replace("  ", " ").split(" ").filter(|c| *c == "na"  || *c == "ba" ).collect::<Vec<&str>>().len();
            countlines += tempcount;
        }
    }
    // problem 1a
    //println!("{:?}", countlines);
    countlines
}