use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Floor,
    Wall,
}

enum TurnDirection {
    Right,
    Left,
}

#[derive(Debug, PartialEq, Eq)]
enum Facing {
    Right,
    Down,
    Left,
    Up,
}

enum Instruction {
    Move(usize),
    Turn(TurnDirection),
}

#[aoc_generator(day22)]
fn input_generator(input: &str) -> (HashMap<(usize, usize), Tile>, Vec<Instruction>) {
    let (map, directions) = input.split_once("\n\n").unwrap();

    let mut hashmap = HashMap::new();
    for (line, y) in map.lines().zip(1..) {
        for (char, x) in line.chars().zip(1..) {
            match char {
                ' ' => (),
                '.' => {
                    hashmap.insert((x, y), Tile::Floor);
                }
                '#' => {
                    hashmap.insert((x, y), Tile::Wall);
                }
                _ => unreachable!(),
            }
        }
    }

    let mut direction_vec = Vec::new();

    let mut current_num = String::new();
    for char in directions.chars() {
        match char {
            n @ '0'..='9' => current_num.push(n),
            'R' => {
                let num = current_num.parse();
                if num.is_ok() {
                    direction_vec.push(Instruction::Move(num.unwrap()));
                    current_num.clear();
                }
                direction_vec.push(Instruction::Turn(TurnDirection::Right));
            }
            'L' => {
                let num = current_num.parse();
                if num.is_ok() {
                    direction_vec.push(Instruction::Move(num.unwrap()));
                    current_num.clear();
                }
                direction_vec.push(Instruction::Turn(TurnDirection::Left));
            }
            _ => unreachable!(),
        }
    }
    if !current_num.is_empty() {
        direction_vec.push(Instruction::Move(current_num.parse().unwrap()));
    }
    (hashmap, direction_vec)
}

fn find_wrapped_location(
    map: &HashMap<(usize, usize), Tile>,
    (cur_x, cur_y): (usize, usize),
    dir: &Facing,
) -> (usize, usize) {
    match dir {
        Facing::Right => {
            for x in 1..map.len() {
                if map.contains_key(&(x, cur_y)) {
                    return (x, cur_y);
                }
            }
            panic!();
        }
        Facing::Left => {
            for x in (1..map.len()).rev() {
                if map.contains_key(&(x, cur_y)) {
                    return (x, cur_y);
                }
            }
            panic!();
        }
        Facing::Down => {
            for y in 1..map.len() {
                if map.contains_key(&(cur_x, y)) {
                    return (cur_x, y);
                }
            }
            panic!();
        }
        Facing::Up => {
            for y in (1..map.len()).rev() {
                if map.contains_key(&(cur_x, y)) {
                    return (cur_x, y);
                }
            }
            panic!();
        }
    }
}

