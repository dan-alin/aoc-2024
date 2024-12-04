mod day_one;
mod day_three;
mod day_two;

fn main() {
    let d1_input = include_str!("../assets/input_day_one.txt");
    let d2_input = include_str!("../assets/input_day_two.txt");
    let d3_input = include_str!("../assets/input_day_three.txt");

    println!("day 1 - part 1: {}", day_one::part_one(&d1_input));
    println!("day 1 - part 1 zip: {}", day_one::part_one_zip(&d1_input));
    println!("day 1 - part 2: {}", day_one::part_two(&d1_input));
    println!(
        "day 1 - part 2 filter: {}",
        day_one::part_two_filter(&d1_input)
    );

    println!("day 2 - part 1: {}", day_two::part_one(&d2_input));
    println!("day 2 - part 2: {}", day_two::part_two(&d2_input));

    println!("day 3 - part 1: {}", day_three::part_one(&d3_input));
    println!("day 3 - part 2: {}", day_three::part_two(&d3_input));
    println!(
        "day 3 - part 2 split: {}",
        day_three::part_two_split(&d3_input)
    );
}
