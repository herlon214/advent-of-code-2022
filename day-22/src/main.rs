use std::collections::HashMap;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Position(i32, i32);

impl Position {
    fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }

    fn x(&self) -> i32 {
        self.0
    }

    fn y(&self) -> i32 {
        self.1
    }

    fn from_direction(dir: &Direction) -> Self {
        match dir {
            Direction::Right => Position(1, 0),
            Direction::Down => Position(0, 1),
            Direction::Left => Position(-1, 0),
            Direction::Up => Position(0, -1),
        }
    }

    fn apply(&self, position: &Position) -> Self {
        let mut new_pos = self.clone();
        new_pos.0 += position.0;
        new_pos.1 += position.1;

        new_pos
    }
}

#[derive(Debug, Clone)]
enum Tile {
    Empty,
    Wall,
}

impl ToString for Tile {
    fn to_string(&self) -> String {
        match self {
            Tile::Empty => ".".to_string(),
            Tile::Wall => "#".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

impl ToString for Direction {
    fn to_string(&self) -> String {
        match self {
            Direction::Down => "v".to_string(),
            Direction::Up => "^".to_string(),
            Direction::Left => "<".to_string(),
            Direction::Right => ">".to_string(),
        }
    }
}

impl Direction {
    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
        }
    }

    fn value(&self) -> i32 {
        match self {
            Direction::Up => 3,
            Direction::Left => 2,
            Direction::Right => 0,
            Direction::Down => 1,
        }
    }
}

#[derive(Debug, Clone)]
enum Movement {
    Forward(i32),
    RotateClockwise,
    RotateCounterClockwise,
}

fn parse_movements(input: &str) -> Vec<Movement> {
    let mut result: Vec<Movement> = vec![];
    let mut current = "".to_string();

    for ch in input.trim().chars() {
        match ch {
            'R' | 'L' => {
                // Parse the current numbers
                let n = current.parse::<i32>().unwrap();
                result.push(Movement::Forward(n));

                // Reset current
                current = "".to_string();

                // Add rotation
                if ch == 'R' {
                    result.push(Movement::RotateClockwise);
                } else {
                    result.push(Movement::RotateCounterClockwise);
                }
            }
            _ => {
                current = format!("{}{}", current, ch);
            }
        }
    }

    // Parse if any number was left
    if current.len() > 0 {
        // Parse the current numbers
        let n = current.parse::<i32>().unwrap();
        result.push(Movement::Forward(n));
    }

    result
}

struct Map {
    grid: HashMap<Position, Tile>,
    current_pos: Position,
    facing: Direction,
    x_boundaries: (i32, i32),
    y_boundaries: (i32, i32),
    visited_positions: HashMap<Position, String>,
}

impl Map {
    fn new(input: &str) -> Self {
        let mut map: HashMap<Position, Tile> = HashMap::new();
        let mut start: Position = Position::new(i32::MAX, i32::MAX);
        let mut x_boundaries = (i32::MAX, i32::MIN);
        let mut y_boundaries = (i32::MAX, i32::MIN);
        let mut visited_positions: HashMap<Position, String> = HashMap::new();

        for (y, line) in input.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                match ch {
                    '.' => {
                        map.insert(Position::new(x as i32, y as i32), Tile::Empty);
                        if (x as i32) < start.0 && (y as i32) < start.1 {
                            start.0 = x as i32;
                            start.1 = y as i32;
                        }
                    }
                    '#' => {
                        map.insert(Position::new(x as i32, y as i32), Tile::Wall);
                    }
                    _ => {}
                }

                x_boundaries.0 = x_boundaries.0.min(x as i32);
                x_boundaries.1 = x_boundaries.1.max(x as i32);
                y_boundaries.0 = y_boundaries.0.min(y as i32);
                y_boundaries.1 = y_boundaries.1.max(y as i32);
            }
        }

        // Add initial state
        visited_positions.insert(start.clone(), Direction::Right.to_string());

        Self {
            grid: map,
            current_pos: start,
            facing: Direction::Right,
            x_boundaries,
            y_boundaries,
            visited_positions,
        }
    }

    fn reset_pos(&mut self) -> bool {
        let pos = Position::from_direction(&self.facing.opposite());
        let mut new_pos = self.current_pos.clone();

        loop {
            new_pos = new_pos.apply(&pos);

            match self.grid.get(&new_pos) {
                None => break,
                _ => {}
            }
        }

        // Check the tile before
        let before = new_pos.apply(&Position::from_direction(&self.facing));
        match self.grid.get(&before) {
            Some(Tile::Empty) => {
                self.current_pos = new_pos;

                return true;
            }
            _ => {
                return false;
            }
        }
    }

    fn mov(&mut self, movement: &Movement) {
        match movement {
            Movement::Forward(mut n) => {
                let pos = Position::from_direction(&self.facing);

                while n > 0 {
                    let new_pos = self.current_pos.apply(&pos);
                    match self.grid.get(&new_pos) {
                        Some(Tile::Wall) => break,
                        Some(Tile::Empty) => {
                            self.current_pos = new_pos.clone();
                            self.visited_positions
                                .insert(new_pos, self.facing.to_string());
                            n -= 1;
                        }
                        None => {
                            if !self.reset_pos() {
                                break;
                            }
                        }
                    }
                }
            }
            Movement::RotateClockwise => match self.facing {
                Direction::Right => self.facing = Direction::Down,
                Direction::Down => self.facing = Direction::Left,
                Direction::Left => self.facing = Direction::Up,
                Direction::Up => self.facing = Direction::Right,
            },
            Movement::RotateCounterClockwise => match self.facing {
                Direction::Right => self.facing = Direction::Up,
                Direction::Up => self.facing = Direction::Left,
                Direction::Left => self.facing = Direction::Down,
                Direction::Down => self.facing = Direction::Right,
            },
        }
    }

    fn password(&self) -> i32 {
        let row = self.current_pos.y() + 1;
        let col = self.current_pos.x() + 1;

        row * 1000 + col * 4 + self.facing.value()
    }

    fn apply_movements(&mut self, movements: Vec<Movement>) {
        for movement in movements.iter() {
            self.mov(movement);
        }
    }

    #[allow(unused)]
    fn print(&self) {
        let mut result = "".to_string();

        for y in self.y_boundaries.0..self.y_boundaries.1 {
            let mut line = "".to_string();

            for x in self.x_boundaries.0..self.x_boundaries.1 {
                let pos = Position(x, y);
                let val = self.grid.get(&pos);
                let visited = self.visited_positions.get(&pos);

                match (val, visited) {
                    (_, Some(val)) => line = format!("{}{}", line, val.to_string()),
                    (None, _) => line = format!("{}{}", line, " "),
                    (Some(tile), None) => line = format!("{}{}", line, tile.to_string()),
                }
            }

            result = format!("{}\n{}", result, line);
        }

        std::fs::write("map", result).expect("Unable to write file");
    }
}

fn main() {
    let input = include_str!("../input_movements");
    let movements = parse_movements(input);
    let input = include_str!("../input_map");
    let mut map = Map::new(input);

    map.apply_movements(movements);
    map.visited_positions
        .insert(map.current_pos.clone(), "X".to_string());
    println!("Part 1: {}", map.password());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("../example_movements");
        let movements = parse_movements(input);
        let input = include_str!("../example_map");
        let mut map = Map::new(input);

        map.apply_movements(movements);

        assert_eq!(map.password(), 6032);
    }
}
