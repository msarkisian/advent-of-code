use std::{collections::HashMap, vec};

#[derive(Debug, Clone)]
struct Warehouse {
    pub stacks: Vec<CrateStack>,
}

impl Warehouse {
    pub fn move_crate(&mut self, from: usize, to: usize, count: usize) {
        for _ in 0..count {
            let c = self.stacks[from - 1].pop();
            self.stacks[to - 1].push(c);
        }
    }

    pub fn move_crates_together(&mut self, from: usize, to: usize, count: usize) {
        let mut crates = Vec::new();
        for _ in 0..count {
            crates.push(self.stacks[from - 1].pop());
        }
        for item in crates.into_iter().rev() {
            self.stacks[to - 1].push(item);
        }
    }
}
#[derive(Debug, Clone)]
struct CrateStack {
    crates: Vec<char>,
}

impl CrateStack {
    pub fn new(base: char) -> Self {
        Self { crates: vec![base] }
    }

    pub fn peek(&self) -> Option<&char> {
        self.crates.get(self.crates.len() - 1)
    }

    fn push(&mut self, item: char) {
        self.crates.push(item);
    }

    fn pop(&mut self) -> char {
        self.crates
            .pop()
            .expect("error, tried to pop an empty stack")
    }
}

#[derive(Debug)]
struct Instruction {
    pub from: usize,
    pub to: usize,
    pub quant: usize,
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> (Warehouse, Vec<Instruction>) {
    let (crate_lines, instruction_str) = input.split_once("\n\n").unwrap();

    let mut crate_map = HashMap::new();
    let mut crate_lines = crate_lines.lines().rev().skip(1);
    // build the cratemap out of the base, saving the horizontal indices for the higher levels
    crate_lines
        .next()
        .unwrap()
        .char_indices()
        .filter(|(_, c)| c.is_alphabetic())
        .for_each(|(i, c)| {
            crate_map.insert(i, CrateStack::new(c));
        });
    // and then add the lines above
    for line in crate_lines {
        line.char_indices()
            .filter(|(_, c)| c.is_alphabetic())
            .for_each(|(i, c)| crate_map.get_mut(&i).unwrap().push(c))
    }

    let mut instructions = Vec::new();
    for line in instruction_str.lines() {
        let mut nums = line
            .split_whitespace()
            .filter_map(|w| w.parse::<usize>().ok());
        let count = nums.next().unwrap();
        let from = nums.next().unwrap();
        let to = nums.next().unwrap();
        instructions.push(Instruction {
            from,
            to,
            quant: count,
        })
    }

    let mut crates = crate_map.drain().collect::<Vec<(_, _)>>();
    crates.sort_unstable_by_key(|(k, _)| *k);

    (
        Warehouse {
            stacks: crates.into_iter().map(|(_, v)| v).collect(),
        },
        instructions,
    )
}

#[aoc(day5, part1)]
fn part1(input: &(Warehouse, Vec<Instruction>)) -> String {
    let (warehouse, instructions) = input;
    let mut warehouse = warehouse.clone();

    for instruction in instructions {
        warehouse.move_crate(instruction.from, instruction.to, instruction.quant)
    }

    let mut output = String::new();
    for stack in &warehouse.stacks {
        output.push(stack.peek().unwrap().clone())
    }
    output
}

#[aoc(day5, part2)]
fn part2(input: &(Warehouse, Vec<Instruction>)) -> String {
    let (warehouse, instructions) = input;
    let mut warehouse = warehouse.clone();

    for instruction in instructions {
        warehouse.move_crates_together(instruction.from, instruction.to, instruction.quant)
    }

    let mut output = String::new();
    for stack in &warehouse.stacks {
        output.push(stack.peek().unwrap().clone())
    }
    output
}
