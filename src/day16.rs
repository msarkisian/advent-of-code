use std::{
    cell::{Cell, RefCell},
    collections::{BinaryHeap, HashMap},
    rc::Rc,
};

#[derive(Debug, PartialEq, Eq)]
struct Node {
    name: RefCell<String>,
    pressure: Cell<usize>,
    connections: RefCell<Vec<Rc<Node>>>,
    nonzero_node_distances: RefCell<HashMap<String, usize>>,
}

impl Default for Node {
    fn default() -> Self {
        Self {
            name: RefCell::new(String::new()),
            pressure: Cell::new(0),
            connections: RefCell::new(Vec::new()),
            nonzero_node_distances: RefCell::new(HashMap::new()),
        }
    }
}

#[derive(PartialEq, Eq)]
struct PopulateDistanceState {
    current: Rc<Node>,
    steps: usize,
}

impl Ord for PopulateDistanceState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.steps.cmp(&other.steps).reverse()
    }
}

impl PartialOrd for PopulateDistanceState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

const HEAD_NODE_NAME: &str = "AA";

#[aoc_generator(day16)]
fn input_generator(input: &str) -> HashMap<String, Rc<Node>> {
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
            .parse::<usize>()
            .unwrap();

        tokens.advance_by(4).unwrap();

        let mut connection_names = Vec::new();
        while let Some(conn) = tokens.next() {
            connection_names.push(conn.trim_end_matches(','))
        }

        let mut connections = Vec::new();
        for name in connection_names {
            let conn = nodes
                .entry(name.to_string())
                .or_insert(Rc::new(Node {
                    ..Default::default()
                }))
                .clone();
            connections.push(conn);
        }

        let this_node = nodes
            .entry(this_node_name.to_string())
            .or_insert(Rc::new(Node {
                ..Default::default()
            }))
            .clone();

        this_node.pressure.set(this_pressure);
        *this_node.name.borrow_mut() = this_node_name.to_string();
        for connection in connections {
            this_node.connections.borrow_mut().push(connection.clone())
        }
    }
    let mut nodes_with_nonzero_pressure = nodes
        .values()
        .filter_map(|n| {
            if n.pressure.get() > 0 {
                Some(n.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    fn populate_node_distances(node: &Rc<Node>, dest_nodes: &[Rc<Node>]) {
        'dest_node: for target in dest_nodes {
            let mut queue = BinaryHeap::new();
            queue.push(PopulateDistanceState {
                current: node.clone(),
                steps: 0,
            });
            while let Some(state) = queue.pop() {
                if state.current == *target {
                    node.nonzero_node_distances
                        .borrow_mut()
                        .insert(target.name.borrow().clone(), state.steps);
                    continue 'dest_node;
                }
                for conn in state.current.connections.borrow().iter() {
                    queue.push(PopulateDistanceState {
                        current: conn.clone(),
                        steps: state.steps + 1,
                    })
                }
            }
            unreachable!()
        }
    }

    for _ in 0..nodes_with_nonzero_pressure.len() {
        let node = nodes_with_nonzero_pressure.pop().unwrap();
        populate_node_distances(&node, &nodes_with_nonzero_pressure);
        nodes_with_nonzero_pressure.insert(0, node);
    }
    populate_node_distances(nodes.get("AA").unwrap(), &nodes_with_nonzero_pressure);
    nodes
}

#[aoc(day16, part1)]
fn part1(input: &HashMap<String, Rc<Node>>) -> usize {
    for node in input.values() {
        println!("{:?}: {:?}", node.name, node.nonzero_node_distances);
    }
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\nValve BB has flow rate=13; tunnels lead to valves CC, AA\nValve CC has flow rate=2; tunnels lead to valves DD, BB\nValve DD has flow rate=20; tunnels lead to valves CC, AA, EE\nValve EE has flow rate=3; tunnels lead to valves FF, DD\nValve FF has flow rate=0; tunnels lead to valves EE, GG\nValve GG has flow rate=0; tunnels lead to valves FF, HH\nValve HH has flow rate=22; tunnel leads to valve GG\nValve II has flow rate=0; tunnels lead to valves AA, JJ\nValve JJ has flow rate=21; tunnel leads to valve II";

        assert_eq!(part1(&(input_generator(input))), 1651);
    }
}
