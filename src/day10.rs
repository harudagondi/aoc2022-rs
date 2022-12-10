use std::{
    collections::{HashMap, VecDeque},
    fmt,
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
enum Command {
    NoOp,
    AddX(i32),
}

fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let mut line = line.split_whitespace();
            match (line.next(), line.next()) {
                (Some("addx"), Some(x)) => Command::AddX(x.parse().unwrap()),
                (Some("noop"), None) => Command::NoOp,
                _ => unreachable!(),
            }
        })
        .collect()
}

struct Clock {
    cycle: u32,
    commands: VecDeque<Command>,
    register: i32,
    memorized_cycles: HashMap<u32, i32>,
}

impl fmt::Debug for Clock {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Clock")
            .field("cycle", &self.cycle)
            .field("next_command", &self.commands.iter().next())
            .field("register", &self.register)
            .field("memorized_cycles", &self.memorized_cycles)
            .finish()
    }
}

impl Clock {
    fn new(commands: Vec<Command>) -> Self {
        Self {
            cycle: 1,
            commands: commands
                .into_iter()
                .flat_map(|command| match command {
                    Command::NoOp => [Command::NoOp].to_vec(),
                    Command::AddX(x) => [Command::NoOp, Command::AddX(x)].to_vec(),
                })
                .collect(),
            register: 1,
            memorized_cycles: HashMap::new(),
        }
    }

    fn run(&mut self) -> HashMap<u32, i32> {
        let mut memorized_cycles = HashMap::new();
        for command in &self.commands {
            memorized_cycles.insert(self.cycle, self.register);

            match command {
                Command::NoOp => {}
                Command::AddX(x) => self.register += x,
            }

            self.cycle += 1;
        }

        memorized_cycles
    }

    fn get_product_cycles(&mut self) -> i32 {
        self.run()
            .iter()
            .filter(|(&cycle, _)| matches!(cycle, 20 | 60 | 100 | 140 | 180 | 220))
            .map(|(&cycle, &register)| TryInto::<i32>::try_into(cycle).unwrap() * register)
            .sum()
    }
}

pub fn solve_part1(input: &str) -> i32 {
    Clock::new(parse(input)).get_product_cycles()
}

pub fn solve_part2(input: &str) -> String {
    let mut cycles = Clock::new(parse(input));
    let mut crt = String::new();
    for (cycle, register) in cycles
        .run()
        .into_iter()
        .take(40 * 6)
        .sorted_by(|a, b| a.0.cmp(&b.0))
        .collect_vec()
    {
        let cycle: i32 = cycle.try_into().unwrap();
        let cycle = cycle - 1;
        let end_of_line = cycle % 40 == 0 && cycle != 0;
        let cycle = cycle % 40;
        let register = register % 40;
        let sprite = register - 1..=register + 1;
        let is_overlapping = sprite.contains(&cycle);

        if end_of_line {
            crt.push('\n');
        }

        if is_overlapping {
            crt.push('#');
        } else {
            crt.push('.');
        }
    }

    crt
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use itertools::Itertools;

    use pretty_assertions::{assert_eq, assert_str_eq};

    use crate::day10::{solve_part1, solve_part2};

    use super::{parse, Clock};

    const INPUT_SMALL: &str = "noop
addx 3
addx -5";

    #[test]
    fn small() {
        let mut clock = Clock::new(parse(INPUT_SMALL))
            .run()
            .into_iter()
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0));
        assert_eq!(clock.next(), Some((1, 1)));
        assert_eq!(clock.next(), Some((2, 1)));
        assert_eq!(clock.next(), Some((3, 4)));
        assert_eq!(clock.next(), Some((4, 4)));
        assert_eq!(clock.next(), Some((5, -1)));
        assert_eq!(clock.next(), None);
    }

    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1() {
        let clock: Vec<(u32, i32)> = Clock::new(parse(INPUT))
            .run()
            .into_iter()
            .filter(|(cycle, _)| matches!(cycle, 20 | 60 | 100 | 140 | 180 | 220))
            .map(|(cycle, register)| {
                (
                    cycle,
                    TryInto::<i32>::try_into(cycle).unwrap() * register as i32,
                )
            })
            .sorted()
            .collect();
        assert_eq!(clock, {
            let mut map = HashMap::new();
            map.insert(20, 420);
            map.insert(60, 1140);
            map.insert(100, 1800);
            map.insert(140, 2940);
            map.insert(180, 2880);
            map.insert(220, 3960);
            map.into_iter().sorted().collect::<Vec<_>>()
        });

        let clock: Vec<(u32, i32)> = Clock::new(parse(INPUT))
            .run()
            .into_iter()
            .filter(|(cycle, _)| matches!(cycle, 20 | 60 | 100 | 140 | 180 | 220))
            .sorted()
            .collect();
        assert_eq!(clock, {
            let mut map = HashMap::new();
            map.insert(20, 21);
            map.insert(60, 19);
            map.insert(100, 18);
            map.insert(140, 21);
            map.insert(180, 16);
            map.insert(220, 18);
            map.into_iter().sorted().collect::<Vec<_>>()
        });

        assert_eq!(solve_part1(INPUT), 13140);
    }

    const ANSWER_PART2: &str = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....";

    #[test]
    fn part2() {
        // println!("{}", solve_part2(INPUT));
        assert_str_eq!(solve_part2(INPUT), ANSWER_PART2);
    }
}
