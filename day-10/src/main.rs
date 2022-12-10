mod cpu;
mod crt;
mod instruction;

use cpu::*;
use crt::*;
use instruction::*;

fn main() {
    let input = include_str!("../input");
    let instructions: Vec<Instruction> =
        input.lines().into_iter().map(|line| line.into()).collect();

    // Part 1
    let mut cpu = Cpu::new(20, 40);
    instructions.iter().for_each(|it| {
        cpu.exec(it);
    });

    println!(
        "Sum of 20th, 60th, 100th, 140th, 180th, and 220th cycles: {}",
        cpu.strength_measurements.iter().take(6).sum::<i32>()
    );

    // Part 2
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
        let mut cpu = Cpu::new(20, 40);
        let instructions: Vec<Instruction> =
            input.lines().into_iter().map(|line| line.into()).collect();

        assert_eq!(cpu.strength(), 0);
        assert_eq!(cpu.cycle, 0);
        assert_eq!(cpu.x, 1);

        // No-Op
        cpu.exec(instructions.get(0).unwrap());
        assert_eq!(cpu.strength(), 1);
        assert_eq!(cpu.cycle, 1);
        assert_eq!(cpu.x, 1);

        // Addx 3
        cpu.exec(instructions.get(1).unwrap());
        assert_eq!(cpu.strength(), 12);
        assert_eq!(cpu.cycle, 3);
        assert_eq!(cpu.x, 4);

        // Addx -5
        cpu.exec(instructions.get(2).unwrap());
        assert_eq!(cpu.strength(), -5);
        assert_eq!(cpu.cycle, 5);
        assert_eq!(cpu.x, -1);
    }

    #[test]
    fn example_2() {
        let input = include_str!("../example");

        let mut cpu = Cpu::new(20, 40);
        let instructions: Vec<Instruction> =
            input.lines().into_iter().map(|line| line.into()).collect();

        for instruction in instructions {
            cpu.exec(&instruction);
        }

        let measurements: i32 = cpu.strength_measurements.iter().sum();
        assert_eq!(measurements, 13140);
    }

    #[test]
    fn draw() {
        let input = include_str!("../example");

        let mut crt = Crt::new(40);
        let instructions: Vec<Instruction> =
            input.lines().into_iter().map(|line| line.into()).collect();

        for instruction in instructions {
            crt.exec(&instruction);
        }

        for line in crt.pixel_lines {
            println!(
                "{}",
                line.iter()
                    .map(|it| it.to_string())
                    .collect::<Vec<String>>()
                    .join("")
            );
        }

        panic!("Test");
    }
}
