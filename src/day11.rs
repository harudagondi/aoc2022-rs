use std::{
    collections::VecDeque,
    ops::{Add, Div, Mul, Sub},
    str::FromStr,
};

use itertools::Itertools;

#[derive(Debug, Clone, Copy)]
struct Item {
    worry_level: u64,
}

impl Add<Item> for Item {
    type Output = Item;

    fn add(self, rhs: Item) -> Self::Output {
        Item {
            worry_level: self.worry_level + rhs.worry_level,
        }
    }
}

impl Sub<Item> for Item {
    type Output = Item;

    fn sub(self, rhs: Item) -> Self::Output {
        Item {
            worry_level: self.worry_level - rhs.worry_level,
        }
    }
}

impl Mul<Item> for Item {
    type Output = Item;

    fn mul(self, rhs: Item) -> Self::Output {
        Item {
            worry_level: self.worry_level * rhs.worry_level,
        }
    }
}

impl Div<Item> for Item {
    type Output = Item;

    fn div(self, rhs: Item) -> Self::Output {
        Item {
            worry_level: self.worry_level / rhs.worry_level,
        }
    }
}

impl From<u64> for Item {
    fn from(worry_level: u64) -> Self {
        Self { worry_level }
    }
}

struct Items(VecDeque<Item>);

#[derive(Debug, Clone, Copy)]
struct Index(usize);

impl FromStr for Index {
    type Err = <usize as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Index(usize::from_str(s)?))
    }
}

struct Operation {
    left: Operand,
    operator: Operator,
    right: Operand,
}

impl Operation {
    fn operate(&self, old: Item) -> Item {
        match (self.left, self.operator, self.right) {
            (Operand::Variable, Operator::Add, Operand::Variable) => old + old,
            (Operand::Variable, Operator::Add, Operand::Number(y)) => old + y.into(),
            (Operand::Variable, Operator::Sub, Operand::Variable) => 0.into(),
            (Operand::Variable, Operator::Sub, Operand::Number(y)) => old - y.into(),
            (Operand::Variable, Operator::Mul, Operand::Variable) => old * old,
            (Operand::Variable, Operator::Mul, Operand::Number(y)) => old * y.into(),
            (Operand::Variable, Operator::Div, Operand::Variable) => 1.into(),
            (Operand::Variable, Operator::Div, Operand::Number(y)) => old / y.into(),
            (Operand::Number(x), Operator::Add, Operand::Variable) => Item::from(x) + old,
            (Operand::Number(x), Operator::Add, Operand::Number(y)) => Item::from(x) + y.into(),
            (Operand::Number(x), Operator::Sub, Operand::Variable) => Item::from(x) - old,
            (Operand::Number(x), Operator::Sub, Operand::Number(y)) => Item::from(x) - y.into(),
            (Operand::Number(x), Operator::Mul, Operand::Variable) => Item::from(x) * old,
            (Operand::Number(x), Operator::Mul, Operand::Number(y)) => Item::from(x) * y.into(),
            (Operand::Number(x), Operator::Div, Operand::Variable) => Item::from(x) / old,
            (Operand::Number(x), Operator::Div, Operand::Number(y)) => Item::from(x) / y.into(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Variable,
    Number(u64),
}

#[derive(Debug, Clone, Copy)]
enum Operator {
    Add,
    Sub,
    Mul,
    Div,
}

struct Test {
    divisibility_factor: u64,
    is_true: Index,
    is_false: Index,
}

impl Test {
    fn test(&self, item: Item) -> Index {
        if item.worry_level % self.divisibility_factor == 0 {
            self.is_true
        } else {
            self.is_false
        }
    }
}

struct Monkey {
    items: Items,
    operation: Operation,
    test: Test,
    counter: usize,
}

impl FromStr for Operand {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            s if s.parse::<u64>().is_ok() => Operand::Number(s.parse().unwrap()),
            "old" => Operand::Variable,
            _ => unreachable!(),
        })
    }
}

