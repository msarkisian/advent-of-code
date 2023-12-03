use std::{fmt::Display, ops::Add, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
struct Snafu(isize);

#[derive(Debug)]
struct ParseSnafuError;

impl FromStr for Snafu {
    type Err = ParseSnafuError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut sum = 0;
        let mut radix = 1;

        for char in s.chars().rev() {
            match char {
                '0' => (),
                '1' => sum += radix,
                '2' => sum += 2 * radix,
                '-' => sum -= radix,
                '=' => sum -= 2 * radix,
                _ => {
                    return Err(ParseSnafuError);
                }
            }
            radix *= 5;
        }
        Ok(Snafu(sum))
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut base_5_digits = Vec::new();
        let mut remainder = self.0;

        while remainder > 0 {
            base_5_digits.push(remainder % 5);
            remainder /= 5;
        }

        let mut snafu_digits = Vec::new();
        let mut carrying = false;
        for digit in base_5_digits {
            snafu_digits.push(match digit {
                d @ (0 | 1) => {
                    if carrying {
                        carrying = false;
                        d + 1
                    } else {
                        d
                    }
                }
                2 => {
                    if carrying {
                        -2
                    } else {
                        2
                    }
                }
                3 => {
                    if carrying {
                        -1
                    } else {
                        carrying = true;
                        -2
                    }
                }
                4 => {
                    if carrying {
                        0
                    } else {
                        carrying = true;
                        -1
                    }
                }
                _ => unreachable!(),
            })
        }
        if carrying {
            snafu_digits.push(1);
        }

        let snafu_string = snafu_digits
            .into_iter()
            .rev()
            .map(|d| {
                if d == -2 {
                    '='
                } else if d == -1 {
                    '-'
                } else {
                    char::from_digit(d as u32, 10).unwrap()
                }
            })
            .collect::<String>();

        write!(f, "{}", snafu_string)
    }
}

impl Add for Snafu {
    type Output = Snafu;

    fn add(self, rhs: Self) -> Self::Output {
        Snafu(self.0 + rhs.0)
    }
}

#[aoc(day25, part1)]
fn part1(input: &str) -> Snafu {
    input
        .lines()
        .map(|l| l.parse::<Snafu>().unwrap())
        .fold(Snafu(0), |a, b| a + b)
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "1=-0-2\n12111\n2=0=\n21\n2=01\n111\n20012\n112\n1=-1=\n1-12\n12\n1=\n122";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), Snafu(4890));
        assert_eq!(Snafu(4890).to_string(), "2=-1=0");
    }
}
