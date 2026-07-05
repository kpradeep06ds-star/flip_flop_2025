
pub fn helper(a:(i32, i32), b:(i32 , i32)) -> i32{
    let mut start = a;
    
    let mut distance:i32 = 0;

    loop {

        if start.0 == b.0 && start.1 == b.1 {
            return distance;
        }

        if start.0 != b.0 && start.1 != b.1 && b.0 > start.0 && b.1 > start.1{
            distance += 1;
            start = (start.0+1, start.1+1);
        } else if start.0 != b.0 && start.1 != b.1 && b.0 < start.0 && b.1 > start.1{
            distance += 1;
            start = (start.0-1, start.1+1);
        } else if start.0 == b.0 && start.1 != b.1{
            distance += (b.1 - start.1).abs();
            break;
        } else if start.0 != b.0 && start.1 == b.1 {
            distance += (b.0 - start.0).abs();
            break;
        }
    }

    distance

}

pub fn part_b(text:String) -> i32 {

    // let x = 2;
    let mut cordinate_stack:Vec<(i32, i32)> = vec![(0, 0)];
    for line in text.lines(){
        //let cordinates: Vec<i32> = line.split(",").map(|c| c.parse().unwrap()).collect::<Vec<i32>>();
        let mut parts = line.split(",").map(|c| c.trim().parse::<i32>().unwrap());
        let coordinates: (i32, i32) = (parts.next().unwrap(), parts.next().unwrap());
        
        cordinate_stack.push(coordinates);
    }

    let mut d = 0;
    let break_length = cordinate_stack.len();
    
    for i in 0..(break_length-1){
        let a=  cordinate_stack[i];
        let b= cordinate_stack[i+1];
        if a.1 <= b.1{
            d += helper(a, b);
        } else if a.1 >= b.1{
            d += helper(b, a);
        }
    }
    d
}