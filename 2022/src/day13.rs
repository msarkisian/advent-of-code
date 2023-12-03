use std::cmp::Ordering;
#[derive(Debug, PartialEq, Eq, Clone)]
enum Data {
    Integer(i32),
    List(List),
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct List {
    data: Vec<Data>,
}

impl List {
    fn new() -> Self {
        Self { data: Vec::new() }
    }
}

impl Ord for List {
    fn cmp(&self, other: &Self) -> Ordering {
        let mut self_iter = self.data.iter();
        let mut other_iter = other.data.iter();

        while let Some(self_item) = self_iter.next() {
            let other_item = match other_iter.next() {
                Some(i) => i,
                None => return Ordering::Greater,
            };
            match (self_item, other_item) {
                (Data::Integer(s), Data::Integer(o)) => {
                    if s > o {
                        return Ordering::Greater;
                    } else if s < o {
                        return Ordering::Less;
                    }
                }
                (Data::Integer(s), Data::List(o)) => {
                    let self_list = List {
                        data: vec![Data::Integer(*s)],
                    };
                    match self_list.cmp(o) {
                        Ordering::Greater => {
                            return Ordering::Greater;
                        }
                        Ordering::Less => {
                            return Ordering::Less;
                        }
                        Ordering::Equal => (),
                    }
                }
                (Data::List(s), Data::Integer(o)) => {
                    let other_list = List {
                        data: vec![Data::Integer(*o)],
                    };
                    match s.cmp(&other_list) {
                        Ordering::Greater => {
                            return Ordering::Greater;
                        }
                        Ordering::Less => {
                            return Ordering::Less;
                        }
                        Ordering::Equal => (),
                    }
                }
                (Data::List(s), Data::List(o)) => match s.cmp(o) {
                    Ordering::Less => {
                        return Ordering::Less;
                    }
                    Ordering::Equal => (),
                    Ordering::Greater => return Ordering::Greater,
                },
            }
        }

        if other_iter.next() == None {
            // lists are the same length
            Ordering::Equal
        } else {
            // right list is greater length
            Ordering::Less
        }
    }
}

impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
fn parse_line_to_list(line: &str) -> List {
    let mut list_stack = Vec::new();
    let mut current_num = String::new();
    assert_eq!(line.chars().next(), Some('['));
    list_stack.push(List::new());
    assert_eq!(line.chars().last(), Some(']'));

    for token in line.chars().skip(1) {
        match token {
            '[' => list_stack.push(List::new()),
            ']' => {
                if current_num.len() > 0 {
                    let len = list_stack.len();
                    list_stack[len - 1]
                        .data
                        .push(Data::Integer(current_num.parse().unwrap()));
                    current_num = "".to_string();
                }
                let len = list_stack.len();
                if len == 1 {
                    // need to leave the last list on the stack so it isn't lost
                    break;
                }
                let last_list = list_stack.pop().unwrap();
                list_stack[len - 2].data.push(Data::List(last_list));
            }
            digit @ '0'..='9' => {
                current_num.push(digit);
            }
            ',' => {
                if current_num.len() == 0 {
                    // commas are also after closing braces
                    continue;
                }
                let len = list_stack.len();
                list_stack[len - 1]
                    .data
                    .push(Data::Integer(current_num.parse().unwrap()));
                current_num = "".to_string();
            }
            _ => panic!("unexpected character in input"),
        }
    }
    list_stack.pop().unwrap()
}

#[aoc_generator(day13, part1)]
fn input_generator_part1(input: &str) -> Vec<(List, List)> {
    let mut pairs = Vec::new();
    for pair in input.split("\n\n") {
        let mut lines = pair.lines();
        let pair1 = parse_line_to_list(lines.next().unwrap());
        let pair2 = parse_line_to_list(lines.next().unwrap());

        pairs.push((pair1, pair2));
    }
    pairs
}

#[aoc(day13, part1)]
fn part1(input: &[(List, List)]) -> usize {
    let mut count = 0;
    for ((first, second), i) in input.iter().zip(1..) {
        if first < second {
            count += i;
        }
    }
    count
}

#[aoc_generator(day13, part2)]
fn input_generator_part2(input: &str) -> Vec<List> {
    let mut lists = Vec::new();
    for line in input.lines().filter(|l| !l.is_empty()) {
        lists.push(parse_line_to_list(line));
    }
    lists
}

#[aoc(day13, part2)]
fn part2(input: &Vec<List>) -> usize {
    let decoder1: List = List {
        data: vec![Data::List(List {
            data: vec![Data::Integer(2)],
        })],
    };
    let decoder2: List = List {
        data: vec![Data::List(List {
            data: vec![Data::Integer(6)],
        })],
    };
    let mut list = input.clone();

    list.push(decoder1.clone());
    list.push(decoder2.clone());

    list.sort();
    let index1 = list.iter().position(|l| *l == decoder1).unwrap() + 1;
    let index2 = list.iter().position(|l| *l == decoder2).unwrap() + 1;

    index1 * index2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parsing_to_lists() {
        let input = "[9]\n[[8,7,6]]";
        assert_eq!(
            input_generator_part1(input),
            vec![(
                List {
                    data: vec![Data::Integer(9)]
                },
                List {
                    data: vec![Data::List(List {
                        data: vec![Data::Integer(8), Data::Integer(7), Data::Integer(6)]
                    })]
                }
            )]
        );

        let input = "[[[]]]\n[[]]";
        assert_eq!(
            input_generator_part1(input),
            vec![(
                List {
                    data: vec![Data::List(List {
                        data: vec![Data::List(List { data: vec![] })]
                    })]
                },
                List {
                    data: vec![Data::List(List { data: vec![] })]
                }
            )]
        )
    }

    #[test]
    fn comparing_lists() {
        let list1 = List {
            data: vec![
                Data::Integer(1),
                Data::Integer(1),
                Data::Integer(3),
                Data::Integer(1),
                Data::Integer(1),
            ],
        };
        let list2 = List {
            data: vec![
                Data::Integer(1),
                Data::Integer(1),
                Data::Integer(5),
                Data::Integer(1),
                Data::Integer(1),
            ],
        };
        assert!(list1 < list2);
        assert!(list2 > list1);

        let list1 = List {
            data: vec![
                Data::List(List {
                    data: vec![Data::Integer(1)],
                }),
                Data::List(List {
                    data: vec![Data::Integer(2), Data::Integer(3), Data::Integer(4)],
                }),
            ],
        };
        let list2 = List {
            data: vec![
                Data::List(List {
                    data: vec![Data::Integer(1)],
                }),
                Data::Integer(4),
            ],
        };
        assert!(list1 < list2);
        assert!(list2 > list1);

        let list1 = List {
            data: vec![
                Data::List(List {
                    data: vec![Data::Integer(4), Data::Integer(4)],
                }),
                Data::Integer(4),
                Data::Integer(4),
            ],
        };
        let list2 = List {
            data: vec![
                Data::List(List {
                    data: vec![Data::Integer(4), Data::Integer(4)],
                }),
                Data::Integer(4),
                Data::Integer(4),
                Data::Integer(4),
            ],
        };
        assert!(list1 < list2);
        assert!(list2 > list1);

        let list1 = List { data: vec![] };
        let list2 = List {
            data: vec![Data::Integer(3)],
        };
        assert!(list1 < list2);
        assert!(list2 > list1);
    }

    #[test]
    fn part1_example() {
        let input = "[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n[[4,4],4,4]\n[[4,4],4,4,4]\n\n[7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]";
        assert_eq!(part1(&input_generator_part1(input)), 13);
    }

    #[test]
    fn part2_example() {
        let input = "[1,1,3,1,1]\n[1,1,5,1,1]\n\n[[1],[2,3,4]]\n[[1],4]\n\n[9]\n[[8,7,6]]\n\n[[4,4],4,4]\n[[4,4],4,4,4]\n\n[7,7,7,7]\n[7,7,7]\n\n[]\n[3]\n\n[[[]]]\n[[]]\n\n[1,[2,[3,[4,[5,6,7]]]],8,9]\n[1,[2,[3,[4,[5,6,0]]]],8,9]";
        assert_eq!(part2(&input_generator_part2(input)), 140);
    }
}
