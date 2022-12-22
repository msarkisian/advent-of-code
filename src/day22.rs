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

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = include_str!("../input/2022/example/day22.txt");

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_generator(INPUT)), 6032)
    }
}
