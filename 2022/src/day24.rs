use std::collections::HashSet;
use std::collections::VecDeque;

use self::Dir::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Dir {
    North,
    East,
    South,
    West,
}

#[derive(Debug, Hash, PartialEq, Eq)]
struct State {
    position: (usize, usize),
    blizzards: Vec<((usize, usize), Dir)>,
    time: usize,
}

struct Part2State {
    position: (usize, usize),
    blizzards: Vec<((usize, usize), Dir)>,
    time: usize,
    reached_end: bool,
    reached_start: bool,
}

#[aoc(day24, part1)]
fn part1(input: &str) -> usize {
    let mut walls = HashSet::new();
    let mut blizzards = Vec::new();
    let row_count = input.lines().count();
    let column_count = input.lines().next().unwrap().chars().count();
    let mut start_point = None;
    let mut end_point = None;
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    walls.insert((x, y));
                }
                '>' => {
                    blizzards.push(((x, y), East));
                }
                'v' => {
                    blizzards.push(((x, y), South));
                }
                '<' => {
                    blizzards.push(((x, y), West));
                }
                '^' => {
                    blizzards.push(((x, y), North));
                }
                '.' => {
                    if y == 0 {
                        start_point = Some((x, y));
                    } else if y == row_count - 1 {
                        end_point = Some((x, y));
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    let start_point = start_point.unwrap();
    let end_point = end_point.unwrap();

    let initial_state = State {
        position: start_point,
        blizzards,
        time: 0,
    };

    let mut queue = VecDeque::new();
    queue.push_back(initial_state);

    let mut seen_states = HashSet::new();

    while let Some(state) = queue.pop_front() {
        if seen_states.contains(&(
            state.position,
            (state.time % ((row_count - 2) * (column_count - 2))),
        )) {
            continue;
        }
        if state.position == end_point {
            return state.time;
        }
        let mut blizzard_locations = HashSet::new();
        let blizzards = state
            .blizzards
            .clone()
            .into_iter()
            .map(|((x, y), dir)| match dir {
                North => {
                    if !walls.contains(&(x, y - 1)) {
                        ((x, y - 1), dir)
                    } else {
                        ((x, row_count - 2), dir)
                    }
                }
                South => {
                    if !walls.contains(&(x, y + 1)) {
                        ((x, y + 1), dir)
                    } else {
                        ((x, 1), dir)
                    }
                }
                East => {
                    if !walls.contains(&(x + 1, y)) {
                        ((x + 1, y), dir)
                    } else {
                        ((1, y), dir)
                    }
                }
                West => {
                    if !walls.contains(&(x - 1, y)) {
                        ((x - 1, y), dir)
                    } else {
                        ((column_count - 2, y), dir)
                    }
                }
            })
            .inspect(|((x, y), _)| {
                blizzard_locations.insert((*x, *y));
            })
            .collect::<Vec<_>>();
        // nonmoving
        if !blizzard_locations.contains(&state.position) {
            queue.push_back(State {
                blizzards: blizzards.clone(),
                position: state.position,
                time: state.time + 1,
            })
        }
        // north
        let north_square = (state.position.0, state.position.1.wrapping_sub(1));
        if !blizzard_locations.contains(&north_square)
            && !walls.contains(&north_square)
            && state.position != start_point
        {
            queue.push_back(State {
                position: north_square,
                blizzards: blizzards.clone(),
                time: state.time + 1,
            })
        }
        // south
        let south_square = (state.position.0, state.position.1 + 1);
        if !blizzard_locations.contains(&south_square) && !walls.contains(&south_square) {
            queue.push_back(State {
                position: south_square,
                blizzards: blizzards.clone(),
                time: state.time + 1,
            })
        }
        // east
        let east_square = (state.position.0 + 1, state.position.1);
        if !blizzard_locations.contains(&east_square) && !walls.contains(&east_square) {
            queue.push_back(State {
                position: east_square,
                blizzards: blizzards.clone(),
                time: state.time + 1,
            })
        }
        // west
        let west_square = (state.position.0 - 1, state.position.1);
        if !blizzard_locations.contains(&west_square) && !walls.contains(&west_square) {
            queue.push_back(State {
                position: west_square,
                blizzards: blizzards.clone(),
                time: state.time + 1,
            })
        }
        seen_states.insert((
            state.position,
            (state.time % ((row_count - 2) * (column_count - 2))),
        ));
    }
    unreachable!()
}

#[aoc(day24, part2)]
fn part2(input: &str) -> usize {
    let mut walls = HashSet::new();
    let mut blizzards = Vec::new();
    let row_count = input.lines().count();
    let column_count = input.lines().next().unwrap().chars().count();
    let mut start_point = None;
    let mut end_point = None;
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            match char {
                '#' => {
                    walls.insert((x, y));
                }
                '>' => {
                    blizzards.push(((x, y), East));
                }
                'v' => {
                    blizzards.push(((x, y), South));
                }
                '<' => {
                    blizzards.push(((x, y), West));
                }
                '^' => {
                    blizzards.push(((x, y), North));
                }
                '.' => {
                    if y == 0 {
                        start_point = Some((x, y));
                    } else if y == row_count - 1 {
                        end_point = Some((x, y));
                    }
                }
                _ => unreachable!(),
            }
        }
    }
    let start_point = start_point.unwrap();
    let end_point = end_point.unwrap();

    let initial_state = Part2State {
        position: start_point,
        blizzards,
        time: 0,
        reached_end: false,
        reached_start: false,
    };

    let mut queue = VecDeque::new();
    queue.push_back(initial_state);

    let mut seen_states = HashSet::new();

    while let Some(state) = queue.pop_front() {
        let mut reached_end = state.reached_end;
        let mut reached_start = state.reached_start;
        if seen_states.contains(&(
            state.position,
            (state.time % ((row_count - 2) * (column_count - 2))),
            reached_end,
            reached_start,
        )) {
            continue;
        }
        if state.position == end_point {
            if state.reached_start {
                return state.time;
            } else if !reached_end {
                reached_end = true;
                queue.clear();
                println!("reached end (first time)");
            }
        }
        if state.position == start_point && reached_end && !reached_start {
            reached_start = true;
            queue.clear();
            println!("reached start again");
        }
        let mut blizzard_locations = HashSet::new();
        let blizzards = state
            .blizzards
            .clone()
            .into_iter()
            .map(|((x, y), dir)| match dir {
                North => {
                    if !walls.contains(&(x, y - 1)) {
                        ((x, y - 1), dir)
                    } else {
                        ((x, row_count - 2), dir)
                    }
                }
                South => {
                    if !walls.contains(&(x, y + 1)) {
                        ((x, y + 1), dir)
                    } else {
                        ((x, 1), dir)
                    }
                }
                East => {
                    if !walls.contains(&(x + 1, y)) {
                        ((x + 1, y), dir)
                    } else {
                        ((1, y), dir)
                    }
                }
                West => {
                    if !walls.contains(&(x - 1, y)) {
                        ((x - 1, y), dir)
                    } else {
                        ((column_count - 2, y), dir)
                    }
                }
            })
            .inspect(|((x, y), _)| {
                blizzard_locations.insert((*x, *y));
            })
            .collect::<Vec<_>>();
        // nonmoving
        if !blizzard_locations.contains(&state.position) {
            queue.push_back(Part2State {
                blizzards: blizzards.clone(),
                position: state.position,
                time: state.time + 1,
                reached_end,
                reached_start,
            })
        }
        // north
        let north_square = (state.position.0, state.position.1.wrapping_sub(1));
        if !blizzard_locations.contains(&north_square)
            && !walls.contains(&north_square)
            && state.position != start_point
        {
            queue.push_back(Part2State {
                position: north_square,
                blizzards: blizzards.clone(),
                time: state.time + 1,
                reached_end,
                reached_start,
            })
        }
        // south
        let south_square = (state.position.0, state.position.1 + 1);
        if !blizzard_locations.contains(&south_square)
            && !walls.contains(&south_square)
            && state.position != end_point
        {
            queue.push_back(Part2State {
                position: south_square,
                blizzards: blizzards.clone(),
                time: state.time + 1,
                reached_end,
                reached_start,
            })
        }
        // east
        let east_square = (state.position.0 + 1, state.position.1);
        if !blizzard_locations.contains(&east_square) && !walls.contains(&east_square) {
            queue.push_back(Part2State {
                position: east_square,
                blizzards: blizzards.clone(),
                time: state.time + 1,
                reached_end,
                reached_start,
            })
        }
        // west
        let west_square = (state.position.0 - 1, state.position.1);
        if !blizzard_locations.contains(&west_square) && !walls.contains(&west_square) {
            queue.push_back(Part2State {
                position: west_square,
                blizzards: blizzards.clone(),
                time: state.time + 1,
                reached_end,
                reached_start,
            })
        }
        seen_states.insert((
            state.position,
            (state.time % ((row_count - 2) * (column_count - 2))),
            reached_end,
            reached_start,
        ));
    }
    unreachable!()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "#.######\n#>>.<^<#\n#.<..<<#\n#>v.><>#\n#<^v^^>#\n######.#";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 18)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 54)
    }
}
