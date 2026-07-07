
pub fn remaining_found(text:String, already_found:Vec<char>) -> String{

    let tchars = text.chars().clone().collect::<Vec<char>>();
    let mut remaining:Vec<char> = Vec::new();
    for i in tchars{
        if already_found.contains(&i) || remaining.contains(&i){
            continue;
        } else{
            remaining.push(i);
        }
    }
    let res = remaining.iter().collect();
    res

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

pub fn part_b(text:String) -> String{

    let v: Vec<char> = text.trim().chars().filter(|c| c.is_alphabetic()).collect();
    let mut start = 0;
    let mut catch:Vec<char> = Vec::new();
    loop{
        
        let target = v.clone()[start];
        let search_index = final_indexes(v.clone(), target) ;
        let other_value_index = search_index.iter().find(|&&i| i != start).unwrap();
        // score += scores.get(&target).unwrap();
        catch.push(target);
        start = *other_value_index + 1;
        if start >= v.len(){
            break;
        }

    }
    remaining_found(text, catch)
}