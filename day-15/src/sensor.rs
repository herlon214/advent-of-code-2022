use lazy_static::lazy_static;
use regex::Regex;

use crate::point::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sensor {
    pub position: Point,
    pub closest_beacon: Point,
}

impl Sensor {
    #[allow(unused)]
    pub fn new(position: Point, closest_beacon: Point) -> Self {
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
            position: Point::new(sensor_x, sensor_y),
            closest_beacon: Point::new(beacon_x, beacon_y),
        }
    }
}

pub trait MinMax {
    fn min_max(&self) -> (Point, Point);
}

impl MinMax for Vec<Sensor> {
    fn min_max(&self) -> (Point, Point) {
        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;

        let mut checkpos = |pos: &Point| {
            if pos.x() > max_x {
                max_x = pos.x();
            }
            if pos.y() > max_y {
                max_y = pos.y();
            }
            if pos.x() < min_x {
                min_x = pos.x();
            }
            if pos.y() < min_y {
                min_y = pos.y();
            }
        };

        for sensor in self {
            checkpos(&sensor.position);
            checkpos(&sensor.closest_beacon);
        }

        (Point::new(min_x, min_y), Point::new(max_x, max_y))
    }
}
