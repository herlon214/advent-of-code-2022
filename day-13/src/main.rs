#[derive(Debug, PartialEq, Clone)]
enum Packet {
    Number(u32),
    List(Vec<Packet>),
}

impl Packet {
    fn lower_than(&self, other: &Packet) -> bool {
        match (self, other) {
            // Number with number
            (Packet::Number(a), Packet::Number(b)) => {
                println!("Comparing {} with {}", *a, *b);

                return *a < *b;
            }

            // List with list
            (Packet::List(a), Packet::List(b)) => {
                let mut left = a.iter();
                let mut right = b.iter();

                let mut decision = false;

                loop {
                    match (left.next(), right.next()) {
                        // Both sides have items
                        (Some(a), Some(b)) => {
                            if a == b {
                                continue;
                            }

                            if !a.lower_than(b) {
                                return false;
                            } else {
                                decision = true;
                            }
                        }
                        // Right side ran out of items
                        (Some(_), None) => return decision,

                        // Left side ran out of items
                        (None, Some(_)) => return true,
                        (_, _) => break,
                    }
                }

                return true;
            }

            // Number with list
            (Packet::Number(x), Packet::List(y)) => {
                let a = Packet::List(vec![Packet::Number(*x)]);
                let b = Packet::List(y.clone());

                return a.lower_than(&b);
            }
            // List with number
            (Packet::List(x), Packet::Number(y)) => {
                let a = Packet::List(x.clone());
                let b = Packet::List(vec![Packet::Number(*y)]);

                return a.lower_than(&b);
            }
        }
    }
}

fn parse_line(input: &Vec<char>, start: usize) -> (Packet, usize) {
    let mut result: Vec<Packet> = vec![];
    let mut current = start;

    while current < input.len() {
        let ch = input[current];

        match ch {
            '[' => {
                // Create a new list
                let new = parse_line(input, current + 1);
                result.push(new.0);
                current = new.1;
            }
            '0'..='9' => {
                let digits: String = input
                    .iter()
                    .skip(current)
                    .take_while(|it| {
                        let predicate = *it != &',' && *it != &']';
                        if predicate {
                            current += 1;
                        }

                        predicate
                    })
                    .collect();

                result.push(Packet::Number(digits.parse().unwrap()));

                continue;
            }
            ']' => break,
            ',' => {}
            _ => panic!("Invalid char: {}", ch),
        }

        current += 1;
    }

    return (Packet::List(result), current);
}

fn pair_sum(input: &str) -> usize {
    let mut sum: usize = 0;
    let input: Vec<&str> = input.lines().collect();

    input.chunks(3).enumerate().for_each(|(i, chunk)| {
        let a = parse_line(&chunk[0].chars().collect(), 0);
        let b = parse_line(&chunk[1].chars().collect(), 0);
        let in_order = a.0.lower_than(&b.0);
        println!(
            "Pair {}: {:?} and {:?} = {}",
            i, chunk[0], chunk[1], in_order
        );
        println!("--------------");

        if in_order {
            sum += i + 1;
        }
    });

    sum
}

fn main() {
    let sum = pair_sum(include_str!("../input"));
    println!("Sum of the indices: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_list() {
        let input = "[1,2,3]";
        let expected = Packet::List(vec![Packet::List(vec![
            Packet::Number(1),
            Packet::Number(2),
            Packet::Number(3),
        ])]);
        let parsed = parse_line(&input.chars().collect(), 0);
        assert_eq!(parsed.0, expected);
    }

    #[test]
    fn nest_1() {
        let input = "[1,[2],3]";
        let expected = Packet::List(vec![Packet::List(vec![
            Packet::Number(1),
            Packet::List(vec![Packet::Number(2)]),
            Packet::Number(3),
        ])]);
        let parsed = parse_line(&input.chars().collect(), 0);
        assert_eq!(parsed.0, expected);
    }

    #[test]
    fn nest_2() {
        let input = "[[1],[2,3,4]]";
        let expected = Packet::List(vec![Packet::List(vec![
            Packet::List(vec![Packet::Number(1)]),
            Packet::List(vec![
                Packet::Number(2),
                Packet::Number(3),
                Packet::Number(4),
            ]),
        ])]);
        let parsed = parse_line(&input.chars().collect(), 0);
        assert_eq!(parsed.0, expected);
    }

    #[test]
    fn compare_simple() {
        let a = parse_line(&"[1,3,5]".chars().collect(), 0).0;
        let b = parse_line(&"[2,4,6]".chars().collect(), 0).0;

        assert_eq!(a.lower_than(&b), true);
        assert_eq!(b.lower_than(&a), false);

        let a = parse_line(&"[2,3,4]".chars().collect(), 0).0;
        let b = parse_line(&"[4]".chars().collect(), 0).0;

        assert_eq!(a.lower_than(&b), true);

        let a = parse_line(&"[9]".chars().collect(), 0).0;
        let b = parse_line(&"[10]".chars().collect(), 0).0;

        assert_eq!(a.lower_than(&b), true);

        let a = parse_line(&"[10]".chars().collect(), 0).0;
        let b = parse_line(&"[9]".chars().collect(), 0).0;

        assert_eq!(a.lower_than(&b), false);
    }

    #[test]
    fn compare_example_pairs() {
        let a = parse_line(&"[1,1,3,1,1]".chars().collect(), 0).0;
        let b = parse_line(&"[1,1,5,1,1]".chars().collect(), 0).0;

        assert_eq!(a.lower_than(&b), true);

        let a = parse_line(&"[[1],[2,3,4]]".chars().collect(), 0).0;
        let b = parse_line(&"[[1],4]".chars().collect(), 0).0;

        assert_eq!(a.lower_than(&b), true);

        let a = parse_line(&"[9]".chars().collect(), 0).0;
        let b = parse_line(&"[[8,7,6]]".chars().collect(), 0).0;

        assert_eq!(a.lower_than(&b), false);

        let a = parse_line(&"[[4,4],4,4]".chars().collect(), 0).0;
        let b = parse_line(&"[[4,4],4,4,4]".chars().collect(), 0).0;

        assert_eq!(a.lower_than(&b), true);

        let a = parse_line(&"[7,7,7,7]".chars().collect(), 0).0;
        let b = parse_line(&"[7,7,7]".chars().collect(), 0).0;

        assert_eq!(a.lower_than(&b), false);

        let a = parse_line(&"[]".chars().collect(), 0).0;
        let b = parse_line(&"[3]".chars().collect(), 0).0;

        assert_eq!(a.lower_than(&b), true);

        let a = parse_line(&"[[[]]]".chars().collect(), 0).0;
        let b = parse_line(&"[[]]".chars().collect(), 0).0;

        assert_eq!(a.lower_than(&b), false);

        let a = parse_line(&"[1,[2,[3,[4,[5,6,7]]]],8,9]".chars().collect(), 0).0;
        let b = parse_line(&"[1,[2,[3,[4,[5,6,0]]]],8,9]".chars().collect(), 0).0;

        assert_eq!(a.lower_than(&b), false);
    }

    #[test]
    fn pair_sum_example() {
        let input = include_str!("../example");

        assert_eq!(pair_sum(input), 13);
    }
}
