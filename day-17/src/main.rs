use std::collections::{HashMap, HashSet};
use std::ops::{Add, Range};
use std::time::Duration;

// Point movements
const DOWN: Point = Point(0, -1);
const LEFT: Point = Point(-1, 0);
const RIGHT: Point = Point(1, 0);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }

    fn apply(&mut self, movement: &Point) {
        self.0 += movement.0;
        self.1 += movement.1;
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl From<char> for Point {
    fn from(input: char) -> Self {
        match input {
            '>' => RIGHT,
            '<' => LEFT,
            _ => unreachable!("Invalid input: {:?}", input),
        }
    }
}

#[derive(Debug)]
enum RockShape {
    A,
    B,
    C,
    D,
    E,
}

impl RockShape {
    fn next_shape(&self) -> RockShape {
        match self {
            RockShape::A => RockShape::B,
            RockShape::B => RockShape::C,
            RockShape::C => RockShape::D,
            RockShape::D => RockShape::E,
            RockShape::E => RockShape::A,
        }
    }
    fn to_point(&self) -> Vec<Point> {
        match self {
            RockShape::A => {
                vec![
                    Point::new(0, 0),
                    Point::new(1, 0),
                    Point::new(2, 0),
                    Point::new(3, 0),
                ]
            }
            RockShape::B => {
                vec![
                    Point::new(1, 0),
                    Point::new(0, 1),
                    Point::new(1, 1),
                    Point::new(2, 1),
                    Point::new(1, 2),
                ]
            }
            RockShape::C => {
                vec![
                    Point::new(0, 0),
                    Point::new(1, 0),
                    Point::new(2, 0),
                    Point::new(2, 1),
                    Point::new(2, 2),
                ]
            }
            RockShape::D => {
                vec![
                    Point::new(0, 0),
                    Point::new(0, 1),
                    Point::new(0, 2),
                    Point::new(0, 3),
                ]
            }
            RockShape::E => {
                vec![
                    Point::new(0, 0),
                    Point::new(1, 0),
                    Point::new(0, 1),
                    Point::new(1, 1),
                ]
            }
        }
    }

    fn relative_to(&self, point: Point) -> Vec<Point> {
        let points = self
            .to_point()
            .iter()
            .map(|it| {
                let mut new = it.clone();
                new.apply(&point);

                new
            })
            .collect();

        points
    }
}

struct Cave {
    map: HashMap<Point, char>,
    highest_y: i32,
    jet_movements: Vec<Point>,
    curr_jet_movement: usize,
    current_shape: RockShape,
    x_boundaries: Range<i32>,
    curr_n: i32,
    deltas: Vec<i32>,
}

impl Cave {
    fn new(jet_movements: Vec<Point>) -> Self {
        Self {
            map: HashMap::new(),
            current_shape: RockShape::A,
            highest_y: 0,
            x_boundaries: 0..7,
            jet_movements,
            curr_jet_movement: 0,
            curr_n: 0,
            deltas: vec![],
        }
    }

    fn get_char(&mut self) -> char {
        let ch = self.curr_n.to_string().chars().last().unwrap();

        self.curr_n = (self.curr_n + 1) % 10;

        ch
    }

    fn spawn_position(&self) -> Point {
        Point::new(2, self.highest_y + 4)
    }

    fn get_jet_movement(&mut self) -> Point {
        let mov = self.jet_movements.get(self.curr_jet_movement).unwrap();

        // Update
        let idx = (self.curr_jet_movement + 1) % self.jet_movements.len();
        self.curr_jet_movement = idx;

        mov.clone()
    }

    fn can_move(&self, point: &Point, movement: &Point) -> bool {
        let mut tmp = point.clone();
        tmp.apply(movement);

        // Floor
        if tmp.1 <= 0 {
            return false;
        }

        // X boundaries
        if !self.x_boundaries.contains(&tmp.0) {
            return false;
        }

        self.map.get(&tmp).is_none()
    }

    fn tick(&mut self) {
        // Spawn shape
        let spawn_pos = self.spawn_position();
        let mut shape = self.current_shape.relative_to(spawn_pos.clone());

        // println!("---------------");
        // println!(
        //     "Spawning shape: {:?} at {:?}",
        //     self.current_shape, spawn_pos
        // );

        // Move the shape down until it's stable
        loop {
            // Apply jet movements
            let jet_mov = self.get_jet_movement();

            // Check if can move
            if shape.iter().all(|p| self.can_move(p, &jet_mov)) {
                // Apply movement
                // println!("--> Applying jet move: {:?}", jet_mov);
                shape.iter_mut().for_each(|p| {
                    p.apply(&jet_mov);
                });
            } else {
                // println!("--> Jet move ignored");
            }

            // Move down
            if shape.iter().all(|p| self.can_move(p, &DOWN)) {
                // println!("Moving down");
                // Apply down movement
                shape.iter_mut().for_each(|p| {
                    p.apply(&DOWN);
                });
            } else {
                // println!("--> Reached floor");
                break;
            }
        }

        // println!("Block stable!");
        let prev_highest = self.highest_y;

        // Write blocks and get highest_y
        let ch = self.get_char();
        shape.iter().for_each(|p| {
            self.highest_y = self.highest_y.max(p.1);

            self.map.insert(p.clone(), ch.clone());
        });

        self.deltas.push(self.highest_y - prev_highest);

        // println!("Highest Y: {}", self.highest_y);
        // println!("Lowest Y: {}", self.lowest_y);

        // Update next shape
        self.current_shape = self.current_shape.next_shape();
    }

    pub fn print(&self) {
        let grid = self.map_to_grid();

        println!("");
        for cols in grid.iter() {
            let mut line = "".to_string();
            for val in cols {
                line = format!("{}{}", line, val);
            }

            println!("{}", line);
        }
    }

    fn map_to_grid(&self) -> Vec<Vec<char>> {
        // Get boundaries
        let mut x_min = i32::MAX;
        let mut x_max = i32::MIN;
        let mut y_min = i32::MAX;
        let mut y_max = i32::MIN;

        for (point, _) in self.map.iter() {
            if point.0 < x_min {
                x_min = point.0;
            }
            if point.0 > x_max {
                x_max = point.0;
            }
            if point.1 < y_min {
                y_min = point.1;
            }
            if point.1 > y_max {
                y_max = point.1;
            }
        }

        println!("Boundaries: {x_min},{x_max} - {y_min},{y_max}");

        let padding = 1;
        let mut result = vec![vec!['.'; (x_max + padding) as usize]; (y_max + padding) as usize];

        for (point, ch) in self.map.iter() {
            result[(point.1 + padding / 2) as usize][(point.0 + padding / 2) as usize] = *ch;
        }

        result
    }
}

fn main() {
    let input = include_str!("../input");

    let movements: Vec<Point> = input
        .chars()
        .filter(|it| it.is_ascii_punctuation())
        .map(|it| it.into())
        .collect();

    // Part 1
    let mut cave = Cave::new(movements.clone());

    for _ in 0..2022 {
        // println!(";;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;");
        cave.tick();
        // cave.print();
        // std::thread::sleep(Duration::from_millis(1000));
    }
    println!("Part 1 tallest tower: {}", cave.highest_y);

    // Part 2
    let mut cave = Cave::new(movements.clone());

    // Run a couple of times to detect cycle
    for _ in 0..3000 {
        cave.tick();
    }

    // Detect cycle tring to find a window that match
    let (offset, size) = (0..500)
        .find_map(|offset| {
            let delta_iter = cave.deltas.iter().skip(offset);

            let size = (2..=2500).find(|size| {
                let window = cave.deltas[offset..offset + size].iter().cycle();

                delta_iter.clone().zip(window).all(|(a, b)| a == b)
            });

            size.map(|size| (offset, size))
        })
        .unwrap();

    let mut target = 1_000_000_000_000i64;
    let deltas = cave.deltas.clone();
    let offset_delta = deltas.iter().by_ref().take(offset).sum::<i32>();

    // Decrease offset
    target -= offset as i64;

    // Get only the repeating values
    let cycle_deltas: Vec<i32> = deltas.iter().take(size).copied().collect();

    // Sum whole cycle
    let cycle_sum = cycle_deltas.iter().sum::<i32>();

    // Check how many times the cycle can repeat until get to target
    let cycle_count = target / size as i64;
    target %= size as i64;

    // Calculate the rest
    let rest = cycle_deltas.into_iter().take(target as usize).sum::<i32>();

    // Calculate the total height
    let height: i64 = offset_delta as i64 + cycle_count as i64 * cycle_sum as i64 + rest as i64;

    println!("Part 2 tallest tower: {}", height);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apply_movement() {
        let movement = RIGHT.clone();
        let mut point = Point::new(10, 10);
        point.apply(&movement);

        assert_eq!(point, Point::new(11, 10));
    }

    #[test]
    fn relative_to() {
        let a = Point::new(10, 10);
        let shape = RockShape::A;
        let result = vec![
            Point::new(10, 10),
            Point::new(11, 10),
            Point::new(12, 10),
            Point::new(13, 10),
        ];

        assert_eq!(shape.relative_to(a), result);
    }
}