fn find_cube_wrapped_location(
    face_width: usize,
    (cur_x, cur_y): (usize, usize),
    dir: &Facing,
) -> ((usize, usize), Facing) {
    let current_face: u8 = {
        if cur_y <= face_width && cur_x <= 2 * face_width {
            1
        } else if cur_y <= face_width && cur_x > 2 * face_width {
            2
        } else if cur_y <= 2 * face_width {
            3
        } else if cur_y <= 3 * face_width && cur_x <= face_width {
            4
        } else if cur_y <= 3 * face_width && cur_x > face_width {
            5
        } else {
            6
        }
    };

    match current_face {
        1 => match dir {
            Facing::Left => return ((1, 3 * face_width - (cur_y - 1)), Facing::Right),
            Facing::Up => return ((1, cur_x + 2 * face_width), Facing::Right),
            Facing::Right => unreachable!(),
            Facing::Down => unreachable!(),
        },
        2 => match dir {
            Facing::Left => unreachable!(),
            Facing::Up => return ((cur_x - 2 * face_width, 4 * face_width), Facing::Up),
            Facing::Right => return ((face_width * 2, 3 * face_width - (cur_y - 1)), Facing::Left),
            Facing::Down => return ((face_width * 2, cur_x - face_width), Facing::Left),
        },
        3 => match dir {
            Facing::Left => return ((cur_y - face_width, 2 * face_width + 1), Facing::Down),
            Facing::Up => unreachable!(),
            Facing::Right => return ((cur_y + face_width, face_width), Facing::Up),
            Facing::Down => unreachable!(),
        },
        4 => match dir {
            Facing::Left => {
                return (
                    (face_width + 1, face_width - (cur_y - 2 * face_width - 1)),
                    Facing::Right,
                )
            }
            Facing::Up => return ((face_width + 1, cur_x + face_width), Facing::Right),
            Facing::Right => unreachable!(),
            Facing::Down => unreachable!(),
        },
        5 => match dir {
            Facing::Left => unreachable!(),
            Facing::Up => unreachable!(),
            Facing::Right => {
                return (
                    (3 * face_width, face_width - (cur_y - 2 * face_width - 1)),
                    Facing::Left,
                )
            }
            Facing::Down => return ((face_width, 2 * face_width + cur_x), Facing::Left),
        },
        6 => match dir {
            Facing::Left => return ((cur_y - 2 * face_width, 1), Facing::Down),
            Facing::Up => unreachable!(),
            Facing::Right => return ((cur_y - 2 * face_width, 3 * face_width), Facing::Up),
            Facing::Down => return ((2 * face_width + cur_x, 1), Facing::Down),
        },
        _ => unreachable!(),
    }
}

