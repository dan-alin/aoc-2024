fn prepare(file: &str) -> (Vec<i32>, Vec<i32>) {
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in file.lines() {
        let mut values = line.split_whitespace();
        left.push(values.next().unwrap().parse::<i32>().unwrap());
        right.push(values.next().unwrap().parse::<i32>().unwrap());
    }

    (left, right)
}

pub fn part_one(file: &str) -> i32 {
    let (mut left, mut right) = prepare(file);

    let mut count = 0;

    left.sort();
    right.sort();

    for i in 0..left.len() {
        count += (left[i] - right[i]).abs();
    }

    count
}

pub fn part_one_zip(file: &str) -> i32 {
    let (mut left, mut right) = prepare(file);

    left.sort();
    right.sort();

    let count = std::iter::zip(left, right)
        .map(|(l, r)| (l - r).abs())
        .sum();
    count
}

pub fn part_two(file: &str) -> i32 {
    let (left, right) = prepare(file);

    let mut similarity: i32 = 0;

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

pub fn part_two_filter(file: &str) -> i32 {
    let (left, right) = prepare(file);

    let similarity = left
        .iter()
        .map(|num| num * right.iter().filter(|v| v == &num).count() as i32)
        .sum();

    similarity
}
