use std::{collections::HashSet, iter::from_fn};

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult, Parser,
};
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::utils::Coord;

pub fn solution(input: &str) -> String {
    let reports = parse(input);
    format!(
        "{}, {}",
        part_one(&reports, 2_000_000),
        part_two(&reports, 0, 4_000_000)
    )
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Closed {
    start: i64,
    end: i64,
}

impl Closed {
    fn new(start: i64, end: i64) -> Self {
        Self { start, end }
    }

    fn len(self) -> i64 {
        self.end - self.start + 1
    }

    fn contains(self, x: i64) -> bool {
        self.start <= x && x <= self.end
    }

    fn connect(self, other: Self) -> Option<Self> {
        assert!(self.start <= other.start);

        if other.start <= self.end + 1 {
            Some(Closed::new(self.start, self.end.max(other.end)))
        } else {
            None
        }
    }

    fn covering(self, other: Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }
}

struct Report {
    sensor: Coord,
    beacon: Coord,
}

impl Report {
    fn cover_at(&self, y: i64) -> Option<Closed> {
        let d_beacon = self.sensor.manhattan_distance(self.beacon);
        let d_y = (self.sensor.y - y).abs();
        let r = d_beacon - d_y;

        if r >= 0 {
            Some(Closed {
                start: self.sensor.x - r,
                end: self.sensor.x + r,
            })
        } else {
            None
        }
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
    preceded(
        tag("Sensor at "),
        separated_pair(p_coord, tag(": closest beacon is at "), p_coord),
    )
    .map(|(sensor, beacon)| Report { sensor, beacon })
    .parse(input)
}

fn p_coord(input: &str) -> IResult<&str, Coord> {
    let (input, _) = tag("x=")(input)?;
    let (input, x) = complete::i64(input)?;
    let (input, _) = tag(", y=")(input)?;
    let (input, y) = complete::i64(input)?;
    Ok((input, Coord::new(x, y)))
}

fn cover_at(reports: &[Report], y: i64) -> Vec<Closed> {
    let mut covers: Vec<_> = reports.iter().filter_map(|r| r.cover_at(y)).collect();
    covers.sort_by_key(|c| c.start);

    let mut i = 0;
    let mut connected = vec![covers[i]];

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

fn spots(min: i64, max: i64, connected: &[Closed]) -> impl Iterator<Item = i64> + '_ {
    let mut x = min;
    let mut i = connected
        .iter()
        .position(|c| c.contains(min) || c.start >= min)
        .unwrap_or(connected.len());

    from_fn(move || {
        while i < connected.len() && connected[i].contains(x) {
            x = connected[i].end + 1;
            i += 1;
        }
        if x <= max {
            let next = Some(x);
            x += 1;
            next
        } else {
            None
        }
    })
}

fn part_two(reports: &[Report], min: i64, max: i64) -> i64 {
    let y = (min..=max)
        .into_par_iter()
        .find_first(|y| {
            let connected = cover_at(reports, *y);
            !connected.iter().any(|c| c.covering(Closed::new(min, max)))
        })
        .unwrap();

    let connected = cover_at(reports, y);
    let x = spots(min, max, &connected).next().unwrap();
    x * 4_000_000 + y
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
        assert_eq!(part_two(&reports, 0, 20), 56000011);
    }
}
