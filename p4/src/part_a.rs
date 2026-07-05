pub fn part_a(text:String) -> i32 {

    let mut start_point : (i32, i32) = (0, 0);
    // let x = 2;
    let mut cordinate_stack:Vec<(i32, i32)> = Vec::new();
    for line in text.lines(){
        //let cordinates: Vec<i32> = line.split(",").map(|c| c.parse().unwrap()).collect::<Vec<i32>>();
        let mut parts = line.split(",").map(|c| c.trim().parse::<i32>().unwrap());
        let coordinates: (i32, i32) = (parts.next().unwrap(), parts.next().unwrap());
        
        cordinate_stack.push(coordinates);
    }
    let mut i = 0;
    let mut d = 0;
    let break_length = cordinate_stack.len();
    loop {
        
        let (a, b) = cordinate_stack.get(i).unwrap() ;
        d += (a - start_point.0).abs() + (b - start_point.1).abs() ;
        start_point = (*a, *b);
        i+= 1;
        if i >= break_length {
            break;
        }
    }
    d
}