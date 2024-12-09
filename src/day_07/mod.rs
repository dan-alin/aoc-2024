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

fn generate_combinations(len: usize, operators: &Vec<char>) -> Vec<Vec<char>> {
    let mut combinations = Vec::new();

    let total_combinations = operators.len().pow(len as u32);

    for i in 0..total_combinations {
        let mut combination = Vec::with_capacity(len);
        let mut num = i;
        for _ in 0..len {
            combination.push(operators[num % operators.len()]);
            num /= operators.len();
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
            "|" => {
                result = (result.to_string() + &number.to_string())
                    .parse::<i64>()
                    .unwrap()
            }
            _ => panic!("Unsupported operator: {}", op),
        }
    }

    result
}

#[allow(dead_code)]
fn evaluate_right_to_left_with_target(tokens: &[&str], target: i64) -> bool {
    if tokens.len() == 1 {
        return tokens[0].parse::<i64>().map_or(false, |n| n == target);
    }

    let last_number = tokens[tokens.len() - 1]
        .parse::<i64>()
        .expect("Invalid number");

    let remaining_tokens = &tokens[..tokens.len() - 2];
    let operator = tokens[tokens.len() - 2];

    match operator {
        "+" => {
            let new_target = target - last_number;
            evaluate_right_to_left_with_target(remaining_tokens, new_target)
        }
        "*" => {
            if target % last_number == 0 {
                let new_target = target / last_number;
                evaluate_right_to_left_with_target(remaining_tokens, new_target)
            } else {
                false
            }
        }
        "|" => {
            let last_str = last_number.to_string();
            let target_str = target.to_string();
            if target_str.ends_with(&last_str) {
                let new_target = target_str[..target_str.len() - last_str.len()]
                    .parse::<i64>()
                    .unwrap_or(-1);
                evaluate_right_to_left_with_target(remaining_tokens, new_target)
            } else {
                false
            }
        }
        _ => panic!("Unsupported operator: {}", operator),
    }
}

pub fn part_one(file: &str) -> i64 {
    let mut count = 0;
    let map = prepare(file);
    let operators = vec!['+', '*'];

    let mut combinations_map: HashMap<i64, Vec<Vec<char>>> = HashMap::new();

    for (key, value) in map.iter() {
        let combination_size = value.len() as i64;

        let val = combinations_map.get(&combination_size);

        if val.is_none() {
            combinations_map.insert(
                combination_size,
                generate_combinations((combination_size - 1) as usize, &operators),
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

#[allow(dead_code)]
pub fn part_one_rtl(file: &str) -> i64 {
    let mut count = 0;
    let map = prepare(file);
    let operators = vec!['+', '*'];

    let mut combinations_map: HashMap<i64, Vec<Vec<char>>> = HashMap::new();

    for (key, value) in map.iter() {
        let combination_size = value.len() as i64;

        let val = combinations_map.get(&combination_size);

        if val.is_none() {
            combinations_map.insert(
                combination_size,
                generate_combinations((combination_size - 1) as usize, &operators),
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

            let result = evaluate_right_to_left_with_target(&merged_str, *key);

            if result {
                count += key;
                break;
            }
        }
    }

    count
}

pub fn part_two(file: &str) -> i64 {
    let mut count = 0;
    let map = prepare(file);
    let operators = vec!['+', '*', '|'];

    let mut combinations_map: HashMap<i64, Vec<Vec<char>>> = HashMap::new();

    for (key, value) in map.iter() {
        let combination_size = value.len() as i64;

        let val = combinations_map.get(&combination_size);

        if val.is_none() {
            combinations_map.insert(
                combination_size,
                generate_combinations((combination_size - 1) as usize, &operators),
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

#[allow(dead_code)]
pub fn part_two_rtl(file: &str) -> i64 {
    let mut count = 0;
    let map = prepare(file);
    let operators = vec!['+', '*', '|'];

    let mut combinations_map: HashMap<i64, Vec<Vec<char>>> = HashMap::new();

    for (key, value) in map.iter() {
        let combination_size = value.len() as i64;

        let val = combinations_map.get(&combination_size);

        if val.is_none() {
            combinations_map.insert(
                combination_size,
                generate_combinations((combination_size - 1) as usize, &operators),
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

            let result = evaluate_right_to_left_with_target(&merged_str, *key);

            if result {
                count += key;
                break;
            }
        }
    }

    count
}
