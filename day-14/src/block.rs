#[derive(Clone)]
pub enum VoidAction {
    Delete,
    Keep,
}

#[derive(Clone)]
pub struct Padding {
    pub x: usize,
    pub y: usize,
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Block {
    pub x: usize,
    pub y: usize,
    pub kind: BlockKind,
}

impl Block {
    pub fn tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    pub fn normalize(&mut self, x: usize, y: usize, padding_shift: Padding) {
        self.x -= x;
        self.x += padding_shift.x;
        self.y -= y;
        self.y += padding_shift.y;
    }
}

impl From<&str> for Block {
    fn from(input: &str) -> Self {
        let input: Vec<&str> = input.split(',').collect();

        Self {
            x: input[1].parse().unwrap(),
            y: input[0].parse().unwrap(),
            kind: BlockKind::Rock,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum BlockKind {
    Rock,
    SandSource,
    SandUnit,
}

impl BlockKind {
    pub fn char(&self) -> char {
        match self {
            BlockKind::Rock => '#',
            BlockKind::SandSource => '+',
            BlockKind::SandUnit => 'o',
        }
    }
}

pub fn parse_line(input: &str) -> Vec<Block> {
    input
        .split(" -> ")
        .into_iter()
        .map(|it| Block::from(it))
        .collect()
}
