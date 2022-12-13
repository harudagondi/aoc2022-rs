use std::{cmp::Ordering, collections::VecDeque, str::FromStr};

use itertools::Itertools;
use serde_json::Value;

#[derive(Debug, Clone)]
enum Packets {
    Array(VecDeque<Packets>),
    Number(u8),
}

impl From<Value> for Packets {
    fn from(value: Value) -> Self {
        match value {
            Value::Number(x) => Packets::Number(x.as_u64().unwrap().try_into().unwrap()),
            Value::Array(arr) => Packets::Array(arr.into_iter().map(Packets::from).collect()),
            _ => panic!(),
        }
    }
}

impl PartialEq for Packets {
    fn eq(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(std::cmp::Ordering::Equal))
    }
}

impl PartialOrd for Packets {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self.clone(), other.clone()) {
            (Packets::Array(mut left), Packets::Array(mut right)) => {
                match (left.pop_front(), right.pop_front()) {
                    (None, None) => Some(std::cmp::Ordering::Equal),
                    (None, Some(_)) => Some(std::cmp::Ordering::Less),
                    (Some(_), None) => Some(std::cmp::Ordering::Greater),
                    (Some(l), Some(r)) => match l.partial_cmp(&r) {
                        Some(std::cmp::Ordering::Equal) => {
                            Packets::Array(left).partial_cmp(&Packets::Array(right))
                        }
                        ord => ord,
                    },
                }
            }
            (left @ Packets::Array(_), right @ Packets::Number(_)) => {
                left.partial_cmp(&Packets::Array(VecDeque::from([right])))
            }
            (left @ Packets::Number(_), right @ Packets::Array(_)) => {
                Packets::Array(VecDeque::from([left])).partial_cmp(&right)
            }
            (Packets::Number(left), Packets::Number(right)) => left.partial_cmp(&right),
        }
    }
}

impl Eq for Packets {}

impl Ord for Packets {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.partial_cmp(other) {
            Some(ord) => ord,
            None => unreachable!(),
        }
    }
}

fn parse(input: &str) -> Vec<[Packets; 2]> {
    let input = input.replace("\r\n", "\n");
    input
        .split("\n\n")
        .map(str::lines)
        .map(|mut pair| [pair.next().unwrap(), pair.next().unwrap()])
        .map(|pair| pair.map(|packets| serde_json::from_str(packets).unwrap()))
        .map(|pair: [Value; 2]| pair.map(Packets::from))
        .collect()
}

pub fn solve_part1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .enumerate()
        .map(|(i, [left, right])| (i, left.cmp(&right)))
        .filter_map(|(i, ord)| (ord == Ordering::Less).then_some(i + 1))
        .sum()
}

pub fn solve_part2(input: &str) -> usize {
    let mut input = parse(input);
    let packets_2 = Packets::from(Value::from_str("[[2]]").unwrap());
    let packets_6 = Packets::from(Value::from_str("[[6]]").unwrap());
    input.extend([[packets_2.clone(), packets_6.clone()]]);
    let input = input
        .into_iter()
        .flatten()
        .sorted_unstable_by(Packets::cmp)
        .collect_vec();
    let index_2 = input
        .iter()
        .find_position(|packets| packets == &&packets_2)
        .unwrap()
        .0;
    let index_6 = input
        .iter()
        .find_position(|packets| packets == &&packets_6)
        .unwrap()
        .0;
    (index_2 + 1) * (index_6 + 1)
}

#[cfg(test)]
mod tests {
    use std::{cmp::Ordering, str::FromStr};

    use pretty_assertions::assert_eq;
    use serde_json::Value;

    use crate::day13::{solve_part1, solve_part2, Packets};

    #[test]
    fn comparison() {
        fn assert_order(left: &str, right: &str, order: Ordering) {
            assert_eq!(
                Packets::from(Value::from_str(left).unwrap())
                    .cmp(&Packets::from(Value::from_str(right).unwrap())),
                order
            );
        }
        assert_order("[1, 1, 3, 1, 1]", "[1, 1, 5, 1, 1]", Ordering::Less);
        assert_order("[[1],[2,3,4]]", "[[1],4]", Ordering::Less);
        assert_order("[9]", "[[8,7,6]]", Ordering::Greater);
        assert_order("[[4,4],4,4]", "[[4,4],4,4,4]", Ordering::Less);
        assert_order("[7,7,7,7]", "[7,7,7]", Ordering::Greater);
        assert_order("[]", "[3]", Ordering::Less);
        assert_order("[[[]]]", "[[]]", Ordering::Greater);
        assert_order(
            "[1,[2,[3,[4,[5,6,7]]]],8,9]",
            "[1,[2,[3,[4,[5,6,0]]]],8,9]",
            Ordering::Greater,
        );
    }

    const INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 13);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 140);
    }
}
