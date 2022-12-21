use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Monkey {
    name: String,
    value: Option<i64>,
    lhs: Option<String>,
    rhs: Option<String>,
    op: Option<String>,
}

impl From<&str> for Monkey {
    fn from(input: &str) -> Self {
        let parts: Vec<&str> = input.split(": ").collect();
        let name = parts.get(0).unwrap().to_string();
        let val = parts.get(1).unwrap();

        let parts: Vec<&str> = val.split(" ").collect();
        let mut value: Option<i64> = None;
        let mut lhs: Option<String> = None;
        let mut rhs: Option<String> = None;
        let mut op: Option<String> = None;

        match parts.len() {
            1 => {
                value = Some(parts.get(0).unwrap().parse::<i64>().unwrap());
            }
            3 => {
                lhs = Some(parts.get(0).unwrap().to_string());
                op = Some(parts.get(1).unwrap().to_string());
                rhs = Some(parts.get(2).unwrap().to_string());
            }
            _ => unreachable!("Failed to parse line: {}", input),
        }

        Monkey {
            name,
            value,
            lhs,
            rhs,
            op,
        }
    }
}

fn solve_equation(lhs: i64, op: &str, rhs: i64) -> i64 {
    match op {
        "+" => lhs + rhs,
        "-" => lhs - rhs,
        "*" => lhs * rhs,
        "/" => lhs / rhs,
        _ => unreachable!("Failed to parse operator: {}", op),
    }
}

fn solve_value(monkeys: &HashMap<String, Monkey>, root: &Monkey, human: i64) -> i64 {
    if root.name == "humn" {
        return human;
    }

    match (&root.lhs, &root.op, &root.rhs) {
        (None, None, None) => root.value.unwrap(),
        (Some(lhs), Some(op), Some(rhs)) => {
            let left = solve_value(monkeys, monkeys.get(lhs).unwrap(), human);
            let right = solve_value(monkeys, monkeys.get(rhs).unwrap(), human);

            solve_equation(left, &op, right)
        }
        _ => unreachable!("Failed to parse root: {:?}", root),
    }
}

fn main() {
    let input = include_str!("../input");
    let monkeys: HashMap<String, Monkey> = input
        .lines()
        .map(|it| Monkey::from(it))
        .map(|it| (it.name.clone(), it))
        .collect();

    // Part 1
    let val = solve_value(&monkeys, monkeys.get("root").unwrap(), 5);

    println!("Part 1: {val}");

    // Part 2
    let root = monkeys.get("root").unwrap();
    let mut n = 1i64;
    loop {
        let lhs = monkeys.get(&root.lhs.clone().unwrap()).unwrap();
        let rhs = monkeys.get(&root.rhs.clone().unwrap()).unwrap();

        // Resolve left
        let left = solve_value(&monkeys, lhs, n);

        // Resolve right
        let right = solve_value(&monkeys, rhs, n);

        // Assert match
        if left == right {
            break;
        }

        n += 1;
    }

    println!("Part 2: {n}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = include_str!("../example");
        let monkeys: HashMap<String, Monkey> = input
            .lines()
            .map(|it| Monkey::from(it))
            .map(|it| (it.name.clone(), it))
            .collect();
        let val = solve_value(&monkeys, monkeys.get("root").unwrap());

        assert_eq!(val, 152);
    }

    #[test]
    fn parse_input_raw_number() {
        let input: Monkey = "dbpl: 5".into();
        let expected = Monkey {
            name: "dbpl".to_string(),
            value: Some(5),
            op: None,
            lhs: None,
            rhs: None,
        };

        assert_eq!(input, expected);
    }
    #[test]
    fn parse_input_op() {
        let input: Monkey = "sjmn: drzm * dbpl".into();
        let expected = Monkey {
            name: "sjmn".to_string(),
            value: None,
            op: Some("*".to_string()),
            lhs: Some("drzm".to_string()),
            rhs: Some("dbpl".to_string()),
        };

        assert_eq!(input, expected);
    }
}
