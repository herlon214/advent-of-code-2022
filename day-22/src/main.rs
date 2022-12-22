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

#[derive(Debug, Clone)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
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
}

impl Map {
    fn new(input: &str) -> Self {
        let mut map: HashMap<Position, Tile> = HashMap::new();
        let mut start: Position = Position::new(i32::MAX, i32::MAX);
        let mut y_max = 0;

        for (y, line) in input.lines().enumerate() {
            y_max += 1;
            let mut x_max = 0;

            for (x, ch) in line.chars().enumerate() {
                x_max += 1;
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
            }
        }

        Self {
            grid: map,
            current_pos: start,
            facing: Direction::Right,
        }
    }

    fn mov(&mut self, movement: &Movement) {
        match movement {
            Movement::Forward(mut n) => {
                let pos = match self.facing {
                    Direction::Right => Position(1, 0),
                    Direction::Down => Position(0, 1),
                    Direction::Left => Position(-1, 0),
                    Direction::Up => Position(0, -1),
                };

                while n > 0 {
                    let new_pos = self.current_pos.apply(&pos);
                    match self.grid.get(&new_pos) {
                        Some(Tile::Wall) | None => break,
                        _ => {
                            self.current_pos = new_pos;
                        }
                    }

                    n -= 1;
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

        println!("Facing: {:?} Position: {:?}", self.facing, self.current_pos);
    }
}

fn main() {
    let input = include_str!("../example_movements");
    let movements = parse_movements(input);
    let input = include_str!("../example_map");
    let mut map = Map::new(input);

    for movement in movements.iter() {
        println!("----------");
        println!("Movement: {:?}", movement);
        map.mov(movement);
    }

    println!("Hello, world!");
}
