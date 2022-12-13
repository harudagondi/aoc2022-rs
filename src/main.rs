#![warn(clippy::pedantic)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;

fn main() {
    const DAY1: &str = include_str!("../day1.txt");
    const DAY2: &str = include_str!("../day2.txt");
    const DAY3: &str = include_str!("../day3.txt");
    const DAY4: &str = include_str!("../day4.txt");
    const DAY5: &str = include_str!("../day5.txt");
    const DAY6: &str = include_str!("../day6.txt");
    const DAY7: &str = include_str!("../day7.txt");
    const DAY8: &str = include_str!("../day8.txt");
    const DAY9: &str = include_str!("../day9.txt");
    const DAY10: &str = include_str!("../day10.txt");
    const DAY11: &str = include_str!("../day11.txt");
    const DAY12: &str = include_str!("../day12.txt");
    const DAY13: &str = include_str!("../day13.txt");
    print_title("Day 1 - Counting Calories");
    dbg!(day01::solve_part1(DAY1));
    dbg!(day01::solve_part2(DAY1));
    print_title("Day 2: Rock Paper Scissors");
    dbg!(day02::solve_part1(DAY2));
    dbg!(day02::solve_part2(DAY2));
    print_title("Day 3: Rucksack Reorganization");
    dbg!(day03::solve_part1(DAY3));
    dbg!(day03::solve_part2(DAY3));
    print_title("Day 4: Camp Cleanup");
    dbg!(day04::solve_part1(DAY4));
    dbg!(day04::solve_part2(DAY4));
    print_title("Day 5: Supply Stacks");
    dbg!(day05::solve_part1(DAY5));
    dbg!(day05::solve_part2(DAY5));
    print_title("Day 6: Tuning Trouble");
    dbg!(day06::solve_part1(DAY6));
    dbg!(day06::solve_part2(DAY6));
    print_title("Day 7: No Space Left On Device");
    dbg!(day07::solve_part1(DAY7));
    dbg!(day07::solve_part2(DAY7));
    print_title("Day 8: Treetop Tree House");
    dbg!(day08::solve_part1(DAY8));
    dbg!(day08::solve_part2(DAY8));
    print_title("Day 9: Rope Bridge");
    dbg!(day09::solve_part1(DAY9));
    dbg!(day09::solve_part2(DAY9));
    print_title("Day 10: Cathode-Ray Tube");
    dbg!(day10::solve_part1(DAY10));
    println!("{}", day10::solve_part2(DAY10));
    print_title("Day 11: Monkey in the Middle");
    dbg!(day11::solve_part1(DAY11));
    dbg!(day11::solve_part2(DAY11));
    print_title("Day 12: Hill Climbing Algorithm");
    dbg!(day12::solve_part1(DAY12));
    dbg!(day12::solve_part2(DAY12));
    print_title("Day 13: Distress Signal");
    dbg!(day13::solve_part1(DAY13));
    dbg!(day13::solve_part2(DAY13));
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
