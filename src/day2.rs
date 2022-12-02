use std::cmp::Ordering;

#[derive(PartialEq, Clone, Copy)]
enum RPSThrow {
    Rock,
    Paper,
    Scissors,
}

impl PartialOrd for RPSThrow {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self {
            Self::Rock => match other {
                RPSThrow::Rock => Some(Ordering::Equal),
                RPSThrow::Paper => Some(Ordering::Less),
                RPSThrow::Scissors => Some(Ordering::Greater),
            },
            Self::Paper => match other {
                RPSThrow::Rock => Some(Ordering::Greater),
                RPSThrow::Paper => Some(Ordering::Equal),
                RPSThrow::Scissors => Some(Ordering::Less),
            },
            Self::Scissors => match other {
                RPSThrow::Rock => Some(Ordering::Less),
                RPSThrow::Paper => Some(Ordering::Greater),
                RPSThrow::Scissors => Some(Ordering::Equal),
            },
        }
    }
}

impl RPSThrow {
    pub fn losing_partner(&self) -> RPSThrow {
        match self {
            RPSThrow::Rock => RPSThrow::Scissors,
            RPSThrow::Paper => RPSThrow::Rock,
            RPSThrow::Scissors => RPSThrow::Paper,
        }
    }

    pub fn winning_partner(&self) -> RPSThrow {
        match self {
            RPSThrow::Rock => RPSThrow::Paper,
            RPSThrow::Paper => RPSThrow::Scissors,
            RPSThrow::Scissors => RPSThrow::Rock,
        }
    }
}

#[aoc(day2, part1)]
fn part1(input: &str) -> i32 {
    let mut score = 0;
    for line in input.lines() {
        let mut chars = line.chars();

        let their_throw = match chars.next().unwrap() {
            'A' => RPSThrow::Rock,
            'B' => RPSThrow::Paper,
            'C' => RPSThrow::Scissors,
            _ => panic!("unexpected character in input"),
        };
        chars.next();
        let my_throw = match chars.next().unwrap() {
            'X' => {
                score += 1;
                RPSThrow::Rock
            }
            'Y' => {
                score += 2;
                RPSThrow::Paper
            }
            'Z' => {
                score += 3;
                RPSThrow::Scissors
            }
            _ => panic!("unexpected character in input"),
        };

        match my_throw.partial_cmp(&their_throw) {
            Some(Ordering::Greater) => score += 6,
            Some(Ordering::Equal) => score += 3,
            Some(Ordering::Less) => score += 0,
            None => unreachable!(),
        }
    }
    score
}

#[aoc(day2, part2)]
fn part2(input: &str) -> i32 {
    let mut score = 0;
    for line in input.lines() {
        let mut chars = line.chars();

        let their_throw = match chars.next().unwrap() {
            'A' => RPSThrow::Rock,
            'B' => RPSThrow::Paper,
            'C' => RPSThrow::Scissors,
            _ => panic!("unexpected character in input"),
        };
        chars.next();

        let my_throw = match chars.next().unwrap() {
            'X' => {
                // I lose
                score += 0;
                their_throw.losing_partner()
            }
            'Y' => {
                // I draw
                score += 3;
                their_throw
            }
            'Z' => {
                // I win
                score += 6;
                their_throw.winning_partner()
            }
            _ => panic!("unexpected character in input"),
        };

        match my_throw {
            RPSThrow::Rock => score += 1,
            RPSThrow::Paper => score += 2,
            RPSThrow::Scissors => score += 3,
        }
    }
    score
}
