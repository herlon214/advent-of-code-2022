#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Number(u32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            // Number with number
            (Packet::Number(a), Packet::Number(b)) => {
                return a.cmp(b);
            }

            // List with list
            (Packet::List(a), Packet::List(b)) => {
                return a.cmp(b);
            }

            // Number with list
            (Packet::Number(x), Packet::List(y)) => {
                let a = Packet::List(vec![Packet::Number(*x)]);
                let b = Packet::List(y.clone());

                return a.cmp(&b);
            }
            // List with number
            (Packet::List(x), Packet::Number(y)) => {
                let a = Packet::List(x.clone());
                let b = Packet::List(vec![Packet::Number(*y)]);

                return a.cmp(&b);
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
            // New list
            '[' => {
                // Create a new list
                let new = parse_line(input, current + 1);
                result.push(new.0);
                current = new.1;
            }
            // Parse digits
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
            // End current list
            ']' => break,
            // No-op
            ',' => {}
            _ => panic!("Invalid char: {}", ch),
        }

        current += 1;
    }

    return (Packet::List(result), current);
}

fn parse_packets(input: &str) -> Vec<Packet> {
    let input: Vec<&str> = input.lines().collect();
    let mut packets: Vec<Packet> = vec![];

    input.chunks(3).for_each(|chunk| {
        let a = parse_line(&chunk[0].chars().collect(), 0);
        let b = parse_line(&chunk[1].chars().collect(), 0);

        packets.push(a.0);
        packets.push(b.0);
    });

    packets
}

// Part 1
fn pair_sum(packets: &Vec<Packet>) -> i32 {
    packets
        .chunks(2)
        .enumerate()
        .map(|(i, chunk)| {
            if chunk[0] < chunk[1] {
                return (i + 1) as i32;
            }

            return -1;
        })
        .filter(|it| *it >= 0)
        .sum()
}

// Part 2
fn decoder_key(packets: &Vec<Packet>) -> usize {
    let mut packets = packets.clone();
    let two = parse_line(&"[[2]]".chars().collect(), 0).0;
    let six = parse_line(&"[[6]]".chars().collect(), 0).0;
    packets.push(two.clone());
    packets.push(six.clone());
    packets.sort();

    let two_pos = packets.iter().position(|it| it == &two).unwrap() + 1;
    let six_pos = packets.iter().position(|it| it == &six).unwrap() + 1;

    two_pos * six_pos
}

fn main() {
    // Parse
    let input = include_str!("../input");
    let packets = parse_packets(input);

    // Part 1
    println!("Index sum of sorted pairs: {}", pair_sum(&packets));

    // Part 2
    println!("Decoder key: {}", decoder_key(&packets));
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

        assert_eq!(a < b, true);
        assert_eq!(b < a, false);

        let a = parse_line(&"[2,3,4]".chars().collect(), 0).0;
        let b = parse_line(&"[4]".chars().collect(), 0).0;

        assert_eq!(a < b, true);

        let a = parse_line(&"[9]".chars().collect(), 0).0;
        let b = parse_line(&"[10]".chars().collect(), 0).0;

        assert_eq!(a < b, true);

        let a = parse_line(&"[10]".chars().collect(), 0).0;
        let b = parse_line(&"[9]".chars().collect(), 0).0;

        assert_eq!(a < b, false);
    }

    #[test]
    fn compare_example_pairs() {
        let a = parse_line(&"[1,1,3,1,1]".chars().collect(), 0).0;
        let b = parse_line(&"[1,1,5,1,1]".chars().collect(), 0).0;

        assert_eq!(a < b, true);

        let a = parse_line(&"[[1],[2,3,4]]".chars().collect(), 0).0;
        let b = parse_line(&"[[1],4]".chars().collect(), 0).0;

        assert_eq!(a < b, true);

        let a = parse_line(&"[9]".chars().collect(), 0).0;
        let b = parse_line(&"[[8,7,6]]".chars().collect(), 0).0;

        assert_eq!(a < b, false);

        let a = parse_line(&"[[4,4],4,4]".chars().collect(), 0).0;
        let b = parse_line(&"[[4,4],4,4,4]".chars().collect(), 0).0;

        assert_eq!(a < b, true);

        let a = parse_line(&"[7,7,7,7]".chars().collect(), 0).0;
        let b = parse_line(&"[7,7,7]".chars().collect(), 0).0;

        assert_eq!(a < b, false);

        let a = parse_line(&"[]".chars().collect(), 0).0;
        let b = parse_line(&"[3]".chars().collect(), 0).0;

        assert_eq!(a < b, true);

        let a = parse_line(&"[[[]]]".chars().collect(), 0).0;
        let b = parse_line(&"[[]]".chars().collect(), 0).0;

        assert_eq!(a < b, false);

        let a = parse_line(&"[1,[2,[3,[4,[5,6,7]]]],8,9]".chars().collect(), 0).0;
        let b = parse_line(&"[1,[2,[3,[4,[5,6,0]]]],8,9]".chars().collect(), 0).0;

        assert_eq!(a < b, false);
    }

    #[test]
    fn pair_sum_example() {
        let input = include_str!("../example");
        let packets = parse_packets(input);

        assert_eq!(pair_sum(&packets), 13);
    }

    #[test]
    fn sort_packets_example() {
        let input = include_str!("../example");
        let packets = parse_packets(input);

        assert_eq!(decoder_key(&packets), 140);
    }
}
