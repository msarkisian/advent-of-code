use std::collections::{HashMap, HashSet};

#[aoc(day3, part1)]
fn part1(input: &str) -> usize {
    let mut total_priority = 0;

    let priority_mapping: HashMap<_, _> = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(l, r)| (r, l + 1))
        .collect();

    for line in input.lines() {
        let (first_compartment, second_compartment) = line.split_at(line.len() / 2);
        let seen_items: HashSet<char> = first_compartment.chars().collect();

        for item in second_compartment.chars() {
            if seen_items.contains(&item) {
                total_priority += priority_mapping[&item];
                break;
            }
        }
    }
    total_priority
}

#[aoc(day3, part2)]
fn part2(input: &str) -> usize {
    let mut priority_sum = 0;
    let priority_mapping: HashMap<_, _> = ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(l, r)| (r, l + 1))
        .collect();

    for [elf1, elf2, elf3] in input.lines().array_chunks() {
        let seen_items: HashSet<char> = elf1.chars().collect();
        let seen_twice: HashSet<char> = elf2.chars().filter(|c| seen_items.contains(c)).collect();

        for item in elf3.chars() {
            if seen_twice.contains(&item) {
                priority_sum += priority_mapping[&item];
                break;
            }
        }
    }
    priority_sum
}
