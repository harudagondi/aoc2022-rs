#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl Hand {
    fn points(self) -> u32 {
        match self {
            Hand::Rock => 1,
            Hand::Paper => 2,
            Hand::Scissors => 3,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Loss,
}

impl Outcome {
    fn points(self) -> u32 {
        match self {
            Outcome::Win => 6,
            Outcome::Draw => 3,
            Outcome::Loss => 0,
        }
    }
}

fn parse(input: &str) -> impl Iterator<Item = (char, char)> + '_ {
    input
        .lines()
        .map(|line| line.split(' '))
        .map(|mut hand| (hand.next().unwrap(), hand.next().unwrap()))
        .map(|(left, right)| (left.chars().next().unwrap(), right.chars().next().unwrap()))
}

pub fn solve_part1(input: &str) -> u32 {
    parse(input)
        .map(|(opponent, player)| (convert_to_hand(opponent), convert_to_hand(player)))
        .map(|(opponent, player)| (player, battle_outcome(opponent, player)))
        .map(|(player, outcome)| player.points() + outcome.points())
        .sum()
}

pub fn solve_part2(input: &str) -> u32 {
    parse(input)
        .map(|(opponent, outcome)| (convert_to_hand(opponent), convert_to_outcome(outcome)))
        .map(|(opponent, outcome)| (get_player_hand(opponent, outcome), outcome))
        .map(|(player, outcome)| player.points() + outcome.points())
        .sum()
}

fn convert_to_hand(c: char) -> Hand {
    match c {
        'A' | 'X' => Hand::Rock,
        'B' | 'Y' => Hand::Paper,
        'C' | 'Z' => Hand::Scissors,
        _ => panic!(),
    }
}

fn battle_outcome(opponent: Hand, player: Hand) -> Outcome {
    match (opponent, player) {
        (Hand::Rock, Hand::Paper)
        | (Hand::Paper, Hand::Scissors)
        | (Hand::Scissors, Hand::Rock) => Outcome::Win,
        (Hand::Rock, Hand::Scissors)
        | (Hand::Paper, Hand::Rock)
        | (Hand::Scissors, Hand::Paper) => Outcome::Loss,
        (Hand::Paper, Hand::Paper)
        | (Hand::Rock, Hand::Rock)
        | (Hand::Scissors, Hand::Scissors) => Outcome::Draw,
    }
}

fn convert_to_outcome(c: char) -> Outcome {
    match c {
        'X' => Outcome::Loss,
        'Y' => Outcome::Draw,
        'Z' => Outcome::Win,
        _ => panic!(),
    }
}

fn get_player_hand(opponent: Hand, outcome: Outcome) -> Hand {
    match outcome {
        Outcome::Win => match opponent {
            Hand::Rock => Hand::Paper,
            Hand::Paper => Hand::Scissors,
            Hand::Scissors => Hand::Rock,
        },
        Outcome::Draw => opponent,
        Outcome::Loss => match opponent {
            Hand::Rock => Hand::Scissors,
            Hand::Paper => Hand::Rock,
            Hand::Scissors => Hand::Paper,
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::day2::{solve_part1, solve_part2};

    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1() {
        assert_eq!(solve_part1(INPUT), 15);
    }

    #[test]
    fn part2() {
        assert_eq!(solve_part2(INPUT), 12);
    }
}
