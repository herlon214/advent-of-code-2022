use std::time::Duration;

type Point = (usize, usize);

trait Parser {
    fn parse(input: &str) -> Self;
}

trait Follow {
    fn calc_move(&self, target: Point) -> Point;
    fn is_tailing(&self, target: Point) -> bool;
}

impl Follow for Point {
    fn is_tailing(&self, target: Point) -> bool {
        let horizontal_distance = self.0.abs_diff(target.0);
        let vertical_distance = self.1.abs_diff(target.1);

        match (horizontal_distance, vertical_distance) {
            (0, 0) => true, // overlapping
            (0, 1) => true,
            (1, 0) => true,
            (1, 1) => true,
            _ => false,
        }
    }
    fn calc_move(&self, target: Point) -> Point {
        let head = (target.0 as i32, target.1 as i32);
        let mut tail = (self.0 as i32, self.1 as i32);
        let diff: (i32, i32) = (tail.0 - head.0, tail.1 - head.1);

        println!("-----------");
        println!("Head {:?}", head);
        println!("Tail {:?}", tail);
        println!("Diff {:?}", diff);

        match diff {
            // ..H.T..
            (2, 0) => {
                tail.1 -= 1;
            }

            // ..T.H..
            (-2, 0) => {
                tail.1 += 1;
            }

            // ...T...
            // .......
            // ...H...
            (0, 2) => {
                tail.0 += 1;
            }

            // ...H...
            // .......
            // ...T...
            (0, -2) => {
                tail.0 -= 1;
            }

            // ...T..
            // ......
            // ..H...
            (n, 2) if n > 0 => {
                tail.0 += 1;
                tail.1 -= 1;
            }

            // ..T...
            // H.....
            // ......
            (2, n) if n > 0 => {
                tail.0 += 1;
                tail.1 -= 1;
            }

            // ......
            // H.....
            // ..T...
            (2, n) if n < 0 => {
                tail.0 -= 1;
                tail.1 -= 1;
            }

            // H.....
            // ......
            // .T....
            (n, -2) if n > 0 => {
                tail.0 -= 1;
                tail.1 -= 1;
            }

            // T.....
            // ......
            // .H....
            (n, 2) if n < 0 => {
                tail.0 += 1;
                tail.1 += 1;
            }

            // ......
            // T.....
            // ..H...
            (-2, n) if n > 0 => {
                tail.0 += 1;
                tail.1 += 1;
            }

            // ..H...
            // ......
            // .T....
            (n, -2) if n < 0 => {
                tail.0 -= 1;
                tail.1 += 1;
            }

            // ..H...
            // T.....
            // ......
            (-2, n) if n < 0 => {
                tail.0 -= 1;
                tail.1 += 1;
            }

            _ => unreachable!("Diff {:?}", diff),
        }

        (tail.0 as usize, tail.1 as usize)
    }
}

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

impl Direction {
    fn as_tuple(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
        }
    }
}

type Movement = (Direction, usize);
impl Parser for Movement {
    fn parse(input: &str) -> Movement {
        let parts: Vec<&str> = input.split(' ').collect();
        let direction = match *parts.get(0).unwrap() {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => unreachable!(),
        };
        let amount = parts.get(1).unwrap().parse::<usize>().unwrap();

        (direction, amount)
    }
}

struct Grid {
    grid: Vec<Vec<char>>,
    head: Point,
    knots: Vec<Point>,
    tail_counter: i32,
}

impl Grid {
    // Generate a MxM grid
    fn new(size: usize, start: Point, knots_number: usize) -> Self {
        let mut grid: Vec<Vec<char>> = vec![];

        for _ in 0..size {
            let col: Vec<char> = vec!['.'; size];
            grid.push(col);
        }

        // Tag start
        grid[start.0][start.1] = 's';

        Grid {
            grid,
            head: start,
            knots: vec![start; knots_number],
            tail_counter: 0,
        }
    }

    fn tag_position(&mut self, pos: Point, identifier: char) {
        self.grid[pos.0][pos.1] = identifier
    }

