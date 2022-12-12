use std::collections::BinaryHeap;

#[derive(PartialEq, Eq, Debug)]
pub struct State {
    pub steps: usize,
    pub position: (usize, usize),
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day12)]
fn input_generator(input: &str) -> Vec<Vec<u8>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for byte in line.as_bytes() {
            row.push(*byte);
        }
        grid.push(row);
    }
    grid
}

#[aoc(day12, part1)]
fn part1(input: &Vec<Vec<u8>>) -> usize {
    const START: u8 = b'S';
    const END: u8 = b'E';
    const HIGHEST_ELEVATION: u8 = b'z';
    let mut start_point = None;

    'search: for (y, row) in input.iter().enumerate() {
        for (x, byte) in row.iter().enumerate() {
            if *byte == START {
                start_point = Some((y, x));
                break 'search;
            }
        }
    }
    let start_point = start_point.unwrap();
    let mut heap = BinaryHeap::new();
    heap.push(State {
        position: start_point,
        steps: 0,
    });
    let mut input = input.clone();
    input[start_point.0][start_point.1] = b'a';
    let mut visited_points = vec![vec![false; input[0].len()]; input.len()];
    visited_points[start_point.0][start_point.1] = true;

    println!("{:?}", input);

    while let Some(state) = heap.pop() {
        let (current_y, current_x) = state.position;
        println!("{:?}", state);
        // up
        if current_y > 0
            && input[current_y - 1][current_x] == END
            && input[current_y][current_x] + 1 >= HIGHEST_ELEVATION
        {
            return state.steps + 1;
        }
        if current_y > 0
            && input[current_y - 1][current_x] <= input[current_y][current_x] + 1
            && !visited_points[current_y - 1][current_x]
        {
            heap.push(State {
                steps: state.steps + 1,
                position: (current_y - 1, current_x),
            });
            visited_points[current_y - 1][current_x] = true;
        };

        // down
        if current_y < input.len() - 1
            && input[current_y + 1][current_x] == END
            && input[current_y][current_x] + 1 >= HIGHEST_ELEVATION
        {
            return state.steps + 1;
        }
        if current_y < input.len() - 1
            && input[current_y + 1][current_x] <= input[current_y][current_x] + 1
            && !visited_points[current_y + 1][current_x]
        {
            heap.push(State {
                steps: state.steps + 1,
                position: (current_y + 1, current_x),
            });
            visited_points[current_y + 1][current_x] = true;
        };

        // left
        if current_x > 0
            && input[current_y][current_x - 1] == END
            && input[current_y][current_x] + 1 >= HIGHEST_ELEVATION
        {
            return state.steps + 1;
        }
        if current_x > 0
            && input[current_y][current_x - 1] <= input[current_y][current_x] + 1
            && !visited_points[current_y][current_x - 1]
        {
            heap.push(State {
                steps: state.steps + 1,
                position: (current_y, current_x - 1),
            });
            visited_points[current_y][current_x - 1] = true;
        };

        // right
        if current_x < input[0].len() - 1
            && input[current_y][current_x + 1] == END
            && input[current_y][current_x] + 1 >= HIGHEST_ELEVATION
        {
            return state.steps + 1;
        }
        if current_x < input[0].len() - 1
            && input[current_y][current_x + 1] <= input[current_y][current_x] + 1
            && !visited_points[current_y][current_x + 1]
        {
            heap.push(State {
                steps: state.steps + 1,
                position: (current_y, current_x + 1),
            });
            visited_points[current_y][current_x + 1] = true;
        };
    }
    0
}

#[aoc(day12, part2)]
fn part2(input: &Vec<Vec<u8>>) -> usize {
    const START: u8 = b'S';
    const END: u8 = b'E';
    const LOWEST_ELEVATION: u8 = b'a';
    const HIGHEST_ELEVATION: u8 = b'z';
    let mut start_points = Vec::new();

    for (y, row) in input.iter().enumerate() {
        for (x, byte) in row.iter().enumerate() {
            if *byte == START || *byte == LOWEST_ELEVATION {
                start_points.push((y, x));
            }
        }
    }
    let mut heap = start_points
        .into_iter()
        .map(|p| State {
            steps: 0,
            position: p,
        })
        .collect::<BinaryHeap<State>>();

    let mut input = input.clone();
    let mut visited_points = vec![vec![false; input[0].len()]; input.len()];
    for start_point in heap.iter() {
        input[start_point.position.0][start_point.position.1] = b'a';
        visited_points[start_point.position.0][start_point.position.1] = true;
    }

    println!("{:?}", input);

    while let Some(state) = heap.pop() {
        let (current_y, current_x) = state.position;
        println!("{:?}", state);
        // up
        if current_y > 0
            && input[current_y - 1][current_x] == END
            && input[current_y][current_x] + 1 >= HIGHEST_ELEVATION
        {
            return state.steps + 1;
        }
        if current_y > 0
            && input[current_y - 1][current_x] <= input[current_y][current_x] + 1
            && !visited_points[current_y - 1][current_x]
        {
            heap.push(State {
                steps: state.steps + 1,
                position: (current_y - 1, current_x),
            });
            visited_points[current_y - 1][current_x] = true;
        };

        // down
        if current_y < input.len() - 1
            && input[current_y + 1][current_x] == END
            && input[current_y][current_x] + 1 >= HIGHEST_ELEVATION
        {
            return state.steps + 1;
        }
        if current_y < input.len() - 1
            && input[current_y + 1][current_x] <= input[current_y][current_x] + 1
            && !visited_points[current_y + 1][current_x]
        {
            heap.push(State {
                steps: state.steps + 1,
                position: (current_y + 1, current_x),
            });
            visited_points[current_y + 1][current_x] = true;
        };

        // left
        if current_x > 0
            && input[current_y][current_x - 1] == END
            && input[current_y][current_x] + 1 >= HIGHEST_ELEVATION
        {
            return state.steps + 1;
        }
        if current_x > 0
            && input[current_y][current_x - 1] <= input[current_y][current_x] + 1
            && !visited_points[current_y][current_x - 1]
        {
            heap.push(State {
                steps: state.steps + 1,
                position: (current_y, current_x - 1),
            });
            visited_points[current_y][current_x - 1] = true;
        };

        // right
        if current_x < input[0].len() - 1
            && input[current_y][current_x + 1] == END
            && input[current_y][current_x] + 1 >= HIGHEST_ELEVATION
        {
            return state.steps + 1;
        }
        if current_x < input[0].len() - 1
            && input[current_y][current_x + 1] <= input[current_y][current_x] + 1
            && !visited_points[current_y][current_x + 1]
        {
            heap.push(State {
                steps: state.steps + 1,
                position: (current_y, current_x + 1),
            });
            visited_points[current_y][current_x + 1] = true;
        };
    }
    0
}
#[cfg(test)]
mod test {
    use super::{input_generator, part1};

    #[test]
    fn part1_example() {
        let input = "Sabqponm\nabcryxxl\naccszExk\nacctuvwj\nabdefghi";

        assert_eq!(part1(&input_generator(input)), 31)
    }
}
