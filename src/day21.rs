use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

#[derive(Debug)]
struct Monkey {
    num: Cell<Option<isize>>,
    operation: Cell<Option<Operation>>,
    dependants: RefCell<Vec<Rc<Monkey>>>,
    // dependees: RefCell<Vec<Rc<Monkey>>>,
}

impl Monkey {
    fn new() -> Self {
        Self {
            num: Cell::new(None),
            operation: Cell::new(None),
            dependants: RefCell::new(Vec::with_capacity(2)),
            // dependees: RefCell::new(Vec::new()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
    Equality,
    Human,
}
#[aoc(day21, part1)]
fn part1(input: &str) -> isize {
    let mut monkeys = HashMap::new();
    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        let monkey_name = tokens.next().unwrap().strip_suffix(':').unwrap();
        let monkey = monkeys
            .entry(monkey_name)
            .or_insert(Rc::new(Monkey::new()))
            .clone();

        let token = tokens.next().unwrap();
        if let Ok(n) = token.parse::<isize>() {
            monkey.num.set(Some(n));
            continue;
        }
        let dependant1 = monkeys
            .entry(token)
            .or_insert(Rc::new(Monkey::new()))
            .clone();
        let op = match tokens.next().unwrap() {
            "+" => Operation::Addition,
            "-" => Operation::Subtraction,
            "*" => Operation::Multiplication,
            "/" => Operation::Division,
            _ => unreachable!(),
        };
        let dependant2 = monkeys
            .entry(tokens.next().unwrap())
            .or_insert(Rc::new(Monkey::new()))
            .clone();
        monkey.operation.set(Some(op));
        // dependant1.dependees.borrow_mut().push(monkey.clone());
        // dependant2.dependees.borrow_mut().push(monkey.clone());

        monkey.dependants.borrow_mut().push(dependant1);
        monkey.dependants.borrow_mut().push(dependant2);
    }
    fn resolve_monkey(monkey: &Rc<Monkey>) -> isize {
        if monkey.num.get().is_some() {
            return monkey.num.get().unwrap();
        }
        let val1 = resolve_monkey(&monkey.dependants.borrow()[0]);
        let val2 = resolve_monkey(&monkey.dependants.borrow()[1]);

        let res;
        match monkey.operation.get().unwrap() {
            Operation::Addition => res = val1 + val2,
            Operation::Subtraction => res = val1 - val2,
            Operation::Multiplication => res = val1 * val2,
            Operation::Division => res = val1 / val2,
            Operation::Equality => unreachable!(),
            Operation::Human => unreachable!(),
        }
        monkey.num.set(Some(res));
        res
    }
    resolve_monkey(&monkeys["root"])
}

#[aoc(day21, part2)]
fn part2(input: &str) -> isize {
    let mut monkeys = HashMap::new();
    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        let monkey_name = tokens.next().unwrap().strip_suffix(':').unwrap();
        let monkey = monkeys
            .entry(monkey_name)
            .or_insert(Rc::new(Monkey::new()))
            .clone();

        let token = tokens.next().unwrap();
        if let Ok(n) = token.parse::<isize>() {
            monkey.num.set(Some(n));
            continue;
        }
        let dependant1 = monkeys
            .entry(token)
            .or_insert(Rc::new(Monkey::new()))
            .clone();
        let op = match tokens.next().unwrap() {
            "+" => Operation::Addition,
            "-" => Operation::Subtraction,
            "*" => Operation::Multiplication,
            "/" => Operation::Division,
            _ => unreachable!(),
        };
        let dependant2 = monkeys
            .entry(tokens.next().unwrap())
            .or_insert(Rc::new(Monkey::new()))
            .clone();
        monkey.operation.set(Some(op));
        // dependant1.dependees.borrow_mut().push(monkey.clone());
        // dependant2.dependees.borrow_mut().push(monkey.clone());

        monkey.dependants.borrow_mut().push(dependant1);
        monkey.dependants.borrow_mut().push(dependant2);
    }
    monkeys["root"].operation.set(Some(Operation::Equality));
    monkeys["humn"].operation.set(Some(Operation::Human));
    monkeys["humn"].num.set(None);

    fn resolve_monkey(monkey: &Rc<Monkey>) -> Option<isize> {
        if monkey.num.get().is_some() {
            return monkey.num.get();
        }
        if monkey.operation.get() == Some(Operation::Human) {
            return None;
        }
        let mut val1 = resolve_monkey(&monkey.dependants.borrow()[0]);
        let mut val2 = resolve_monkey(&monkey.dependants.borrow()[1]);

        if monkey.operation.get() == Some(Operation::Equality) {
            if val1.is_none() {
                val1 = Some(bubble_needed_value(
                    &monkey.dependants.borrow()[0],
                    val2.unwrap(),
                ));
            } else {
                val2 = Some(bubble_needed_value(
                    &monkey.dependants.borrow()[1],
                    val1.unwrap(),
                ));
            }
            assert_eq!(val1, val2);
            return val1;
        }

        if val1.is_some() && val2.is_some() {
            let res;
            match monkey.operation.get().unwrap() {
                Operation::Addition => res = val1.unwrap() + val2.unwrap(),
                Operation::Subtraction => res = val1.unwrap() - val2.unwrap(),
                Operation::Multiplication => res = val1.unwrap() * val2.unwrap(),
                Operation::Division => res = val1.unwrap() / val2.unwrap(),
                Operation::Equality => unreachable!(),
                Operation::Human => unreachable!(),
            }
            monkey.num.set(Some(res));
            return Some(res);
        }
        monkey.num.get()
    }

    fn bubble_needed_value(monkey: &Rc<Monkey>, needed: isize) -> isize {
        if monkey.operation.get() == Some(Operation::Human) {
            monkey.num.set(Some(needed));
            return needed;
        }
        let val1 = resolve_monkey(&monkey.dependants.borrow()[0]);
        let val2 = resolve_monkey(&monkey.dependants.borrow()[1]);

        match monkey.operation.get().unwrap() {
            Operation::Addition => {
                if val1.is_none() {
                    bubble_needed_value(&monkey.dependants.borrow()[0], needed - val2.unwrap());
                } else if val2.is_none() {
                    bubble_needed_value(&monkey.dependants.borrow()[1], needed - val1.unwrap());
                } else {
                    panic!()
                }
            }
            Operation::Subtraction => {
                if val1.is_none() {
                    bubble_needed_value(&monkey.dependants.borrow()[0], needed + val2.unwrap());
                } else if val2.is_none() {
                    bubble_needed_value(&monkey.dependants.borrow()[1], val1.unwrap() - needed);
                } else {
                    panic!()
                }
            }
            Operation::Multiplication => {
                if val1.is_none() {
                    bubble_needed_value(&monkey.dependants.borrow()[0], needed / val2.unwrap());
                } else if val2.is_none() {
                    bubble_needed_value(&monkey.dependants.borrow()[1], needed / val1.unwrap());
                } else {
                    panic!()
                }
            }
            Operation::Division => {
                if val1.is_none() {
                    bubble_needed_value(&monkey.dependants.borrow()[0], needed * val2.unwrap());
                } else if val2.is_none() {
                    bubble_needed_value(&monkey.dependants.borrow()[1], val1.unwrap() / needed);
                } else {
                    panic!()
                }
            }
            Operation::Equality => unreachable!(),
            Operation::Human => unreachable!(),
        }
        resolve_monkey(monkey);
        monkey.num.get().unwrap()
    }
    resolve_monkey(&monkeys["root"]);
    monkeys["humn"].num.get().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "\
root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&INPUT), 152);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 301);
    }
}
