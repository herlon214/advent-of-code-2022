// 0 -> Row
// 1 -> Column
type Point = (usize, usize);

// 0 -> Max so far from left to right
// 1 -> Max so far from right to left
type PrecalcRow = Vec<(usize, usize)>;

// 0 -> Max so far from top to bottom
// 1 -> Max so far from bottom to top
type PrecalcColumn = Vec<(usize, usize)>;

fn parse_grid(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|it| {
            it.chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect()
}

fn scenic_score(value: usize, point: Point, grid: &Vec<Vec<usize>>) -> usize {
    let m = grid.len();
    let n = grid[0].len();
    let mut left = 0;
    let mut right = 0;
    let mut top = 0;
    let mut bottom = 0;

    // View on left
    for i in (0..point.1).rev() {
        let current = grid[point.0][i];
        left += 1;

        if current >= value {
            break;
        }
    }

    // View on right
    for i in (point.1 + 1)..n {
        let current = grid[point.0][i];
        right += 1;

        if current >= value {
            break;
        }
    }

    // View on top
    for i in (0..point.0).rev() {
        let current = grid[i][point.1];
        top += 1;

        if current >= value {
            break;
        }
    }

    // View on bottom
    for i in (point.0 + 1)..m {
        let current = grid[i][point.1];
        bottom += 1;

        if current >= value {
            break;
        }
    }

    left * right * top * bottom
}

fn is_visible(
    value: usize,
    point: Point,
    max_grid: &(Vec<PrecalcRow>, Vec<PrecalcColumn>),
) -> bool {
    // All of the trees around the edge of the grid are visible
    if point.0 == 0
        || point.0 == max_grid.0.len() - 1
        || point.1 == 0
        || point.1 == max_grid.1.len() - 1
    {
        return true;
    }

    // Row wise
    let row_before = max_grid.0[point.0][point.1 - 1];
    let row_after = max_grid.0[point.0][point.1 + 1];

    // Column wise
    let column_before = max_grid.1[point.1][point.0 - 1];
    let column_after = max_grid.1[point.1][point.0 + 1];

    // Check if the current value is greater than
    // the calculated so far for column and row
    value > row_before.0 || value > row_after.1 || value > column_before.0 || value > column_after.1
}

// Calculate the max left -> right, right -> left for each row
// and the max top -> bottom, bottom -> top for each col
fn pre_calc(grid: &Vec<Vec<usize>>) -> (Vec<PrecalcRow>, Vec<PrecalcColumn>) {
    let m = grid.len();
    let n = grid[0].len();
    let mut row: Vec<PrecalcRow> = vec![];
    let mut column: Vec<PrecalcColumn> = vec![];

    // Calculate row wise
    for i in 0..m {
        // Left to right
        let mut left: Vec<usize> = vec![];
        let mut max = usize::MIN;
        for j in 0..n {
            max = max.max(grid[i][j]);
            left.push(max);
        }

        // Right to left
        let mut right: Vec<usize> = vec![];
        let mut max = usize::MIN;
        for j in (0..n).rev() {
            max = max.max(grid[i][j]);
            right.push(max);
        }
        right.reverse();

        // Zip values
        let zipped: Vec<(usize, usize)> = left
            .iter()
            .zip(right)
            .map(|(max_left, max_right)| (*max_left, max_right))
            .collect();

        row.push(zipped);
    }

    // Calculate column wise
    for i in 0..n {
        // Top to bottom
        let mut top: Vec<usize> = vec![];
        let mut max = usize::MIN;
        for j in 0..m {
            max = max.max(grid[j][i]);
            top.push(max);
        }

        // Bottom to top
        let mut bottom: Vec<usize> = vec![];
        let mut max = usize::MIN;
        for j in (0..m).rev() {
            max = max.max(grid[j][i]);
            bottom.push(max);
        }
        bottom.reverse();

        // Zip values
        let zipped: Vec<(usize, usize)> = top
            .iter()
            .zip(bottom)
            .map(|(max_top, max_bottom)| (*max_top, max_bottom))
            .collect();

        column.push(zipped);
    }

    (row, column)
}

fn main() {
    let input = include_str!("../input");
    let grid = parse_grid(input);
    let max_grid = pre_calc(&grid);

    // Part 1
    let total: usize = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, val)| is_visible(*val, (i, j), &max_grid))
                .filter(|it| *it)
                .count()
        })
        .sum();
    println!("Total visible trees: {}", total);

    // Part 2
    let highest_scenic: usize = grid
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .map(|(j, val)| scenic_score(*val, (i, j), &grid))
                .collect::<Vec<usize>>()
                .iter()
                .max()
                .unwrap()
                .clone()
        })
        .max()
        .unwrap();

    println!("Highest scenic score: {}", highest_scenic);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"30373
25512
65332
33549
35390";
        let grid = parse_grid(input);
        assert_eq!(grid.len(), 5);
        assert_eq!(grid.get(0).unwrap().len(), 5);
    }

    #[test]
    fn visibility() {
        let input = r"30373
25512
65332
33549
35390";
        let grid = parse_grid(input);
        let max_grid = pre_calc(&grid);

        assert_eq!(true, is_visible(5, (1, 1), &max_grid));
        assert_eq!(true, is_visible(5, (1, 2), &max_grid));
        assert_eq!(false, is_visible(1, (1, 3), &max_grid));
        assert_eq!(true, is_visible(5, (2, 1), &max_grid));
        assert_eq!(false, is_visible(3, (2, 2), &max_grid));
        assert_eq!(true, is_visible(3, (2, 3), &max_grid));
        assert_eq!(false, is_visible(3, (3, 1), &max_grid));
        assert_eq!(true, is_visible(5, (3, 2), &max_grid));
        assert_eq!(false, is_visible(3, (3, 3), &max_grid));
    }

    #[test]
    fn scenic() {
        let input = r"30373
25512
65332
33549
35390";
        let grid = parse_grid(input);
        // assert_eq!(4, scenic_score(5, (1, 2), &grid));
        assert_eq!(8, scenic_score(5, (3, 2), &grid));
    }
}
