use crate::game::*;
use crate::op::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
pub struct Monkey {
    pub items: Vec<Item>,
    pub test: Test,
    pub op: Operation,
    pub target_true: usize,
    pub target_false: usize,
    pub inspection_counter: usize,
}

impl Monkey {
    pub fn receive_item(&mut self, item: Item) {
        self.items.push(item);
    }

    pub fn catch_items(&mut self, air: &mut HashMap<usize, Vec<Item>>, id: usize) {
        match air.get_mut(&id) {
            Some(items) => {
                for item in items {
                    self.receive_item(item.clone());
                }

                // Delete
                air.remove(&id);
            }
            None => {}
        }
    }

    pub fn inspect_items(&mut self, decrease_lv: u128) -> HashMap<usize, Vec<Item>> {
        // Targets
        let mut targets: HashMap<usize, Vec<Item>> =
            HashMap::from([(self.target_true, vec![]), (self.target_false, vec![])]);

        self.items
            .iter()
            .map(|it| {
                // Increase inspection counter
                self.inspection_counter += 1;

                let mut new_item = it.clone();

                // Perform operation
                // pt1
                match self.op {
                    Operation::Add(Some(val)) => new_item.worry_level += val,
                    Operation::Add(None) => new_item.worry_level *= 2,
                    Operation::Multiply(Some(val)) => new_item.worry_level *= val,
                    Operation::Multiply(None) => new_item.worry_level *= new_item.worry_level,
                }

                // Divide by a factor
                new_item.worry_level /= decrease_lv;

                // Product of all divisors
                new_item.worry_level %= 9699690;

                new_item
            })
            .for_each(|it| {
                // Test
                match self.test {
                    Test::Divisible(val) => {
                        if it.worry_level % val == 0 {
                            targets.get_mut(&self.target_true).unwrap().push(it);
                        } else {
                            targets.get_mut(&self.target_false).unwrap().push(it);
                        }
                    }
                }
            });

        // Reset all current items
        self.items = vec![];

        targets
    }
}

impl From<String> for Monkey {
    fn from(input: String) -> Self {
        let input: Vec<&str> = input.lines().collect();

        let starting_items: Vec<Item> = input
            .get(0)
            .unwrap()
            .split(':')
            .last()
            .unwrap()
            .replace(",", "")
            .split_whitespace()
            .map(|it| it.parse::<u128>().unwrap())
            .map(|it| Item { worry_level: it })
            .collect();

        let op_data: Vec<&str> = input
            .get(1)
            .unwrap()
            .split_whitespace()
            .rev()
            .take(2)
            .collect();
        let op_val = op_data.get(0).unwrap().parse::<u128>();
        let op = match op_data.get(1).unwrap().to_owned() {
            "*" => {
                if op_val.is_err() {
                    Operation::Multiply(None)
                } else {
                    Operation::Multiply(Some(op_val.unwrap()))
                }
            }
            "+" => {
                if op_val.is_err() {
                    Operation::Add(None)
                } else {
                    Operation::Add(Some(op_val.unwrap()))
                }
            }
            op => unreachable!("Operation not valid: {}", op),
        };

        let test: u128 = input
            .get(2)
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let target_true: usize = input
            .get(3)
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        let target_false: usize = input
            .get(4)
            .unwrap()
            .split_whitespace()
            .last()
            .unwrap()
            .parse()
            .unwrap();

        Self {
            items: starting_items,
            test: Test::Divisible(test),
            op,
            target_true,
            target_false,
            inspection_counter: 0,
        }
    }
}
