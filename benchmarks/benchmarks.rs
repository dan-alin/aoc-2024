use aoc_2024::*;

fn main() {
    divan::main();
}
mod day_01_benches {
    use super::*;

    #[divan::bench]
    fn part_one() {
        day_01::part_one(divan::black_box(include_str!("../assets/input_day_01.txt")));
    }

    #[divan::bench]
    fn part_one_zip() {
        day_01::part_one_zip(divan::black_box(include_str!("../assets/input_day_01.txt")));
    }

    #[divan::bench]
    fn part_two() {
        day_01::part_two(divan::black_box(include_str!("../assets/input_day_01.txt")));
    }

    #[divan::bench]
    fn part_two_filter() {
        day_01::part_two_filter(divan::black_box(include_str!("../assets/input_day_01.txt")));
    }
}

mod day_02_benches {
    use super::*;

    #[divan::bench]
    fn part_one() {
        day_02::part_one(divan::black_box(include_str!("../assets/input_day_02.txt")));
    }

    #[divan::bench]
    fn part_two() {
        day_02::part_two(divan::black_box(include_str!("../assets/input_day_02.txt")));
    }
}

mod day_03_benches {
    use super::*;

    #[divan::bench]
    fn part_one() {
        day_03::part_one(divan::black_box(include_str!("../assets/input_day_03.txt")));
    }

    #[divan::bench]
    fn part_two() {
        day_03::part_two(divan::black_box(include_str!("../assets/input_day_03.txt")));
    }

    #[divan::bench]
    fn part_two_split() {
        day_03::part_two_split(divan::black_box(include_str!("../assets/input_day_03.txt")));
    }
}

mod day_04_benches {
    use super::*;

    #[divan::bench]
    fn part_one() {
        day_04::part_one(divan::black_box(include_str!("../assets/input_day_04.txt")));
    }

    #[divan::bench]
    fn part_two() {
        day_04::part_two(divan::black_box(include_str!("../assets/input_day_04.txt")));
    }
}

mod day_05_benches {
    use super::*;

    #[divan::bench]
    fn part_one() {
        day_05::part_one(divan::black_box(include_str!("../assets/input_day_05.txt")));
    }

    #[divan::bench]
    fn part_two() {
        day_05::part_two(divan::black_box(include_str!("../assets/input_day_05.txt")));
    }
}
