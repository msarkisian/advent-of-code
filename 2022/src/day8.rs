use std::cmp;

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<Vec<i32>> {
    let mut grid = Vec::new();
    for line in input.lines() {
        let mut row = Vec::new();
        for tree in line.chars() {
            row.push(
                tree.to_digit(10)
                    .expect("all trees in the input will be a valid digit") as i32,
            )
        }
        grid.push(row)
    }
    grid
}

#[aoc(day8, part1)]
fn part1(input: &Vec<Vec<i32>>) -> usize {
    let mut seen = vec![vec![false; input.len()]; input[0].len()];
    // left
    for y in 0..input.len() {
        let mut tallest_seen = -1;
        for x in 0..input[y].len() {
            if input[y][x] > tallest_seen {
                seen[y][x] = true;
                tallest_seen = input[y][x]
            }
        }
    }

    // right
    for y in 0..input.len() {
        let mut tallest_seen = -1;
        for x in (0..input[y].len()).rev() {
            if input[y][x] > tallest_seen {
                seen[y][x] = true;
                tallest_seen = input[y][x]
            }
        }
    }

    // top
    for x in 0..input[0].len() {
        let mut tallest_seen = -1;
        for y in 0..input.len() {
            if input[y][x] > tallest_seen {
                seen[y][x] = true;
                tallest_seen = input[y][x]
            }
        }
    }

    // bottom
    for x in 0..input[0].len() {
        let mut tallest_seen = -1;
        for y in (0..input.len()).rev() {
            if input[y][x] > tallest_seen {
                seen[y][x] = true;
                tallest_seen = input[y][x]
            }
        }
    }
    seen.into_iter().flatten().filter(|x| *x).count()
}

#[aoc(day8, part2)]
fn part2(input: &Vec<Vec<i32>>) -> usize {
    let mut best = 0;

    for y in 0..input.len() {
        for x in 0..input[y].len() {
            let current_size = input[y][x];
            // up
            let mut up_count = 0;
            for y2 in (0..y).rev() {
                up_count += 1;
                if input[y2][x] >= current_size {
                    break;
                }
            }
            // down
            let mut down_count = 0;
            for y2 in y + 1..input.len() {
                down_count += 1;
                if input[y2][x] >= current_size {
                    break;
                }
            }
            // left
            let mut left_count = 0;
            for x2 in (0..x).rev() {
                left_count += 1;
                if input[y][x2] >= current_size {
                    break;
                }
            }
            // right
            let mut right_count = 0;
            for x2 in x + 1..input[0].len() {
                right_count += 1;
                if input[y][x2] >= current_size {
                    break;
                }
            }
            best = cmp::max(best, up_count * down_count * left_count * right_count)
        }
    }

    best
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn part1_works_on_example() {
        let example_input = "30373\n25512\n65332\n33549\n35390";
        let parsed_input = input_generator(example_input);
        assert_eq!(part1(&parsed_input), 21)
    }
    #[test]
    fn part2_works_on_example() {
        let example_input = "30373\n25512\n65332\n33549\n35390";
        let parsed_input = input_generator(example_input);
        assert_eq!(part2(&parsed_input), 8)
    }
}
