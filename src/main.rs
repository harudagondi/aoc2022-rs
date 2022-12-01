mod day1;

fn main() {
    const DAY1: &str = include_str!("../day1.txt");
    print_title("Day 1 - Counting Calories");
    println!("{}", day1::solve_part1(DAY1));
    println!("{}", day1::solve_part2(DAY1));
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
