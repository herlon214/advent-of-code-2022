use std::ops::Range;

mod point;
mod scanner;
mod sensor;

use scanner::*;
use sensor::*;

fn main() {
    let input = include_str!("../input");
    let sensors: Vec<Sensor> = input.lines().map(|it| it.into()).collect();

    // Part 1
    let mut scanner = Scanner::new(
        sensors.clone(),
        Range {
            start: 0,
            end: 4_000_000,
        },
        Range {
            start: 0,
            end: 4_000_000,
        },
    );
    scanner.mark_non_beacon();

    println!("Non-beacon: {}", scanner.count_non_beacon(2_000_000));

    println!(
        "Hidden beacon: {:?}",
        scanner.hidden_beacon().tuning_frequency()
    );
}

#[cfg(test)]
mod tests {
    use super::point::*;
    use super::*;

    #[test]
    fn calc_distance() {
        let a = Point::new(8, 7);
        let b = Point::new(2, 10);

        assert_eq!(a.manhattan_distance(&b), 9);
    }

    #[test]
    fn parse_sensor() {
        let sensor: Sensor = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15".into();
        assert_eq!(sensor, Sensor::new(Point::new(2, 18), Point::new(-2, 15)));
    }

    #[test]
    fn sensors_min_max() {
        let input = include_str!("../example");
        let sensors: Vec<Sensor> = input.lines().map(|it| it.into()).collect();
        assert_eq!(sensors.min_max(), (Point::new(-2, 0), Point::new(25, 22)));
    }

    #[test]
    fn example_count() {
        let input = include_str!("../example");
        let sensors: Vec<Sensor> = input.lines().map(|it| it.into()).collect();
        let mut scanner = Scanner::new(
            sensors,
            Range {
                start: i32::MIN,
                end: i32::MAX,
            },
            Range { start: 10, end: 11 },
        );
        scanner.mark_non_beacon();

        assert_eq!(scanner.count_non_beacon(10), 26);
    }

    #[test]
    fn example_hidden_beacon() {
        let input = include_str!("../example");
        let sensors: Vec<Sensor> = input.lines().map(|it| it.into()).collect();
        let mut scanner = Scanner::new(
            sensors,
            Range {
                start: i32::MIN,
                end: 21,
            },
            Range {
                start: i32::MIN,
                end: 21,
            },
        );
        scanner.mark_non_beacon();
        let hidden_beacon = scanner.hidden_beacon();

        assert_eq!(hidden_beacon.tuning_frequency(), 56000011);
    }
}
