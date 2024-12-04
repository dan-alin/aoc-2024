use regex::Regex;

pub fn part_one(file: &str) -> i32 {
    let mut count = 0;
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    for capture in re.captures_iter(file) {
        let mul: i32 = capture[1].parse::<i32>().unwrap() * capture[2].parse::<i32>().unwrap();
        count += mul
    }

    count
}

pub fn part_two(file: &str) -> i32 {
    let mut count = 0;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

    let mut enabled = true;
    for capture in re.captures_iter(&file) {
        match &capture[0] {
            "do()" => {
                enabled = true;
            }

            "don't()" => enabled = false,
            _ => {
                if enabled {
                    let mul: i32 =
                        capture[1].parse::<i32>().unwrap() * capture[2].parse::<i32>().unwrap();
                    count += mul
                }
            }
        }
    }

    count
}

pub fn part_two_split(file: &str) -> i32 {
    let mut count = 0;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let splitted_by_do = file.split("do()").collect::<Vec<&str>>();

    for sub in splitted_by_do {
        let splitted_by_dont = sub.split("don't()").collect::<Vec<&str>>();

        for capture in re.captures_iter(splitted_by_dont[0]) {
            let mul: i32 = capture[1].parse::<i32>().unwrap() * capture[2].parse::<i32>().unwrap();
            count += mul
        }
    }

    count
}
