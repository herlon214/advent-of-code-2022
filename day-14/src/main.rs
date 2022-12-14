#[derive(PartialEq, Eq, Debug)]
struct Position {
    x: usize,
    y: usize,
}

impl From<&str> for Position {
    fn from(input: &str) -> Self {
        let input: Vec<&str> = input.split(',').collect();

        Self {
            x: input[1].parse().unwrap(),
            y: input[0].parse().unwrap(),
        }
    }
}

fn parse_lines(input: &str) -> Vec<Position> {
    input
        .split(" -> ")
        .into_iter()
        .map(|it| Position::from(it))
        .collect()
}

struct Cave {
    grid: Vec<Vec<char>>,
}

impl Cave {}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_position() {
        let input: Position = "498,4".into();
        assert_eq!(input.y, 498);
        assert_eq!(input.x, 4);
    }

    #[test]
    fn positions() {
        let input = "498,4 -> 498,6 -> 496,6";
        let positions = parse_lines(input);
        assert_eq!(positions[0], Position { y: 498, x: 4 });
        assert_eq!(positions[1], Position { y: 498, x: 6 });
        assert_eq!(positions[2], Position { y: 496, x: 6 });
    }
}
