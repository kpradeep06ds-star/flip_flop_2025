// pub fn helper_movement(v:Vec<i64>, sky_grid:i64) -> Vec<i64>{
//     let mut vnew = Vec::new();
//     let mut is_negative_x = 0;
//     let mut is_negative_y = 0;
    
//     if v[0] == sky_grid || v[0] == 0{
//         vnew.push(0);
//     } else if v[0] < 0 {
//         vnew.push(sky_grid - v[0]);
//         is_negative_x = 1;
//     } else{
//         vnew.push(v[0]+1);
//     }

//     if v[1] == sky_grid || v[1] == 0{
//         vnew.push(0);
//     } else if v[1] < 0 {
//         vnew.push(sky_grid - v[1]);
//         is_negative_y = 1;
//     } else{
//         vnew.push(v[1]+1);
//     }

//     vnew.push(is_negative_x);
//     vnew.push(is_negative_y);

//     vnew
// }
pub fn helper(v:Vec<i64>) -> Vec<i64>{
    
}

pub fn move_score(text:String, sky_grid:i64, pframe:i64) -> i64{
    let mut v:Vec<Vec<i64>> = Vec::new();
    for line in text.lines(){
        let temp:Vec<i64> = line.split(",").into_iter().map(|c| c.parse().unwrap()).collect();
        v.push(temp);
    }
    for i in 0..100{
        
    }
    2
}