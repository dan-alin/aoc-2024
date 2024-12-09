use std::collections::HashMap;

fn prepare(file: &str) -> HashMap<i64, Vec<i64>> {
    let mut hash_map: HashMap<i64, Vec<i64>> = HashMap::new();

    for line in file.lines() {
        let row: Vec<&str> = line.split(":").collect();
        hash_map.insert(
            row[0].parse::<i64>().unwrap(),
            row[1]
                .trim()
                .split(" ")
                .map(|x| x.parse::<i64>().unwrap())
                .collect(),
        );
    }

    hash_map
}

pub fn part_one(file: &str) -> i64 {
    let mut count = 0;
    let map = prepare(file);

    let mut combinations_map: HashMap<i64, Vec<Vec<char>>> = HashMap::new();

    for (key, value) in map.iter() {
        let combination_size = value.len() as i64;

        let val = combinations_map.get(&combination_size);

        if val.is_none() {
            combinations_map.insert(
                combination_size,
                generate_combinations((combination_size - 1) as usize),
            );
        }

        let combination = combinations_map.get(&combination_size).unwrap();

        for comb in combination {
            let mut merged: Vec<String> = Vec::new();
            for (i, v) in value.iter().enumerate() {
                merged.push(v.to_string());
                if i < comb.len() {
                    merged.push(comb[i].to_string());
                }
            }

            let merged_str: Vec<&str> = merged.iter().map(|s| s.as_str()).collect();

            let result = *key == evaluate_left_to_right(merged_str);

            if result {
                count += key;
                break;
            }
        }
    }

    count
}

fn generate_combinations(len: usize) -> Vec<Vec<char>> {
    let mut combinations = Vec::new();
    let total_combinations = 2_usize.pow(len as u32);

    for i in 0..total_combinations {
        let mut combination = Vec::with_capacity(len);
        for j in 0..len {
            if i & (1 << j) != 0 {
                combination.push('*');
            } else {
                combination.push('+');
            }
        }
        combinations.push(combination);
    }

    combinations
}

fn evaluate_left_to_right(tokens: Vec<&str>) -> i64 {
    let mut result = tokens[0].parse::<i64>().expect("Invalid number");

    let mut iter = tokens.iter().skip(1);
    while let (Some(&op), Some(&num)) = (iter.next(), iter.next()) {
        let number = num.parse::<i64>().expect("Invalid number");
        match op {
            "+" => result += number,
            "*" => result *= number,
            _ => panic!("Unsupported operator: {}", op),
        }
    }

    result
}

pub fn part_two(file: &str) -> i64 {
    0
}
