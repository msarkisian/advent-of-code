struct Inventory {
    ore: usize,
    clay: usize,
    obsidian: usize,
    geodes: usize,
    robots: Vec<Robot>,
}

impl Inventory {
    fn gather_resources(&mut self) {
        for robot in self.robots.iter() {
            match robot {
                Robot::Ore => self.ore += 1,
                Robot::Clay => self.clay += 1,
                Robot::Obsidian => self.obsidian += 1,
                Robot::Geodes => self.geodes += 1,
            }
        }
    }

    fn buy_robot(&mut self, blueprint: &Blueprint, robot: &Robot) {
        match robot {
            Robot::Ore => {
                self.ore -= blueprint.ore_robot_cost;
                self.robots.push(Robot::Ore);
            }
            Robot::Clay => {
                self.ore -= blueprint.clay_robot_cost;
                self.robots.push(Robot::Clay);
            }
            Robot::Obsidian => {
                self.ore -= blueprint.obsidian_robot_cost.0;
                self.clay -= blueprint.obsidian_robot_cost.1;
                self.robots.push(Robot::Obsidian);
            }
            Robot::Geodes => {
                self.ore -= blueprint.geode_robot_cost.0;
                self.obsidian -= blueprint.geode_robot_cost.1;
                self.robots.push(Robot::Obsidian);
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

struct Blueprint {
    ore_robot_cost: usize,
    clay_robot_cost: usize,
    obsidian_robot_cost: (usize, usize),
    geode_robot_cost: (usize, usize),
}

enum Robot {
    Ore,
    Clay,
    Obsidian,
    Geodes,
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
