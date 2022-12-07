#![allow(dead_code)]
// I thought the file names might come up in part 2...

use std::{
    cell::{Cell, RefCell},
    cmp,
    collections::HashMap,
    rc::Rc,
};

struct File {
    pub name: String,
    pub size: usize,
}

struct Directory {
    pub name: String,
    pub parent: Option<Rc<Directory>>,
    pub files: RefCell<Vec<File>>,
    pub subdirectories: RefCell<HashMap<String, Rc<Directory>>>,
    pub recursive_size: Cell<usize>,
}

impl Directory {
    fn new(name: String, parent: Option<Rc<Directory>>) -> Self {
        Self {
            name,
            parent,
            files: RefCell::new(Vec::new()),
            subdirectories: RefCell::new(HashMap::new()),
            recursive_size: Cell::new(0),
        }
    }

    fn add_directory(current: &Rc<Directory>, name: &str) {
        current.subdirectories.borrow_mut().insert(
            name.to_string(),
            Rc::new(Directory::new(name.to_string(), Some(current.clone()))),
        );
    }

    fn add_file(current: &Rc<Directory>, name: &str, size: usize) {
        current.files.borrow_mut().push(File {
            name: name.to_string(),
            size,
        });

        current
            .recursive_size
            .set(current.recursive_size.get() + size);

        let mut this = current;
        while let Some(above) = &this.parent {
            above.recursive_size.set(above.recursive_size.get() + size);
            this = above;
        }
    }
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Rc<Directory> {
    // builds the tree and returns the root directory
    let root = Rc::new(Directory::new("/".to_string(), None));
    let mut current_directory = root.clone();
    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        match tokens.next().expect("every line has at least 1 token") {
            "$" => match tokens.next().expect("every command has a following token") {
                "cd" => match tokens
                    .next()
                    .expect("every cd has a directory associated with it")
                {
                    "/" => current_directory = root.clone(),
                    ".." => {
                        current_directory = current_directory
                            .parent
                            .as_ref()
                            .expect("all directories (but the root) have a parent")
                            .clone()
                    }
                    x => {
                        let next = current_directory.subdirectories.borrow_mut()[x].clone();
                        current_directory = next;
                    }
                },
                // feels lazy, but the way the input is written, we can just do nothing on the `ls` command
                // and assume that if we're seeing files and folders, they must've just done `ls`
                "ls" => (),
                _ => unreachable!(),
            },
            "dir" => {
                let directory_name = tokens
                    .next()
                    .expect("all dirs in input should be succeeded by a directory name");
                Directory::add_directory(&current_directory, directory_name);
            }
            file_size => {
                let file_size = file_size
                    .parse::<usize>()
                    .expect("all file sizes in the input are parsible");
                let file_name = tokens.next().expect("all files have a name");

                Directory::add_file(&current_directory, file_name, file_size);
            }
        }
    }
    root
}

#[aoc(day7, part1)]
fn part1(input: &Rc<Directory>) -> usize {
    // recursive size-getter
    fn get_size(input: &Rc<Directory>) -> usize {
        let mut output = 0;
        if input.recursive_size.get() <= 100000 {
            output += input.recursive_size.get();
        }

        for subdirectory in input.subdirectories.borrow().values() {
            output += get_size(subdirectory);
        }
        output
    }

    get_size(input)
}

#[aoc(day7, part2)]
fn part2(input: &Rc<Directory>) -> usize {
    let used_space = input.recursive_size.get();
    let unused_space = 70_000_000 - used_space;
    let needed_space = 30_000_000 - unused_space;

    fn get_smallest_subdirectory_above_target(input: &Rc<Directory>, target: usize) -> usize {
        let mut output = usize::MAX;
        if input.recursive_size.get() > target {
            output = input.recursive_size.get();
        }
        for subdirectory in input.subdirectories.borrow().values() {
            output = cmp::min(
                output,
                get_smallest_subdirectory_above_target(subdirectory, target),
            );
        }
        output
    }

    get_smallest_subdirectory_above_target(input, needed_space)
}