#[aoc(day22, part1)]
fn part1((map, instructions): &(HashMap<(usize, usize), Tile>, Vec<Instruction>)) -> usize {
    let mut current_position = None;
    for x in 1..map.len() {
        if map.get(&(x, 1)) == Some(&Tile::Floor) {
            current_position = Some((x, 1));
            break;
        }
    }
    let mut current_position = current_position.unwrap();
    let mut current_facing = Facing::Right;
    for instruction in instructions {
        match instruction {
            Instruction::Turn(dir) => match dir {
                TurnDirection::Right => match current_facing {
                    Facing::Right => current_facing = Facing::Down,
                    Facing::Down => current_facing = Facing::Left,
                    Facing::Left => current_facing = Facing::Up,
                    Facing::Up => current_facing = Facing::Right,
                },
                TurnDirection::Left => match current_facing {
                    Facing::Right => current_facing = Facing::Up,
                    Facing::Down => current_facing = Facing::Right,
                    Facing::Left => current_facing = Facing::Down,
                    Facing::Up => current_facing = Facing::Left,
                },
            },
            Instruction::Move(steps) => {
                for _ in 0..*steps {
                    match current_facing {
                        Facing::Right => {
                            let next_space = map.get(&(current_position.0 + 1, current_position.1));
                            match next_space {
                                Some(Tile::Floor) => {
                                    current_position = (current_position.0 + 1, current_position.1);
                                }
                                Some(Tile::Wall) => break,
                                None => {
                                    let wrapped_pos = find_wrapped_location(
                                        map,
                                        current_position,
                                        &current_facing,
                                    );
                                    match map.get(&wrapped_pos) {
                                        Some(Tile::Floor) => current_position = wrapped_pos,
                                        Some(Tile::Wall) => break,
                                        None => unreachable!(),
                                    }
                                }
                            }
                        }
                        Facing::Left => {
                            let next_space = map.get(&(current_position.0 - 1, current_position.1));
                            match next_space {
                                Some(Tile::Floor) => {
                                    current_position = (current_position.0 - 1, current_position.1);
                                }
                                Some(Tile::Wall) => break,
                                None => {
                                    let wrapped_pos = find_wrapped_location(
                                        map,
                                        current_position,
                                        &current_facing,
                                    );
                                    match map.get(&wrapped_pos) {
                                        Some(Tile::Floor) => current_position = wrapped_pos,
                                        Some(Tile::Wall) => break,
                                        None => unreachable!(),
                                    }
                                }
                            }
                        }
                        Facing::Down => {
                            let next_space = map.get(&(current_position.0, current_position.1 + 1));
                            match next_space {
                                Some(Tile::Floor) => {
                                    current_position = (current_position.0, current_position.1 + 1);
                                }
                                Some(Tile::Wall) => break,
                                None => {
                                    let wrapped_pos = find_wrapped_location(
                                        map,
                                        current_position,
                                        &current_facing,
                                    );
                                    match map.get(&wrapped_pos) {
                                        Some(Tile::Floor) => current_position = wrapped_pos,
                                        Some(Tile::Wall) => break,
                                        None => unreachable!(),
                                    }
                                }
                            }
                        }
                        Facing::Up => {
                            let next_space = map.get(&(current_position.0, current_position.1 - 1));
                            match next_space {
                                Some(Tile::Floor) => {
                                    current_position = (current_position.0, current_position.1 - 1);
                                }
                                Some(Tile::Wall) => break,
                                None => {
                                    let wrapped_pos = find_wrapped_location(
                                        map,
                                        current_position,
                                        &current_facing,
                                    );
                                    match map.get(&wrapped_pos) {
                                        Some(Tile::Floor) => current_position = wrapped_pos,
                                        Some(Tile::Wall) => break,
                                        None => unreachable!(),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    current_position.1 * 1000
        + current_position.0 * 4
        + match current_facing {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
}

#[aoc(day22, part2)]
fn part2((map, instructions): &(HashMap<(usize, usize), Tile>, Vec<Instruction>)) -> usize {
    let face_width: usize;
    if cfg!(test) {
        face_width = 4;
    } else {
        face_width = 50;
    }
    let mut current_position = None;
    for x in 1..map.len() {
        if map.get(&(x, 1)) == Some(&Tile::Floor) {
            current_position = Some((x, 1));
            break;
        }
    }
    let mut current_position = current_position.unwrap();
    let mut current_facing = Facing::Right;
    for instruction in instructions {
        match instruction {
            Instruction::Turn(dir) => match dir {
                TurnDirection::Right => match current_facing {
                    Facing::Right => current_facing = Facing::Down,
                    Facing::Down => current_facing = Facing::Left,
                    Facing::Left => current_facing = Facing::Up,
                    Facing::Up => current_facing = Facing::Right,
                },
                TurnDirection::Left => match current_facing {
                    Facing::Right => current_facing = Facing::Up,
                    Facing::Down => current_facing = Facing::Right,
                    Facing::Left => current_facing = Facing::Down,
                    Facing::Up => current_facing = Facing::Left,
                },
            },
            Instruction::Move(steps) => {
                for _ in 0..*steps {
                    match current_facing {
                        Facing::Right => {
                            let next_space = map.get(&(current_position.0 + 1, current_position.1));
                            match next_space {
                                Some(Tile::Floor) => {
                                    current_position = (current_position.0 + 1, current_position.1);
                                }
                                Some(Tile::Wall) => break,
                                None => {
                                    println!(
                                        "calculating cube wrap for: {:?}, {:?}",
                                        current_position, current_facing
                                    );
                                    let (wrapped_pos, wrapped_facing) = find_cube_wrapped_location(
                                        face_width,
                                        current_position,
                                        &current_facing,
                                    );
                                    println!(
                                        "wrapped result: {:?} facing {:?}",
                                        wrapped_pos, wrapped_facing
                                    );
                                    match map.get(&wrapped_pos) {
                                        Some(Tile::Floor) => {
                                            current_position = wrapped_pos;
                                            current_facing = wrapped_facing;
                                        }
                                        Some(Tile::Wall) => break,
                                        None => unreachable!(),
                                    }
                                }
                            }
                        }
                        Facing::Left => {
                            let next_space = map.get(&(current_position.0 - 1, current_position.1));
                            match next_space {
                                Some(Tile::Floor) => {
                                    current_position = (current_position.0 - 1, current_position.1);
                                }
                                Some(Tile::Wall) => break,
                                None => {
                                    println!(
                                        "calculating cube wrap for: {:?}, {:?}",
                                        current_position, current_facing
                                    );
                                    let (wrapped_pos, wrapped_facing) = find_cube_wrapped_location(
                                        face_width,
                                        current_position,
                                        &current_facing,
                                    );
                                    println!(
                                        "wrapped result: {:?} facing {:?}",
                                        wrapped_pos, wrapped_facing
                                    );
                                    match map.get(&wrapped_pos) {
                                        Some(Tile::Floor) => {
                                            current_position = wrapped_pos;
                                            current_facing = wrapped_facing;
                                        }
                                        Some(Tile::Wall) => break,
                                        None => unreachable!(),
                                    }
                                }
                            }
                        }
                        Facing::Down => {
                            let next_space = map.get(&(current_position.0, current_position.1 + 1));
                            match next_space {
                                Some(Tile::Floor) => {
                                    current_position = (current_position.0, current_position.1 + 1);
                                }
                                Some(Tile::Wall) => break,
                                None => {
                                    println!(
                                        "calculating cube wrap for: {:?}, {:?}",
                                        current_position, current_facing
                                    );
                                    let (wrapped_pos, wrapped_facing) = find_cube_wrapped_location(
                                        face_width,
                                        current_position,
                                        &current_facing,
                                    );
                                    println!(
                                        "wrapped result: {:?} facing {:?}",
                                        wrapped_pos, wrapped_facing
                                    );
                                    match map.get(&wrapped_pos) {
                                        Some(Tile::Floor) => {
                                            current_position = wrapped_pos;
                                            current_facing = wrapped_facing;
                                        }
                                        Some(Tile::Wall) => break,
                                        None => unreachable!(),
                                    }
                                }
                            }
                        }
                        Facing::Up => {
                            let next_space = map.get(&(current_position.0, current_position.1 - 1));
                            match next_space {
                                Some(Tile::Floor) => {
                                    current_position = (current_position.0, current_position.1 - 1);
                                }
                                Some(Tile::Wall) => break,
                                None => {
                                    println!(
                                        "calculating cube wrap for: {:?}, {:?}",
                                        current_position, current_facing
                                    );
                                    let (wrapped_pos, wrapped_facing) = find_cube_wrapped_location(
                                        face_width,
                                        current_position,
                                        &current_facing,
                                    );
                                    println!(
                                        "wrapped result: {:?} facing {:?}",
                                        wrapped_pos, wrapped_facing
                                    );
                                    match map.get(&wrapped_pos) {
                                        Some(Tile::Floor) => {
                                            current_position = wrapped_pos;
                                            current_facing = wrapped_facing;
                                        }
                                        Some(Tile::Wall) => break,
                                        None => unreachable!(),
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    current_position.1 * 1000
        + current_position.0 * 4
        + match current_facing {
            Facing::Right => 0,
            Facing::Down => 1,
            Facing::Left => 2,
            Facing::Up => 3,
        }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cube_wrapping_from_1() {
        const FACE_WIDTH: usize = 4;
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (5, 1), &Facing::Left),
            ((1, 12), Facing::Right)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (5, 2), &Facing::Left),
            ((1, 11), Facing::Right)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (5, 3), &Facing::Left),
            ((1, 10), Facing::Right)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (5, 4), &Facing::Left),
            ((1, 9), Facing::Right)
        );

        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (5, 1), &Facing::Up),
            ((1, 13), Facing::Right)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (6, 1), &Facing::Up),
            ((1, 14), Facing::Right)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (7, 1), &Facing::Up),
            ((1, 15), Facing::Right)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (8, 1), &Facing::Up),
            ((1, 16), Facing::Right)
        );
    }

    #[test]
    fn cube_wrapping_from_2() {
        const FACE_WIDTH: usize = 4;
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (9, 1), &Facing::Up),
            ((1, 16), Facing::Up)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (10, 1), &Facing::Up),
            ((2, 16), Facing::Up)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (11, 1), &Facing::Up),
            ((3, 16), Facing::Up)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (12, 1), &Facing::Up),
            ((4, 16), Facing::Up)
        );

        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (12, 1), &Facing::Right),
            ((8, 12), Facing::Left)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (12, 2), &Facing::Right),
            ((8, 11), Facing::Left)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (12, 3), &Facing::Right),
            ((8, 10), Facing::Left)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (12, 4), &Facing::Right),
            ((8, 9), Facing::Left)
        );

        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (9, 4), &Facing::Down),
            ((8, 5), Facing::Left)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (10, 4), &Facing::Down),
            ((8, 6), Facing::Left)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (11, 4), &Facing::Down),
            ((8, 7), Facing::Left)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (12, 4), &Facing::Down),
            ((8, 8), Facing::Left)
        );
    }

    #[test]
    fn cube_wrapping_from_3() {
        const FACE_WIDTH: usize = 4;
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (5, 5), &Facing::Left),
            ((1, 9), Facing::Down)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (5, 6), &Facing::Left),
            ((2, 9), Facing::Down)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (5, 7), &Facing::Left),
            ((3, 9), Facing::Down)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (5, 8), &Facing::Left),
            ((4, 9), Facing::Down)
        );

        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (8, 5), &Facing::Right),
            ((9, 4), Facing::Up)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (8, 6), &Facing::Right),
            ((10, 4), Facing::Up)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (8, 7), &Facing::Right),
            ((11, 4), Facing::Up)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (8, 8), &Facing::Right),
            ((12, 4), Facing::Up)
        );
    }

    #[test]
    fn cube_wrapping_from_4() {
        const FACE_WIDTH: usize = 4;

        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (1, 9), &Facing::Up),
            ((5, 5), Facing::Right)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (2, 9), &Facing::Up),
            ((5, 6), Facing::Right)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (3, 9), &Facing::Up),
            ((5, 7), Facing::Right)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (4, 9), &Facing::Up),
            ((5, 8), Facing::Right)
        );

        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (1, 9), &Facing::Left),
            ((5, 4), Facing::Right)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (1, 10), &Facing::Left),
            ((5, 3), Facing::Right)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (1, 11), &Facing::Left),
            ((5, 2), Facing::Right)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (1, 12), &Facing::Left),
            ((5, 1), Facing::Right)
        );
    }

    #[test]
    fn cube_wrapping_from_5() {
        const FACE_WIDTH: usize = 4;

        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (8, 9), &Facing::Right),
            ((12, 4), Facing::Left)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (8, 10), &Facing::Right),
            ((12, 3), Facing::Left)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (8, 11), &Facing::Right),
            ((12, 2), Facing::Left)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (8, 12), &Facing::Right),
            ((12, 1), Facing::Left)
        );

        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (5, 12), &Facing::Down),
            ((4, 13), Facing::Left)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (6, 12), &Facing::Down),
            ((4, 14), Facing::Left)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (7, 12), &Facing::Down),
            ((4, 15), Facing::Left)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (8, 12), &Facing::Down),
            ((4, 16), Facing::Left)
        );
    }

    #[test]
    fn cube_wrapping_from_6() {
        const FACE_WIDTH: usize = 4;

        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (1, 13), &Facing::Left),
            ((5, 1), Facing::Down)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (1, 14), &Facing::Left),
            ((6, 1), Facing::Down)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (1, 15), &Facing::Left),
            ((7, 1), Facing::Down)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (1, 16), &Facing::Left),
            ((8, 1), Facing::Down)
        );

        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (1, 16), &Facing::Down),
            ((9, 1), Facing::Down)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (2, 16), &Facing::Down),
            ((10, 1), Facing::Down)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (3, 16), &Facing::Down),
            ((11, 1), Facing::Down)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (4, 16), &Facing::Down),
            ((12, 1), Facing::Down)
        );

        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (4, 13), &Facing::Right),
            ((5, 12), Facing::Up)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (4, 14), &Facing::Right),
            ((6, 12), Facing::Up)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (4, 15), &Facing::Right),
            ((7, 12), Facing::Up)
        );
        assert_eq!(
            find_cube_wrapped_location(FACE_WIDTH, (4, 16), &Facing::Right),
            ((8, 12), Facing::Up)
        );
    }
}
