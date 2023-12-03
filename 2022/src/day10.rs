use std::collections::HashMap;

#[aoc(day10, part1)]
fn part1(input: &str) -> isize {
    let mut cycles: isize = 0;
    let mut current_strength: isize = 1;
    let mut strengths = HashMap::with_capacity(220);
    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        match tokens.next().unwrap() {
            "noop" => {
                cycles += 1;
                strengths.insert(cycles, current_strength);
            }
            "addx" => {
                cycles += 1;
                strengths.insert(cycles, current_strength);
                let addend = tokens.next().unwrap().parse::<isize>().unwrap();
                current_strength += addend;
                cycles += 1;
                strengths.insert(cycles, current_strength);
            }
            _ => unreachable!("bad input"),
        }
    }
    strengths
        .iter()
        .filter_map(|(k, v)| {
            if *k == 20 || *k == 60 || *k == 100 || *k == 140 || *k == 180 || *k == 220 {
                Some(k * v)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day10, part2)]
fn part2(input: &str) -> String {
    let mut cycles: isize = 0;
    let mut current_strength: isize = 1;
    let mut strengths = HashMap::with_capacity(220);
    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        match tokens.next().unwrap() {
            "noop" => {
                cycles += 1;
                strengths.insert(cycles, current_strength);
            }
            "addx" => {
                cycles += 1;
                strengths.insert(cycles, current_strength);
                let addend = tokens.next().unwrap().parse::<isize>().unwrap();
                current_strength += addend;
                cycles += 1;
                strengths.insert(cycles, current_strength);
            }
            _ => unreachable!("bad input"),
        }
    }
    let mut crt = [['x'; 40]; 6];
    let mut current_cycle = 0;
    strengths.insert(0, 1);

    for row in crt.iter_mut() {
        for (column, pixel) in row.iter_mut().enumerate() {
            let current_strength = strengths[&current_cycle];
            if current_strength == column as isize
                || current_strength == column as isize - 1
                || current_strength == column as isize + 1
            {
                *pixel = '#';
            } else {
                *pixel = '.';
            }
            current_cycle += 1;
        }
    }

    let mut output = String::from('\n');
    for row in crt {
        for pixel in row {
            output.push(pixel)
        }
        output.push('\n')
    }

    output
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example_case() {
        let input = "addx 15
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

        assert_eq!(part1(input), 13360);
    }
}
