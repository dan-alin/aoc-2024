
fn prepare(file:&str) -> (Vec<i32>, Vec<i32>){

    let mut left:Vec<i32> = Vec::new();
    let mut right:Vec<i32> = Vec::new();

    for line in file.lines() {
        if let [left_value, right_value] = line.split_whitespace().collect::<Vec<&str>>()[..] {
            left.push(left_value.parse().unwrap());
            right.push(right_value.parse().unwrap());
        }
    }

    (left, right)
    
}



pub fn part_one(file:&str)-> i32 {
    let (mut left, mut right) = prepare(file);

    let mut count = 0;
    
    left.sort();
    right.sort();

    for i in 0..left.len() {
        count += (left[i] - right[i]).abs();
    }

    count
    
}


pub fn part_two(file:&str)-> i32 {
    let ( left,  right) = prepare(file);

    let mut similarity = 0;
    

    let mut right_map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();
    for value in right {
        *right_map.entry(value).or_insert(0) += 1;
    }

    for value in left {
        if let Some(count) = right_map.get(&value) {
            similarity += value * count;
        }
    }

    similarity
}