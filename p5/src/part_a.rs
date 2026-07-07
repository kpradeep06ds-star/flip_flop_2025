// create a hashmap with distance b/w the values 
use std::{collections::HashMap};

pub fn score(text:String) -> HashMap<char, i32>{

    let mut h:HashMap<char, i32> = HashMap::new();
    for (idx, t) in text.chars().enumerate(){
        let id = (idx + 1) as i32;
        if h.contains_key(&t){
            let new_value =  id as i32 - h.get(&t).unwrap();
            h.insert(t, new_value.try_into().unwrap());
        } else{
            h.insert(t, id as i32);
        }
    }
    //println!("{:?}", h);
    h

}

pub fn final_indexes(v:Vec<char>, target:char) -> Vec<usize>{
    
    let indices_a: Vec<usize> = v
        .iter()
        .enumerate()
        .filter(|&(_, &value)| value == target)
        .map(|(index, _)| index)
        .collect();
    indices_a
}

pub fn part_a(text:String) -> i32{
    let scores = score(text.clone());
    let v = text.clone().chars().collect::<Vec<char>>();
    let mut start = 0;
    let mut score = 0;
    loop{
        if start >= v.len(){
            break;
        }
        let target = v.clone()[start];
        let search_index = final_indexes(v.clone(), target) ;
        let other_value_index = search_index.iter().find(|&&i| i != start).unwrap();
        score += scores.get(&target).unwrap();
        start = *other_value_index + 1;
    }
    score
}