use itertools::Itertools;
use std::{
    cell::{Cell, RefCell},
    collections::{BTreeSet, BinaryHeap, HashMap, HashSet, VecDeque},
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

struct State {
    current: Rc<Node>,
    remaining_time: isize,
    pressure: usize,
    closed_valves: HashSet<String>,
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
    populate_node_distances(
        nodes.get(HEAD_NODE_NAME).unwrap(),
        &nodes_with_nonzero_pressure,
    );
    nodes
}

#[aoc(day16, part1)]
fn part1(input: &HashMap<String, Rc<Node>>) -> usize {
    // for v in input.values() {
    //     println!(
    //         "{}: ({}) {:?}",
    //         v.name.borrow(),
    //         v.pressure.get(),
    //         v.nonzero_node_distances.borrow()
    //     );
    // }
    let mut max_pressure = 0;
    let mut queue = VecDeque::new();
    let pressure_valves = input
        .values()
        .filter_map(|n| {
            if n.pressure.get() > 0 {
                Some(n.name.borrow().clone())
            } else {
                None
            }
        })
        .collect::<HashSet<_>>();
    queue.push_back(State {
        current: input.get(HEAD_NODE_NAME).unwrap().clone(),
        pressure: 0,
        remaining_time: 30,
        closed_valves: pressure_valves.clone(),
    });

    while let Some(state) = queue.pop_front() {
        max_pressure = std::cmp::max(max_pressure, state.pressure);
        if state.remaining_time < 0 || state.closed_valves.len() == 0 {
            continue;
        }
        for dest in state.closed_valves.iter() {
            let dist_to_dest = *state
                .current
                .nonzero_node_distances
                .borrow()
                .get(dest)
                .unwrap() as isize;
            if dist_to_dest >= state.remaining_time {
                continue;
            }

            queue.push_back(State {
                current: input.get(dest).unwrap().clone(),
                remaining_time: state.remaining_time - dist_to_dest - 1,
                pressure: (state.pressure
                    + (state.remaining_time - dist_to_dest - 1) as usize
                        * input.get(dest).unwrap().pressure.get()),
                closed_valves: state
                    .closed_valves
                    .clone()
                    .into_iter()
                    .filter(|v| v != dest)
                    .collect(),
            })
        }
    }
    max_pressure
}

#[aoc(day16, part2)]
fn part2(input: &HashMap<String, Rc<Node>>) -> usize {
    let mut max_pressure = 0;
    let mut queue = VecDeque::new();
    let pressure_valves = input
        .values()
        .filter_map(|n| {
            if n.pressure.get() > 0 {
                Some(n.name.borrow().clone())
            } else {
                None
            }
        })
        .collect::<BTreeSet<_>>();
    for i in 0..pressure_valves.len() {
        println!("{}", i);
        let mut human_valve_iter = pressure_valves.iter().combinations(i);
        while let Some(human_valves) = human_valve_iter.next() {
            let human_valves = HashSet::from_iter(human_valves.into_iter().map(|s| s.clone()));
            let mut human_max_pressure = 0;
            let mut elephant_max_pressure = 0;

            let elephant_valves = pressure_valves
                .iter()
                .map(|s| s.clone())
                .filter(|n| !human_valves.contains(n))
                .collect::<HashSet<_>>();

            // println!("human valves: {:?}", human_valves);
            // println!("elephant valves: {:?}", elephant_valves);

            assert_eq!(
                human_valves.len() + elephant_valves.len(),
                pressure_valves.len()
            );
            assert_eq!(human_valves.intersection(&elephant_valves).count(), 0);

            // run human
            queue.push_back(State {
                current: input.get(HEAD_NODE_NAME).unwrap().clone(),
                pressure: 0,
                remaining_time: 26,
                closed_valves: human_valves.clone(),
            });
            while let Some(state) = queue.pop_front() {
                human_max_pressure = std::cmp::max(human_max_pressure, state.pressure);
                if state.remaining_time < 0 || state.closed_valves.len() == 0 {
                    continue;
                }
                for dest in state.closed_valves.iter() {
                    let dist_to_dest = *state
                        .current
                        .nonzero_node_distances
                        .borrow()
                        .get(dest)
                        .unwrap() as isize;
                    if dist_to_dest >= state.remaining_time {
                        continue;
                    }

                    queue.push_back(State {
                        current: input.get(dest).unwrap().clone(),
                        remaining_time: state.remaining_time - dist_to_dest - 1,
                        pressure: (state.pressure
                            + (state.remaining_time - dist_to_dest - 1) as usize
                                * input.get(dest).unwrap().pressure.get()),
                        closed_valves: state
                            .closed_valves
                            .clone()
                            .into_iter()
                            .filter(|v| v != dest)
                            .collect(),
                    })
                }
            }
            // run elephant
            queue.push_back(State {
                current: input.get(HEAD_NODE_NAME).unwrap().clone(),
                pressure: 0,
                remaining_time: 26,
                closed_valves: elephant_valves.clone(),
            });
            while let Some(state) = queue.pop_front() {
                elephant_max_pressure = std::cmp::max(elephant_max_pressure, state.pressure);
                if state.remaining_time < 0 || state.closed_valves.len() == 0 {
                    continue;
                }
                for dest in state.closed_valves.iter() {
                    let dist_to_dest = *state
                        .current
                        .nonzero_node_distances
                        .borrow()
                        .get(dest)
                        .unwrap() as isize;
                    if dist_to_dest >= state.remaining_time {
                        continue;
                    }

                    queue.push_back(State {
                        current: input.get(dest).unwrap().clone(),
                        remaining_time: state.remaining_time - dist_to_dest - 1,
                        pressure: (state.pressure
                            + (state.remaining_time - dist_to_dest - 1) as usize
                                * input.get(dest).unwrap().pressure.get()),
                        closed_valves: state
                            .closed_valves
                            .clone()
                            .into_iter()
                            .filter(|v| v != dest)
                            .collect(),
                    })
                }
            }
            // println!("{}, {}", human_max_pressure, elephant_max_pressure);
            max_pressure = std::cmp::max(max_pressure, human_max_pressure + elephant_max_pressure);
        }
    }
    max_pressure
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_example() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\nValve BB has flow rate=13; tunnels lead to valves CC, AA\nValve CC has flow rate=2; tunnels lead to valves DD, BB\nValve DD has flow rate=20; tunnels lead to valves CC, AA, EE\nValve EE has flow rate=3; tunnels lead to valves FF, DD\nValve FF has flow rate=0; tunnels lead to valves EE, GG\nValve GG has flow rate=0; tunnels lead to valves FF, HH\nValve HH has flow rate=22; tunnel leads to valve GG\nValve II has flow rate=0; tunnels lead to valves AA, JJ\nValve JJ has flow rate=21; tunnel leads to valve II";
        assert_eq!(part1(&(input_generator(input))), 1651);
    }

    #[test]
    fn part2_example() {
        let input = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB\nValve BB has flow rate=13; tunnels lead to valves CC, AA\nValve CC has flow rate=2; tunnels lead to valves DD, BB\nValve DD has flow rate=20; tunnels lead to valves CC, AA, EE\nValve EE has flow rate=3; tunnels lead to valves FF, DD\nValve FF has flow rate=0; tunnels lead to valves EE, GG\nValve GG has flow rate=0; tunnels lead to valves FF, HH\nValve HH has flow rate=22; tunnel leads to valve GG\nValve II has flow rate=0; tunnels lead to valves AA, JJ\nValve JJ has flow rate=21; tunnel leads to valve II";
        assert_eq!(part2(&(input_generator(input))), 1707);
    }
}
