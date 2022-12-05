enum MovementType {
    Stack,
    InOrder,
}

#[derive(Debug, Clone)]
struct Movement {
    amount: usize,
    from: usize,
    to: usize,
}

impl From<String> for Movement {
    fn from(input: String) -> Self {
        let parts: Vec<&str> = input.split(' ').collect();

        Self {
            amount: parts.get(1).unwrap().parse::<usize>().unwrap(),
            from: parts.get(3).unwrap().parse::<usize>().unwrap(),
            to: parts.get(5).unwrap().parse::<usize>().unwrap(),
        }
    }
}

#[derive(Debug, Clone)]
struct Stacks {
    list: Vec<Vec<String>>,
}

impl Stacks {
    fn apply_movements(&mut self, mv_type: MovementType, movements: Vec<Movement>) {
        for movement in movements {
            // Remove from
            let from = self.list.get_mut(movement.from - 1).unwrap();
            let mut elements: Vec<String> = from.drain(from.len() - movement.amount..).collect();

            // Add to
            let target = self.list.get_mut(movement.to - 1).unwrap();

            match mv_type {
                MovementType::Stack => {
                    while elements.len() > 0 {
                        target.push(elements.pop().unwrap());
                    }
                }
                MovementType::InOrder => {
                    target.append(&mut elements);
                }
            }
        }
    }

    fn top_crates(self) -> Vec<String> {
        self.list
            .iter()
            .map(|it| it.last().unwrap().to_string())
            .collect()
    }
}

impl From<Vec<String>> for Stacks {
    fn from(input: Vec<String>) -> Self {
        let mut stacks = Stacks { list: vec![] };

        // Reverse the input
        let mut input = input.clone();
        input.reverse();

        input
            .iter()
            .map(|it| it.split(' ').collect::<Vec<&str>>())
            .for_each(|items| {
                // Initialize
                if stacks.list.len() == 0 {
                    items
                        .iter()
                        .for_each(|it| stacks.list.push(vec![it.to_string()]));
                } else {
                    // Add the next items
                    let mut idx = 0;
                    let mut skip = 0;
                    items.iter().for_each(|it| {
                        // Skip empty
                        if it.is_empty() {
                            skip += 1;

                            return;
                        }

                        if skip != 0 {
                            idx += skip / 4;
                            skip = 0;
                        }

                        // Add the item into the correct index
                        stacks.list.get_mut(idx).unwrap().push(it.to_string());
                        idx += 1;
                    })
                }
            });

        stacks
    }
}

fn main() {
    let content = include_str!("input");
    let original_state: Stacks = content
        .lines()
        .take(8)
        .map(|it| it.to_string())
        .collect::<Vec<String>>()
        .into();

    let movements: Vec<Movement> = content
        .lines()
        .skip(10)
        .map(|it| Movement::from(it.to_string()))
        .collect();

    // Part 1, stack movements
    let mut stack_state = original_state.clone();
    stack_state.apply_movements(MovementType::Stack, movements.clone());

    println!(
        "Top crates with stack movements: {:?}",
        stack_state.top_crates()
    );

    // Part 2, in order movements
    let mut inorder_state = original_state.clone();
    inorder_state.apply_movements(MovementType::InOrder, movements);
    println!(
        "Top crates with inorder movements: {:?}",
        inorder_state.top_crates()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_movement() {
        let movement: Movement = "move 3 from 5 to 2".to_string().into();

        assert_eq!(movement.amount, 3);
        assert_eq!(movement.from, 5);
        assert_eq!(movement.to, 2);
    }

    #[test]
    fn parse_state() {
        let current_state = r"        [F] [Q]         [Q]        
[B]     [Q] [V] [D]     [S]        
[S] [P] [T] [R] [M]     [D]        
[J] [V] [W] [M] [F]     [J]     [J]
[Z] [G] [S] [W] [N] [D] [R]     [T]
[V] [M] [B] [G] [S] [C] [T] [V] [S]
[D] [S] [L] [J] [L] [G] [G] [F] [R]
[G] [Z] [C] [H] [C] [R] [H] [P] [D]"
            .to_string();

        let stacks: Stacks = current_state
            .lines()
            .map(|it| it.to_string())
            .collect::<Vec<String>>()
            .into();

        assert_eq!(
            stacks.list.get(0).unwrap().join(","),
            "[G],[D],[V],[Z],[J],[S],[B]"
        );
    }

    #[test]
    fn move_items() {
        let mut stacks = Stacks {
            list: vec![
                vec!["Z".to_string(), "N".to_string()],
                vec!["M".to_string(), "C".to_string(), "D".to_string()],
                vec!["P".to_string()],
            ],
        };

        let movements: Vec<Movement> = r"move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
            .lines()
            .map(|it| Movement::from(it.to_string()))
            .collect();

        // Apply stack movements
        stacks.apply_movements(MovementType::Stack, movements);

        assert_eq!(stacks.list.get(0).unwrap().join(","), "C");
        assert_eq!(stacks.list.get(1).unwrap().join(","), "M");
        assert_eq!(stacks.list.get(2).unwrap().join(","), "P,D,N,Z");

        // Test in order
        stacks.apply_movements(
            MovementType::InOrder,
            vec![Movement {
                amount: 2,
                from: 3,
                to: 1,
            }],
        );
        assert_eq!(stacks.list.get(0).unwrap().join(","), "C,N,Z");
        assert_eq!(stacks.list.get(2).unwrap().join(","), "P,D");
    }
}
