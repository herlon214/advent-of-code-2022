use std::collections::HashMap;
use std::collections::VecDeque;
use std::ops::Range;

use crate::point::*;
use crate::sensor::*;

pub struct Scanner {
    pub sensors: Vec<Sensor>,
    pub range_x: Range<i32>,
    pub range_y: Range<i32>,
    pub ranges: HashMap<i32, Vec<Range<i32>>>,
}

impl Scanner {
    pub fn new(sensors: Vec<Sensor>, range_x: Range<i32>, range_y: Range<i32>) -> Self {
        Self {
            sensors,
            range_x,
            range_y,
            ranges: HashMap::new(),
        }
    }

    #[inline]
    pub fn make_pyramid(&mut self, point: &Point, distance: i32, increase: i32) {
        let mut y = point.y();
        let x = point.x();

        // Check for range x
        if !self.range_x.contains(&x) {
            return;
        }

        for distance in (0..=distance).rev() {
            // Check for range y
            if !self.range_y.contains(&y) {
                y += increase;

                continue;
            }

            let row = self.ranges.entry(y).or_insert(vec![]);
            row.push((x - distance)..(x + distance));

            y += increase;
        }
    }

    pub fn mark_non_beacon(&mut self) {
        let sensors: Vec<Sensor> = self.sensors.clone();

        for sensor in sensors {
            // Calculate distance
            let beacon_distance = sensor.position.manhattan_distance(&sensor.closest_beacon) as i32;

            // Going up
            self.make_pyramid(&sensor.position, beacon_distance, -1);

            // Going down
            self.make_pyramid(&sensor.position, beacon_distance, 1);
        }

        // Sort and merge all ranges
        for (_, ranges) in self.ranges.iter_mut() {
            ranges.sort_unstable_by(|a, b| a.start.cmp(&b.start));

            let mut merged: VecDeque<Range<i32>> = VecDeque::new();
            merged.push_back(ranges.get(0).unwrap().clone());

            for range in ranges.iter().skip(1) {
                let prev = merged.back().unwrap();
                if range.start <= prev.end {
                    // Overlap
                    let new = Range {
                        start: prev.start,
                        end: prev.end.max(range.end),
                    };

                    merged.pop_back();
                    merged.push_back(new);
                } else {
                    merged.push_back(range.clone());
                }
            }

            *ranges = Vec::from(merged);
        }
    }

    pub fn hidden_beacon(&self) -> Point {
        let multiple: HashMap<&i32, &Vec<Range<i32>>> = self
            .ranges
            .iter()
            .filter(|(_, ranges)| ranges.len() == 2 && ranges[1].start == ranges[0].end + 2)
            .collect();

        let (y, ranges) = multiple.into_iter().last().unwrap();

        Point::new(ranges[0].end + 1, *y)
    }

    pub fn count_non_beacon(&self, y: i32) -> usize {
        self.ranges
            .get(&y)
            .unwrap()
            .iter()
            .map(|it| it.clone().count())
            .sum()
    }
}
