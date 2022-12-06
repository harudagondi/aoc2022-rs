use std::collections::{HashSet, VecDeque};

#[derive(Default)]
struct Solver {
    buffer: VecDeque<char>,
    counter: u32,
    buffer_size: usize,
}

impl Solver {
    fn new(buffer_size: usize) -> Self {
        Self {
            buffer: VecDeque::new(),
            counter: 0,
            buffer_size,
        }
    }

    fn update(&mut self, c: char) {
        if self.buffer.len() < self.buffer_size {
            self.buffer.push_back(c);
        } else {
            self.buffer.pop_front();
            self.buffer.push_back(c);
        }
        self.counter += 1;
    }

    fn starter(&self) -> Option<u32> {
        let buffer = self.buffer.iter().fold(HashSet::new(), |mut acc, c| {
            acc.insert(*c);
            acc
        });

        if buffer.len() == self.buffer_size {
            Some(self.counter)
        } else {
            None
        }
    }
}

pub fn solve_part1(input: &str) -> u32 {
    let mut solver = Solver::new(4);
    for c in input.chars() {
        solver.update(c);
        if let Some(counter) = solver.starter() {
            return counter;
        }
    }
    unreachable!();
}

pub fn solve_part2(input: &str) -> u32 {
    let mut solver = Solver::new(14);
    for c in input.chars() {
        solver.update(c);
        if let Some(counter) = solver.starter() {
            return counter;
        }
    }
    unreachable!();
}

#[cfg(test)]
mod tests {
    use crate::day6::{solve_part1, solve_part2};

    const INPUT1: &str = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
    const INPUT2: &str = "bvwbjplbgvbhsrlpgdmjqwftvncz";
    const INPUT3: &str = "nppdvjthqldpwncqszvftbrmjlhg";
    const INPUT4: &str = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
    const INPUT5: &str = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT1), 7);
        assert_eq!(solve_part1(INPUT2), 5);
        assert_eq!(solve_part1(INPUT3), 6);
        assert_eq!(solve_part1(INPUT4), 10);
        assert_eq!(solve_part1(INPUT5), 11);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT1), 19);
        assert_eq!(solve_part2(INPUT2), 23);
        assert_eq!(solve_part2(INPUT3), 23);
        assert_eq!(solve_part2(INPUT4), 29);
        assert_eq!(solve_part2(INPUT5), 26);
    }
}
