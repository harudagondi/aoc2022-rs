use core::str::FromStr;
use std::collections::{hash_map::Entry, HashMap};
use std::fmt;

use itertools::Itertools;

#[derive(Debug)]
struct Map {
    buffer: HashMap<Coordinate, Point>,
    maximum_depth: i32,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Map::construct(
            s.lines()
                .map(Path::from_str)
                .collect::<Result<_, _>>()
                .map_err(|_| ())?,
        ))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("").field(&self.x).field(&self.y).finish()
    }
}

impl Coordinate {
    const fn down(self) -> Coordinate {
        Coordinate {
            x: self.x,
            y: self.y + 1,
        }
    }

    const fn down_left(self) -> Coordinate {
        Coordinate {
            x: self.x - 1,
            y: self.y + 1,
        }
    }

    const fn down_right(self) -> Coordinate {
        Coordinate {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
}

impl FromStr for Coordinate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(())?;
        Ok(Coordinate {
            x: x.parse().map_err(|_| ())?,
            y: y.parse().map_err(|_| ())?,
        })
    }
}

impl Coordinate {
    fn from_to(from: Coordinate, to: Coordinate) -> Vec<Coordinate> {
        if from.x != to.x && from.y != to.y {
            panic!()
        }

        if from.x == to.x {
            let [f, t] = {
                let mut x = [from.y, to.y];
                x.sort_unstable();
                x
            };
            (f..=t).map(|y| Coordinate { x: from.x, y }).collect()
        } else {
            let [f, t] = {
                let mut x = [from.x, to.x];
                x.sort_unstable();
                x
            };
            (f..=t).map(|x| Coordinate { x, y: from.y }).collect()
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Point {
    Rock,
    Sand,
    RestSand,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rock => write!(f, "#"),
            Self::Sand => write!(f, "~"),
            Self::RestSand => write!(f, "o"),
        }
    }
}

struct Path(Vec<Coordinate>);

impl FromStr for Path {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Path(
            s.split("->")
                .map(str::trim)
                .map(Coordinate::from_str)
                .collect::<Result<_, _>>()
                .map_err(|_| ())?,
        ))
    }
}

impl Map {
    fn construct(paths: Vec<Path>) -> Self {
        let buffer: HashMap<Coordinate, Point> = paths
            .into_iter()
            .flat_map(|path| {
                path.0
                    .into_iter()
                    .tuple_windows()
                    .map(|(from, to)| Coordinate::from_to(from, to))
            })
            .flat_map(|coordinates| {
                coordinates
                    .into_iter()
                    .map(|coordinate| (coordinate, Point::Rock))
            })
            .collect();
        let maximum_depth = buffer.keys().max_by(|a, b| a.y.cmp(&b.y)).unwrap().y;
        Map {
            buffer,
            maximum_depth,
        }
    }

    fn sand_coordinate(&self) -> Option<Coordinate> {
        self.buffer
            .iter()
            .find(|(_, v)| v == &&mut Point::Sand)
            .map(|(coordinate, _)| *coordinate)
    }

    fn spawn_sand(&mut self) {
        self.buffer.insert(Coordinate { x: 500, y: 0 }, Point::Sand);
    }

    fn tick(&mut self, coordinate: Coordinate) {
        self.buffer.remove(&coordinate);
        if let Entry::Vacant(entry) = self.buffer.entry(coordinate.down()) {
            entry.insert(Point::Sand);
        } else if let Entry::Vacant(entry) = self.buffer.entry(coordinate.down_left()) {
            entry.insert(Point::Sand);
        } else if let Entry::Vacant(entry) = self.buffer.entry(coordinate.down_right()) {
            entry.insert(Point::Sand);
        } else {
            self.buffer.insert(coordinate, Point::RestSand);
        }
    }

    fn run(&mut self, end_goal: impl Fn(&Map, Coordinate) -> bool) -> usize {
        loop {
            if let Some(coordinate) = self.sand_coordinate() {
                if end_goal(self, coordinate) {
                    break self
                        .buffer
                        .values()
                        .filter(|&&point| point == Point::RestSand)
                        .count();
                }
                self.tick(coordinate);
            } else {
                self.spawn_sand();
            }
        }
    }
}

pub fn solve_part1(input: &str) -> usize {
    input
        .parse::<Map>()
        .unwrap()
        .run(|map, coordinate| coordinate.y > map.maximum_depth)
}

pub fn solve_part2(input: &str) -> usize {
    let mut map = input.parse::<Map>().unwrap();
    for coordinate in Coordinate::from_to(
        Coordinate {
            x: 0,
            y: map.maximum_depth + 2,
        },
        Coordinate {
            x: 1000,
            y: map.maximum_depth + 2,
        },
    ) {
        map.buffer.insert(coordinate, Point::Rock);
    }
    map.run(|map, coordinate| {
        coordinate == Coordinate { x: 500, y: 0 }
            && map.buffer.contains_key(&coordinate.down())
            && map.buffer.contains_key(&coordinate.down_left())
            && map.buffer.contains_key(&coordinate.down_right())
    }) + 1
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::day14::{solve_part1, solve_part2};

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 24);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 93);
    }
}
