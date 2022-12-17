use std::collections::HashMap;

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

#[aoc(day17, part2)]
fn part2(input: &str) -> usize {
    let mut rocks = vec![
        Rock::HorizontalLine,
        Rock::Plus,
        Rock::ReverseL,
        Rock::VerticalLine,
        Rock::Square,
    ]
    .into_iter()
    .cycle();
    let mut wind = input.chars().enumerate().cycle().peekable();

    let mut column = vec![vec![false; 7]];
    let mut drop_height = 3;

    let mut seen_topologies: HashMap<([usize; 7], usize), (usize, usize)> = HashMap::new();

    let mut i = 0usize;
    let mut cycle_height = 0;
    let mut found_cycle = false;

    loop {
        if i == 1000000000000 {
            break drop_height - 3 + cycle_height;
        }
        while column.len() <= drop_height + 4 {
            // tallest blocks are 3 units
            column.push(vec![false; 7]);
        }
        let mut rock = match rocks.next().unwrap() {
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
            match wind.next().unwrap().1 {
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
        let mut distance_from_top = [0usize; 7];
        'x: for i in 0..7 {
            for h in (0..drop_height - 3).rev() {
                if column[h][i] {
                    distance_from_top[i] = drop_height - h;
                    continue 'x;
                }
            }
        }
        let next_wind_index = wind.peek().unwrap().0;

        if !found_cycle && seen_topologies.contains_key(&(distance_from_top, next_wind_index)) {
            let (cycle_start, last_drop_height) = seen_topologies
                .get(&(distance_from_top, next_wind_index))
                .unwrap()
                .clone();
            println!("CYCLE FOUND!! current iteration: {i}; topology last seen at {cycle_start}.",);
            println!("Current drop height is {drop_height}, last was {last_drop_height}");
            let cycle_len = i - cycle_start;
            let times_in_cycle = (1000000000000 - i) / cycle_len;
            println!("times in cycle :{}", times_in_cycle);
            println!("cycle len {}", cycle_len);

            let per_cycle_height = drop_height - last_drop_height;

            cycle_height = per_cycle_height * times_in_cycle;
            println!("cycle height {}", cycle_height);

            i = 1000000000000 - ((1000000000000 - i) % cycle_len);
            found_cycle = true;
        } else {
            seen_topologies.insert((distance_from_top, next_wind_index), (i, drop_height));
        }
        i += 1;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 3068);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 1514285714288);
    }
}
