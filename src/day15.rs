use std::collections::HashSet;

struct SensorCircle {
    x: i32,
    y: i32,
    radius: u32,
    beacon: (i32, i32),
}

impl SensorCircle {
    fn contains(&self, (x, y): (i32, i32)) -> bool {
        x.abs_diff(self.x) + y.abs_diff(self.y) <= self.radius && (x, y) != self.beacon
    }

    fn contains_y_line(&self, y_line: i32) -> Vec<(i32, i32)> {
        let mut points = Vec::new();
        let distance_to_line = self.y.abs_diff(y_line);
        let line_rad = self.radius - distance_to_line;

        for x in self.x - line_rad as i32..=self.x + line_rad as i32 {
            if self.contains((x, y_line)) && (x, y_line) != self.beacon {
                points.push((x, y_line));
            }
        }
        points
    }
}

#[aoc(day15, part1)]
fn part1(input: &str) -> usize {
    const SEARCH_Y: i32 = 2000000;
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
            .parse::<i32>()
            .unwrap();
        let sensor_y = words
            .next()
            .unwrap()
            .strip_prefix("y=")
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .parse::<i32>()
            .unwrap();

        words.advance_by(4).unwrap();
        let beacon_x = words
            .next()
            .unwrap()
            .strip_prefix("x=")
            .unwrap()
            .strip_suffix(',')
            .unwrap()
            .parse::<i32>()
            .unwrap();
        let beacon_y = words
            .next()
            .unwrap()
            .strip_prefix("y=")
            .unwrap()
            .parse::<i32>()
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
