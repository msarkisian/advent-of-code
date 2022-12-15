use std::collections::HashSet;

struct SensorCircle {
    x: i64,
    y: i64,
    radius: u64,
    beacon: (i64, i64),
}

impl SensorCircle {
    fn contains(&self, (x, y): (i64, i64)) -> bool {
        x.abs_diff(self.x) + y.abs_diff(self.y) <= self.radius && (x, y) != self.beacon
    }

    fn contains_y_line(&self, y_line: i64) -> Vec<(i64, i64)> {
        let mut points = Vec::new();
        let distance_to_line = self.y.abs_diff(y_line);
        let line_rad = self.radius - distance_to_line;

        for x in self.x - line_rad as i64..=self.x + line_rad as i64 {
            if self.contains((x, y_line)) && (x, y_line) != self.beacon {
                points.push((x, y_line));
            }
        }
        points
    }

    fn get_bordering_points(&self) -> HashSet<(i64, i64)> {
        let mut points = HashSet::new();
        for y in self.y - self.radius as i64..=self.y + self.radius as i64 {
            // radius = 3
            // self.x = 5
            // self.y = 5
            // y = 2..=8
            points.insert((
                self.x + (self.radius as i64 - self.y.abs_diff(y) as i64 + 1),
                y,
            ));
            points.insert((
                self.x - (self.radius as i64 - self.y.abs_diff(y) as i64 + 1),
                y,
            ));
            // y = 2
            // x = 4, 6
            // x = self.x ± 1

            // y = 3
            // x = 3, 7
            // x = self.x ± 2
            // 2 = radius - |self.y - y| + 1
            // 2 = 3      - |5      - 3| + 1

            // y = 4
            // x = 2, 8
            // x = self.x ± 3
            // 3 = radius - |self.y - y| + 1
            // 3 = 3      - |5      - 4| + 1

            // y = 5
            // x = 1, 9
            // x = self.x ± 4
            // 4 = radius - |self.y - y| + 1
            // 4 = 3      - |5      - 5| + 1

            // y = 6
            // x = 2, 8
            // x = self.x ± 3

            // wanted: (4,2)(6,2) (3,3)(7,3) (2,4)(8,4)...
        }

        points.insert((self.x, self.y + self.radius as i64 + 1));
        points.insert((self.x, self.y - self.radius as i64 - 1));
        points
    }
}

#[aoc(day15, part1)]
fn part1(input: &str) -> usize {
    const SEARCH_Y: i64 = 2000000;
    let mut sensor_circles = Vec::new();
    for line in input.lines() {
        let mut words = line.split_whitespace().skip(2);
        let sensor_x = words
            .next()
            .unwrap()
            .strip_prefix("x=")
            .unwrap()
            .strip_suffix(',')
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let sensor_y = words
            .next()
            .unwrap()
            .strip_prefix("y=")
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .parse::<i64>()
            .unwrap();

        words.advance_by(4).unwrap();
        let beacon_x = words
            .next()
            .unwrap()
            .strip_prefix("x=")
            .unwrap()
            .strip_suffix(',')
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let beacon_y = words
            .next()
            .unwrap()
            .strip_prefix("y=")
            .unwrap()
            .parse::<i64>()
            .unwrap();

        let radius = sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y);
        sensor_circles.push(SensorCircle {
            x: sensor_x,
            y: sensor_y,
            radius,
            beacon: (beacon_x, beacon_y),
        });
    }
    let mut never_points_on_line = HashSet::new();
    for circle in sensor_circles {
        never_points_on_line.extend(circle.contains_y_line(SEARCH_Y).into_iter())
    }
    never_points_on_line.iter().count()
}

#[aoc(day15, part2)]
fn part2(input: &str) -> i64 {
    let mut sensor_circles = Vec::new();
    for line in input.lines() {
        let mut words = line.split_whitespace().skip(2);
        let sensor_x = words
            .next()
            .unwrap()
            .strip_prefix("x=")
            .unwrap()
            .strip_suffix(',')
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let sensor_y = words
            .next()
            .unwrap()
            .strip_prefix("y=")
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .parse::<i64>()
            .unwrap();

        words.advance_by(4).unwrap();
        let beacon_x = words
            .next()
            .unwrap()
            .strip_prefix("x=")
            .unwrap()
            .strip_suffix(',')
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let beacon_y = words
            .next()
            .unwrap()
            .strip_prefix("y=")
            .unwrap()
            .parse::<i64>()
            .unwrap();

        let radius = sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y);
        sensor_circles.push(SensorCircle {
            x: sensor_x,
            y: sensor_y,
            radius,
            beacon: (beacon_x, beacon_y),
        });
    }
    let mut possible_points = HashSet::new();
    for circle in sensor_circles.iter() {
        possible_points.extend(circle.get_bordering_points());
    }
    let (answer_x, answer_y) = possible_points
        .into_iter()
        .filter(|(x, y)| {
            *x >= 0 && *x <= 4000000 && *y >= 0 && *y <= 4000000 && {
                let mut uncontained = true;
                for circle in sensor_circles.iter() {
                    if circle.contains((*x, *y)) {
                        uncontained = false;
                        break;
                    }
                }
                uncontained
            }
        })
        .inspect(|point| println!("{:?}", point))
        .next()
        .unwrap();

    answer_x * 4000000 + answer_y
}

#[test]
fn boundary_points_test() {
    let circle = SensorCircle {
        x: 5,
        y: 5,
        radius: 3,
        beacon: (4, 3),
    };
    // 0123456789
    // 1.........
    // 2....#....
    // 3...B##...
    // 4..#####..
    // 5.###S###.
    // 6..#####..
    // 7...###...
    // 8....#....
    // 9.........

    let boundary_points = circle.get_bordering_points();
    println!("{:?}", boundary_points);
    assert_eq!(boundary_points.len(), 16)
}
