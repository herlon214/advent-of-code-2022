use std::{
    collections::{HashSet, VecDeque},
    time::Duration,
};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn position(&self, position: (usize, usize)) -> (usize, usize) {
        match self {
            Direction::Up => (position.0 - 1, position.1),
            Direction::Down => (position.0 + 1, position.1),
            Direction::Left => (position.0, position.1 - 1),
            Direction::Right => (position.0, position.1 + 1),
        }
    }

    fn is_allowed(&self, current: u8, prev: u8) -> bool {
        if (current >= 'a' as u8 && current <= prev)
            || current == prev + 1
            || (current == 'E' as u8 && prev == 'z' as u8)
        {
            return true;
        }

        false
    }
}

#[derive(Debug)]
struct Node {
    prev: u8,
    position: (usize, usize),
    counter: usize,
}

fn print(grid: Vec<Vec<char>>) {
    for line in grid {
        println!("{:?}", line);
    }
}

fn bfs(grid: &mut Vec<Vec<char>>, start: (usize, usize)) -> usize {
    let mut queue: VecDeque<Node> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    queue.push_front(Node {
        prev: 96,
        position: start,
        counter: 0,
    });

    grid[start.0][start.1] = 'a';

    while queue.len() != 0 {
        let current = queue.pop_front().unwrap();
        let position = current.position;
        let val = grid[position.0][position.1] as u8;

        // Found the target
        if val == 'E' as u8 {
            println!("Found!");
            dbg!(&current);

            return current.counter;
        }

        // Skip the node if visited
        if visited.get(&current.position).is_some() {
            continue;
        }

        // Add the node as visited
        visited.insert(position);

        // Store the targets
        let mut targets = vec![];

        // Up
        if position.0 > 0 {
            targets.push(Direction::Up);
        }

        // Down
        if position.0 < grid.len() - 1 {
            targets.push(Direction::Down);
        }

        // Left
        if position.1 > 0 {
            targets.push(Direction::Left);
        }

        // Right
        if position.1 < grid[0].len() - 1 {
            targets.push(Direction::Right);
        }

        // Add targets to queue
        for dir in targets {
            let pos = dir.position(current.position);
            let target_val = grid[pos.0][pos.1] as u8;

            // Only if it's allowed
            if dir.is_allowed(target_val, val) {
                queue.push_back(Node {
                    position: pos,
                    prev: val,
                    counter: current.counter + 1,
                });
            }
        }

        // print!("\x1B[2J\x1B[1;1H");
        // print(grid.clone());

        // std::thread::sleep(Duration::from_millis(50));
    }

    0
}

fn main() {
    let mut grid: Vec<Vec<char>> = include_str!("../input")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // Example
    // let total = bfs(&mut grid, (0, 0));

    // Part 1
    let total = bfs(&mut grid, (20, 0));

    println!("Part1 steps: {}", total);
}
