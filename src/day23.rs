use std::{
    cell::Cell,
    collections::{HashMap, HashSet},
};

enum Dir {
    North,
    South,
    West,
    East,
}

struct Elf {
    considering: Cell<Option<(isize, isize)>>,
}

impl Elf {
    fn new() -> Self {
        Self {
            considering: Cell::new(None),
        }
    }
}

#[aoc(day23, part1)]
fn part1(input: &str) -> isize {
    let mut elves = HashMap::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '#' {
                elves.insert((x as isize, y as isize), Elf::new());
            }
        }
    }
    let mut directions = [Dir::North, Dir::South, Dir::West, Dir::East]
        .iter()
        .cycle();

    for _ in 0..10 {
        let round_priorities = match directions.next().unwrap() {
            Dir::North => [Dir::North, Dir::South, Dir::West, Dir::East],
            Dir::South => [Dir::South, Dir::West, Dir::East, Dir::North],
            Dir::West => [Dir::West, Dir::East, Dir::North, Dir::South],
            Dir::East => [Dir::East, Dir::North, Dir::South, Dir::West],
        };

        let mut considered_squares = HashSet::new();
        let mut banned_squares = HashSet::new();

        for (&(x, y), elf) in elves.iter() {
            if !elves.contains_key(&(x, y - 1))
                && !elves.contains_key(&(x + 1, y - 1))
                && !elves.contains_key(&(x + 1, y))
                && !elves.contains_key(&(x + 1, y + 1))
                && !elves.contains_key(&(x, y + 1))
                && !elves.contains_key(&(x - 1, y + 1))
                && !elves.contains_key(&(x - 1, y))
                && !elves.contains_key(&(x - 1, y - 1))
            {
                continue;
            }

            for priority in round_priorities.iter() {
                match priority {
                    Dir::North => {
                        if !elves.contains_key(&(x, y - 1))
                            && !elves.contains_key(&(x + 1, y - 1))
                            && !elves.contains_key(&(x - 1, y - 1))
                        {
                            if banned_squares.contains(&(x, y - 1)) {
                                break;
                            } else if considered_squares.remove(&(x, y - 1)) {
                                banned_squares.insert((x, y - 1));
                                break;
                            } else {
                                considered_squares.insert((x, y - 1));
                                elf.considering.set(Some((x, y - 1)));
                                break;
                            }
                        }
                    }
                    Dir::South => {
                        if !elves.contains_key(&(x, y + 1))
                            && !elves.contains_key(&(x + 1, y + 1))
                            && !elves.contains_key(&(x - 1, y + 1))
                        {
                            if banned_squares.contains(&(x, y + 1)) {
                                break;
                            } else if considered_squares.remove(&(x, y + 1)) {
                                banned_squares.insert((x, y + 1));
                                break;
                            } else {
                                considered_squares.insert((x, y + 1));
                                elf.considering.set(Some((x, y + 1)));
                                break;
                            }
                        }
                    }
                    Dir::West => {
                        if !elves.contains_key(&(x - 1, y))
                            && !elves.contains_key(&(x - 1, y - 1))
                            && !elves.contains_key(&(x - 1, y + 1))
                        {
                            if banned_squares.contains(&(x - 1, y)) {
                                break;
                            } else if considered_squares.remove(&(x - 1, y)) {
                                banned_squares.insert((x - 1, y));
                                break;
                            } else {
                                considered_squares.insert((x - 1, y));
                                elf.considering.set(Some((x - 1, y)));
                                break;
                            }
                        }
                    }
                    Dir::East => {
                        if !elves.contains_key(&(x + 1, y))
                            && !elves.contains_key(&(x + 1, y - 1))
                            && !elves.contains_key(&(x + 1, y + 1))
                        {
                            if banned_squares.contains(&(x + 1, y)) {
                                break;
                            } else if considered_squares.remove(&(x + 1, y)) {
                                banned_squares.insert((x + 1, y));
                                break;
                            } else {
                                considered_squares.insert((x + 1, y));
                                elf.considering.set(Some((x + 1, y)));
                                break;
                            }
                        }
                    }
                }
            }
        }
        elves = elves
            .drain()
            .map(|((x, y), elf)| {
                if let Some((new_x, new_y)) = elf.considering.get() {
                    elf.considering.set(None);
                    if !banned_squares.contains(&(new_x, new_y)) {
                        ((new_x, new_y), elf)
                    } else {
                        ((x, y), elf)
                    }
                } else {
                    ((x, y), elf)
                }
            })
            .collect();
    }

    let (min_x, max_x, min_y, max_y) = elves.keys().fold(
        (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
        |(min_x, max_x, min_y, max_y), (x, y)| {
            (min_x.min(*x), max_x.max(*x), min_y.min(*y), max_y.max(*y))
        },
    );
    println!("x: {}-{}, y: {}-{}", min_x, max_x, min_y, max_y);
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if elves.contains_key(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        print!("\n");
    }
    println!("{}", elves.len());
    (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as isize
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "....#..\n..###.#\n#...#.#\n.#...##\n#.###..\n##.#.##\n.#..#..";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 110);
    }
}
