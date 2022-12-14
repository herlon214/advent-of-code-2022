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

    fn normalize(&mut self, x: usize, y: usize) {
        self.x -= x;
        self.y -= y;
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
    x_norm: usize,
    y_norm: usize,
}

impl Cave {
    fn new(rocks: Vec<Vec<Block>>, sand_source: &mut Block) -> Self {
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

        // Normalize
        let x_norm = x.0;
        let y_norm = y.0;
        let rocks: Vec<Vec<Block>> = rocks
            .into_iter()
            .map(|line| {
                line.into_iter()
                    .map(|mut it| {
                        it.normalize(x_norm, y_norm);

                        it
                    })
                    .collect::<Vec<Block>>()
            })
            .collect();

        println!("Norm x: {} Norm y: {}", x_norm, y_norm);

        // Initialize grid with air blocks
        let mut grid = vec![vec![BlockKind::Air; y.1 - y_norm + 1]; x.1 - x_norm + 1];

        // Add sand source
        sand_source.normalize(x_norm, y_norm);
        grid[sand_source.x][sand_source.y] = sand_source.kind.clone();

        // Add rock lines
        rocks.iter().for_each(|lines| {
            lines
                .windows(2)
                .for_each(|pos| match (pos[0].tuple(), pos[1].tuple()) {
                    ((a, b), (c, d)) if a == c => {
                        for i in (d..=b).rev() {
                            grid[a][i] = BlockKind::Rock;
                        }
                    }
                    ((a, b), (c, d)) if b == d => {
                        for i in a..=c {
                            grid[i][b] = BlockKind::Rock;
                        }
                    }
                    _ => unreachable!("Diagonal lines not supported: {:?} -> {:?}", pos[0], pos[1]),
                });
        });

        Self {
            grid,
            x_norm,
            y_norm,
        }
    }

    fn produce_block(&mut self, block: &Block) {}

    fn tick(&mut self) {
        let mut produce: Vec<Block> = vec![];
        let mut swap: Vec<((usize, usize), (usize, usize))> = vec![];

        // 1st pass move blocks
        for (i, cols) in self.grid.iter().enumerate() {
            for (j, block) in cols.iter().enumerate() {
                match block {
                    &BlockKind::SandUnit => {
                        if self.grid[i + 1][j] == BlockKind::Air {
                            println!("Moving sand below");
                            swap.push(((i, j), (i + 1, j)));
                        }
                    }
                    _ => {}
                };
            }
        }

        // Move blocks
        swap.iter().for_each(|(a, b)| {
            let tmp = self.grid[a.0][a.1].clone();
            self.grid[a.0][a.1] = self.grid[b.0][b.1].clone();
            self.grid[b.0][b.1] = tmp;
        });

        // 2st pass generate new blocks
        self.grid.iter().enumerate().for_each(|(i, cols)| {
            cols.iter().enumerate().for_each(|(j, block)| match block {
                &BlockKind::SandSource => {
                    if self.grid[i + 1][j] == BlockKind::Air {
                        println!("Produce sand below");
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
        produce.iter().for_each(|block| {
            self.grid[block.x][block.y] = block.kind.clone();
        });
    }

    fn print(&self) {
        for cols in self.grid.iter() {
            let mut line = "".to_string();
            for val in cols {
                line = format!("{}{}", line, val.char());
            }

            println!("{}", line);
        }
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
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
        );
        let ticks = 3;

        for _ in 0..ticks {
            cave.print();
            cave.tick();
        }

        cave.print();

        panic!("asd");
    }
}
