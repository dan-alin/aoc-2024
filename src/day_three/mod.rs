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
    let mut enabled = true;

    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)|don't\(\)").unwrap();

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
