use std::{fmt, str::FromStr};

use pathfinding::prelude::bfs;

struct Map {
    buffer: Vec<Point>,
    width: usize,
    height: usize,
}

impl fmt::Debug for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut map = String::new();
        for (i, point) in self.buffer.iter().enumerate() {
            map.push_str(&format!("{point:?}"));
            if i + 1 % self.width == 0 {
                map.push('\n');
            }
        }

        writeln!(f, "(w: {}, h: {})\n{map}", self.width, self.height)
    }
}

impl FromStr for Map {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map {
            buffer: s
                .lines()
                .flat_map(|line| line.chars().map(to_level))
                .collect(),
            width: s.lines().next().unwrap().len(),
            height: s.lines().count(),
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coordinate {
    x: usize,
    y: usize,
}

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("").field(&self.x).field(&self.y).finish()
    }
}

impl Coordinate {
    fn from_index(index: usize, width: usize) -> Self {
        Coordinate {
            x: index % width,
            y: index / width,
        }
    }

    fn to_index(self, width: usize) -> usize {
        self.x + self.y * width
    }
}

impl Map {
    fn index(&self, coordinate: Coordinate) -> &Point {
        &self.buffer[coordinate.to_index(self.width)]
    }

    fn successors(&self, coordinate: Coordinate) -> Vec<Coordinate> {
        let point = self.index(coordinate);
        let neighbors = [
            Coordinate {
                x: coordinate.x.saturating_sub(1).clamp(0, self.width - 1),
                y: coordinate.y,
            },
            Coordinate {
                x: coordinate.x.saturating_add(1).clamp(0, self.width - 1),
                y: coordinate.y,
            },
            Coordinate {
                x: coordinate.x,
                y: coordinate.y.saturating_add(1).clamp(0, self.height - 1),
            },
            Coordinate {
                x: coordinate.x,
                y: coordinate.y.saturating_sub(1).clamp(0, self.height - 1),
            },
        ];
        neighbors
            .into_iter()
            .filter(|coordinate| {
                (..=1).contains(&(self.index(*coordinate).level() - point.level()))
            })
            .collect()
    }

    fn start_end_coordinates(&self) -> (Coordinate, Coordinate) {
        let start = self
            .buffer
            .iter()
            .enumerate()
            .find(|(_, point)| matches!(point, Point::Start))
            .map(|(index, _)| Coordinate::from_index(index, self.width))
            .unwrap();
        let end = self
            .buffer
            .iter()
            .enumerate()
            .find(|(_, point)| matches!(point, Point::End))
            .map(|(index, _)| Coordinate::from_index(index, self.width))
            .unwrap();
        (start, end)
    }
}

#[derive(Clone, Copy)]
enum Point {
    Start,
    End,
    Level(u8),
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Start => write!(f, "S"),
            Self::End => write!(f, "E"),
            Self::Level(arg0) => write!(f, "{}", char::from(arg0 + 97 - 1)),
        }
    }
}

impl Point {
    fn level(self) -> i16 {
        match self {
            Point::Start => 1,
            Point::End => 26,
            Point::Level(x) => x.into(),
        }
    }
}

fn to_level(c: char) -> Point {
    match c {
        'a'..='z' => Point::Level((c as u8) - 97 + 1),
        'S' => Point::Start,
        'E' => Point::End,
        _ => unreachable!(),
    }
}

pub fn solve_part1(input: &str) -> usize {
    let map: Map = input.parse().unwrap();
    let (start, end) = map.start_end_coordinates();
    bfs(
        &start,
        |coordinate| map.successors(*coordinate),
        |coordinate| *coordinate == end,
    )
    .unwrap()
    .len()
    // exclude starting
        - 1
}

pub fn solve_part2(input: &str) -> usize {
    let map: Map = input.parse().unwrap();
    let (_, end) = map.start_end_coordinates();
    map.buffer
        .iter()
        .enumerate()
        .filter_map(|(index, point)| {
            (point.level() == 1).then_some(Coordinate::from_index(index, map.width))
        })
        .filter_map(|start| {
            bfs(
                &start,
                |coordinate| map.successors(*coordinate),
                |coordinate| *coordinate == end,
            )
            .map(|path| path.len() - 1)
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::day12::{solve_part1, solve_part2};

    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 31);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 29);
    }
}
