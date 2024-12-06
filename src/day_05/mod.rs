use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

fn prepare(file: &str) -> (HashSet<(i32, i32)>, Vec<Vec<i32>>) {
    let data: Vec<&str> = file.split("\n\n").collect();

    let rules = data[0].split("\n").collect::<Vec<&str>>();

    let updates: Vec<Vec<i32>> = data[1]
        .split("\n")
        .collect::<Vec<&str>>()
        .iter()
        .map(|x| x.split(",").map(|y| y.parse().unwrap()).collect())
        .collect();

    let mut rules_map: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut rules_set: HashSet<(i32, i32)> = HashSet::new();

    for line in &rules {
        let mut line = line.trim().split('|');
        let left: i32 = line.next().unwrap().parse().unwrap();
        let right: i32 = line.next().unwrap().parse().unwrap();

        rules_set.insert((left, right));
    }

    for rule in &rules {
        if let Some((left, right)) = rule.split_once('|') {
            if let (Ok(left), Ok(right)) = (left.parse::<i32>(), right.parse::<i32>()) {
                rules_map.entry(left).or_insert_with(Vec::new).push(right);
            }
        }
    }

    (rules_set, updates)
}

pub fn part_one(file: &str) -> i32 {
    let (rules_set, updates) = prepare(file);
    let mut count = 0;

    updates.iter().for_each(|upt| {
        if upt.is_sorted_by(|&a, &b| !rules_set.contains(&(b, a))) {
            let mid_idx = upt.len() / 2;
            count += upt[mid_idx];
        }
    });

    count
}

//rearrange the wrong rules and sum up their middle values
pub fn part_two(file: &str) -> i32 {
    let (rules_set, mut updates) = prepare(file);
    let mut count = 0;

    updates.iter_mut().for_each(|upt| {
        if !upt.is_sorted_by(|&a, &b| !rules_set.contains(&(b, a))) {
            upt.sort_by(|&a, &b| {
                if rules_set.contains(&(a, b)) {
                    Ordering::Less
                } else if rules_set.contains(&(b, a)) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });

            let mid_idx = upt.len() / 2;
            count += upt[mid_idx];
        }
    });

    count
}
