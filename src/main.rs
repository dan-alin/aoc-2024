mod day_01;
mod day_02;
mod day_03;

fn main() {
    let d1_input = include_str!("../assets/input_day_01.txt");
    let d2_input = include_str!("../assets/input_day_02.txt");
    let d3_input = include_str!("../assets/input_day_03.txt");

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
}
