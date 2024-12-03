use aoc_2024::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn day_01_part_one() {
    day_one::part_one(divan::black_box(include_str!(
        "../assets/input_day_one.txt"
    )));
}

#[divan::bench]
fn day_01_part_two() {
    day_one::part_two(divan::black_box(include_str!(
        "../assets/input_day_one.txt"
    )));
}

#[divan::bench]
fn day_02_part_one() {
    day_two::part_one(divan::black_box(include_str!(
        "../assets/input_day_two.txt"
    )));
}

#[divan::bench]
fn day_02_part_two() {
    day_two::part_two(divan::black_box(include_str!(
        "../assets/input_day_two.txt"
    )));
}

#[divan::bench]
fn day_03_part_one() {
    day_three::part_one(divan::black_box(include_str!(
        "../assets/input_day_three.txt"
    )));
}

#[divan::bench]
fn day_03_part_two() {
    day_three::part_two(divan::black_box(include_str!(
        "../assets/input_day_three.txt"
    )));
}
