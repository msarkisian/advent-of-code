#[aoc(day1, part1)]
pub fn part1(input: &str) -> i32 {
    let mut max_calories = 0;
    let mut current_sum = 0;

    for line in input.lines() {
        if line == "" {
            max_calories = std::cmp::max(max_calories, current_sum);
            current_sum = 0
        } else {
            current_sum += line.parse::<i32>().expect("Error, nonparsable in input")
        }
    }
    max_calories
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> i32 {
    let mut top_three = (0, 0, 0);

    let mut current_sum = 0;

    for line in input.lines() {
        if line == "" {
            if current_sum > top_three.0 {
                top_three.2 = top_three.1;
                top_three.1 = top_three.0;
                top_three.0 = current_sum;
            } else if current_sum > top_three.1 {
                top_three.2 = top_three.1;
                top_three.1 = current_sum;
            } else if current_sum > top_three.2 {
                top_three.2 = current_sum
            }
            current_sum = 0
        } else {
            current_sum += line.parse::<i32>().expect("Error, nonparsable in input")
        }
    }
    top_three.0 + top_three.1 + top_three.2
}
