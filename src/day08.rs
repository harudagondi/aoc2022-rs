use std::str::FromStr;

use itertools::iproduct;

struct Grid {
    buffer: Vec<u8>,
    width: usize,
    height: usize,
}

impl FromStr for Grid {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap().chars().count();
        Ok(Grid {
            buffer: s
                .lines()
                .flat_map(|line| line.chars().map(|c| c.to_string().parse().unwrap()))
                .collect(),
            width,
            height,
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    const DIRECTIONS: [Direction; 4] = [
        Direction::North,
        Direction::South,
        Direction::East,
        Direction::West,
    ];
}

impl Grid {
    fn get(&self, x: usize, y: usize) -> u8 {
        self.buffer[(y * self.width) + x]
    }

    fn trees(&self, x: usize, y: usize, direction: Direction) -> (u8, Vec<u8>) {
        let blocking_trees = match direction {
            Direction::North => (0..y).rev().map(|row| self.get(x, row)).collect(),
            Direction::South => (y + 1..self.height).map(|row| self.get(x, row)).collect(),
            Direction::East => (x + 1..self.width).map(|col| self.get(col, y)).collect(),
            Direction::West => (0..x).rev().map(|col| self.get(col, y)).collect(),
        };
        (self.get(x, y), blocking_trees)
    }

    fn is_visible(&self, x: usize, y: usize) -> bool {
        (Direction::DIRECTIONS)
            .into_iter()
            .map(|direction| {
                let (tree, blocking_trees) = self.trees(x, y, direction);
                // all blocking trees must be less than the tree for it to be visible
                blocking_trees
                    .into_iter()
                    .all(|blocking_tree| tree > blocking_tree)
            })
            .any(|x| x)
    }

    fn scenic_score(&self, x: usize, y: usize) -> usize {
        (Direction::DIRECTIONS)
            .into_iter()
            .map(|direction| {
                let (tree, blocking_trees) = self.trees(x, y, direction);

                let mut count = 0;
                for blocking_tree in blocking_trees {
                    count += 1;
                    if blocking_tree >= tree {
                        break;
                    }
                }
                count
            })
            .product()
    }
}

pub fn solve_part1(input: &str) -> usize {
    let grid = input.parse::<Grid>().unwrap();
    iproduct!(0..grid.width, 0..grid.height)
        .map(|(x, y)| grid.is_visible(x, y))
        .filter(|&is_visible| is_visible)
        .count()
}

pub fn solve_part2(input: &str) -> usize {
    let grid = input.parse::<Grid>().unwrap();
    iproduct!(0..grid.width, 0..grid.height)
        .map(|(x, y)| grid.scenic_score(x, y))
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use crate::day08::{solve_part1, solve_part2};

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 21);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 8);
    }
}
