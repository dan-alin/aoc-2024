use std::fs;

fn prepare() -> (Vec<i32>, Vec<i32>){
    let file = fs::read_to_string("src/day_one/input.txt").expect("file not found");
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

pub fn day_one_1() {
    let (mut left, mut right) = prepare();

    let mut count = 0;
    
    left.sort();
    right.sort();

    for i in 0..left.len() {
        count += (left[i] - right[i]).abs();
    }

    println!("day one part 1: {}", count);
    
}


pub fn day_one_2() {
    let ( left,  right) = prepare();

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

    println!("day one part 2: {}", similarity);
}