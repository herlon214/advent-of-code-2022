use crate::instruction::*;

pub struct Crt {
    pub cycle: i32,
    pub cycle_counter_increment: i32,
    pub x: i32,
    pub pixel_lines: Vec<Vec<char>>,
}

impl Crt {
    pub fn new(cycle_counter_increment: i32) -> Self {
        Self {
            x: 1,
            pixel_lines: vec![vec!['.'; cycle_counter_increment as usize]],
            cycle: 0,
            cycle_counter_increment,
        }
    }

    pub fn print(&self) {
        self.pixel_lines
            .chunks(6)
            .map(|screen| {
                let mut output = "".to_string();

                screen.iter().for_each(|items| {
                    output = format!(
                        "{}\n{}",
                        output,
                        items
                            .iter()
                            .map(|it| it.to_string())
                            .collect::<Vec<String>>()
                            .join("")
                    );
                });

                output
            })
            .for_each(|screen| {
                println!("{}", screen);
            })
    }

    pub fn exec(&mut self, instruction: &Instruction) {
        // Tick
        for _ in 0..instruction.cycles {
            self.cycle += 1;

            // Draw
            let line = self.pixel_lines.last_mut().unwrap();

            let offset = self.cycle - 1;
            if offset >= self.x - 1 && offset <= self.x + 1 {
                // Lit pixel
                line[(offset) as usize] = '#';
            }

            // End cycle
            if self.cycle == 40 {
                self.cycle = 0;
                self.pixel_lines
                    .push(vec!['.'; self.cycle_counter_increment as usize]);
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
