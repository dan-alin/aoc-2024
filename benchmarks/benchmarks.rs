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

mod day_06_benches {
    use super::*;

    #[divan::bench]
    fn part_one() {
        day_06::part_one(divan::black_box(include_str!("../assets/input_day_06.txt")));
    }

    // #[divan::bench]
    // fn part_two() {
    //     day_06::part_two(divan::black_box(include_str!("../assets/input_day_06.txt")));
    // }
}

mod day_07_benches {
    use super::*;

    #[divan::bench]
    fn part_one() {
        day_07::part_one(divan::black_box(include_str!("../assets/input_day_07.txt")));
    }

    #[divan::bench]
    fn part_one_rtl() {
        day_07::part_one_rtl(divan::black_box(include_str!("../assets/input_day_07.txt")));
    }

    #[divan::bench]
    fn part_one_filter() {
        day_07::part_one_filter(divan::black_box(include_str!("../assets/input_day_07.txt")));
    }

    #[divan::bench(sample_count = 10)]
    fn part_two() {
        day_07::part_two(divan::black_box(include_str!("../assets/input_day_07.txt")));
    }

    #[divan::bench(sample_count = 10)]
    fn part_two_rtl() {
        day_07::part_two_rtl(divan::black_box(include_str!("../assets/input_day_07.txt")));
    }

    #[divan::bench]
    fn part_two_filter() {
        day_07::part_two_filter(divan::black_box(include_str!("../assets/input_day_07.txt")));
    }
}

// mod day_10_benches {
//     use super::*;

//     #[divan::bench]
//     fn part_one() {
//         day_10::part_one(divan::black_box(include_str!("../assets/input_day_10.txt")));
//     }

//     // #[divan::bench]
//     // fn part_one_rtl() {
//     //     day_10::part_one_rtl(divan::black_box(include_str!("../assets/input_day_10.txt")));
//     // }
// }
