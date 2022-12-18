use std::collections::HashSet;

#[aoc_generator(day18)]
fn input_generator(input: &str) -> HashSet<(isize, isize, isize)> {
    let mut cubes = HashSet::new();
    for line in input.lines() {
        let mut nums = line.split(',');

        let x = nums.next().unwrap().parse().unwrap();
        let y = nums.next().unwrap().parse().unwrap();
        let z = nums.next().unwrap().parse().unwrap();

        cubes.insert((x, y, z));
    }

    cubes
}

#[aoc(day18, part1)]
fn part1(input: &HashSet<(isize, isize, isize)>) -> isize {
    let mut surface_area = 0;
    for (x, y, z) in input.iter() {
        let mut temp_area = 6;
        if input.contains(&(x - 1, *y, *z)) {
            temp_area -= 1;
        }
        if input.contains(&(x + 1, *y, *z)) {
            temp_area -= 1;
        }
        if input.contains(&(*x, y - 1, *z)) {
            temp_area -= 1;
        }
        if input.contains(&(*x, y + 1, *z)) {
            temp_area -= 1;
        }
        if input.contains(&(*x, *y, z - 1)) {
            temp_area -= 1;
        }
        if input.contains(&(*x, *y, z + 1)) {
            temp_area -= 1;
        }
        surface_area += temp_area;
    }
    surface_area
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str =
        "2,2,2\n1,2,2\n3,2,2\n2,1,2\n2,3,2\n2,2,1\n2,2,3\n2,2,4\n2,2,6\n1,2,5\n3,2,5\n2,1,5\n2,3,5";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_generator(INPUT)), 64);
    }
}
