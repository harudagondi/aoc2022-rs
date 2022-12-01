fn parse(input: &str) -> impl Iterator<Item = u32> + '_ {
    input
        // Each elf is separated by two new lines,
        // so we split by that first,
        // returning an iterator of each elf's calories in string form.
        .split("\r\n\r\n")
        .map(|elf| {
            // We split each line.
            elf.split("\r\n")
                // Assuming we only get blank lines as errors,
                // we simply just ignore then,
                // only taking those that can be parsed.
                .filter_map(|calories| calories.trim().parse::<u32>().ok())
                // We only need the sum of all calories for each elf.
                .sum()
        })
}

pub fn solve_part1(input: &str) -> u32 {
    // Part 1 only requires the elf with the maximum amount of calories.
    // We unwrap here because we know there is at least one elf.
    parse(input).max().unwrap()
}

pub fn solve_part2(input: &str) -> u32 {
    // You cannot sort an iterator.
    // Therefore, we convert this into a Vec and sort it.
    let mut vec: Vec<u32> = parse(input).collect();
    // This sorts from smallest to largest.
    vec.sort();
    // To find the three elves with the largest amount of calories,
    // we reverse the iterator to take the first three numbers,
    // and then sum them.
    vec.into_iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::{solve_part1, solve_part2};

    const INPUT: &str = "1000\r
2000\r
3000\r
\r
4000\r
\r
5000\r
6000\r
\r
7000\r
8000\r
9000\r
\r
10000\r
";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 24000);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 45000);
    }
}
