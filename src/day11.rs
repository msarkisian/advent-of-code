use std::cell::{Cell, RefCell};

const PRODUCT_OF_COMPARITORS: i128 = 11 * 2 * 5 * 7 * 17 * 19 * 3 * 13;

#[derive(Debug, Clone)]
struct Monkey {
    items: RefCell<Vec<i128>>,
    pub inspect_count: Cell<usize>,
    operation: fn(i128) -> i128,
    test: fn(i128) -> bool,
    pub if_true: usize,
    pub if_false: usize,
}

impl Monkey {
    pub fn new(
        starting_items: Vec<i128>,
        operation: fn(i128) -> i128,
        test: fn(i128) -> bool,
        if_true: usize,
        if_false: usize,
    ) -> Self {
        Monkey {
            items: RefCell::new(starting_items),
            inspect_count: Cell::new(0),
            operation,
            test,
            if_true,
            if_false,
        }
    }

    pub fn throw_items(&self, part2: bool) -> (Vec<i128>, Vec<i128>) {
        let mut true_items = Vec::new();
        let mut false_items = Vec::new();
        for item in self.items.borrow_mut().drain(..) {
            let mut item = (self.operation)(item);
            self.inspect_count.set(self.inspect_count.get() + 1);
            if !part2 {
                item = item / 3;
            } else {
                item = item % PRODUCT_OF_COMPARITORS;
            }

            if (self.test)(item) {
                true_items.push(item)
            } else {
                false_items.push(item)
            }
        }
        (true_items, false_items)
    }

    pub fn push_items(&self, new_items: Vec<i128>) {
        self.items.borrow_mut().extend(new_items)
    }
}

#[aoc_generator(day11)]
fn input_generator(_input: &str) -> Vec<Monkey> {
    let mut monkeys = Vec::with_capacity(8);

    monkeys.push(Monkey::new(vec![75, 63], |x| x * 3, |x| x % 11 == 0, 7, 2));

    monkeys.push(Monkey::new(
        vec![65, 79, 98, 77, 56, 54, 83, 94],
        |x| x + 3,
        |x| x % 2 == 0,
        2,
        0,
    ));

    monkeys.push(Monkey::new(vec![66], |x| x + 5, |x| x % 5 == 0, 7, 5));

    monkeys.push(Monkey::new(
        vec![51, 89, 90],
        |x| x * 19,
        |x| x % 7 == 0,
        6,
        4,
    ));

    monkeys.push(Monkey::new(
        vec![75, 94, 66, 90, 77, 82, 61],
        |x| x + 1,
        |x| x % 17 == 0,
        6,
        1,
    ));

    monkeys.push(Monkey::new(
        vec![53, 76, 59, 92, 95],
        |x| x + 2,
        |x| x % 19 == 0,
        4,
        3,
    ));

    monkeys.push(Monkey::new(
        vec![81, 61, 75, 89, 70, 92],
        |x| x * x,
        |x| x % 3 == 0,
        0,
        1,
    ));

    monkeys.push(Monkey::new(
        vec![81, 86, 62, 87],
        |x| x + 8,
        |x| x % 13 == 0,
        3,
        5,
    ));
    monkeys
}

#[aoc(day11, part1)]
fn part1(input: &Vec<Monkey>) -> usize {
    let monkeys = input.clone();
    for _ in 0..20 {
        for monkey in monkeys.iter() {
            let (true_items, false_items) = monkey.throw_items(false);
            let (if_true, if_false) = (monkey.if_true, monkey.if_false);
            monkeys[if_true].push_items(true_items);
            monkeys[if_false].push_items(false_items);
        }
    }

    let mut inspect_counts = monkeys
        .iter()
        .map(|m| m.inspect_count.get())
        .collect::<Vec<_>>();
    inspect_counts.sort();

    inspect_counts[inspect_counts.len() - 1] * inspect_counts[inspect_counts.len() - 2]
}
#[aoc(day11, part2)]
fn part2(input: &Vec<Monkey>) -> usize {
    let monkeys = input.clone();
    for _ in 0..10000 {
        for monkey in monkeys.iter() {
            let (true_items, false_items) = monkey.throw_items(true);
            let (if_true, if_false) = (monkey.if_true, monkey.if_false);
            monkeys[if_true].push_items(true_items);
            monkeys[if_false].push_items(false_items);
        }
    }

    let mut inspect_counts = monkeys
        .iter()
        .map(|m| m.inspect_count.get())
        .collect::<Vec<_>>();
    inspect_counts.sort();

    inspect_counts[inspect_counts.len() - 1] * inspect_counts[inspect_counts.len() - 2]
}
