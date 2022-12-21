use std::{collections::LinkedList, rc::Rc};

#[aoc(day20, part1)]
fn part1(input: &str) -> isize {
    let mut list = LinkedList::new();
    for line in input.lines() {
        list.push_back(Rc::new(line.parse::<isize>().unwrap()))
    }
    let list_len = list.len();
    let mut original_order = Vec::with_capacity(list.len());
    for rc in list.iter() {
        original_order.push(rc.clone());
    }
    for item in original_order.iter() {
        let mut cursor = list.cursor_front_mut();
        while !Rc::ptr_eq(cursor.current().unwrap(), item) {
            cursor.move_next();
        }
        if *item.as_ref() == 0 {
            continue;
        }

        let steps_to_new = item.as_ref() % (list_len as isize - 1);

        let item = cursor.remove_current().unwrap();
        if steps_to_new > 0 {
            for _ in 0..steps_to_new {
                if cursor.current() == None {
                    cursor.move_next()
                }
                cursor.move_next();
                if cursor.current() == None {
                    cursor.move_next()
                }
            }
        } else if steps_to_new < 0 {
            for _ in 0..steps_to_new.abs() {
                cursor.move_prev();
                if cursor.current() == None {
                    cursor.move_prev()
                }
            }
        }
        if cursor.peek_prev() == None {
            cursor.move_prev()
        }
        cursor.insert_before(item);
    }
    let mut cursor = list.cursor_front();
    let mut zero_index = 0;
    while cursor.current().unwrap().as_ref() != &0 {
        cursor.move_next();
        zero_index += 1;
    }

    let first;
    let mut cursor = list.cursor_front();
    let first_steps = (1000 + zero_index) % list_len;
    for _ in 0..first_steps {
        cursor.move_next();
    }
    first = cursor.current().unwrap().as_ref();
    let second;
    let mut cursor = list.cursor_front();
    let second_steps = (2000 + zero_index) % list_len;
    for _ in 0..second_steps {
        cursor.move_next();
    }
    second = cursor.current().unwrap().as_ref();
    let third;
    let mut cursor = list.cursor_front();
    let third_steps = (3000 + zero_index) % list_len;
    for _ in 0..third_steps {
        cursor.move_next();
    }
    third = cursor.current().unwrap().as_ref();

    first + second + third
}

#[aoc(day20, part2)]
fn part2(input: &str) -> isize {
    const DECRYPTION_KEY: isize = 811589153;
    let mut list = LinkedList::new();
    for line in input.lines() {
        list.push_back(Rc::new(line.parse::<isize>().unwrap() * DECRYPTION_KEY))
    }
    let list_len = list.len();
    let mut original_order = Vec::with_capacity(list.len());
    for rc in list.iter() {
        original_order.push(rc.clone());
    }

    for item in std::iter::repeat(original_order.iter()).take(10).flatten() {
        let mut cursor = list.cursor_front_mut();
        while !Rc::ptr_eq(cursor.current().unwrap(), item) {
            cursor.move_next();
        }

        if *item.as_ref() == 0 {
            continue;
        }

        let steps_to_new = item.as_ref() % (list_len as isize - 1);

        let item = cursor.remove_current().unwrap();
        if steps_to_new > 0 {
            for _ in 0..steps_to_new {
                if cursor.current() == None {
                    cursor.move_next();
                }
                cursor.move_next();
            }
        } else if steps_to_new < 0 {
            for _ in 0..steps_to_new.abs() {
                cursor.move_prev();
                if cursor.current() == None {
                    cursor.move_prev()
                }
            }
        }
        if cursor.peek_prev() == None {
            cursor.move_prev()
        }
        cursor.insert_before(item);
    }
    let mut cursor = list.cursor_front();
    let mut zero_index = 0;
    while cursor.current().unwrap().as_ref() != &0 {
        cursor.move_next();
        zero_index += 1;
    }

    let first;
    let mut cursor = list.cursor_front();
    let first_steps = (1000 + zero_index) % list_len;
    for _ in 0..first_steps {
        cursor.move_next();
    }
    first = cursor.current().unwrap().as_ref();
    let second;
    let mut cursor = list.cursor_front();
    let second_steps = (2000 + zero_index) % list_len;
    for _ in 0..second_steps {
        cursor.move_next();
    }
    second = cursor.current().unwrap().as_ref();
    let third;
    let mut cursor = list.cursor_front();
    let third_steps = (3000 + zero_index) % list_len;
    for _ in 0..third_steps {
        cursor.move_next();
    }
    third = cursor.current().unwrap().as_ref();

    first + second + third
}

#[cfg(test)]
mod test {
    use super::*;
    const INPUT: &str = "1\n2\n-3\n3\n-2\n0\n4";

    #[test]
    fn part1_example() {
        assert_eq!(part1(INPUT), 3)
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(INPUT), 1623178306)
    }
}
