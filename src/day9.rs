use std::{collections::HashSet, fmt, ops::Sub};

#[derive(Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    x: isize,
    y: isize,
}

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("").field(&self.x).field(&self.y).finish()
    }
}

impl Sub<Coordinate> for Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Coordinate) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Coordinate {
    fn is_adjacent_with(self, other: Coordinate) -> bool {
        match self - other {
            Coordinate {
                x: 0 | 1 | -1,
                y: 0 | 1 | -1,
            } => true,
            Coordinate { .. } => false,
        }
    }
}

#[derive(Debug)]
struct Interpreter<const N: usize> {
    dirty: HashSet<Coordinate>,
    head: Coordinate,
    tail: [Coordinate; N],
}

impl<const N: usize> Interpreter<N> {
    fn new() -> Self {
        Self {
            dirty: HashSet::new(),
            head: Coordinate::default(),
            tail: [Coordinate::default(); N],
        }
    }

    fn run(&mut self, commands: Vec<Command>) -> usize {
        self.dirty();
        self.interpret_commands(commands);
        self.dirty.len()
    }

    fn interpret_commands(&mut self, commands: Vec<Command>) {
        for command in commands {
            self.interpret_command(command);
        }
    }

    fn interpret_command(&mut self, Command(direction, distance): Command) {
        for _ in 0..distance {
            self.move_head(direction);
            self.move_tail();
            self.dirty();
        }
    }

    fn move_head(&mut self, direction: Direction) {
        match direction {
            Direction::Right => self.head.x += 1,
            Direction::Left => self.head.x -= 1,
            Direction::Up => self.head.y += 1,
            Direction::Down => self.head.y -= 1,
        }
    }

    fn move_tail(&mut self) {
        let move_tail_inner = |head: &mut Coordinate, tail: &mut Coordinate| {
            if tail.is_adjacent_with(*head) {
                return;
            }

            if tail.y > head.y {
                // since tail is above head, go down
                tail.y -= 1;
            }

            if tail.y < head.y {
                // since tail is below head, go up
                tail.y += 1;
            }

            if tail.x > head.x {
                // since tail is right of head, go left
                tail.x -= 1;
            }

            if tail.x < head.x {
                // since tail is left of head, go right
                tail.x += 1;
            }
        };

        move_tail_inner(&mut self.head, &mut self.tail[0]);

        for n in 0..self.tail.len() - 1 {
            let [head, tail, ..] = &mut self.tail[n..] else { unreachable!() };
            move_tail_inner(head, tail);
        }
    }

    fn dirty(&mut self) {
        self.dirty.insert(*self.tail.last().unwrap());
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl<T: AsRef<str>> From<T> for Direction {
    fn from(t: T) -> Self {
        match t.as_ref() {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "D" => Direction::Down,
            "U" => Direction::Up,
            _ => unreachable!(),
        }
    }
}

struct Command(Direction, u8);

fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .filter_map(|line| line.split_once(' '))
        .map(|(direction, distance)| Command(direction.into(), distance.parse().unwrap()))
        .collect()
}

pub fn solve_part1(input: &str) -> usize {
    Interpreter::<1>::new().run(parse(input))
}

pub fn solve_part2(input: &str) -> usize {
    Interpreter::<9>::new().run(parse(input))
}

#[cfg(test)]
mod tests {
    use crate::day9::{solve_part1, solve_part2};

    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT2: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 13);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 1);
        assert_eq!(solve_part2(INPUT2), 36);
    }
}