impl FromStr for Monkey {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let _index = Index(
            lines
                .next()
                .unwrap()
                .split_whitespace()
                .nth(1)
                .unwrap()
                .chars()
                .next()
                .unwrap()
                .to_string()
                .parse()
                .unwrap(),
        );
        let starting_items = Items(
            lines
                .next()
                .unwrap()
                .split_whitespace()
                .skip(2)
                .map(|number| {
                    number
                        .chars()
                        .take_while(|digit| digit.to_string().parse::<u8>().is_ok())
                        .collect::<String>()
                        .parse()
                        .unwrap()
                })
                .map(|number| Item {
                    worry_level: number,
                })
                .collect(),
        );
        let operation = {
            let (left, operator, right) = lines
                .next()
                .unwrap()
                .split_whitespace()
                .skip(3)
                .collect_tuple()
                .unwrap();
            let left: Operand = left.parse().unwrap();
            let operator = match operator {
                "+" => Operator::Add,
                "-" => Operator::Sub,
                "*" => Operator::Mul,
                "/" => Operator::Div,
                _ => unreachable!(),
            };
            let right: Operand = right.parse().unwrap();
            Operation {
                left,
                operator,
                right,
            }
        };

        let test = {
            let divisibility_factor = lines
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
            let is_true = lines
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();
            let is_false = lines
                .next()
                .unwrap()
                .split_whitespace()
                .last()
                .unwrap()
                .parse()
                .unwrap();

            Test {
                divisibility_factor,
                is_true,
                is_false,
            }
        };

        Ok(Self {
            items: starting_items,
            operation,
            test,
            counter: 0,
        })
    }
}

impl Monkey {
    fn inspect<const DIV3: bool>(&mut self, divisor: u64) -> Vec<(Index, Item)> {
        let mut to_throw = Vec::new();
        while let Some(item) = self.items.0.pop_front() {
            let item = Item {
                worry_level: self.operation.operate(item).worry_level % divisor,
            };
            let item = if DIV3 { item / 3.into() } else { item };
            let index = self.test.test(item);
            to_throw.push((index, item));
            self.counter += 1;
        }
        to_throw
    }
}

fn parse(input: &str) -> Vec<Monkey> {
    let input = input.replace("\r\n", "\n");
    input
        .split("\n\n")
        .map(Monkey::from_str)
        .try_collect()
        .unwrap()
}

fn throw_items(monkeys: &mut [Monkey], items: Vec<(Index, Item)>) {
    for (index, item) in items {
        monkeys[index.0].items.0.push_back(item);
    }
}

pub fn solve_part1(input: &str) -> usize {
    let mut monkeys = parse(input);
    for _ in 0..20 {
        for index in 0..monkeys.len() {
            let to_throw = monkeys[index].inspect::<true>(u64::MAX);
            throw_items(&mut monkeys, to_throw);
        }
    }
    monkeys
        .into_iter()
        .sorted_by(|a, b| a.counter.cmp(&b.counter))
        .rev()
        .take(2)
        .map(|monkey| monkey.counter)
        .product()
}
fn lcm(numbers: &[u64]) -> u64 {
    let mut lcm = 1;
    for &number in numbers {
        lcm = lcm * number / gcd(lcm, number);
    }
    lcm
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn solve_part2(input: &str) -> usize {
    let mut monkeys = parse(input);
    let lcm = lcm(&monkeys
        .iter()
        .map(|monkey| monkey.test.divisibility_factor)
        .collect_vec());
    for _ in 0..10_000 {
        for index in 0..monkeys.len() {
            let to_throw = monkeys[index].inspect::<false>(lcm);
            throw_items(&mut monkeys, to_throw);
        }
    }
    monkeys
        .into_iter()
        .sorted_by(|a, b| a.counter.cmp(&b.counter))
        .rev()
        .take(2)
        .map(|monkey| monkey.counter)
        .product()
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;

    use crate::day11::{solve_part1, solve_part2};

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 10605);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 2_713_310_158);
    }
}
