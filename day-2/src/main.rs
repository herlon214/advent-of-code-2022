#[derive(Debug, Clone)]
enum Shape {
    Rock,
    Paper,
    Scissor,
}

impl Shape {
    fn value(&self) -> usize {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissor => 3,
        }
    }
}

enum Player {
    One,
    Two,
}

fn parse_shape(input: &str, player: Player) -> Shape {
    return match (player, input) {
        (Player::One, "A") => Shape::Rock,
        (Player::One, "B") => Shape::Paper,
        (Player::One, "C") => Shape::Scissor,
        (Player::Two, "X") => Shape::Rock,
        (Player::Two, "Y") => Shape::Paper,
        (Player::Two, "Z") => Shape::Scissor,
        _ => panic!("unknown"),
    };
}

fn parse_round(line: &str) -> (Shape, Shape) {
    let parts: Vec<&str> = line.split(' ').collect();
    let shape_one = parse_shape(parts.get(0).unwrap(), Player::One);
    let shape_two = parse_shape(parts.get(1).unwrap(), Player::Two);

    (shape_one, shape_two)
}

fn calc_score(input: (Shape, Shape)) -> (usize, usize) {
    let score = match input {
        (Shape::Rock, Shape::Paper) => (0, 6),
        (Shape::Rock, Shape::Scissor) => (6, 0),
        (Shape::Paper, Shape::Rock) => (6, 0),
        (Shape::Paper, Shape::Scissor) => (0, 6),
        (Shape::Scissor, Shape::Rock) => (0, 6),
        (Shape::Scissor, Shape::Paper) => (6, 0),
        _ => (3, 3),
    };

    return (score.0 + input.0.value(), score.1 + input.1.value());
}

fn decision(input: (Shape, Shape)) -> (Shape, Shape) {
    let choice = match input {
        // Lose (X => Rock)
        (Shape::Rock, Shape::Rock) => Shape::Scissor,
        (Shape::Paper, Shape::Rock) => Shape::Rock,
        (Shape::Scissor, Shape::Rock) => Shape::Paper,

        // Draw (Y => Paper)
        (_, Shape::Paper) => input.0.clone(),

        // Win (Z => Scissor)
        (Shape::Rock, Shape::Scissor) => Shape::Paper,
        (Shape::Paper, Shape::Scissor) => Shape::Scissor,
        (Shape::Scissor, Shape::Scissor) => Shape::Rock,
    };

    return (input.0, choice);
}

fn main() {
    let input = include_str!("../input");
    let mut score_one: usize = 0;
    let mut score_two: usize = 0;

    for line in input.lines() {
        // Part 1
        // let round = parse_round(content);
        let round = decision(parse_round(line));
        let score = calc_score(round);
        score_one += score.0;
        score_two += score.1;
    }

    println!("Player A: {}", score_one);
    println!("Player B (me): {}", score_two);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1() {
        let round_1 = calc_score((Shape::Rock, Shape::Paper));
        let round_2 = calc_score((Shape::Paper, Shape::Rock));
        let round_3 = calc_score((Shape::Scissor, Shape::Scissor));

        assert_eq!(round_1, (1, 8));
        assert_eq!(round_2, (8, 1));
        assert_eq!(round_3, (6, 6));
    }

    #[test]
    fn part_2() {
        let round_1 = calc_score(decision((Shape::Rock, Shape::Paper)));
        let round_2 = calc_score(decision((Shape::Paper, Shape::Rock)));
        let round_3 = calc_score(decision((Shape::Scissor, Shape::Scissor)));

        assert_eq!(round_1, (4, 4));
        assert_eq!(round_2, (8, 1));
        assert_eq!(round_3, (3, 7));
    }
}
