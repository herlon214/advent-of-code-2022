use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn as_tuple(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

impl From<&str> for Direction {
    fn from(input: &str) -> Self {
        match input {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Movement(Direction, usize);

impl From<&str> for Movement {
    fn from(input: &str) -> Self {
        let parts: Vec<&str> = input.split(' ').collect();
        let direction: Direction = parts.get(0).unwrap().to_owned().into();
        let amount = parts.get(1).unwrap().parse::<usize>().unwrap();

        Movement(direction, amount)
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Point(i32, i32);
impl Point {
    fn move_to(&mut self, direction: &Direction) {
        let tuple = direction.as_tuple();

        self.0 += tuple.0;
        self.1 += tuple.1;
    }

    fn is_tailed_to(&self, target: &Point) -> bool {
        let diff_x = target.0 - self.0;
        let diff_y = target.1 - self.1;

        match (diff_x, diff_y) {
            // Same
            (0, 0) => true,
            // Right
            (1, 0) => true,
            // Left
            (-1, 0) => true,
            // Top
            (0, 1) => true,
            // Down
            (0, -1) => true,
            // Top left
            (-1, 1) => true,
            // Bottom left
            (-1, -1) => true,
            // Top right
            (1, 1) => true,
            // Bottom right
            (1, -1) => true,
            _ => false,
        }
    }

    fn follow(&mut self, target: &Point) {
        let diff_x = target.0 - self.0;
        let diff_y = target.1 - self.1;

        if self.is_tailed_to(target) {
            return;
        }

        match (diff_x, diff_y) {
            (2, 0) => self.move_to(&Direction::Right),
            (-2, 0) => self.move_to(&Direction::Left),
            (-2, n) if n > 0 => {
                self.move_to(&Direction::Up);
                self.move_to(&Direction::Left);
            }
            (2, n) if n > 0 => {
                self.move_to(&Direction::Up);
                self.move_to(&Direction::Right);
            }
            (2, n) if n < 0 => {
                self.move_to(&Direction::Down);
                self.move_to(&Direction::Right);
            }
            (-2, n) if n < 0 => {
                self.move_to(&Direction::Down);
                self.move_to(&Direction::Left);
            }
            (0, 2) => self.move_to(&Direction::Up),
            (0, -2) => self.move_to(&Direction::Down),
            (1, 2) => {
                self.move_to(&Direction::Right);
                self.move_to(&Direction::Up);
            }
            (1, -2) => {
                self.move_to(&Direction::Right);
                self.move_to(&Direction::Down);
            }
            (-1, 2) => {
                self.move_to(&Direction::Left);
                self.move_to(&Direction::Up);
            }
            (-1, -2) => {
                self.move_to(&Direction::Left);
                self.move_to(&Direction::Down);
            }

            _ => unreachable!("Diff x {} diff y {}", diff_x, diff_y),
        }
    }
}

fn read_input(input: &str, knots_number: usize) -> usize {
    let mut head = Point(0, 0);
    let mut knots = vec![Point(0, 0); knots_number];

    let mut pos: HashSet<Point> = HashSet::new();

    for line in input.lines() {
        let movement: Movement = line.into();
        for _ in 0..movement.1 {
            head.move_to(&movement.0);

            let mut prev = head.clone();
            for i in 0..knots.len() {
                knots[i].follow(&prev);
                prev = knots[i].clone();
            }

            // Store the position of the last knot
            pos.insert(prev.clone());
        }
    }

    pos.len()
}

fn main() {
    let input = include_str!("../input");

    let visited = read_input(input, 1);
    println!("Total visited with 1 knot: {}", visited);

    let visited = read_input(input, 9);
    println!("Total visited with 9 knots: {}", visited);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        let visited = read_input(input, 1);
        assert_eq!(visited, 13);
    }
}
