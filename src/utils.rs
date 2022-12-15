use std::ops::{Add, Sub};

pub const UP: Coord = Coord { x: 0, y: -1 };
pub const DOWN: Coord = Coord { x: 0, y: 1 };
pub const LEFT: Coord = Coord { x: -1, y: 0 };
pub const RIGHT: Coord = Coord { x: 1, y: 0 };

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coord {
    pub x: i64,
    pub y: i64,
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Coord {
    pub fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    pub fn to_tuple(self) -> (i64, i64) {
        (self.x, self.y)
    }

    pub fn manhattan_distance(self, other: Self) -> i64 {
        let (dx, dy) = (self - other).to_tuple();
        dx.abs() + dy.abs()
    }
}
