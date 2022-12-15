#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Point(i32, i32);
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self(x, y)
    }

    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }

    pub fn manhattan_distance(&self, b: &Point) -> u32 {
        self.0.abs_diff(b.0) + self.1.abs_diff(b.1)
    }

    pub fn tuning_frequency(&self) -> i64 {
        self.0 as i64 * 4000000 + self.1 as i64
    }
}
