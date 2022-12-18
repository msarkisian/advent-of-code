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

#[aoc(day18, part2)]
fn part2(input: &HashSet<(isize, isize, isize)>) -> isize {
    let (min_x, min_y, min_z) = input.iter().fold(
        (isize::MAX, isize::MAX, isize::MAX),
        |(x1, y1, z1), (x2, y2, z2)| {
            (
                std::cmp::min(x1, *x2),
                std::cmp::min(y1, *y2),
                std::cmp::min(z1, *z2),
            )
        },
    );

    let (min_x, min_y, min_z) = (min_x - 1, min_y - 1, min_z - 1);

    let (max_x, max_y, max_z) = input.iter().fold(
        (isize::MIN, isize::MIN, isize::MIN),
        |(x1, y1, z1), (x2, y2, z2)| {
            (
                std::cmp::max(x1, *x2),
                std::cmp::max(y1, *y2),
                std::cmp::max(z1, *z2),
            )
        },
    );
    let (max_x, max_y, max_z) = (max_x + 1, max_y + 1, max_z + 1);

    let mut external_squares = HashSet::new();
    let mut flood_queue = vec![(min_x, min_y, min_z)];
    while let Some((x, y, z)) = flood_queue.pop() {
        external_squares.insert((x, y, z));
        if !input.contains(&(x - 1, y, z))
            && !external_squares.contains(&(x - 1, y, z))
            && x - 1 >= min_x
        {
            flood_queue.push((x - 1, y, z));
        }
        if !input.contains(&(x + 1, y, z))
            && !external_squares.contains(&(x + 1, y, z))
            && x + 1 <= max_x
        {
            flood_queue.push((x + 1, y, z));
        }
        if !input.contains(&(x, y - 1, z))
            && !external_squares.contains(&(x, y - 1, z))
            && y - 1 >= min_y
        {
            flood_queue.push((x, y - 1, z));
        }
        if !input.contains(&(x, y + 1, z))
            && !external_squares.contains(&(x, y + 1, z))
            && y + 1 <= max_y
        {
            flood_queue.push((x, y + 1, z));
        }
        if !input.contains(&(x, y, z - 1))
            && !external_squares.contains(&(x, y, z - 1))
            && z - 1 >= min_z
        {
            flood_queue.push((x, y, z - 1));
        }
        if !input.contains(&(x, y, z + 1))
            && !external_squares.contains(&(x, y, z + 1))
            && z + 1 <= max_z
        {
            flood_queue.push((x, y, z + 1));
        }
    }

    let mut surface_area = 0;

    for (x, y, z) in external_squares.into_iter() {
        if input.contains(&(x - 1, y, z)) {
            surface_area += 1
        }
        if input.contains(&(x + 1, y, z)) {
            surface_area += 1
        }
        if input.contains(&(x, y - 1, z)) {
            surface_area += 1
        }
        if input.contains(&(x, y + 1, z)) {
            surface_area += 1
        }
        if input.contains(&(x, y, z - 1)) {
            surface_area += 1
        }
        if input.contains(&(x, y, z + 1)) {
            surface_area += 1
        }
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

    #[test]
    fn part2_example() {
        assert_eq!(part2(&input_generator(INPUT)), 58);
    }
}
