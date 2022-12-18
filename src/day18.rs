use std::collections::HashSet;

#[derive(Hash, PartialEq, Eq)]
struct Cube {
    x: usize,
    y: usize,
    z: usize,
}

#[aoc_generator(day18)]
fn input_generator(input: &str) -> HashSet<Cube> {
    let mut cubes = HashSet::new();
    for line in input.lines() {
        let mut nums = line.split(',');

        let x = nums.next().unwrap().parse().unwrap();
        let y = nums.next().unwrap().parse().unwrap();
        let z = nums.next().unwrap().parse().unwrap();

        cubes.insert(Cube { x, y, z });
    }

    cubes
}

#[aoc(day18, part1)]
fn part1(input: &HashSet<Cube>) -> usize {
    let mut surface_area = 0;
    for cube in input.iter() {
        let mut temp_area = 6;
        if input.contains(&Cube {
            x: cube.x - 1,
            y: cube.y,
            z: cube.z,
        }) {
            temp_area -= 1;
        }
        if input.contains(&Cube {
            x: cube.x + 1,
            y: cube.y,
            z: cube.z,
        }) {
            temp_area -= 1;
        }
        if input.contains(&Cube {
            x: cube.x,
            y: cube.y - 1,
            z: cube.z,
        }) {
            temp_area -= 1;
        }
        if input.contains(&Cube {
            x: cube.x,
            y: cube.y + 1,
            z: cube.z,
        }) {
            temp_area -= 1;
        }
        if input.contains(&Cube {
            x: cube.x,
            y: cube.y,
            z: cube.z - 1,
        }) {
            temp_area -= 1;
        }
        if input.contains(&Cube {
            x: cube.x,
            y: cube.y,
            z: cube.z + 1,
        }) {
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
