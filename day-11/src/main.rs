mod game;
mod monkey;
mod op;

use game::*;

fn main() {
    let input = include_str!("../input");

    // Part 1
    let mut game = Game::new(20, 3);
    game.read_input(input);
    game.start();

    println!("Monkey bussiness part 1: {}", game.monkey_business());

    // Part 2
    let mut game = Game::new(10000, 1);
    game.read_input(input);
    game.start();

    println!("Monkey bussiness part 2: {}", game.monkey_business());
}

#[cfg(test)]
mod tests {
    use super::*;
    use op::*;

    #[test]
    fn monkey_0() {
        let input = r"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";

        let mut game = Game::new(0, 3);
        game.parse_str(&input);

        // Monkey data
        let monkey = game.monkeys.get_mut(0).unwrap();
        assert_eq!(monkey.test, Test::Divisible(23));
        assert_eq!(monkey.items[0], Item { worry_level: 79 });
        assert_eq!(monkey.items[1], Item { worry_level: 98 });
        assert_eq!(monkey.target_true, 2);
        assert_eq!(monkey.target_false, 3);
        assert_eq!(monkey.op, Operation::Multiply(Some(19)));

        // Inspect
        let inspect = monkey.inspect_items(3);
        assert_eq!(inspect.get(&3).unwrap().get(0).unwrap().worry_level, 500);
        assert_eq!(inspect.get(&3).unwrap().get(1).unwrap().worry_level, 620);
    }

    #[test]
    fn monkey_1() {
        let input = r"Monkey 1:
  Starting items: 69, 99, 95, 62
  Operation: new = old * old
  Test: divisible by 17
    If true: throw to monkey 2
    If false: throw to monkey 5";
        let mut game = Game::new(0, 3);
        game.parse_str(&input);

        let monkey = game.monkeys.get(0).unwrap();
        assert_eq!(monkey.test, Test::Divisible(17));
        assert_eq!(monkey.items[0], Item { worry_level: 69 });
        assert_eq!(monkey.items[1], Item { worry_level: 99 });
        assert_eq!(monkey.items[2], Item { worry_level: 95 });
        assert_eq!(monkey.items[3], Item { worry_level: 62 });
        assert_eq!(monkey.target_true, 2);
        assert_eq!(monkey.target_false, 5);
        assert_eq!(monkey.op, Operation::Multiply(None));
    }

    #[test]
    fn monkey_2() {
        let input = r"Monkey 2:
  Starting items: 59, 81
  Operation: new = old + 8
  Test: divisible by 7
    If true: throw to monkey 4
    If false: throw to monkey 3";
        let mut game = Game::new(0, 3);
        game.parse_str(&input);

        let monkey = game.monkeys.get(0).unwrap();
        assert_eq!(monkey.test, Test::Divisible(7));
        assert_eq!(monkey.items[0], Item { worry_level: 59 });
        assert_eq!(monkey.items[1], Item { worry_level: 81 });
        assert_eq!(monkey.target_true, 4);
        assert_eq!(monkey.target_false, 3);
        assert_eq!(monkey.op, Operation::Add(Some(8)));
    }

    #[test]
    fn example() {
        let input = include_str!("../example");
        let mut game = Game::new(20, 3);

        game.read_input(input);
        game.start();

        assert_eq!(game.monkey_business(), 10605);
    }
}
