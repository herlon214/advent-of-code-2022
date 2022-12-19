#[macro_use]
extern crate lazy_static;
use std::collections::HashMap;

use regex::Regex;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Resources {
    ore: usize,
    clay: usize,
    obsidian: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Blueprint {
    id: usize,
    ore_robot_cost: Resources,
    clay_robot_cost: Resources,
    obsidian_robot_cost: Resources,
    geode_robot_cost: Resources,
}

impl From<&str> for Blueprint {
    fn from(input: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"Blueprint ([0-9]+): Each ore robot costs ([0-9]+) ore\. Each clay robot costs ([0-9]+) ore\. Each obsidian robot costs ([0-9]+) ore and ([0-9]+) clay\. Each geode robot costs ([0-9]+) ore and ([0-9]+) obsidian\.").unwrap();
        }

        let captures = RE.captures(input).unwrap();

        Blueprint {
            id: captures.get(1).unwrap().as_str().parse().unwrap(),
            ore_robot_cost: Resources {
                ore: captures.get(2).unwrap().as_str().parse().unwrap(),
                clay: 0,
                obsidian: 0,
            },
            clay_robot_cost: Resources {
                ore: captures.get(3).unwrap().as_str().parse().unwrap(),
                clay: 0,
                obsidian: 0,
            },
            obsidian_robot_cost: Resources {
                ore: captures.get(4).unwrap().as_str().parse().unwrap(),
                clay: captures.get(5).unwrap().as_str().parse().unwrap(),
                obsidian: 0,
            },
            geode_robot_cost: Resources {
                ore: captures.get(6).unwrap().as_str().parse().unwrap(),
                clay: 0,
                obsidian: captures.get(7).unwrap().as_str().parse().unwrap(),
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct State {
    time: usize,
    ore: usize,
    clay: usize,
    obsidian: usize,
    geode: usize,
    ore_rate: usize,
    clay_rate: usize,
    obsidian_rate: usize,
    geode_rate: usize,
}

impl State {
    fn can_build(&self, cost: &Resources) -> bool {
        self.ore >= cost.ore && self.clay >= cost.clay && self.obsidian >= cost.obsidian
    }

    fn collect_resources(&mut self) {
        // Collect ore
        if self.ore_rate > 0 {
            self.ore += self.ore_rate;
        }

        // Collect clay
        if self.clay_rate > 0 {
            self.clay += self.clay_rate;
        }

        // Collect obsidian
        if self.obsidian_rate > 0 {
            self.obsidian += self.obsidian_rate;
        }

        // Collect geode
        if self.geode_rate > 0 {
            self.geode += self.geode_rate;
        }
    }

    fn consume_resources(&mut self, resources: &Resources) {
        self.ore -= resources.ore;
        self.clay -= resources.clay;
        self.obsidian -= resources.obsidian;
    }

    fn build_ore(&mut self, cost: &Resources) {
        self.consume_resources(cost);
        self.ore_rate += 1;
    }

    fn build_clay(&mut self, cost: &Resources) {
        self.consume_resources(cost);
        self.clay_rate += 1;
    }

    fn build_obsidian(&mut self, cost: &Resources) {
        self.consume_resources(cost);
        self.obsidian_rate += 1;
    }

    fn build_geode(&mut self, cost: &Resources) {
        self.consume_resources(cost);
        self.geode_rate += 1;
    }
}

fn find_max_geodes(blueprint: &Blueprint, initial_state: State) -> usize {
    let mut queue: Vec<State> = vec![];
    let mut dp: HashMap<State, usize> = HashMap::new();
    let mut max_geodes = 0;

    // Calculate max spend per minute by resource
    let mut max_ore_pm = 0;
    let mut max_clay_pm = 0;
    let mut max_obsidian_pm = 0;

    for costs in vec![
        &blueprint.ore_robot_cost,
        &blueprint.clay_robot_cost,
        &blueprint.obsidian_robot_cost,
        &blueprint.geode_robot_cost,
    ]
    .iter()
    {
        max_ore_pm = max_ore_pm.max(costs.ore);
        max_clay_pm = max_clay_pm.max(costs.clay);
        max_obsidian_pm = max_obsidian_pm.max(costs.obsidian);
    }

    // Initial state
    queue.push(initial_state);

    while queue.len() > 0 {
        let mut state = queue.pop().unwrap();

        // State seen
        if dp.get(&state).is_some() {
            continue;
        }

        // Prune state based on possible max geode rate
        queue = queue
            .into_iter()
            .filter(|it| {
                let max_possible_geodes = max_rate(it.time, it.geode_rate);

                return max_possible_geodes > max_geodes;
            })
            .collect();

        // Update dp
        dp.insert(state.clone(), state.geode);

        // Check
        let should_build_ore_robot =
            state.can_build(&blueprint.ore_robot_cost) && state.ore_rate < max_ore_pm;
        let should_build_clay_robot =
            state.can_build(&blueprint.clay_robot_cost) && state.clay_rate < max_clay_pm;
        let should_build_obsidian_robot = state.can_build(&blueprint.obsidian_robot_cost)
            && state.obsidian_rate < max_obsidian_pm;
        let should_build_geode_robot = state.can_build(&blueprint.geode_robot_cost);

        // Collect resources
        state.collect_resources();

        // Decrease minute
        state.time -= 1;

        // Update max geodes built
        if state.geode > max_geodes {
            max_geodes = max_geodes.max(state.geode);
        }

        // Time out
        if state.time == 0 {
            continue;
        }

        // Just wait
        queue.push(state.clone());

        // Build ore robot
        if should_build_ore_robot {
            let mut state = state.clone();
            state.build_ore(&blueprint.ore_robot_cost);

            // Add if we haven't checked this state before
            if dp.get(&state).is_none() {
                queue.push(state);
            }
        }

        // Build clay robot
        if should_build_clay_robot {
            let mut state = state.clone();
            state.build_clay(&blueprint.clay_robot_cost);

            // Add if we haven't checked this state before
            if dp.get(&state).is_none() {
                queue.push(state);
            }
        }

        // Build obsidian robot
        if should_build_obsidian_robot {
            let mut state = state.clone();
            state.build_obsidian(&blueprint.obsidian_robot_cost);

            // Add if we haven't checked this state before
            if dp.get(&state).is_none() {
                queue.push(state);
            }
        }

        // Build geode robot
        if should_build_geode_robot {
            let mut state = state.clone();
            state.build_geode(&blueprint.geode_robot_cost);

            // Add if we haven't checked this state before
            if dp.get(&state).is_none() {
                queue.push(state);
            }
        }
    }

    max_geodes
}

// Calculate max possible rate given a fixed time
fn max_rate(time: usize, current_rate: usize) -> usize {
    let mut time = time;
    let mut total = 0;
    let mut current_rate = current_rate;

    while time > 0 {
        total += current_rate;
        current_rate += 1;

        time -= 1;
    }

    total
}

fn sum_blueprint_quality(blueprints: &Vec<Blueprint>, initial_state: State) -> usize {
    let mut sum = 0;

    for blueprint in blueprints {
        let max_geodes = find_max_geodes(&blueprint, initial_state.clone());
        sum += blueprint.id * max_geodes;
    }

    sum
}

fn multiply_max_geodes(blueprints: &Vec<Blueprint>, initial_state: State) -> usize {
    let mut result = 1;
    blueprints.iter().for_each(|it| {
        result *= find_max_geodes(it, initial_state.clone());
    });

    result
}

fn main() {
    let input = include_str!("../example");
    let blueprints: Vec<Blueprint> = input
        .lines()
        .into_iter()
        .map(|line| Blueprint::from(line))
        .collect();

    // Part 1
    let initial_state = State {
        time: 24,
        ore: 0,
        clay: 0,
        geode: 0,
        obsidian: 0,
        ore_rate: 1,
        clay_rate: 0,
        obsidian_rate: 0,
        geode_rate: 0,
    };
    println!(
        "Part 1: {}",
        sum_blueprint_quality(&blueprints, initial_state)
    );

    // Part 2
    let initial_state = State {
        time: 32,
        ore: 0,
        clay: 0,
        geode: 0,
        obsidian: 0,
        ore_rate: 1,
        clay_rate: 0,
        obsidian_rate: 0,
        geode_rate: 0,
    };
    let blueprints = blueprints.into_iter().take(3).collect();
    println!(
        "Part 2: {}",
        multiply_max_geodes(&blueprints, initial_state)
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_blueprint() {
        let input = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.";
        let blueprint: Blueprint = input.into();
        let expected = Blueprint {
            id: 1,
            ore_robot_cost: Resources {
                ore: 4,
                clay: 0,
                obsidian: 0,
            },
            clay_robot_cost: Resources {
                ore: 2,
                clay: 0,
                obsidian: 0,
            },
            obsidian_robot_cost: Resources {
                ore: 3,
                clay: 14,
                obsidian: 0,
            },
            geode_robot_cost: Resources {
                ore: 2,
                clay: 0,
                obsidian: 7,
            },
        };

        assert_eq!(blueprint, expected);
    }

    #[test]
    fn example_part1_blueprint1() {
        let input = include_str!("../example");
        let blueprints: Vec<Blueprint> = input
            .lines()
            .into_iter()
            .map(|line| Blueprint::from(line))
            .collect();
        let initial_state = State {
            time: 24,
            ore: 0,
            clay: 0,
            geode: 0,
            obsidian: 0,
            ore_rate: 1,
            clay_rate: 0,
            obsidian_rate: 0,
            geode_rate: 0,
        };

        let max_geodes = find_max_geodes(blueprints.get(0).unwrap(), initial_state);
        assert_eq!(max_geodes, 9);
    }

    #[test]
    fn calc_max_rate() {
        assert_eq!(max_rate(3, 2), 9);
        assert_eq!(max_rate(1, 1), 1);
    }
}
