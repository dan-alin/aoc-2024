mod day_01;
mod day_02;
mod day_03;
mod day_04;
mod day_05;
mod day_06;
mod day_07;

fn main() {
    let d1_input = include_str!("../assets/input_day_01.txt");
    let d2_input = include_str!("../assets/input_day_02.txt");
    let d3_input = include_str!("../assets/input_day_03.txt");
    let d4_input = include_str!("../assets/input_day_04.txt");
    let d5_input = include_str!("../assets/input_day_05.txt");
    let d6_input = include_str!("../assets/input_day_06.txt");
    let d7_input = include_str!("../assets/input_day_07.txt");

    println!("day 1 - part 1: {}", day_01::part_one(&d1_input));
    println!("day 1 - part 1 zip: {}", day_01::part_one_zip(&d1_input));
    println!("day 1 - part 2: {}", day_01::part_two(&d1_input));
    println!(
        "day 1 - part 2 filter: {}",
        day_01::part_two_filter(&d1_input)
    );

    println!("day 2 - part 1: {}", day_02::part_one(&d2_input));
    println!("day 2 - part 2: {}", day_02::part_two(&d2_input));

    println!("day 3 - part 1: {}", day_03::part_one(&d3_input));
    println!("day 3 - part 2: {}", day_03::part_two(&d3_input));
    println!(
        "day 3 - part 2 split: {}",
        day_03::part_two_split(&d3_input)
    );

    println!("day 4 - part 1: {}", day_04::part_one(&d4_input));
    println!("day 4 - part 2: {}", day_04::part_two(&d4_input));

    println!("day 5 - part 1: {}", day_05::part_one(&d5_input));
    println!("day 5 - part 2: {}", day_05::part_two(&d5_input));

    println!("day 6 - part 1: {}", day_06::part_one(&d6_input));
    println!("day 6 - part 2: {}", day_06::part_two(&d6_input));

    println!("day 7 - part 1: {}", day_07::part_one(&d7_input));
    println!("day 7 - part 2: {}", day_07::part_two(&d7_input));
}
