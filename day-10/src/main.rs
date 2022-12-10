use std::collections::VecDeque;

#[derive(Clone, PartialEq, Eq, Debug)]
enum Op {
    NoOp = 1,
    Add = 2,
}

#[derive(PartialEq, Eq, Debug, Clone)]
enum Register {
    X,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Instruction {
    op: Op,
    register: Option<Register>,
    value: Option<i32>,
    cycles: usize,
}

impl From<&str> for Instruction {
    fn from(input: &str) -> Self {
        let parts: Vec<&str> = input.split(" ").collect();
        let op = match *parts.get(0).unwrap() {
            "noop" => Op::NoOp,
            "addx" => Op::Add,
            other => unreachable!("Invalid operation: {}", other),
        };
        let cycles = op.clone() as usize;
        let (register, value) = match (parts.get(0).unwrap(), parts.get(1)) {
            (op, Some(val)) => {
                if op.ends_with("x") {
                    (Some(Register::X), Some(val.parse::<i32>().unwrap()))
                } else {
                    (None, None)
                }
            }
            (_, None) => (None, None),
        };

        Self {
            op,
            register,
            value,
            cycles,
        }
    }
}

struct Cpu {
    cycle: usize,
    cycle_counter: usize,
    x: i32,
    instructions: Vec<Instruction>,
    strength_measurements: Vec<i32>,
}

impl Cpu {
    fn new() -> Self {
        Self {
            cycle: 0,
            cycle_counter: 20, // 1st strength measurement at cycle 20
            x: 1,
            instructions: vec![],
            strength_measurements: vec![],
        }
    }

    fn strength(&self) -> i32 {
        self.x * self.cycle as i32
    }

    fn exec(&mut self, instruction: Instruction) {
        // Tick
        for _ in 0..instruction.cycles {
            self.cycle_counter -= 1;
            self.cycle += 1;

            // Measure strength
            if self.cycle_counter == 0 {
                self.strength_measurements.push(self.strength());
                self.cycle_counter = 40;
            }
        }

        // Perform operation
        match (instruction.op, instruction.register, instruction.value) {
            (Op::NoOp, _, _) => {}
            (Op::Add, Some(Register::X), Some(val)) => {
                self.x += val;
            }
            (op, _, _) => unreachable!("Operation not implemented: {:?}", op),
        }
    }
}

fn main() {
    let input = include_str!("../input");
    let instructions: VecDeque<Instruction> =
        input.lines().into_iter().map(|line| line.into()).collect();

    // Part 1
    let mut cpu = Cpu::new();
    instructions.iter().for_each(|it| {
        cpu.exec(it.clone());
    });

    println!("Measurements: {:?}", cpu.strength_measurements);

    println!(
        "Sum of 20th, 60th, 100th, 140th, 180th, and 220th cycles: {}",
        cpu.strength_measurements.iter().take(6).sum::<i32>()
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_instructions() {
        let instruction: Instruction = "noop".into();
        assert_eq!(
            instruction,
            Instruction {
                op: Op::NoOp,
                register: None,
                value: None,
                cycles: 1
            }
        );

        let instruction: Instruction = "addx 5".into();
        assert_eq!(
            instruction,
            Instruction {
                op: Op::Add,
                register: Some(Register::X),
                value: Some(5),
                cycles: 2,
            }
        );
    }

    #[test]
    fn example_1() {
        let input = r"noop
addx 3
addx -5";
        let mut cpu = Cpu::new();
        let mut instructions: VecDeque<Instruction> =
            input.lines().into_iter().map(|line| line.into()).collect();

        assert_eq!(cpu.strength(), 0);
        assert_eq!(cpu.cycle, 0);
        assert_eq!(cpu.x, 1);

        // No-Op
        cpu.exec(instructions.pop_front().unwrap());
        assert_eq!(cpu.strength(), 1);
        assert_eq!(cpu.cycle, 1);
        assert_eq!(cpu.x, 1);

        // Addx 3
        cpu.exec(instructions.pop_front().unwrap());
        assert_eq!(cpu.strength(), 12);
        assert_eq!(cpu.cycle, 3);
        assert_eq!(cpu.x, 4);

        // Addx -5
        cpu.exec(instructions.pop_front().unwrap());
        assert_eq!(cpu.strength(), -5);
        assert_eq!(cpu.cycle, 5);
        assert_eq!(cpu.x, -1);
    }

    #[test]
    fn example_2() {
        let input = include_str!("../example");

        let mut cpu = Cpu::new();
        let mut instructions: VecDeque<Instruction> =
            input.lines().into_iter().map(|line| line.into()).collect();

        while instructions.len() > 0 {
            let instruction = instructions.pop_front().unwrap();

            cpu.exec(instruction);
        }

        let measurements: i32 = cpu.strength_measurements.iter().sum();
        assert_eq!(measurements, 13140);
    }
}
