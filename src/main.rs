mod day_one;
mod day_two;

fn main() {
    let d1_input = include_str!("../assets/input_day_one.txt");
    let d2_input = include_str!("../assets/input_day_two.txt");

    println!("day one - part one: {}", day_one::part_one(&d1_input));
    println!("day one - part two: {}", day_one::part_two(&d1_input));
    println!("day two - part one: {}", day_two::part_one(&d2_input));
    println!("day two - part two: {}", day_two::part_two(&d2_input));
}
