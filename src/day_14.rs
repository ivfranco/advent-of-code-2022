use std::{
    collections::{HashSet, VecDeque},
    iter::from_fn,
};

use nom::{
    bytes::complete::tag,
    character::complete::{i64, line_ending},
    combinator::all_consuming,
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

use crate::utils::{Coord, DOWN, LEFT, RIGHT};

pub fn solution(input: &str) -> String {
    let paths = parse(input);
    format!("{}, {}", part_one(&paths), part_two(&paths))
}

#[derive(Debug, Clone, Copy)]
struct Line {
    from: Coord,
    to: Coord,
}

impl Line {
    #[cfg(test)]
    fn len(self) -> i64 {
        let (dx, dy) = (self.from - self.to).to_tuple();
        dx.abs() + dy.abs() + 1
    }

    fn rocks(self) -> impl Iterator<Item = Coord> {
        let mut dot = self.from;
        let (dx, dy) = (self.to - dot).to_tuple();
        let delta = Coord::new(dx.signum(), dy.signum());
        let mut terminated = false;

        from_fn(move || {
            if !terminated {
                let next = dot;
                if dot == self.to {
                    terminated = true;
                }
                dot = dot + delta;
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
    // orthogonal lines
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

    fn floor(&self) -> i64 {
        self.deepest + 2
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

    fn dir(&self, sand: Coord) -> Option<Coord> {
        if !self.contains(&(sand + DOWN)) {
            Some(DOWN)
        } else if !self.contains(&(sand + DOWN + LEFT)) {
            Some(DOWN + LEFT)
        } else if !self.contains(&(sand + DOWN + RIGHT)) {
            Some(DOWN + RIGHT)
        } else {
            None
        }
    }
}

const START: Coord = Coord { x: 500, y: 0 };

fn part_one(paths: &[Path]) -> i64 {
    let mut cave = Cave::new(paths);
    let mut cnt = 0;
    loop {
        let mut sand = START;
        while let Some(dir) = cave.dir(sand) {
            sand = sand + dir;
            if cave.abysmal(sand) {
                return cnt;
            }
        }
        cave.rest(sand);
        cnt += 1;
    }
}

fn part_two(paths: &[Path]) -> usize {
    let cave = Cave::new(paths);
    let mut cnt = 1;

    let mut scan_line: VecDeque<bool> = VecDeque::new();
    scan_line.push_front(true);
    let mut start = START.x;

    // assume coordinate (x, y) will be filled with resting sand
    // 1.   coordinate (x, y + 1) is rock or eventually will be filled with resting sand
    // 2.   because of 1, coordinate (x - 1, y + 1) is rock or eventually will be filled
    // 3.   because of 1, coordinate (x + 1, y + 1) is rock or eventually will be filled
    // 4.   by rules, no other space on depth y will be filled by sand falling from (x, y)
    for y in START.y + 1..cave.floor() {
        let mut next_line = scan_line.clone();
        next_line.push_back(false);
        next_line.push_front(false);
        start -= 1;

        for (i, b) in next_line.iter_mut().enumerate() {
            if i >= 2 {
                *b = *b || scan_line[i - 2];
            }
            if i < scan_line.len() {
                *b = *b || scan_line[i];
            }
            let x = i as i64 + start;
            *b = *b && !cave.contains(&Coord { x, y });
        }

        cnt += next_line.iter().filter(|b| **b).count();
        scan_line = next_line
    }

    cnt
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
            .count() as i64;
        assert_eq!(len_sum, rock_count)
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
