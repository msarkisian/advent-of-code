#[derive(Copy, Clone, Debug)]
enum Rock {
    HorizontalLine,
    Plus,
    ReverseL,
    VerticalLine,
    Square,
}

#[aoc(day17, part1)]
fn part1(input: &str) -> usize {
    let mut rocks = vec![
        Rock::HorizontalLine,
        Rock::Plus,
        Rock::ReverseL,
        Rock::VerticalLine,
        Rock::Square,
    ]
    .into_iter()
    .cycle();
    let mut wind = input.chars().cycle();

    let mut column = vec![vec![false; 7]];
    let mut drop_height = 3;

    for _ in 0..2022 {
        while column.len() <= drop_height + 4 {
            // tallest blocks are 3 units
            column.push(vec![false; 7]);
        }
        let mut rock: Vec<(usize, usize)> = match rocks.next().unwrap() {
            Rock::HorizontalLine => vec![
                (2, drop_height),
                (3, drop_height),
                (4, drop_height),
                (5, drop_height),
            ],
            Rock::Plus => vec![
                (3, drop_height),
                (2, drop_height + 1),
                (3, drop_height + 1),
                (4, drop_height + 1),
                (3, drop_height + 2),
            ],
            Rock::ReverseL => vec![
                (2, drop_height),
                (3, drop_height),
                (4, drop_height),
                (4, drop_height + 1),
                (4, drop_height + 2),
            ],
            Rock::VerticalLine => vec![
                (2, drop_height),
                (2, drop_height + 1),
                (2, drop_height + 2),
                (2, drop_height + 3),
            ],
            Rock::Square => vec![
                (2, drop_height),
                (2, drop_height + 1),
                (3, drop_height),
                (3, drop_height + 1),
            ],
        };

        loop {
            match wind.next().unwrap() {
                '>' => {
                    if rock.iter().all(|(x, y)| x + 1 < 7 && !column[*y][*x + 1]) {
                        rock = rock.into_iter().map(|(x, y)| (x + 1, y)).collect();
                    }
                }
                '<' => {
                    if rock
                        .iter()
                        .all(|(x, y)| x.wrapping_sub(1) < 7 && !column[*y][*x - 1])
                    {
                        rock = rock.into_iter().map(|(x, y)| (x - 1, y)).collect();
                    }
                }
                _ => panic!("unexpected character in input"),
            }
            if rock.iter().any(|(x, y)| *y == 0 || column[*y - 1][*x]) {
                for (x, y) in rock {
                    drop_height = std::cmp::max(drop_height, y + 4);
                    column[y][x] = true;
                }
                break;
            }

            rock = rock.into_iter().map(|(x, y)| (x, y - 1)).collect();
        }
    }
    drop_height - 3
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        let input = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        assert_eq!(part1(input), 3068);
    }
}
