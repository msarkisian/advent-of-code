#[derive(Debug, PartialEq)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

impl Range {
    /// Converts two hyphen-seperated numbers into a range struct
    pub fn from(str: &str) -> Self {
        let mut nums = str.split('-');

        Self {
            start: nums.next().unwrap().parse().unwrap(),
            end: nums.next().unwrap().parse().unwrap(),
        }
    }
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<(Range, Range)> {
    let mut output = Vec::new();
    for line in input.lines() {
        let mut ranges = line.split(',');
        output.push((
            Range::from(ranges.next().unwrap()),
            Range::from(ranges.next().unwrap()),
        ))
    }
    output
}

#[aoc(day4, part1)]
fn part1(input: &[(Range, Range)]) -> u32 {
    let mut count = 0;

    for (first, second) in input {
        if (first.start <= second.start && first.end >= second.end)
            || (second.start <= first.start && second.end >= first.end)
        {
            count += 1
        }
    }
    count
}

#[aoc(day4, part2)]
fn part2(input: &[(Range, Range)]) -> u32 {
    let mut count = 0;

    for (first, second) in input {
        if (second.start <= first.end && second.start >= first.start)
            || (first.start <= second.end && first.start >= second.start)
        {
            count += 1
        }
    }
    count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn range_generation() {
        assert_eq!(Range::from("1-5"), Range { start: 1, end: 5 })
    }

    #[test]
    fn input_parsing() {
        assert_eq!(
            input_generator("2-4,6-8\n2-3,4-5\n5-7,7-9\n"),
            vec![
                (Range { start: 2, end: 4 }, Range { start: 6, end: 8 }),
                (Range { start: 2, end: 3 }, Range { start: 4, end: 5 }),
                (Range { start: 5, end: 7 }, Range { start: 7, end: 9 }),
            ]
        )
    }
}
