use std::{collections::HashSet, ops::RangeInclusive};

use itertools::Itertools;

fn parse(input: &str) -> impl Iterator<Item = (HashSet<u32>, HashSet<u32>)> + '_ {
    input
        .lines()
        .filter_map(|line| {
            line.split(',')
                .map(|sections| {
                    sections
                        .split('-')
                        .map(|section| section.parse::<u32>().unwrap())
                        .collect_tuple::<(u32, u32)>()
                })
                .filter_map(|sections| sections.map(|(x, y)| x..=y))
                .collect_tuple::<(RangeInclusive<u32>, RangeInclusive<u32>)>()
        })
        .map(|(section1, section2)| {
            (
                section1.collect::<HashSet<u32>>(),
                section2.collect::<HashSet<u32>>(),
            )
        })
}

pub fn solve_part1(input: &str) -> usize {
    parse(input)
        .filter(|(section1, section2)| {
            let count = section1.union(section2).count();
            section1.len() == count || section2.len() == count
        })
        .count()
}

pub fn solve_part2(input: &str) -> usize {
    parse(input)
        .filter(|(section1, section2)| section1.intersection(section2).count() != 0)
        .count()
}

#[cfg(test)]
mod tests {
    use crate::day4::{solve_part1, solve_part2};

    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 2);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 4);
    }
}
