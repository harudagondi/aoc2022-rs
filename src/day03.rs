use std::collections::HashSet;

use itertools::Itertools;

pub fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|rucksack| rucksack.split_at(rucksack.len() / 2))
        .map(|(comp1, comp2)| {
            (
                comp1.chars().collect::<HashSet<char>>(),
                comp2.chars().collect::<HashSet<char>>(),
            )
        })
        .map(|(comp1, comp2)| comp1.intersection(&comp2).copied().collect::<Vec<_>>())
        .map(|common| common.into_iter().map(priority).sum::<u32>())
        .sum()
}

pub fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .chunks(3)
        .into_iter()
        .map(|mut chunk| {
            (
                chunk.next().unwrap().chars().collect::<HashSet<char>>(),
                chunk.next().unwrap().chars().collect::<HashSet<char>>(),
                chunk.next().unwrap().chars().collect::<HashSet<char>>(),
            )
        })
        .map(|(rs1, rs2, rs3)| {
            rs1.intersection(&rs2)
                .copied()
                .collect::<HashSet<char>>()
                .intersection(&rs3)
                .copied()
                .collect_vec()
        })
        .map(|badge| badge.into_iter().map(priority).sum::<u32>())
        .sum()
}

fn priority(c: char) -> u32 {
    match c {
        'a'..='z' => (c as u32) - 97 + 1,
        'A'..='Z' => (c as u32) - 65 + 26 + 1,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::day03::{priority, solve_part1, solve_part2};

    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn priority_test() {
        for (c, p) in ('a'..='z').zip(1..=26) {
            assert_eq!(priority(c), p);
        }

        for (c, p) in ('A'..='Z').zip(27..=52) {
            assert_eq!(priority(c), p);
        }
    }

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 157);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 70);
    }
}
