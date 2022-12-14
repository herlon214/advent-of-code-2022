use std::{collections::HashSet, time::Duration};

#[derive(PartialEq, Eq, Debug, Clone)]
struct Block {
    x: usize,
    y: usize,
    kind: BlockKind,
}

impl Block {
    fn tuple(&self) -> (usize, usize) {
        (self.x, self.y)
    }

    fn normalize(&mut self, x: usize, y: usize, padding_shift: usize) {
        self.x -= x;
        self.x += padding_shift;
        self.y -= y;
        self.y += padding_shift;
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
enum BlockKind {
    Rock,
    SandSource,
    SandUnit,
    Air,
}

impl BlockKind {
    fn char(&self) -> char {
        match self {
            BlockKind::Air => '.',
            BlockKind::Rock => '#',
            BlockKind::SandSource => '+',
            BlockKind::SandUnit => 'o',
        }
    }
}

fn parse_line(input: &str) -> Vec<Block> {
    input
        .split(" -> ")
        .into_iter()
        .map(|it| Block::from(it))
        .collect()
}

struct Cave {
    grid: Vec<Vec<BlockKind>>,
    last_tick_produced: usize,
    last_tick_moved: usize,
    ticks: usize,
    void_reached: bool,
    sands_stable: HashSet<(usize, usize)>,
}

impl Cave {
    fn new(rocks: Vec<Vec<Block>>, sand_source: &mut Block, padding: usize) -> Self {
        // Calculate min and max for x,y
        let mut x = (sand_source.x, sand_source.x);
        let mut y = (sand_source.y, sand_source.y);
        let padding_shift = padding / 2;

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

        // Normalize
        let x_norm = x.0;
        let y_norm = y.0;
        let rocks: Vec<Vec<Block>> = rocks
            .into_iter()
            .map(|line| {
                line.into_iter()
                    .map(|mut it| {
                        it.normalize(x_norm, y_norm, padding_shift);

                        it
                    })
                    .collect::<Vec<Block>>()
            })
            .collect();

        // Initialize grid with air blocks
        let m = x.1 - x_norm;
        let n = y.1 - y_norm;
        let mut grid = vec![vec![BlockKind::Air; n + padding]; m + padding];

        // Add sand source
        sand_source.normalize(x_norm, y_norm, padding_shift);
        grid[sand_source.x][sand_source.y] = sand_source.kind.clone();

        // Add rock lines
        rocks.iter().for_each(|lines| {
            lines
                .windows(2)
                .for_each(|pos| match (pos[0].tuple(), pos[1].tuple()) {
                    // Horizontal
                    ((a, b), (c, d)) if a == c => {
                        if b > d {
                            for i in (d..=b).rev() {
                                grid[a][i] = BlockKind::Rock;
                            }
                        } else {
                            for i in b..=d {
                                grid[a][i] = BlockKind::Rock;
                            }
                        }
                    }
                    // Vertical
                    ((a, b), (c, d)) if b == d => {
                        if a > c {
                            for i in (c..=a).rev() {
                                grid[i][b] = BlockKind::Rock;
                            }
                        } else {
                            for i in a..=c {
                                grid[i][b] = BlockKind::Rock;
                            }
                        }
                    }
                    _ => unreachable!("Diagonal lines not supported: {:?} -> {:?}", pos[0], pos[1]),
                });
        });

        Self {
            grid,
            last_tick_moved: 0,
            last_tick_produced: 0,
            ticks: 0,
            void_reached: false,
            sands_stable: HashSet::new(),
        }
    }

    fn stable(&self) -> bool {
        self.last_tick_moved == 0 && self.last_tick_produced == 0 && self.ticks > 0
    }

    fn get_block(&self, x: usize, y: usize) -> Option<&BlockKind> {
        if let Some(cols) = self.grid.get(x) {
            if let Some(block) = cols.get(y) {
                return Some(block);
            }
        }

        None
    }

    fn tick_move(&mut self) {
        let mut swap: Vec<((usize, usize), (usize, usize))> = vec![];
        let mut stable: Vec<(usize, usize)> = vec![];

        // 1st pass move blocks
        self.grid.iter().enumerate().for_each(|(i, cols)| {
            cols.iter().enumerate().for_each(|(j, block)| match block {
                &BlockKind::SandUnit => {
                    // Move down
                    if i + 1 < self.grid.len() {
                        let down = self.get_block(i + 1, j);
                        if let Some(BlockKind::Air) = down {
                            swap.push(((i, j), (i + 1, j)));

                            return;
                        }

                        // Move diagonally left
                        if j > 0 {
                            let diag_left = self.get_block(i + 1, j - 1);
                            if let Some(BlockKind::Air) = diag_left {
                                swap.push(((i, j), (i + 1, j - 1)));

                                return;
                            }
                        }

                        // Move diagonally right
                        if j + 1 < cols.len() {
                            let diag_right = self.get_block(i + 1, j + 1);
                            if let Some(BlockKind::Air) = diag_right {
                                swap.push(((i, j), (i + 1, j + 1)));

                                return;
                            }
                        }
                    }

                    // Stable
                    if self.sands_stable.get(&(i, j)).is_none() {
                        stable.push((i, j));
                    }
                }
                _ => {}
            })
        });

        // Update stable hashset
        stable.iter().for_each(|it| {
            self.sands_stable.insert(*it);
        });

        // Move blocks
        self.last_tick_moved += swap.len();
        let void = self.grid.len() - 1;
        swap.iter().for_each(|(a, b)| {
            // Void reached
            if a.0 == void || b.0 == void {
                self.grid[a.0][a.1] = BlockKind::Air;
                self.void_reached = true;

                // Remove from hashset
                self.sands_stable.remove(a);
            } else {
                let tmp = self.grid[a.0][a.1].clone();
                self.grid[a.0][a.1] = self.grid[b.0][b.1].clone();
                self.grid[b.0][b.1] = tmp;
            }
        });
    }

    fn tick_produce(&mut self) {
        let mut produce: Vec<Block> = vec![];

        // 2st pass generate new blocks
        self.grid.iter().enumerate().for_each(|(i, cols)| {
            cols.iter().enumerate().for_each(|(j, block)| match block {
                &BlockKind::SandSource => {
                    let below = &self.grid[i + 1][j];
                    // Below block is empty, push a new sand unit
                    if matches!(below, BlockKind::Air) {
                        produce.push(Block {
                            x: i + 1,
                            y: j,
                            kind: BlockKind::SandUnit,
                        });
                    }
                }
                _ => {}
            });
        });

        // Produce more blocks
        self.last_tick_produced += produce.len();
        produce.iter().for_each(|block| {
            self.grid[block.x][block.y] = block.kind.clone();
        });
    }

    fn tick(&mut self) {
        // Increment tick
        self.ticks += 1;

        // Reset counters
        self.last_tick_moved = 0;
        self.last_tick_produced = 0;

        self.tick_produce();
        self.tick_move();

        // Move blocks twice before producing a new one
        while self.last_tick_moved > 0 {
            self.last_tick_moved = 0;

            self.tick_move();
        }

        // self.print();
        // std::thread::sleep(Duration::from_millis(50));
        // print!("\x1B[2J\x1B[1;1H");
    }

    fn print(&self) {
        for (i, cols) in self.grid.iter().skip(30).enumerate() {
            let mut line = "".to_string();
            for val in cols {
                line = format!("{}{}", line, val.char());
            }

            println!("{}", line);
            if i == 50 {
                break;
            }
        }
    }
}

fn main() {
    let input = include_str!("../input");
    let positions: Vec<Vec<Block>> = input.lines().map(|it| parse_line(it)).collect();
    let mut cave = Cave::new(
        positions,
        &mut Block {
            x: 0,
            y: 500,
            kind: BlockKind::SandSource,
        },
        5,
    );

    while !cave.stable() && !cave.void_reached {
        cave.tick();
    }

    println!("Stable sands: {}", cave.sands_stable.len());
    cave.print();
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn parse_position() {
        let input: Block = "498,4".into();
        assert_eq!(input.y, 498);
        assert_eq!(input.x, 4);
    }

    #[test]
    fn positions() {
        let input = "498,4 -> 498,6 -> 496,6";
        let positions = parse_line(input);
        assert_eq!(
            positions[0],
            Block {
                y: 498,
                x: 4,
                kind: BlockKind::Rock
            }
        );
        assert_eq!(
            positions[1],
            Block {
                y: 498,
                x: 6,
                kind: BlockKind::Rock
            }
        );
        assert_eq!(
            positions[2],
            Block {
                y: 496,
                x: 6,
                kind: BlockKind::Rock
            }
        );
    }

    #[test]
    fn example() {
        let input = include_str!("../example");
        let positions: Vec<Vec<Block>> = input.lines().map(|it| parse_line(it)).collect();
        let mut cave = Cave::new(
            positions,
            &mut Block {
                x: 0,
                y: 500,
                kind: BlockKind::SandSource,
            },
            2,
        );

        while !cave.stable() && !cave.void_reached {
            cave.tick();
        }

        assert_eq!(cave.sands_stable.len(), 24);
    }

    #[test]
    fn lines() {
        let input = r"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
510,4 -> 510,2
510,4 -> 515,4";
        let positions: Vec<Vec<Block>> = input.lines().map(|it| parse_line(it)).collect();
        let mut cave = Cave::new(
            positions,
            &mut Block {
                x: 0,
                y: 500,
                kind: BlockKind::SandSource,
            },
            2,
        );

        while !cave.stable() && !cave.void_reached {
            cave.tick();
        }

        cave.print();

        panic!("asd");
    }
}
