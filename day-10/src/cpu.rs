use crate::instruction::*;

pub struct Cpu {
    pub cycle: usize,
    pub cycle_counter: usize,
    pub cycle_counter_increment: usize,
    pub x: i32,
    pub strength_measurements: Vec<i32>,
    pub sprites: Vec<Vec<char>>,
}

impl Cpu {
    pub fn new(cycle_counter: usize, cycle_counter_increment: usize) -> Self {
        Self {
            cycle: 0,
            x: 1,
            strength_measurements: vec![],
            sprites: vec![vec!['.'; cycle_counter]],
            cycle_counter,
            cycle_counter_increment,
        }
    }

    pub fn strength(&self) -> i32 {
        self.x * self.cycle as i32
    }

    pub fn exec(&mut self, instruction: &Instruction) {
        // Tick
        for _ in 0..instruction.cycles {
            self.cycle_counter -= 1;
            self.cycle += 1;

            // Measure strength
            if self.cycle_counter == 0 {
                self.strength_measurements.push(self.strength());
                self.cycle_counter = self.cycle_counter_increment;
            }
        }

        // Perform operation
        match (
            instruction.op.clone(),
            instruction.register.clone(),
            instruction.value,
        ) {
            (Op::NoOp, _, _) => {}
            (Op::Add, Some(Register::X), Some(val)) => {
                self.x += val;
            }
            (op, _, _) => unreachable!("Operation not implemented: {:?}", op),
        }
    }
}
