use std::{collections::HashSet, thread, time::Duration};

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

        println!("\t\tTHE DIFF IS {:?} => tail {:?}", diff, tail);

        match diff {
            // x
            // 0.....T
            // 1......
            // 2.....H
            //  543210 <- y
            //
            // T (50, 200) -> (0, 0)
            // H (50, 202) -> (0, 2)
            // (0, 1)
            (2, 0) => {
                tail.0 -= 1;
            }
            (-2, 0) => {
                tail.0 += 1;
            }
            (0, 2) => {
                tail.1 -= 1;
            }
            (0, -2) => {
                tail.1 += 1;
            }

            // x
            // 0...T..
            // 1.....H
            // 2......
            //  543210 <- y
            //
            // T (50, 203) -> (2, 0)
            // H (48, 204) -> (0, 1)
            // (-1, +1)
            (2, -1) => {
                tail.0 -= 1;
                tail.1 += 1;
            }

            // x
            // 0.....H
            // 1......
            // 2....T.
            //  543210 <- y
            //
            // H (46, 202) -> (0, 0)
            // T (47, 204) -> (1, 2)
            // (-1, -1)
            (1, 2) => {
                tail.0 -= 1;
                tail.1 -= 1;
            }

            // x
            // 0.....T
            // 1......
            // 2....H.
            //  543210 <- y
            //
            // H (47, 204) -> (1, 2)
            // T (46, 202) -> (0, 0)
            // (1, 1)
            (-1, -2) => {
                tail.0 += 1;
                tail.1 += 1;
            }

            // x
            // 0....H.
            // 1......
            // 2.....T
            //  543210 <- y
            //
            // H (48, 202) -> (1, 0)
            // T (47, 204) -> (0, 2)
            // (1,-1)
            (-1, 2) => {
                tail.0 += 1;
                tail.1 -= 1;
            }

            // x
            // 0.....T
            // 1...H..
            // 2......
            //  543210 <- y
            //
            // H (56, 202) -> (2, 1)
            // T (54, 201) -> (0, 0)
            // (+1,+1)
            (-2, -1) => {
                tail.0 += 1;
                tail.1 += 1;
            }

            // x
            // 0....T.
            // 1......
            // 2.....H
            //  543210 <- y
            //
            // H (57, 200) -> (2, 0)
            // T (55, 201) -> (0, 1)
            // (+1,-1)
            (-2, 1) => {
                tail.0 += 1;
                tail.1 -= 1;
            }

            // x
            // 0.....H
            // 1...T..
            // 2......
            //  543210 <- y
            //
            // H (54, 197) -> (0, 0)
            // T (56, 198) -> (2, 1)
            // (-1,-1)
            (2, 1) => {
                tail.0 -= 1;
                tail.1 -= 1;
            }

            // x
            // 0....T.
            // 1......
            // 2.....H
            //  543210 <- y
            //
            // H (53, 198) -> (0, 2)
            // T (54, 196) -> (1, 0)
            // (-1,1)
            (1, -2) => {
                tail.0 -= 1;
                tail.1 += 1;
            }

            // x
            // 0...H..
            // 1......
            // 2.....T
            //  543210 <- y
            //
            // H (302, 298) -> (2, 0)
            // T (300, 300) -> (0, 2)
            // (-1,1)
            (-2, 2) => {
                tail.0 -= 1;
                tail.1 += 1;
            }

            // x
            // 0...H..
            // 1......
            // 2.....T
            //  543210 <- y
            //
            // H (300, 302) -> (0, 2)
            // T (302, 300) -> (2, 0)
            // (-1,1)
            (2, -2) => {
                tail.0 -= 1;
                tail.1 += 1;
            }

            _ => unreachable!("Diff {:?} with head={:?} and tail={:?}", diff, head, tail),
        }

        (tail.0 as usize, tail.1 as usize)
    }
}

#[derive(Clone, Debug, PartialEq)]
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
    tail_counter: HashSet<Point>,
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
            tail_counter: HashSet::new(),
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
            .enumerate()
            .map(|(idx, knot)| {
                if knot.is_tailing(prev) {
                    // println!("\t[TAIL] {:?} is tailing {:?}", &knot, &prev);

                    return *knot;
                }

                let new_position = prev.calc_move(*knot);

                println!("\t[TAIL] Moving from {:?} to {:?}", &knot, new_position);

                prev = new_position;

                if idx == &self.knots.len() - 1 {
                    self.tail_counter.insert(new_position.clone());
                }

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

        if self.grid[self.head.0][self.head.1] == 'H' {
            self.grid[self.head.0][self.head.1] = '.';
        }

        let mut new_position = (self.head.0 as i32, self.head.1 as i32);

        new_position.0 += calc.0;
        new_position.1 += calc.1;

        println!(
            "[HEAD] Moving {:?} from {:?} to {:?}",
            movement.0, &self.head, new_position
        );

        self.head = (new_position.0 as usize, new_position.1 as usize);

        self.tag_position(self.head, 'H');

        self.move_knots();
    }

    fn print(&self) {
        let mut output = "".to_string();

        for i in 0..self.grid.len() {
            let mut line: Vec<String> = vec![];
            for j in 0..self.grid.len() {
                line.push(self.grid[i][j].to_string());
            }

            output = format!("{}\n{}", output, line.join(""));
        }

        std::fs::write("output", output).expect("Unable to write file");
    }
}

fn main() {
    let input = include_str!("../input");

    let mut grid = Grid::new(900, (300, 300), 5);

    let movements = input
        .lines()
        .map(|line| Movement::parse(line))
        .flat_map(|(direction, count)| (0..count).map(move |_| (direction.clone(), 1)))
        .collect::<Vec<_>>();

    for movement in movements {
        grid.print();
        grid.walk(movement);
    }

    grid.tag_position((300, 300), 's');
    grid.tag_position(grid.head, 'H');
    grid.tag_position(grid.knots[0], 'T');
    grid.print();
    println!("Tail visited {} positions", grid.tail_counter.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn small_grid() {
        let grid = Grid::new(10, (0, 0), 2);

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
