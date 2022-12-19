use std::ops::{Add, RangeInclusive, Sub};

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
    pub const fn new(x: i64, y: i64) -> Self {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Closed {
    pub start: i64,
    pub end: i64,
}

impl Closed {
    pub fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(self) -> i64 {
        self.end - self.start + 1
    }

    pub fn contains(self, x: i64) -> bool {
        self.start <= x && x <= self.end
    }

    pub fn intersection(self, other: Self) -> Option<Self> {
        if self.start > other.start {
            return other.intersection(self);
        }

        if self.end >= other.start {
            Some(Closed::new(other.start, self.end.min(other.end)))
        } else {
            None
        }
    }

    pub fn connect(self, other: Self) -> Option<Self> {
        debug_assert!(self.start <= other.start);

        if other.start <= self.end + 1 {
            Some(Closed::new(self.start, self.end.max(other.end)))
        } else {
            None
        }
    }

    pub fn covering(self, other: Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

impl IntoIterator for Closed {
    type Item = i64;
    type IntoIter = RangeInclusive<i64>;

    fn into_iter(self) -> Self::IntoIter {
        self.start..=self.end
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BitSet(u64);

impl BitSet {
    pub const fn new() -> Self {
        Self(0)
    }

    pub const fn from_bits(bits: u64) -> Self {
        Self(bits)
    }

    pub fn insert(&mut self, k: usize) {
        self.0 |= 0b1 << k;
    }

    pub fn contains(&self, k: usize) -> bool {
        self.0 & (0b1 << k) != 0
    }

    pub fn len(&self) -> usize {
        self.0.count_ones() as usize
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn union(&self, other: &Self) -> Self {
        Self(self.0 | other.0)
    }

    pub fn intersection(&self, other: &Self) -> Self {
        Self(self.0 & other.0)
    }

    pub fn is_superset(&self, other: &Self) -> bool {
        self.0 & other.0 == other.0
    }
}

impl Default for BitSet {
    fn default() -> Self {
        Self::new()
    }
}
