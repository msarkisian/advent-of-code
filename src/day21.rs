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

#[derive(Debug, Clone, Copy)]
enum Operation {
    Addition,
    Subtraction,
    Multiplication,
    Division,
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
        }
        monkey.num.set(Some(res));
        res
    }
    resolve_monkey(&monkeys["root"])
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
        assert_eq!(part1(&INPUT), 152)
    }
}
