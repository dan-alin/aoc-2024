use std::collections::HashMap;

pub fn part_one(file: &str) -> i64 {
    let mut frequencies = HashMap::new();
    for stone in file.split_whitespace().map(|s| s.parse::<i64>().unwrap()) {
        *frequencies.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..25 {
        let mut new_frequencies = HashMap::new();
        for (stone, count) in frequencies.iter() {
            match stone {
                0 => {
                    *new_frequencies.entry(1).or_insert(0) += count;
                }
                _ if stone.to_string().len() % 2 == 0 => {
                    let stone_str = stone.to_string();
                    let mid = stone_str.len() / 2;
                    let (left, right) = stone_str.split_at(mid);
                    let left: i64 = left.parse().unwrap();
                    let right: i64 = right.parse().unwrap();
                    *new_frequencies.entry(left).or_insert(0) += count;
                    *new_frequencies.entry(right).or_insert(0) += count;
                }
                _ => {
                    let new_stone = stone * 2024;
                    *new_frequencies.entry(new_stone).or_insert(0) += count;
                }
            }
        }
        frequencies = new_frequencies;
    }

    frequencies.iter().map(|(_, count)| *count).sum::<usize>() as i64
}

pub fn part_two(file: &str) -> i64 {
    let mut frequencies = HashMap::new();
    for stone in file.split_whitespace().map(|s| s.parse::<i64>().unwrap()) {
        *frequencies.entry(stone).or_insert(0) += 1;
    }

    for _ in 0..75 {
        let mut new_frequencies = HashMap::new();
        for (stone, count) in frequencies.iter() {
            match stone {
                0 => {
                    *new_frequencies.entry(1).or_insert(0) += count;
                }
                _ if stone.to_string().len() % 2 == 0 => {
                    let stone_str = stone.to_string();
                    let mid = stone_str.len() / 2;
                    let (left, right) = stone_str.split_at(mid);
                    let left: i64 = left.parse().unwrap();
                    let right: i64 = right.parse().unwrap();
                    *new_frequencies.entry(left).or_insert(0) += count;
                    *new_frequencies.entry(right).or_insert(0) += count;
                }
                _ => {
                    let new_stone = stone * 2024;
                    *new_frequencies.entry(new_stone).or_insert(0) += count;
                }
            }
        }
        frequencies = new_frequencies;
    }

    frequencies.iter().map(|(_, count)| *count).sum::<i64>()
}
