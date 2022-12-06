use std::collections::HashSet;

#[aoc(day6, part1)]
fn part1(input: &[u8]) -> usize {
    'windows: for (index, window) in input.windows(4).enumerate() {
        let mut items = HashSet::with_capacity(4);
        for item in window {
            if !items.insert(item) {
                continue 'windows;
            }
        }
        return index + 4;
    }
    unreachable!("4 consecutive unique characters not found in input")
}

#[aoc(day6, part2)]
fn part2(input: &[u8]) -> usize {
    'windows: for (index, window) in input.windows(14).enumerate() {
        let mut items = HashSet::with_capacity(14);
        for item in window {
            if !items.insert(item) {
                continue 'windows;
            }
        }
        return index + 14;
    }
    unreachable!("14 consecutive unique characters not found in input")
}
