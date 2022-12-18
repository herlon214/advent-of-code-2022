use std::collections::{HashMap, HashSet, VecDeque};
use std::ops;

const UP: Point = Point { x: 0, y: 1, z: 0 };
const DOWN: Point = Point { x: 0, y: -1, z: 0 };
const LEFT: Point = Point { x: -1, y: 0, z: 0 };
const RIGHT: Point = Point { x: 1, y: 0, z: 0 };
const FRONT: Point = Point { x: 0, y: 0, z: 1 };
const BACK: Point = Point { x: 0, y: 0, z: -1 };

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

impl Point {
    #[allow(unused)]
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn is_inside(&self, min: &Point, max: &Point) -> bool {
        self.x > min.x
            && self.x < max.x
            && self.y > min.y
            && self.y < max.y
            && self.z > min.z
            && self.z < max.z
    }
}

impl ops::Add<Point> for Point {
    type Output = Point;
    fn add(self, rhs: Point) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl From<&str> for Point {
    fn from(input: &str) -> Self {
        let parts: Vec<&str> = input.split(",").collect();

        Self {
            x: parts[0].parse().unwrap(),
            y: parts[1].parse().unwrap(),
            z: parts[2].parse().unwrap(),
        }
    }
}

// Calculate the visible sides, returning points probably of trapped airs
fn calc_faces(
    target: &Point,
    cubes: &mut HashMap<Point, usize>,
    visited: &mut HashSet<Point>,
) -> HashSet<Point> {
    let mut result: HashSet<Point> = HashSet::new();

    // Check all sides
    let left = target.clone() + LEFT;
    let right = target.clone() + RIGHT;
    let up = target.clone() + UP;
    let down = target.clone() + DOWN;
    let front = target.clone() + FRONT;
    let back = target.clone() + BACK;

    // Current
    let mut current = cubes.get(target).unwrap().clone();

    // Check all sides
    for side in vec![left, right, up, down, front, back] {
        // Check if exist and wasn't visited yet
        match (cubes.get_mut(&side), visited.get(&side)) {
            (Some(other), None) => {
                *other -= 1;
                current -= 1;
            }
            // Probably trapped air
            (None, _) => {
                result.insert(side.clone());
            }
            _ => {}
        }
    }

    // Update
    cubes.insert(target.clone(), current);

    // Add as visited
    visited.insert(target.clone());

    result
}

fn sum_faces(cubes: &HashMap<Point, usize>) -> usize {
    cubes.into_iter().map(|it| it.1).sum()
}

fn extract_boundaries(cubes: &HashSet<Point>) -> (Point, Point) {
    let mut min = Point::new(i32::MAX, i32::MAX, i32::MAX);
    let mut max = Point::new(i32::MIN, i32::MIN, i32::MIN);

    for point in cubes.iter() {
        min.x = min.x.min(point.x);
        min.y = min.y.min(point.y);
        min.z = min.z.min(point.z);

        max.x = max.x.max(point.x);
        max.y = max.y.max(point.y);
        max.z = max.z.max(point.z);
    }

    (min, max)
}

fn count_external_faces(
    cubes: &HashSet<Point>,
    boundaries: (Point, Point),
) -> usize {
    let mut counter = 0;
    let mut visited: HashSet<Point> = HashSet::new();
    let mut queue: VecDeque<Point> = VecDeque::new();
    
    // Add one point outside the real boundaries
    queue.push_back(Point::new(boundaries.1.x, boundaries.1.y, boundaries.1.z));

    // Increase the boundaries, so we can navigate from outside
    let mut boundaries = boundaries.clone();
    let padding = 2;
    boundaries.0.x -= padding;
    boundaries.0.y -= padding;
    boundaries.0.z -= padding;
    boundaries.1.x += padding;
    boundaries.1.y += padding;
    boundaries.1.z += padding;

    while queue.len() > 0 {
        // Get the next item
        let current = queue.pop_front().unwrap();

        // Check if was visited
        if visited.get(&current).is_some() {
            continue;
        }

        // Check all sides
        let left = current.clone() + LEFT;
        let right = current.clone() + RIGHT;
        let up = current.clone() + UP;
        let down = current.clone() + DOWN;
        let front = current.clone() + FRONT;
        let back = current.clone() + BACK;

        // Check all sides
        for side in vec![left, right, up, down, front, back] {
            // If it's a cube, sum the faces
            if cubes.get(&side).is_some() {
                counter += 1;
            } else if side.is_inside(&boundaries.0, &boundaries.1){
                queue.push_back(side);
            }
        }

        // Add as visited
        visited.insert(current);
    }

    counter

}

fn main() {
    let input = include_str!("../input");
    let cubes: HashSet<Point> = input
        .lines()
        .into_iter()
        .map(|it| Point::from(it))
        .collect();
    let mut visited: HashSet<Point> = HashSet::new();

    // Part 1
    let mut cubes_faces: HashMap<Point, usize> = cubes.clone().into_iter().map(|it| (it, 6)).collect();
    for (point, _) in cubes_faces.clone().iter() {
        calc_faces(point, &mut cubes_faces, &mut visited);
    }

    // Sum
    println!("Part 1, sum of showing faces: {}", sum_faces(&cubes_faces));

    // Part 2
    let boundaries = extract_boundaries(&cubes);
    let external_faces = count_external_faces(&cubes, boundaries);
    println!("Part 2, sum of showing faces: {}", external_faces);
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn check_cube() {
        let mut visited: HashSet<Point> = HashSet::new();
        let mut cubes: HashMap<Point, usize> = HashMap::new();

        // Left
        cubes.insert(Point::new(-1, 0, 0), 6);
        // Up
        cubes.insert(Point::new(0, 1, 0), 6);
        // Back
        cubes.insert(Point::new(0, 0, -1), 6);
        // Center
        cubes.insert(Point::new(0, 0, 0), 6);
        // Front
        cubes.insert(Point::new(0, 0, 1), 6);
        // Down
        cubes.insert(Point::new(0, -1, 0), 6);
        // Right
        cubes.insert(Point::new(1, 0, 0), 6);
        // Another on right
        cubes.insert(Point::new(2, 0, 0), 6);

        for (point, _) in cubes.clone().iter() {
            calc_faces(point, &mut cubes, &mut visited);
        }

        let sum: usize = cubes.iter().map(|it| it.1).sum();
        assert_eq!(sum, 34);
    }

    #[test]
    fn parse_point() {
        let input = "2,2,2";
        let point: Point = input.into();

        assert_eq!(point, Point::new(2, 2, 2));
    }

    #[test]
    fn example_boundaries() {
        let input = include_str!("../example");
        let cubes: HashSet<Point> = input
            .lines()
            .into_iter()
            .map(|it| Point::from(it))
            .collect();

        let min = Point::new(1, 1, 1);
        let max = Point::new(3, 3, 6);
        assert_eq!(extract_boundaries(&cubes), (min, max));
    }

    #[test]
    fn example_p1() {
        let input = include_str!("../example");
        let mut cubes: HashMap<Point, usize> = input
            .lines()
            .into_iter()
            .map(|it| (Point::from(it), 6))
            .collect();
        let mut visited: HashSet<Point> = HashSet::new();

        // Check all cubes
        for (point, _) in cubes.clone().iter() {
            calc_faces(point, &mut cubes, &mut visited);
        }

        // Sum
        assert_eq!(sum_faces(&cubes), 64);
    }

    #[test]
    fn example_p2() {
        let input = include_str!("../example");
        let cubes: HashSet<Point> = input
            .lines()
            .into_iter()
            .map(|it| Point::from(it))
            .collect();
        let boundaries = extract_boundaries(&cubes);
        assert_eq!(count_external_faces(&cubes, boundaries), 58);
    }
}
