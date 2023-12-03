use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}
fn move_head_and_tail(
    head: &mut (i32, i32),
    tail: &mut (i32, i32),
    visited: &mut HashSet<(i32, i32)>,
    direction: Direction,
    count: u16,
) {
    for _ in 0..count {
        match direction {
            Direction::Up => (head.0, head.1) = (head.0, head.1 + 1),
            Direction::Down => (head.0, head.1) = (head.0, head.1 - 1),
            Direction::Left => (head.0, head.1) = (head.0 - 1, head.1),
            Direction::Right => (head.0, head.1) = (head.0 + 1, head.1),
        }
        if head.0.abs_diff(tail.0) <= 1 && head.1.abs_diff(tail.1) <= 1 {
            // tail doesn't move
            continue;
        } else if tail.0 == head.0 && tail.1 == head.1 - 2 {
            // tail moves up
            (tail.0, tail.1) = (tail.0, tail.1 + 1)
        } else if tail.0 == head.0 && tail.1 == head.1 + 2 {
            // tail moves down
            (tail.0, tail.1) = (tail.0, tail.1 - 1)
        } else if tail.1 == head.1 && tail.0 == head.0 + 2 {
            // tail moves left
            (tail.0, tail.1) = (tail.0 - 1, tail.1)
        } else if tail.1 == head.1 && tail.0 == head.0 - 2 {
            // tail moves right
            (tail.0, tail.1) = (tail.0 + 1, tail.1)
        } else if head.0 > tail.0 && head.1 > tail.1 {
            // tail moves up right
            (tail.0, tail.1) = (tail.0 + 1, tail.1 + 1)
        } else if head.0 < tail.0 && head.1 > tail.1 {
            // tail moves up left
            (tail.0, tail.1) = (tail.0 - 1, tail.1 + 1)
        } else if head.0 < tail.0 && head.1 < tail.1 {
            // tail moves down left
            (tail.0, tail.1) = (tail.0 - 1, tail.1 - 1)
        } else if head.0 > tail.0 && head.1 < tail.1 {
            // tail moves down right
            (tail.0, tail.1) = (tail.0 + 1, tail.1 - 1)
        } else {
            unreachable!()
        }

        visited.insert((tail.0, tail.1));
    }
}

#[aoc(day9, part1)]
fn part1(input: &str) -> usize {
    let mut head = (0, 0);
    let mut tail = (0, 0);

    let mut tail_visited = HashSet::new();
    tail_visited.insert((0, 0));

    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        let dir = match tokens.next().unwrap() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "R" => Direction::Right,
            "L" => Direction::Left,
            _ => panic!("unexpected character in input"),
        };
        let count = tokens.next().unwrap().parse().unwrap();

        move_head_and_tail(&mut head, &mut tail, &mut tail_visited, dir, count);
    }
    tail_visited.len()
}

#[aoc(day9, part2)]
fn part2(input: &str) -> usize {
    let mut knots = [(0, 0); 10];
    let mut tail_visited = HashSet::new();
    tail_visited.insert((0, 0));

    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        let dir = match tokens.next().unwrap() {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "R" => Direction::Right,
            "L" => Direction::Left,
            _ => panic!("unexpected character in input"),
        };
        let count = tokens.next().unwrap().parse().unwrap();
        for _ in 0..count {
            match dir {
                Direction::Up => (knots[0].0, knots[0].1) = (knots[0].0, knots[0].1 + 1),
                Direction::Down => (knots[0].0, knots[0].1) = (knots[0].0, knots[0].1 - 1),
                Direction::Left => (knots[0].0, knots[0].1) = (knots[0].0 - 1, knots[0].1),
                Direction::Right => (knots[0].0, knots[0].1) = (knots[0].0 + 1, knots[0].1),
            }
            for knot_index in 1..knots.len() {
                move_tails(&mut knots, knot_index);
            }
            tail_visited.insert(knots[knots.len() - 1]);
        }
    }
    tail_visited.len()
}
fn move_tails(knots: &mut [(i32, i32)], index: usize) {
    let (ahead, behind) = (knots[index - 1], &mut knots[index]);
    if ahead.0.abs_diff(behind.0) <= 1 && ahead.1.abs_diff(behind.1) <= 1 {
        // tail doesn't move
        return;
    } else if behind.0 == ahead.0 && behind.1 == ahead.1 - 2 {
        // tail moves up
        (behind.0, behind.1) = (behind.0, behind.1 + 1)
    } else if behind.0 == ahead.0 && behind.1 == ahead.1 + 2 {
        // tail moves down
        (behind.0, behind.1) = (behind.0, behind.1 - 1)
    } else if behind.1 == ahead.1 && behind.0 == ahead.0 + 2 {
        // tail moves left
        (behind.0, behind.1) = (behind.0 - 1, behind.1)
    } else if behind.1 == ahead.1 && behind.0 == ahead.0 - 2 {
        // tail moves right
        (behind.0, behind.1) = (behind.0 + 1, behind.1)
    } else if ahead.0 > behind.0 && ahead.1 > behind.1 {
        // tail moves up right
        (behind.0, behind.1) = (behind.0 + 1, behind.1 + 1)
    } else if ahead.0 < behind.0 && ahead.1 > behind.1 {
        // tail moves up left
        (behind.0, behind.1) = (behind.0 - 1, behind.1 + 1)
    } else if ahead.0 < behind.0 && ahead.1 < behind.1 {
        // tail moves down left
        (behind.0, behind.1) = (behind.0 - 1, behind.1 - 1)
    } else if ahead.0 > behind.0 && ahead.1 < behind.1 {
        // tail moves down right
        (behind.0, behind.1) = (behind.0 + 1, behind.1 - 1)
    } else {
        unreachable!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn p1_example_case() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn p2_example_case1() {
        let input = "R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2";
        assert_eq!(part2(input), 1);
    }
    #[test]
    fn p2_example_case2() {
        let input = "R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20";
        assert_eq!(part2(input), 36);
    }
}
