mod block;
mod cave;

use block::*;
use cave::*;

fn main() {
    let input = include_str!("../input");
    let positions: Vec<Vec<Block>> = input.lines().map(|it| parse_line(it)).collect();

    // Part 1
    let mut cave = Cave::new(
        positions.clone(),
        Block {
            x: 0,
            y: 500,
            kind: BlockKind::SandSource,
        },
        Padding { x: 5, y: 20 },
        VoidAction::Delete,
    );

    while !cave.stable() && !cave.void_reached {
        cave.tick();
    }

    println!("Part 1, stable sands: {}", cave.sands_stable.len());

    // Part 2
    let mut cave = Cave::new(
        positions,
        Block {
            x: 0,
            y: 500,
            kind: BlockKind::SandSource,
        },
        Padding { x: 5, y: 200 },
        VoidAction::Keep,
    );

    while !cave.stable() {
        cave.tick();
    }

    println!("Part 2, stable sands: {}", cave.sands_stable.len());
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
            Block {
                x: 0,
                y: 500,
                kind: BlockKind::SandSource,
            },
            Padding { x: 2, y: 10 },
            VoidAction::Delete,
        );

        while !cave.stable() && !cave.void_reached {
            cave.tick();
        }

        assert_eq!(cave.sands_stable.len(), 24);
    }
}
