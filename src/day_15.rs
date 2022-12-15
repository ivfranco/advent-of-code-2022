use std::collections::HashSet;

use itertools::iproduct;
use nom::{
    bytes::complete::tag,
    character::complete::{i64, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};

use crate::utils::{Closed, Coord};

pub fn solution(input: &str) -> String {
    let reports = parse(input);
    format!(
        "{}, {}",
        part_one(&reports, 2_000_000),
        path_two(&reports, 0, 4_000_000)
    )
}

struct Report {
    sensor: Coord,
    beacon: Coord,
}

impl Report {
    fn radius(&self) -> i64 {
        self.sensor.manhattan_distance(self.beacon)
    }

    fn contains(&self, c: Coord) -> bool {
        self.sensor.manhattan_distance(c) <= self.radius()
    }

    fn cover_at(&self, y: i64) -> Option<Closed> {
        let d_beacon = self.sensor.manhattan_distance(self.beacon);
        let d_y = (self.sensor.y - y).abs();
        let r = d_beacon - d_y;

        if r >= 0 {
            Some(Closed::new(self.sensor.x - r, self.sensor.x + r))
        } else {
            None
        }
    }

    fn edges(&self) -> [Line; 4] {
        let r = self.radius();
        let (x, y) = self.sensor.to_tuple();
        [
            Line::new_diagonal(x + y + r + 1),
            Line::new_diagonal(x + y - r - 1),
            Line::new_anti_diagonal(y - x - r - 1),
            Line::new_anti_diagonal(y - x + r + 1),
        ]
    }
}

// y = ax + b, a = 1 or -1
#[derive(Clone)]
struct Line {
    a: bool,
    b: i64,
}

impl Line {
    fn new_diagonal(b: i64) -> Self {
        Self { a: false, b }
    }

    fn new_anti_diagonal(b: i64) -> Self {
        Self { a: true, b }
    }

    fn y(&self, x: i64) -> i64 {
        if self.a {
            x + self.b
        } else {
            -x + self.b
        }
    }

    fn intersect(&self, other: &Self) -> Option<Coord> {
        if self.a == other.a {
            return None;
        }

        let diff = if self.a {
            other.b - self.b
        } else {
            self.b - other.b
        };

        if diff % 2 != 0 {
            return None;
        }

        let x = diff / 2;
        let y = self.y(x);
        Some(Coord::new(x, y))
    }
}

fn parse(input: &str) -> Vec<Report> {
    let (_, reports) = all_consuming(p_reports)(input).expect("valid complete parse");
    reports
}

fn p_reports(input: &str) -> IResult<&str, Vec<Report>> {
    separated_list1(line_ending, p_report)(input)
}

fn p_report(input: &str) -> IResult<&str, Report> {
    separated_pair(
        preceded(tag("Sensor at "), p_coord),
        tag(": "),
        preceded(tag("closest beacon is at "), p_coord),
    )
    .map(|(sensor, beacon)| Report { sensor, beacon })
    .parse(input)
}

fn p_coord(input: &str) -> IResult<&str, Coord> {
    separated_pair(
        preceded(tag("x="), i64),
        tag(", "),
        preceded(tag("y="), i64),
    )
    .map(|(x, y)| Coord::new(x, y))
    .parse(input)
}

fn cover_at(reports: &[Report], y: i64) -> Vec<Closed> {
    let mut covers: Vec<Closed> = reports.iter().filter_map(|r| r.cover_at(y)).collect();
    covers.sort_by_key(|c| c.start);

    let mut i = 0;
    let mut connected: Vec<Closed> = vec![covers[0]];

    for cover in covers.into_iter().skip(1) {
        if let Some(c) = connected[i].connect(cover) {
            connected[i] = c;
        } else {
            connected.push(cover);
            i += 1;
        }
    }

    connected
}

fn part_one(reports: &[Report], y: i64) -> i64 {
    let connected = cover_at(reports, y);

    let cover_len = connected.iter().map(|c| c.len()).sum::<i64>();
    let beacons = reports
        .iter()
        .map(|r| r.beacon)
        .filter(|b| b.y == y)
        .collect::<HashSet<_>>();
    cover_len - beacons.len() as i64
}

fn path_two(reports: &[Report], min: i64, max: i64) -> i64 {
    let (anti, orth): (Vec<_>, Vec<_>) = reports.iter().flat_map(|r| r.edges()).partition(|s| s.a);

    let uncovered = iproduct!(anti, orth)
        .filter_map(|(s0, s1)| s0.intersect(&s1))
        .find(|c| {
            c.x >= min
                && c.x <= max
                && c.y >= min
                && c.y <= max
                && reports.iter().all(|r| !r.contains(*c))
        })
        .unwrap();

    uncovered.x * 4_000_000 + uncovered.y
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn complete_parse() {
        parse(INPUT);
    }

    #[test]
    fn report_cover_at() {
        let report = Report {
            sensor: Coord::new(8, 7),
            beacon: Coord::new(2, 10),
        };

        assert_eq!(report.cover_at(10), Some(Closed::new(2, 14)));
    }

    #[test]
    fn example_part_one() {
        let reports = parse(INPUT);
        assert_eq!(part_one(&reports, 10), 26);
    }

    #[test]
    fn example_part_two() {
        let reports = parse(INPUT);
        assert_eq!(path_two(&reports, 0, 20), 56000011);
    }
}
