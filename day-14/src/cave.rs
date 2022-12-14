use crate::block::*;
use std::collections::{HashMap, HashSet};

pub struct Cave {
    pub padding: Padding,
    pub blocks: HashMap<(usize, usize), BlockKind>,
    pub unstable_blocks: HashMap<(usize, usize), BlockKind>,
    pub ground_pos: usize,
    sand_producer: Block,
    pub last_tick_produced: usize,
    pub last_tick_moved: usize,
    pub ticks: usize,
    pub void_reached: bool,
    pub sands_stable: HashSet<(usize, usize)>,
    pub void_action: VoidAction,
}

impl Cave {
    pub fn new(
        rocks: Vec<Vec<Block>>,
        sand_source: Block,
        padding: Padding,
        void_action: VoidAction,
    ) -> Self {
        // Initialize block map
        let mut blocks: HashMap<(usize, usize), BlockKind> = HashMap::new();

        // Calculate min and max for x,y
        let mut x = (sand_source.x, sand_source.x);
        let mut y = (sand_source.y, sand_source.y);

        rocks.iter().flatten().for_each(|it| {
            if it.x < x.0 {
                x.0 = it.x;
            }
            if it.x > x.1 {
                x.1 = it.x;
            }

            if it.y < y.0 {
                y.0 = it.y;
            }
            if it.y > y.1 {
                y.1 = it.y;
            }
        });

        // Add rock lines
        rocks.iter().for_each(|lines| {
            lines
                .windows(2)
                .for_each(|pos| match (pos[0].tuple(), pos[1].tuple()) {
                    // Horizontal
                    ((a, b), (c, d)) if a == c => {
                        if b > d {
                            for i in (d..=b).rev() {
                                blocks.insert((a, i), BlockKind::Rock);
                            }
                        } else {
                            for i in b..=d {
                                blocks.insert((a, i), BlockKind::Rock);
                            }
                        }
                    }
                    // Vertical
                    ((a, b), (c, d)) if b == d => {
                        if a > c {
                            for i in (c..=a).rev() {
                                blocks.insert((i, b), BlockKind::Rock);
                            }
                        } else {
                            for i in a..=c {
                                blocks.insert((i, b), BlockKind::Rock);
                            }
                        }
                    }
                    _ => unreachable!("Diagonal lines not supported: {:?} -> {:?}", pos[0], pos[1]),
                });
        });

        // Ground
        let ground_pos = x.1 + 2;

        Self {
            ground_pos,
            last_tick_moved: 0,
            last_tick_produced: 0,
            ticks: 0,
            padding,
            sand_producer: sand_source.clone(),
            void_reached: false,
            sands_stable: HashSet::new(),
            blocks,
            unstable_blocks: HashMap::new(),
            void_action,
        }
    }

    pub fn stable(&self) -> bool {
        self.last_tick_moved == 0 && self.last_tick_produced == 0 && self.ticks > 0
    }

