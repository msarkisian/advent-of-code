use std::{
    cell::{Cell, RefCell},
    collections::HashMap,
    rc::Rc,
};

struct Node {
    pressure: Cell<isize>,
    connections: RefCell<Vec<Rc<Node>>>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            pressure: Cell::new(-1),
            connections: RefCell::new(Vec::new()),
        }
    }
}

#[aoc_generator(day16)]
fn input_generator(input: &str) -> Rc<Node> {
    const HEAD_NODE_NAME: &str = "AA";
    let mut nodes = HashMap::new();
    for line in input.lines() {
        let mut tokens = line.split_whitespace().skip(1);

        let this_node_name = tokens.next().unwrap();
        tokens.advance_by(2).unwrap();
        let this_pressure = tokens
            .next()
            .unwrap()
            .strip_prefix("rate=")
            .unwrap()
            .strip_suffix(';')
            .unwrap()
            .parse::<isize>()
            .unwrap();

        tokens.advance_by(4).unwrap();

        let mut connection_names = Vec::new();
        while let Some(conn) = tokens.next() {
            connection_names.push(conn.trim_end_matches(','))
        }

        let mut connections = Vec::new();
        for name in connection_names {
            let conn = nodes
                .entry(name)
                .or_insert(Rc::new(Node {
                    ..Default::default()
                }))
                .clone();
            connections.push(conn);
        }

        let this_node = nodes
            .entry(this_node_name)
            .or_insert(Rc::new(Node {
                ..Default::default()
            }))
            .clone();

        this_node.pressure.set(this_pressure);
        for connection in connections {
            this_node.connections.borrow_mut().push(connection.clone())
        }
    }
    nodes[HEAD_NODE_NAME].clone()
}
