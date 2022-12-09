#![warn(clippy::pedantic)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

fn main() {
    const DAY1: &str = include_str!("../day1.txt");
    const DAY2: &str = include_str!("../day2.txt");
    const DAY3: &str = include_str!("../day3.txt");
    const DAY4: &str = include_str!("../day4.txt");
    const DAY5: &str = include_str!("../day5.txt");
    const DAY6: &str = include_str!("../day6.txt");
    const DAY7: &str = include_str!("../day7.txt");
    print_title("Day 1 - Counting Calories");
    dbg!(day1::solve_part1(DAY1));
    dbg!(day1::solve_part2(DAY1));
    print_title("Day 2: Rock Paper Scissors");
    dbg!(day2::solve_part1(DAY2));
    dbg!(day2::solve_part2(DAY2));
    print_title("Day 3: Rucksack Reorganization");
    dbg!(day3::solve_part1(DAY3));
    dbg!(day3::solve_part2(DAY3));
    print_title("Day 4: Camp Cleanup");
    dbg!(day4::solve_part1(DAY4));
    dbg!(day4::solve_part2(DAY4));
    print_title("Day 5: Supply Stacks");
    dbg!(day5::solve_part1(DAY5));
    dbg!(day5::solve_part2(DAY5));
    print_title("Day 6: Tuning Trouble");
    dbg!(day6::solve_part1(DAY6));
    dbg!(day6::solve_part2(DAY6));
    print_title("Day 7: No Space Left On Device");
    dbg!(day7::solve_part1(DAY7));
    dbg!(day7::solve_part2(DAY7));
}

fn print_title(title: &str) {
    const DELIMITER: char = '*';
    let border = std::iter::repeat(DELIMITER)
        .take(title.len() + 4)
        .collect::<String>();
    println!("{}", border);
    println!("{DELIMITER} {title} {DELIMITER}");
    println!("{}", border);
}
