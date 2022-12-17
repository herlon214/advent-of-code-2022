use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::{Add, Range};

// Point movements
const DOWN: Point = Point(0, -1);
const UP: Point = Point(0, 1);
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
    map: HashSet<Point>,
    highest_y: i32,
    movements: VecDeque<Point>,
    current_shape: RockShape,
    x_boundaries: Range<i32>,
}

impl Cave {
    fn new(movements: Vec<Point>) -> Self {
        Self {
            map: HashSet::new(),
            current_shape: RockShape::A,
            highest_y: 0,
            x_boundaries: 0..7,
            movements: VecDeque::from(movements),
        }
    }

    fn spawn_position(&self) -> Point {
        Point::new(0, self.highest_y + 3)
    }

    fn can_move(&self, point: &Point, target: &Point) -> bool {
        if point.1 <= 0 {
            return false;
        }

        !self.map.contains(target)
    }

    fn tick(&mut self) {
        // Spawn shape
        let spawn_pos = self.spawn_position();
        let mut shape = self.current_shape.relative_to(spawn_pos);

        println!("---------------");
        println!("Spawning shape: {:?}", self.current_shape);

        // Move the shape down until it's stable
        while shape.iter().all(|p| self.can_move(p, &DOWN)) {
            println!("Moving down");
            // Apply down movement
            shape.iter_mut().for_each(|p| {
                p.apply(&DOWN);
            });

            // Apply jet movements
            if let Some(jet_mov) = self.movements.pop_front() {
                // Check if can move
                if shape.iter().all(|p| self.can_move(p, &jet_mov)) {
                    // Apply movement
                    println!("Applying jet move");
                    shape.iter_mut().for_each(|p| {
                        p.apply(&jet_mov);
                    });
                }
            }
        }

        // Write blocks
        shape.iter().for_each(|p| {
            self.map.insert(p.clone());
        });

        // Update next shape
        self.current_shape = self.current_shape.next_shape();
    }
}

fn main() {
    let input = include_str!("../example");
    let movements: Vec<Point> = input
        .chars()
        .filter(|it| it.is_ascii_punctuation())
        .map(|it| it.into())
        .collect();

    let mut cave = Cave::new(movements);

    cave.tick();
    dbg!(&cave.map);
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
