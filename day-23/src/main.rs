use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Direction {
    North,
    NorthEast,
    NorthWest,
    South,
    SouthEast,
    SouthWest,
    West,
    East,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
struct Position(i32, i32);

impl Position {
    fn apply(&self, direction: &Direction) -> Self {
        let mut new_pos = self.clone();

        match direction {
            Direction::North => {
                new_pos.1 -= 1;
            }
            Direction::South => {
                new_pos.1 += 1;
            }
            Direction::West => {
                new_pos.0 -= 1;
            }
            Direction::NorthWest => {
                new_pos.1 -= 1;
                new_pos.0 -= 1;
            }
            Direction::SouthWest => {
                new_pos.0 -= 1;
                new_pos.1 += 1;
            }
            Direction::East => {
                new_pos.0 += 1;
            }
            Direction::NorthEast => {
                new_pos.0 += 1;
                new_pos.1 -= 1;
            }
            Direction::SouthEast => {
                new_pos.0 += 1;
                new_pos.1 += 1;
            }
        }

        new_pos
    }
}

#[derive(Debug)]
struct Map {
    elves: HashSet<Position>,
    movements: VecDeque<(Direction, Vec<Direction>)>,
    votes: HashMap<Position, Vec<Position>>,
    round: usize,
}

impl Map {
    fn boundaries(&self) -> ((i32, i32), (i32, i32)) {
        let mut x = (i32::MAX, i32::MIN);
        let mut y = (i32::MAX, i32::MIN);

        for pos in self.elves.iter() {
            x.0 = x.0.min(pos.0);
            x.1 = x.1.max(pos.0);
            y.0 = y.0.min(pos.1);
            y.1 = y.1.max(pos.1);
        }

        (x, y)
    }

    fn count_empty(&self) -> i32 {
        let (x, y) = self.boundaries();
        let mut counter = 0i32;

        for y in y.0..=y.1 {
            for x in x.0..=x.1 {
                let pos = Position(x, y);
                let val = self.elves.get(&pos);

                match val {
                    None => counter += 1,
                    _ => {}
                }
            }
        }

        counter
    }

    #[allow(unused)]
    fn print(&self) {
        let (x, y) = self.boundaries();

        let mut result = "".to_string();

        for y in y.0..=y.1 {
            let mut line = "".to_string();

            for x in x.0..=x.1 {
                let pos = Position(x, y);
                let val = self.elves.get(&pos);

                match val {
                    Some(_) => line = format!("{}{}", line, "#"),
                    None => line = format!("{}{}", line, "."),
                }
            }

            println!("{}", line);
            result = format!("{}\n{}", result, line);
        }

        std::fs::write("map", result).expect("Unable to write file");
    }

    fn anyone_at(&self, positions: Vec<Position>) -> bool {
        positions.iter().any(|it| self.elves.get(it).is_some())
    }

    fn all_empty(&self, positions: Vec<&Position>) -> bool {
        positions.iter().all(|it| self.elves.get(it).is_none())
    }

    fn simulate(&mut self, n: i32) {
        for _ in 0..n {
            self.tick();
        }
    }

    fn simulate_until_no_movement(&mut self) {
        loop {
            self.tick();

            if self.votes.len() == 0 {
                break;
            }
        }
    }

    fn tick(&mut self) {
        // Reset votes
        self.votes = HashMap::new();

        // Check every elf
        for pos in self.elves.iter() {
            // Movement directions
            let north = pos.apply(&Direction::North);
            let north_east = pos.apply(&Direction::NorthEast);
            let north_west = pos.apply(&Direction::NorthWest);
            let south = pos.apply(&Direction::South);
            let south_east = pos.apply(&Direction::SouthEast);
            let south_west = pos.apply(&Direction::SouthWest);
            let west = pos.apply(&Direction::West);
            let east = pos.apply(&Direction::East);

            // Don't do anything if no elves around
            if self.all_empty(vec![
                &north,
                &north_east,
                &north_west,
                &south,
                &south_east,
                &south_west,
                &west,
                &east,
            ]) {
                continue;
            }

            // Check for movement in the directions
            for (direction, checks) in self.movements.iter() {
                let target = pos.apply(direction);
                let check_positions: Vec<Position> =
                    checks.clone().iter().map(|it| pos.apply(it)).collect();

                // Check if there's nobody in those positions
                if !self.anyone_at(check_positions) {
                    let votes = self.votes.entry(target.clone()).or_insert(vec![]);
                    votes.push(pos.clone());

                    break;
                }
            }
        }

        // Verify the votes
        for (pos, voters) in self.votes.iter() {
            // Move the elf if it's the only voter
            if voters.len() == 1 {
                self.elves.remove(voters.get(0).unwrap());
                self.elves.insert(pos.clone());
            }
        }

        // Rotate decision
        self.movements.rotate_left(1);

        // Increase round counter
        self.round += 1;
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        // Elves positions
        let mut elves: HashSet<Position> = HashSet::new();

        // Movements and checks
        let mut movements: VecDeque<(Direction, Vec<Direction>)> = VecDeque::new();

        // Move north
        movements.push_back((
            Direction::North,
            vec![Direction::North, Direction::NorthEast, Direction::NorthWest],
        ));

        // Move south
        movements.push_back((
            Direction::South,
            vec![Direction::South, Direction::SouthEast, Direction::SouthWest],
        ));

        // Move west
        movements.push_back((
            Direction::West,
            vec![Direction::West, Direction::NorthWest, Direction::SouthWest],
        ));

        // Move east
        movements.push_back((
            Direction::East,
            vec![Direction::East, Direction::NorthEast, Direction::SouthEast],
        ));

        for (y, line) in input.lines().enumerate() {
            for (x, tile) in line.chars().enumerate() {
                if tile == '#' {
                    elves.insert(Position(x as i32, y as i32));
                }
            }
        }

        Map {
            elves,
            movements,
            votes: HashMap::new(),
            round: 0,
        }
    }
}

fn main() {
    let input = include_str!("../input");

    // Part 1
    let mut map: Map = input.into();
    map.simulate(10);
    println!("Part 1: {}", map.count_empty());

    // Part 2
    let mut map: Map = input.into();
    map.simulate_until_no_movement();
    println!("Part 2: {}", map.round);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("../example");
        let mut map: Map = input.into();

        map.simulate(10);

        assert_eq!(map.count_empty(), 110);
    }
}
