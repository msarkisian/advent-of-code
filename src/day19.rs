use std::collections::{BinaryHeap, HashSet};

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
struct Inventory {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
    robots: (usize, usize, usize, usize),
}

impl Inventory {
    fn new() -> Self {
        Self {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geodes: 0,
            robots: (1, 0, 0, 0),
        }
    }
    fn gather_resources(&mut self) {
        self.ore += self.robots.0;
        self.clay += self.robots.1;
        self.obsidian += self.robots.2;
        self.geodes += self.robots.3;
    }

    fn buy_robot(&mut self, blueprint: &Blueprint, robot: &Robot) {
        match robot {
            Robot::Ore => {
                self.ore -= blueprint.ore_robot_cost;
                self.robots.0 += 1;
            }
            Robot::Clay => {
                self.ore -= blueprint.clay_robot_cost;
                self.robots.1 += 1;
            }
            Robot::Obsidian => {
                self.ore -= blueprint.obsidian_robot_cost.0;
                self.clay -= blueprint.obsidian_robot_cost.1;
                self.robots.2 += 1;
            }
            Robot::Geodes => {
                self.ore -= blueprint.geode_robot_cost.0;
                self.obsidian -= blueprint.geode_robot_cost.1;
                self.robots.3 += 1
            }
        }
    }

    fn get_buyable_robots(&self, blueprint: &Blueprint) -> Vec<Robot> {
        let mut buyable_robots = Vec::new();
        if self.ore >= blueprint.ore_robot_cost {
            buyable_robots.push(Robot::Ore);
        }
        if self.ore >= blueprint.clay_robot_cost {
            buyable_robots.push(Robot::Clay);
        }
        if self.ore >= blueprint.obsidian_robot_cost.0
            && self.clay >= blueprint.obsidian_robot_cost.1
        {
            buyable_robots.push(Robot::Obsidian);
        }
        if self.ore >= blueprint.geode_robot_cost.0 && self.obsidian >= blueprint.geode_robot_cost.1
        {
            buyable_robots.push(Robot::Geodes);
        }
        buyable_robots
    }
}

impl Ord for Inventory {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.geodes.cmp(&other.geodes)
    }
}

impl PartialOrd for Inventory {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Blueprint {
    ore_robot_cost: usize,
    clay_robot_cost: usize,
    obsidian_robot_cost: (usize, usize),
    geode_robot_cost: (usize, usize),
}

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geodes,
}

#[derive(PartialEq, Eq, Debug, Hash)]
struct State {
    inventory: Inventory,
    elapsed_time: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        if self.inventory.geodes.eq(&other.inventory.geodes) {
            self.elapsed_time.cmp(&other.elapsed_time).reverse()
        } else {
            self.inventory.cmp(&other.inventory)
        }
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[aoc_generator(day19)]
fn input_generator(input: &str) -> Vec<Blueprint> {
    let mut blueprints = Vec::new();
    for line in input.lines() {
        let mut tokens = line.split_whitespace();
        tokens.advance_by(6).unwrap();
        let ore_robot_cost = tokens.next().unwrap().parse().unwrap();
        tokens.advance_by(5).unwrap();
        let clay_robot_cost = tokens.next().unwrap().parse().unwrap();
        tokens.advance_by(5).unwrap();
        let obsidian_robot_ore_cost = tokens.next().unwrap().parse().unwrap();
        tokens.advance_by(2).unwrap();
        let obsidian_robot_clay_cost = tokens.next().unwrap().parse().unwrap();
        tokens.advance_by(5).unwrap();
        let geode_robot_ore_cost = tokens.next().unwrap().parse().unwrap();
        tokens.advance_by(2).unwrap();
        let geode_robot_obsidian_cost = tokens.next().unwrap().parse().unwrap();

        blueprints.push(Blueprint {
            ore_robot_cost,
            clay_robot_cost,
            obsidian_robot_cost: (obsidian_robot_ore_cost, obsidian_robot_clay_cost),
            geode_robot_cost: (geode_robot_ore_cost, geode_robot_obsidian_cost),
        })
    }
    blueprints
}

#[aoc(day19, part1)]
fn part1(input: &Vec<Blueprint>) -> usize {
    let mut blueprint_max_geodes = Vec::with_capacity(input.len());
    for blueprint in input {
        let mut max_geodes = 0;
        let mut queue = BinaryHeap::new();
        let mut seen_state = HashSet::new();
        let mut max_geode_robots_at_time = vec![0usize; 25];
        queue.push(State {
            inventory: Inventory::new(),
            elapsed_time: 0,
        });
        // let mut search_depth: usize = 0;
        while let Some(state) = queue.pop() {
            // println!("{}", queue.len());
            // println!("{:?}", max_geode_robots_at_time);
            if seen_state.contains(&state)
                || (state.elapsed_time > 0
                    && state.inventory.robots.3 < max_geode_robots_at_time[state.elapsed_time - 1])
            {
                continue;
            }
            if state.elapsed_time == 24 {
                // if search_depth % 100 == 0 {
                //     println!("{}%", search_depth / 100);
                // }
                // if search_depth == 1000 {
                //     break;
                // }
                max_geodes = std::cmp::max(max_geodes, state.inventory.geodes);
                // search_depth += 1;
                continue;
            }
            let buyable_robots = state.inventory.get_buyable_robots(blueprint);
            for robot in buyable_robots {
                let mut next_inventory = state.inventory.clone();
                next_inventory.gather_resources();
                next_inventory.buy_robot(blueprint, &robot);
                queue.push(State {
                    inventory: next_inventory,
                    elapsed_time: state.elapsed_time + 1,
                })
            }
            let mut next_inventory = state.inventory.clone();
            next_inventory.gather_resources();
            queue.push(State {
                inventory: next_inventory,
                elapsed_time: state.elapsed_time + 1,
            });
            max_geode_robots_at_time[state.elapsed_time] =
                max_geode_robots_at_time[state.elapsed_time].max(state.inventory.robots.3);
            seen_state.insert(state);
        }
        blueprint_max_geodes.push(max_geodes);
    }
    println!("{:?}", blueprint_max_geodes);
    blueprint_max_geodes
        .iter()
        .zip(1..)
        .fold(0, |prev, (max_geodes, id)| prev + (max_geodes * id))
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.\nBlueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&input_generator(INPUT)), 33)
    }
}
