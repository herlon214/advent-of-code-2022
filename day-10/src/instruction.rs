#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Op {
    NoOp = 1,
    Add = 2,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Register {
    X,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Instruction {
    pub op: Op,
    pub register: Option<Register>,
    pub value: Option<i32>,
    pub cycles: usize,
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
