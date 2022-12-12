use std::collections::{HashSet, VecDeque};

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
    position: (usize, usize),
    counter: usize,
}

fn bfs(grid: &mut Vec<Vec<char>>, start: (usize, usize)) -> usize {
    let mut queue: VecDeque<Node> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    queue.push_front(Node {
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
                    counter: current.counter + 1,
                });
            }
        }
    }

    0
}

fn main() {
    let grid: Vec<Vec<char>> = include_str!("../input")
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    // Example
    // let total = bfs(&mut grid, (0, 0));

    // Part 1
    let total = bfs(&mut grid.clone(), (20, 0));

    println!("Part 1 steps: {}", total);

    // Find all starting points with elevation 'a'
    let mut starting_points: Vec<(usize, usize)> = vec![];
    let mut completions: Vec<usize> = vec![];

    grid.iter().enumerate().for_each(|(i, cols)| {
        cols.iter().enumerate().for_each(|(j, ch)| {
            if *ch == 'a' || *ch == 'S' {
                starting_points.push((i, j));
            }
        })
    });

    // BFS on each starting point
    for start in starting_points {
        let result = bfs(&mut grid.clone(), start);
        if result > 0 {
            completions.push(result);
        }
    }

    // Sort
    completions.sort();

    println!("Part 2 steps: {}", completions.first().unwrap());
}
