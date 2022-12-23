use std::{
    collections::HashMap,
    fmt::{Display, Write},
    str::from_utf8,
};

use crate::utils::{Closed, Coord, DOWN, LEFT, RIGHT, UP};

pub fn solution(input: &str) -> String {
    let groves = parse(input);
    format!("{}, {}", part_one(&groves), part_two(&groves))
}

#[derive(Default, Clone, Copy)]
struct Elf {
    first_direction: usize,
}

impl Elf {
    fn new() -> Self {
        Self::default()
    }
}

#[derive(Clone, Copy)]
enum Propose {
    N,
    S,
    W,
    E,
}
use itertools::Itertools;
use Propose::*;

impl Propose {
    fn adjacent(self) -> [Coord; 3] {
        match self {
            N => [UP, UP + LEFT, UP + RIGHT],
            S => [DOWN, DOWN + LEFT, DOWN + RIGHT],
            W => [LEFT, LEFT + UP, LEFT + DOWN],
            E => [RIGHT, RIGHT + UP, RIGHT + DOWN],
        }
    }

    fn dir(self) -> Coord {
        match self {
            N => UP,
            S => DOWN,
            W => LEFT,
            E => RIGHT,
        }
    }
}

const PROPOSES: [Propose; 4] = [N, S, W, E];

const SURROUNDING: [Coord; 8] = [
    Coord::new(-1, -1),
    Coord::new(-1, 0),
    Coord::new(-1, 1),
    Coord::new(0, -1),
    Coord::new(0, 1),
    Coord::new(1, -1),
    Coord::new(1, 0),
    Coord::new(1, 1),
];

#[derive(Clone)]
struct Grove {
    elves: HashMap<Coord, Elf>,
}

impl Grove {
    fn adjacent_to_one(&self, coord: Coord) -> bool {
        SURROUNDING
            .into_iter()
            .any(|d| self.elves.contains_key(&(coord + d)))
    }

    fn destination(&self, coord: Coord, elf: Elf) -> Option<Coord> {
        let mut proposes = PROPOSES
            .into_iter()
            .cycle()
            .skip(elf.first_direction)
            .take(4);

        proposes
            .find(|p| {
                p.adjacent()
                    .into_iter()
                    .all(|d| !self.elves.contains_key(&(d + coord)))
            })
            .map(|p| p.dir() + coord)
    }

    fn bounding(&self) -> (Closed, Closed) {
        let (min_x, max_x) = self
            .elves
            .keys()
            .map(|c| c.x)
            .minmax()
            .into_option()
            .unwrap();
        let (min_y, max_y) = self
            .elves
            .keys()
            .map(|c| c.y)
            .minmax()
            .into_option()
            .unwrap();

        (Closed::new(min_x, max_x), Closed::new(min_y, max_y))
    }
}

impl Display for Grove {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (x_range, y_range) = self.bounding();

        let mut buffer = vec![vec![b'.'; x_range.len() as usize]; y_range.len() as usize];
        for coord in self.elves.keys() {
            let x = (coord.x - x_range.start) as usize;
            let y = (coord.y - y_range.start) as usize;

            buffer[y][x] = b'#';
        }

        for row in buffer {
            f.write_str(from_utf8(&row).unwrap())?;
            f.write_char('\n')?;
        }

        Ok(())
    }
}

fn parse(input: &str) -> Grove {
    let mut elves = HashMap::new();

    for (r, l) in input.lines().enumerate() {
        for (c, b) in l.bytes().enumerate() {
            if b == b'#' {
                let coord = Coord::new(c as i64, r as i64);
                elves.insert(coord, Elf::new());
            }
        }
    }

    Grove { elves }
}

fn round(grove: &mut Grove) -> bool {
    let mut moved = false;

    let mut proposed = HashMap::new();
    for (&coord, &elf) in grove.elves.iter() {
        let dest = if !grove.adjacent_to_one(coord) {
            coord
        } else if let Some(dest) = grove.destination(coord, elf) {
            dest
        } else {
            coord
        };

        proposed.insert(coord, dest);
    }

    let mut contention: HashMap<Coord, i64> = HashMap::new();
    for &dest in proposed.values() {
        *contention.entry(dest).or_default() += 1;
    }

    let elves = proposed
        .iter()
        .map(|(coord, dest)| {
            let final_dest = if contention[dest] >= 2 { *coord } else { *dest };
            if final_dest != *coord {
                moved = true;
            }

            let mut elf = grove.elves[coord];
            elf.first_direction = (elf.first_direction + 1) % PROPOSES.len();

            (final_dest, elf)
        })
        .collect();
    *grove = Grove { elves };

    moved
}

fn part_one(grove: &Grove) -> i64 {
    let mut grove = grove.clone();

    for _ in 0..10 {
        round(&mut grove);
    }

    let (x_range, y_range) = grove.bounding();
    x_range.len() * y_range.len() - grove.elves.len() as i64
}

fn part_two(grove: &Grove) -> i64 {
    let mut grove = grove.clone();
    let mut r = 0;

    loop {
        let moved = round(&mut grove);
        r += 1;

        if !moved {
            return r;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

    #[test]
    fn example_part_one() {
        let grove = parse(INPUT);
        assert_eq!(part_one(&grove), 110);
    }

    #[test]
    fn example_part_two() {
        let grove = parse(INPUT);
        assert_eq!(part_two(&grove), 20);
    }
}
