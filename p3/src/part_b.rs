// two are equal then special
// all three are unique but one of them is high then r, g, b

pub fn two_equal(v:Vec<&str>) -> bool{
    let b: bool = v.get(0).unwrap() == v.get(1).unwrap() || v.get(1).unwrap() == v.get(2).unwrap() || v.get(0).unwrap() == v.get(2).unwrap() ;
    b
}

pub fn rgb(v: Vec<&str>) -> i32 {
    let r = v.get(0).unwrap() > v.get(1).unwrap() && v.get(0).unwrap() > v.get(2).unwrap();
    let g = v.get(1).unwrap() > v.get(2).unwrap() && v.get(1).unwrap() > v.get(0).unwrap();
    let b = v.get(2).unwrap() > v.get(0).unwrap() && v.get(2).unwrap() > v.get(1).unwrap();
    if two_equal(v){
        return 4;
    } else {
        if r == true{
            return 1;
        } else if g == true{
            return 2;
        } else if b == true{
            return 3;
        } else {
            return 0;
        }
     }
}

pub fn part_b<E>(text:String) -> Result<(i32,i32,i32,i32),E> {

    let mut green = 0;
    let mut red = 0;
    let mut blue = 0;
    let mut special = 0;
    for line in text.lines(){
        let linev = line.split(",").collect::<Vec<&str>>();
        let rgb_value = rgb(linev);
        //println!("{rgb_value}");
        if rgb_value == 2{
            green += 1;
        } else if rgb_value == 1{
            red += 1;
        } else if rgb_value == 3{
            blue += 1;
        } else if rgb_value == 4{
            special += 1;
        }
    }

    Ok((red, green, blue, special))    
}