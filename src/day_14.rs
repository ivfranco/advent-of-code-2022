use std::{collections::HashSet, iter::from_fn, ops::Add};

use nom::{
    bytes::complete::tag,
    character::complete::{i64, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

pub fn solution(input: &str) -> String {
    let paths = parse(input);
    format!("{}, {}", part_one(&paths), part_two(&paths))
}

const DOWN: Coord = Coord { x: 0, y: 1 };
const LEFT: Coord = Coord { x: -1, y: 0 };
const RIGHT: Coord = Coord { x: 1, y: 0 };

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i64,
    y: i64,
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Coord {
    fn to_tuple(self) -> (i64, i64) {
        (self.x, self.y)
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    from: Coord,
    to: Coord,
}

impl Line {
    #[allow(dead_code)]
    fn len(self) -> i64 {
        let (x0, y0) = self.from.to_tuple();
        let (x1, y1) = self.to.to_tuple();
        let (dx, dy) = (x0 - x1, y0 - y1);
        dx.abs() + dy.abs() + 1
    }

    fn rocks(self) -> impl Iterator<Item = Coord> {
        let (mut x, mut y) = self.from.to_tuple();
        let (x1, y1) = self.to.to_tuple();
        let (dx, dy) = ((x1 - x).signum(), (y1 - y).signum());
        let mut terminated = false;

        from_fn(move || {
            if !terminated {
                let next = Coord { x, y };
                if x == x1 && y == y1 {
                    terminated = true;
                }
                x += dx;
                y += dy;
                Some(next)
            } else {
                None
            }
        })
    }
}

struct Path {
    vertices: Vec<Coord>,
}

impl Path {
    fn lines(&self) -> impl Iterator<Item = Line> + '_ {
        self.vertices
            .iter()
            .zip(self.vertices.iter().skip(1))
            .map(|(from, to)| Line {
                from: *from,
                to: *to,
            })
    }
}

fn parse(input: &str) -> Vec<Path> {
    let (_, paths) = all_consuming(p_paths)(input).expect("valid complete parse");
    assert!(paths.iter().all(|p| p.lines().all(|l| {
        let (x0, y0) = l.from.to_tuple();
        let (x1, y1) = l.to.to_tuple();
        x0 == x1 || y0 == y1
    })));
    paths
}

fn p_paths(input: &str) -> IResult<&str, Vec<Path>> {
    separated_list1(line_ending, p_path)(input)
}

fn p_path(input: &str) -> IResult<&str, Path> {
    separated_list1(tag(" -> "), p_coord)
        .map(|vertices| Path { vertices })
        .parse(input)
}

fn p_coord(input: &str) -> IResult<&str, Coord> {
    separated_pair(i64, tag(","), i64)
        .map(|(x, y)| Coord { x, y })
        .parse(input)
}

// "trait alias"
trait Checker: Fn(&Cave, &Coord) -> bool {}
impl<P: Fn(&Cave, &Coord) -> bool> Checker for P {}

struct Cave {
    rocks: HashSet<Coord>,
    sands: HashSet<Coord>,
    deepest: i64,
}

impl Cave {
    fn new(paths: &[Path]) -> Self {
        let rocks: HashSet<_> = paths
            .iter()
            .flat_map(|p| p.lines())
            .flat_map(|l| l.rocks())
            .collect();

        let deepest = rocks.iter().map(|c| c.y).max().unwrap();

        Self {
            rocks,
            sands: HashSet::new(),
            deepest,
        }
    }

    fn rest(&mut self, sand: Coord) {
        self.sands.insert(sand);
    }

    fn abysmal(&self, sand: Coord) -> bool {
        sand.y >= self.deepest
    }

    fn contains(&self, coord: &Coord) -> bool {
        self.rocks.contains(coord) || self.sands.contains(coord)
    }

    fn contains_with_floor(&self, coord: &Coord) -> bool {
        self.contains(coord) || coord.y >= self.deepest + 2
    }

    fn dir<P: Checker>(&self, sand: Coord, checker: P) -> Option<Coord> {
        if !checker(self, &(sand + DOWN)) {
            Some(DOWN)
        } else if !checker(self, &(sand + DOWN + LEFT)) {
            Some(DOWN + LEFT)
        } else if !checker(self, &(sand + DOWN + RIGHT)) {
            Some(DOWN + RIGHT)
        } else {
            None
        }
    }

    fn dir_with_abyss(&self, sand: Coord) -> Option<Coord> {
        self.dir(sand, Cave::contains)
    }

    fn dir_with_floor(&self, sand: Coord) -> Option<Coord> {
        self.dir(sand, Cave::contains_with_floor)
    }
}

const START: Coord = Coord { x: 500, y: 0 };

fn part_one(paths: &[Path]) -> i64 {
    let mut cave = Cave::new(paths);
    let mut cnt = 0;
    loop {
        let mut sand = START;
        while let Some(dir) = cave.dir_with_abyss(sand) {
            sand = sand + dir;
            if cave.abysmal(sand) {
                return cnt;
            }
        }
        cave.rest(sand);
        cnt += 1;
    }
}

fn part_two(paths: &[Path]) -> i64 {
    let mut cave = Cave::new(paths);
    let mut cnt = 0;
    loop {
        let mut sand = Coord { x: 500, y: 0 };
        while let Some(dir) = cave.dir_with_floor(sand) {
            sand = sand + dir;
        }
        cave.rest(sand);
        cnt += 1;
        if sand == START {
            return cnt;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn len_and_rocks_match_in_number() {
        let paths = parse(INPUT);
        let len_sum = paths
            .iter()
            .flat_map(|p| p.lines())
            .map(|l| l.len())
            .sum::<i64>();
        let rock_count = paths
            .iter()
            .flat_map(|p| p.lines())
            .flat_map(|l| l.rocks())
            .count();
        assert_eq!(len_sum as usize, rock_count)
    }

    #[test]
    fn example_part_one() {
        let paths = parse(INPUT);
        assert_eq!(part_one(&paths), 24);
    }

    #[test]
    fn example_part_two() {
        let paths = parse(INPUT);
        assert_eq!(part_two(&paths), 93);
    }
}
