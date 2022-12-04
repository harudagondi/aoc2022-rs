#![warn(clippy::pedantic)]

mod day1;
mod day2;
mod day3;
mod day4;

fn main() {
    const DAY1: &str = include_str!("../day1.txt");
    const DAY2: &str = include_str!("../day2.txt");
    const DAY3: &str = include_str!("../day3.txt");
    const DAY4: &str = include_str!("../day4.txt");
    print_title("Day 1 - Counting Calories");
    println!("{}", day1::solve_part1(DAY1));
    println!("{}", day1::solve_part2(DAY1));
    print_title("Day 2: Rock Paper Scissors");
    println!("{}", day2::solve_part1(DAY2));
    println!("{}", day2::solve_part2(DAY2));
    print_title("Day 3: Rucksack Reorganization");
    println!("{}", day3::solve_part1(DAY3));
    println!("{}", day3::solve_part2(DAY3));
    print_title("Day 4: Camp Cleanup");
    println!("{}", day4::solve_part1(DAY4));
    println!("{}", day4::solve_part2(DAY4));
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
