use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
enum State {
    Rock,
    Sand,
}

#[aoc_generator(day14)]
fn input_generator(input: &str) -> HashMap<(u16, u16), State> {
    let mut map = HashMap::new();

    for line in input.lines() {
        let mut points = line.split(" -> ");
        let mut last = points.next().expect("all rows have at least 2 points");
        while let Some(next) = points.next() {
            let mut last_nums = last.split(',');
            let last_x = last_nums.next().unwrap().parse().unwrap();
            let last_y = last_nums.next().unwrap().parse().unwrap();

            let mut next_nums = next.split(',');
            let next_x = next_nums.next().unwrap().parse().unwrap();
            let next_y = next_nums.next().unwrap().parse().unwrap();

            if last_x == next_x {
                let range;
                if last_y > next_y {
                    range = next_y..=last_y;
                } else {
                    range = last_y..=next_y;
                }
                for y in range {
                    map.insert((last_x, y), State::Rock);
                }
            } else if last_y == next_y {
                let range;
                if last_x > next_x {
                    range = next_x..=last_x;
                } else {
                    range = last_x..=next_x;
                }
                for x in range {
                    map.insert((x, last_y), State::Rock);
                }
            } else {
                panic!("non horizontal/vertical line")
            }

            last = next;
        }
    }
    map
}

#[aoc(day14, part1)]
fn part1(input: &HashMap<(u16, u16), State>) -> usize {
    let mut map: HashMap<(u16, u16), State> = input.clone();
    const SAND_START_POINT: (u16, u16) = (500, 0);
    let mut sand_count = 0;
    'outer: loop {
        let (mut current_x, mut current_y) = SAND_START_POINT;
        loop {
            // arbitrary cutoff beyond the grid
            if current_y > 200 {
                break 'outer;
            } else if !map.contains_key(&(current_x, current_y + 1)) {
                current_y += 1;
                continue;
            } else if !map.contains_key(&(current_x - 1, current_y + 1)) {
                current_y += 1;
                current_x -= 1;
                continue;
            } else if !map.contains_key(&(current_x + 1, current_y + 1)) {
                current_y += 1;
                current_x += 1;
                continue;
            } else {
                map.insert((current_x, current_y), State::Sand);
                sand_count += 1;
                continue 'outer;
            }
        }
    }
    sand_count
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn input_parsing() {
        let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9\n";
        let map = input_generator(input);
        println!("{:?}", map);
        assert_eq!(map.len(), 20);

        for x in 494..=502 {
            assert_eq!(map.get(&(x, 9)), Some(&State::Rock));
        }
    }

    #[test]
    fn part1_test() {
        let input = "498,4 -> 498,6 -> 496,6\n503,4 -> 502,4 -> 502,9 -> 494,9\n";
        assert_eq!(part1(&input_generator(input)), 24);
    }
}
