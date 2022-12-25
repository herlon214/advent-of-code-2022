use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point(i32, i32);

impl Point {
    fn apply(&self, point: Point) -> Point {
        let mut new_pos = self.clone();
        new_pos.0 += point.0;
        new_pos.1 += point.1;

        new_pos
    }
}

impl From<Direction> for Point {
    fn from(dir: Direction) -> Self {
        match dir {
            Direction::Up => Point(0, -1),
            Direction::Left => Point(-1, 0),
            Direction::Right => Point(1, 0),
            Direction::Down => Point(0, 1),
        }
    }
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(input: char) -> Self {
        match input {
            '>' => Direction::Right,
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            _ => unreachable!("Faile to parse: {}", input),
        }
    }
}

#[derive(Debug, Clone)]
struct Map {
    grid: HashMap<Point, char>,
}

impl Map {
    fn boundaries(&self) -> ((i32, i32), (i32, i32)) {
        let mut x = (i32::MAX, i32::MIN);
        let mut y = (i32::MAX, i32::MIN);

        for (pos, _) in &self.grid {
            x.0 = x.0.min(pos.0);
            x.1 = x.1.max(pos.0);
            y.0 = y.0.min(pos.1);
            y.1 = y.1.max(pos.1);
        }

        (x, y)
    }

    fn print(&self) {
        let (x, y) = self.boundaries();

        let mut result = "".to_string();

        for y in y.0..=y.1 {
            let mut line = "".to_string();

            for x in x.0..=x.1 {
                let pos = Point(x, y);

                match self.grid.get(&pos) {
                    None => {
                        line = format!("{}{}", line, '.');
                    }
                    Some(ch) => {
                        line = format!("{}{}", line, ch);
                    }
                }
            }

            println!("{}", line);
            result = format!("{}\n{}", result, line);
        }

        // std::fs::write("map", result).expect("Unable to write file");
    }

    fn tick(&mut self) {
        let pos = self.grid.clone();

        for (pos, char) in pos.into_iter() {
            match char {
                '>' | '<' | '^' | 'v' => {
                    self.grid.remove(&pos);

                    let dir: Direction = char.into();
                    let new = pos.apply(dir.into());

                    self.grid.insert(pos, '.');
                    self.grid.insert(new, char);
                }
                _ => {}
            };
        }
    }
}

impl From<&str> for Map {
    fn from(input: &str) -> Self {
        let mut grid: HashMap<Point, char> = HashMap::new();
        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().into_iter().for_each(|(x, ch)| {
                grid.insert(Point(x as i32, y as i32), ch);
            })
        });

        Map { grid }
    }
}

fn main() {
    let input = include_str!("../example");
    let mut map: Map = input.into();

    for _ in 0..2 {
        println!("-------------");
        map.print();
        map.tick();
    }
}
