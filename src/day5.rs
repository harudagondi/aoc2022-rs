use itertools::Itertools;

#[derive(Debug, Clone)]
struct Stack(Vec<char>);

pub struct Stacks(Vec<Stack>);

impl Stacks {
    fn perform_step_part1(&mut self, step: &Step) {
        for _ in 0..step.amount {
            let held_crate = self.0[step.from - 1].0.pop().unwrap();
            self.0[step.to - 1].0.push(held_crate);
        }
    }

    fn perform_step_part2(&mut self, step: &Step) {
        let mut held_crates = Vec::with_capacity(step.amount);
        for _ in 0..step.amount {
            held_crates.push(self.0[step.from - 1].0.pop().unwrap());
        }
        held_crates.reverse();
        self.0[step.to - 1].0.extend(held_crates);
    }

    fn top_crates(&self) -> String {
        self.0
            .iter()
            .map(|stack| stack.0.last().copied().unwrap())
            .collect()
    }
}

struct Step {
    amount: usize,
    from: usize,
    to: usize,
}

pub struct Steps(Vec<Step>);

pub fn parse(input: &str) -> (Stacks, Steps) {
    let input = input.replace("\r\n", "\n");

    let (stacks, steps) = input.split("\n\n").collect_tuple().unwrap();

    let mut stacks_lines = stacks.lines().rev();

    let number_of_stacks = stacks_lines.next().unwrap().split_whitespace().count();

    let mut stacks = Stacks(vec![Stack(Vec::new()); number_of_stacks]);

    for stack in
        stacks_lines.map(|line| (0..number_of_stacks).map(|n| line.chars().nth(1 + n * 4).unwrap()))
    {
        for (index, crate_) in stack.enumerate() {
            if !crate_.is_whitespace() {
                stacks.0[index].0.push(crate_);
            }
        }
    }

    let steps = Steps(
        steps
            .lines()
            .map(str::split_whitespace)
            .map(|mut line| {
                (
                    line.nth(1).unwrap().parse().unwrap(),
                    line.nth(1).unwrap().parse().unwrap(),
                    line.nth(1).unwrap().parse().unwrap(),
                )
            })
            .map(|(amount, from, to)| Step { amount, from, to })
            .collect(),
    );

    (stacks, steps)
}

pub fn solve_part1(input: &str) -> String {
    let (mut stacks, steps) = parse(input);

    for step in &steps.0 {
        stacks.perform_step_part1(step);
    }

    stacks.top_crates()
}

pub fn solve_part2(input: &str) -> String {
    let (mut stacks, steps) = parse(input);

    for step in &steps.0 {
        stacks.perform_step_part2(step);
    }

    stacks.top_crates()
}

#[cfg(test)]
mod tests {
    use crate::day5::{solve_part1, solve_part2};

    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), "CMZ");
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), "MCD");
    }
}
