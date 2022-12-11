use crate::monkey::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Item {
    pub worry_level: u128,
}

#[derive(Debug)]
pub struct Game {
    pub monkeys: Vec<Monkey>,
    pub rounds: usize,
    pub decrease_lv: u128,
}

impl Game {
    pub fn new(rounds: usize, decrease_lv: u128) -> Self {
        Game {
            monkeys: vec![],
            decrease_lv,
            rounds,
        }
    }

    pub fn read_input(&mut self, input: &str) {
        for line in input.lines().collect::<Vec<&str>>().chunks(7) {
            let full_lines = line.join("\n");
            self.parse_str(&full_lines);
        }
    }

    pub fn parse_str(&mut self, input: &str) {
        let input: Vec<&str> = input.lines().collect();

        let monkey_txt: String = input
            .iter()
            .skip(1)
            .map(|it| it.clone())
            .collect::<Vec<&str>>()
            .join("\n");

        self.monkeys.push(monkey_txt.into());
    }

    pub fn start(&mut self) {
        let mut air: HashMap<usize, Vec<Item>> = HashMap::new();

        for round in 0..self.rounds {
            println!("Round {}", round);
            for (i, monkey) in self.monkeys.iter_mut().enumerate() {
                // Receive items
                monkey.catch_items(&mut air, i);

                // Inspect items
                let inspected_items = monkey.inspect_items(self.decrease_lv);

                // Add the items to the other monkeys
                inspected_items.iter().for_each(|(id, items)| {
                    // Check if the list is created
                    if air.get(id).is_none() {
                        air.insert(*id, vec![]);
                    }

                    // Add the items to the list
                    air.get_mut(id).unwrap().append(items.clone().as_mut());
                })
            }
        }

        // Receive remaining items
        for (i, monkey) in self.monkeys.iter_mut().enumerate() {
            monkey.catch_items(&mut air, i);
        }
    }

    pub fn monkey_business(&self) -> usize {
        let mut counters: Vec<usize> = self
            .monkeys
            .iter()
            .map(|it| it.inspection_counter)
            .collect();
        counters.sort_unstable();

        let top_2: Vec<usize> = counters.into_iter().rev().take(2).collect();

        top_2.get(0).unwrap() * top_2.get(1).unwrap()
    }
}