    fn move_knots(&mut self) {
        let mut prev = self.head;

        let knots: Vec<Point> = self
            .knots
            .iter()
            .map(|it| {
                if it.is_tailing(prev) {
                    return *it;
                }

                let new_position = prev.calc_move(*it);
                prev = new_position;

                new_position
            })
            .collect();

        for (i, knot) in knots.iter().rev().enumerate() {
            self.tag_position(*knot, char::from_digit(i as u32, 10).unwrap());
        }

        // Tag last knot
        // let last = knots.last().unwrap();
        // if self.grid[last.0][last.1] != '#' {
        //     self.tag_position(*last, '#');
        //     self.tail_counter += 1;
        // }

        // Update
        self.knots = knots;
    }

    fn walk(&mut self, movement: Movement) {
        let calc = movement.0.as_tuple();

        for _ in 0..movement.1 {
            if self.grid[self.head.0][self.head.1] == 'H' {
                self.grid[self.head.0][self.head.1] = '.';
            }
            let mut new_position = (self.head.0 as i32, self.head.1 as i32);
            new_position.0 += calc.0;
            new_position.1 += calc.1;

            self.head = (new_position.0 as usize, new_position.1 as usize);
            self.tag_position(self.head, 'H');
            // self.move_tail();
            self.move_knots();
        }
    }

    fn print(&self) {
        let mut output = "".to_string();

        for i in 0..self.grid.len() {
            let mut line: Vec<String> = vec![];
            for j in 0..self.grid.len() {
                line.push(self.grid[i][j].to_string());
            }

            output = format!("{}\n{}", output, line.join(" "));
        }

        std::fs::write("output", output).expect("Unable to write file");
    }
}

fn main() {
    let input = include_str!("../input");

    let mut grid = Grid::new(400, (50, 200), 1);
    for line in input.lines() {
        grid.print();
        // std::thread::sleep(Duration::from_secs(1));
        grid.walk(Movement::parse(line));
    }

    grid.tag_position((50, 200), 's');
    grid.tag_position(grid.head, 'H');
    grid.tag_position(grid.knots[0], 'T');
    grid.print();
    println!("Tail visited {} positions", grid.tail_counter);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_grid() {
        let grid = Grid::new(10, (0, 0), 1);

        assert_eq!(grid.grid.len(), 10);
        assert_eq!(grid.grid[0].len(), 10);
        assert_eq!(grid.grid[0][5], '.');
    }

    #[test]
    fn parse_movement_str() {
        assert_eq!(Movement::parse("R 4"), (Direction::Right, 4));
        assert_eq!(Movement::parse("L 10"), (Direction::Left, 10));
        assert_eq!(Movement::parse("D 40"), (Direction::Down, 40));
        assert_eq!(Movement::parse("U 5"), (Direction::Up, 5));
    }

    fn example() {
        let mut grid = Grid::new(7, (5, 1), 0);
        let input = r"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        for line in input.lines() {
            grid.walk(Movement::parse(line));
        }

        grid.print();

        assert_eq!(grid.grid[1][3], '#');
        assert_eq!(grid.grid[1][4], '#');
        assert_eq!(grid.grid[2][5], '#');
        assert_eq!(grid.grid[3][5], '#');
        assert_eq!(grid.grid[3][4], '#');
        assert_eq!(grid.grid[3][3], '#');
        assert_eq!(grid.grid[4][5], '#');
        assert_eq!(grid.grid[5][4], '#');
        assert_eq!(grid.grid[5][3], '#');
        assert_eq!(grid.grid[5][2], '#');
        assert_eq!(grid.grid[5][1], '#');

        assert_eq!(grid.tail_counter, 13);
    }

    #[test]
    fn part_2() {
        let mut grid = Grid::new(100, (50, 50), 9);
        let input = r"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        for line in input.lines() {
            grid.print();
            grid.walk(Movement::parse(line));
        }

        grid.print();
        panic!("asdasd");
    }
}
