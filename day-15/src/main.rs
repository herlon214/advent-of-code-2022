use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

enum DetectionType {
    Sensor,
    Beacon,
    NonBeacon,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point(i32, i32);
impl Point {
    fn manhattan_distance(&self, b: &Point) -> u32 {
        self.0.abs_diff(b.0) + self.1.abs_diff(b.1)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Sensor {
    position: Point,
    closest_beacon: Point,
}

impl Sensor {
    fn new(position: Point, closest_beacon: Point) -> Self {
        Self {
            position,
            closest_beacon,
        }
    }
}

impl From<&str> for Sensor {
    fn from(input: &str) -> Self {
        lazy_static! {
            static ref RE: Regex = Regex::new(
                r"Sensor at x=([0-9\-]+), y=([0-9\-]+): closest beacon is at x=([0-9\-]+), y=([0-9\-]+)"
            )
            .unwrap();
        }

        let captures = RE.captures(input).unwrap();
        let sensor_x = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
        let sensor_y = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();
        let beacon_x = captures.get(3).unwrap().as_str().parse::<i32>().unwrap();
        let beacon_y = captures.get(4).unwrap().as_str().parse::<i32>().unwrap();

        Sensor {
            position: Point(sensor_x, sensor_y),
            closest_beacon: Point(beacon_x, beacon_y),
        }
    }
}

trait MinMax {
    fn min_max(&self) -> (Point, Point);
}

impl MinMax for Vec<Sensor> {
    fn min_max(&self) -> (Point, Point) {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;

        let mut checkpos = |pos: &Point| {
            if pos.0 > max_x {
                max_x = pos.0;
            }
            if pos.1 > max_y {
                max_y = pos.1;
            }
            if pos.0 < min_x {
                min_x = pos.0;
            }
            if pos.1 < min_y {
                min_y = pos.1;
            }
        };

        for sensor in self {
            checkpos(&sensor.position);
            checkpos(&sensor.closest_beacon);
        }

        (Point(min_x, min_y), Point(max_x, max_y))
    }
}

struct Scanner {
    sensors: Vec<Sensor>,
    map: HashMap<Point, DetectionType>,
    non_beacon: HashMap<i32, i32>,
    target_y: i32,
}

impl Scanner {
    fn new(sensors: Vec<Sensor>, target_y: i32) -> Self {
        // Register all positions
        let mut map = HashMap::new();
        sensors.iter().for_each(|it| {
            map.insert(it.position.clone(), DetectionType::Sensor);
            map.insert(it.closest_beacon.clone(), DetectionType::Beacon);
        });

        Self {
            sensors,
            target_y,
            map,
            non_beacon: HashMap::new(),
        }
    }

    fn make_pyramid(&mut self, point: &Point, distance: i32, increase: i32) {
        let mut y = point.1;
        let x = point.0;

        for distance in (0..=distance).rev() {
            if y != self.target_y {
                y += increase;
                continue;
            }

            let mut counter = 0;

            for i in (x - distance)..=(x + distance) {
                let target = Point(i, y);

                match self.map.get(&target) {
                    None => {
                        self.map.insert(target, DetectionType::NonBeacon);

                        counter += 1;
                    }
                    Some(_) => {
                        continue;
                    }
                }
            }

            // Row counter
            let row = self.non_beacon.entry(y).or_insert(0);
            *row = *row + counter;

            y += increase;
        }
    }

    fn mark_non_beacon(&mut self) {
        let sensors: Vec<Sensor> = self.sensors.clone();

        for sensor in sensors {
            // Calculate distance
            let beacon_distance = sensor.position.manhattan_distance(&sensor.closest_beacon) as i32;

            // Going up
            self.make_pyramid(&sensor.position, beacon_distance, -1);

            // Going down
            self.make_pyramid(&sensor.position, beacon_distance, 1);
        }

        println!("Marked all non-beacons");
        dbg!(&self.non_beacon);
    }

    fn count_non_beacon(&self) -> i32 {
        self.non_beacon.get(&self.target_y).unwrap().clone()
    }
}

fn main() {
    let input = include_str!("../input");
    let sensors: Vec<Sensor> = input.lines().map(|it| it.into()).collect();

    let mut scanner = Scanner::new(sensors.clone(), 2000000);
    scanner.mark_non_beacon();

    println!("Non-beacon: {}", scanner.count_non_beacon());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_distance() {
        let a = Point(8, 7);
        let b = Point(2, 10);

        assert_eq!(a.manhattan_distance(&b), 9);
    }

    #[test]
    fn parse_sensor() {
        let sensor: Sensor = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15".into();
        assert_eq!(sensor, Sensor::new(Point(2, 18), Point(-2, 15)));
    }

    #[test]
    fn sensors_min_max() {
        let input = include_str!("../example");
        let sensors: Vec<Sensor> = input.lines().map(|it| it.into()).collect();
        assert_eq!(sensors.min_max(), (Point(-2, 0), Point(25, 22)));
    }

    #[test]
    fn example() {
        let input = include_str!("../example");
        let sensors: Vec<Sensor> = input.lines().map(|it| it.into()).collect();
        let mut scanner = Scanner::new(sensors, 10);
        scanner.mark_non_beacon();

        assert_eq!(scanner.count_non_beacon(), 26);
    }
}