    fn tick_move(&mut self) {
        let mut swap: Vec<((usize, usize), (usize, usize))> = vec![];
        let mut stable: Vec<((usize, usize), BlockKind)> = vec![];

        self.unstable_blocks.iter().for_each(|(pos, kind)| {
            let x = pos.0;
            let y = pos.1;

            if pos.0 >= self.ground_pos && matches!(self.void_action, VoidAction::Keep) {
                stable.push((*pos, kind.clone()));

                return;
            }

            match kind {
                &BlockKind::SandUnit => {
                    // Move down
                    if let None = self.blocks.get(&(x + 1, y)) {
                        swap.push(((x, y), (x + 1, y)));

                        return;
                    }

                    // Move diagonally left
                    if let None = self.blocks.get(&(x + 1, y - 1)) {
                        swap.push(((x, y), (x + 1, y - 1)));

                        return;
                    }

                    // Move diagonally right
                    if let None = self.blocks.get(&(x + 1, y + 1)) {
                        swap.push(((x, y), (x + 1, y + 1)));

                        return;
                    }

                    // Stable
                    stable.push((*pos, kind.clone()));
                    if self.sands_stable.get(pos).is_none() {
                        self.sands_stable.insert(*pos);
                    }
                }
                _ => {}
            }
        });

        // Remove stable blocks
        stable.iter().for_each(|(pos, kind)| {
            self.unstable_blocks.remove(pos);
            self.blocks.insert(*pos, kind.clone());
        });

        // Move blocks
        self.last_tick_moved += swap.len();
        swap.iter().for_each(|(current, new)| {
            // Void reached
            match &self.void_action {
                VoidAction::Delete => {
                    if new.0 >= self.ground_pos {
                        self.void_reached = true;

                        // Remove
                        self.blocks.remove(&current);
                        self.unstable_blocks.remove(&current);
                        self.sands_stable.remove(current);

                        return;
                    }
                }

                _ => {}
            }

            // Move
            self.unstable_blocks.insert(*new, BlockKind::SandUnit);
            self.unstable_blocks.remove(current);
        });
    }

    fn tick_produce(&mut self) {
        // Produce more sand
        let x = self.sand_producer.x;
        let y = self.sand_producer.y;

        // Current block is empty push a new sand unit
        if let None = self.blocks.get(&(x, y)) {
            self.unstable_blocks.insert((x, y), BlockKind::SandUnit);
            self.last_tick_produced += 1;
        }
    }

    pub fn tick(&mut self) {
        // Increment tick
        self.ticks += 1;

        // Reset counters
        self.last_tick_moved = 0;
        self.last_tick_produced = 0;

        self.tick_produce();
        self.tick_move();

        // Wait for current blocks to become stable
        while self.last_tick_moved > 0 {
            self.last_tick_moved = 0;

            self.tick_move();
        }
    }

    pub fn grid(&self) -> Vec<Vec<char>> {
        // Clone producer
        let mut sand_source = self.sand_producer.clone();

        // Calculate min and max for x,y
        let mut x = (sand_source.x, sand_source.x);
        let mut y = (sand_source.y, sand_source.y);
        let padding_shift = Padding {
            x: self.padding.x / 2,
            y: self.padding.y / 2,
        };

        self.blocks.iter().for_each(|(pos, _)| {
            if pos.0 < x.0 {
                x.0 = pos.0;
            }
            if pos.0 > x.1 {
                x.1 = pos.0;
            }

            if pos.1 < y.0 {
                y.0 = pos.1;
            }
            if pos.1 > y.1 {
                y.1 = pos.1;
            }
        });

        // Normalize
        let x_norm = x.0;
        let y_norm = y.0;

        let mut objects: Vec<((usize, usize), BlockKind)> = vec![];
        self.blocks.iter().for_each(|(pos, kind)| {
            let mut new_pos = pos.clone();
            new_pos.0 -= x_norm;
            new_pos.0 += padding_shift.x;
            new_pos.1 -= y_norm;
            new_pos.1 += padding_shift.y;

            objects.push((new_pos, kind.clone()));
        });

        // Initialize grid with air blocks
        let m = x.1 - x_norm;
        let n = y.1 - y_norm;
        let mut grid = vec![vec!['.'; n + self.padding.y]; m + self.padding.x];

        // Normalize and add sand source
        sand_source.normalize(x_norm, y_norm, padding_shift);
        grid[sand_source.x][sand_source.y] = BlockKind::SandSource.char();

        // Add objects
        objects.iter().for_each(|(pos, kind)| {
            grid[pos.0][pos.1] = kind.char();
        });

        grid
    }

    pub fn print(&self) {
        let grid = self.grid();

        println!("");
        for cols in grid.iter() {
            let mut line = "".to_string();
            for val in cols {
                line = format!("{}{}", line, val);
            }

            println!("{}", line);
        }
    }
}
