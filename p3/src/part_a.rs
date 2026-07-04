
use std::collections::HashMap ;
pub fn part_a<E>(text:String) -> Result<String,E> {

    let mut h:HashMap<&str, i32> = HashMap::new();
    for line in text.lines(){
         let count = h.entry(line).or_insert(0);
        *count += 1; // Dereference to update the value in place
    }

    if let Some((max_key, max_val)) = h.iter().max_by_key(|&(_, val)| val) {
        println!("The key with the max value is '{}' (value: {})", max_key, max_val);
        return Ok(max_key.to_string());
    // Output: The key with the max value is 'banana' (value: 42)
    }
    Ok("Error".to_string())
    
}