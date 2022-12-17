use std::{fmt::Display, str::from_utf8};

use crate::utils::{BitSet, Coord, LEFT, RIGHT};

const DOWN: Coord = Coord::new(0, -1);

pub fn solution(input: &str) -> String {
    let jets = parse(input);
    format!("{}, {}", part_one(&jets), part_two(&jets))
}

#[derive(Clone, Copy)]
enum Jet {
    Left,
    Right,
}

impl Jet {
    fn to_dir(self) -> Coord {
        match self {
            Jet::Left => LEFT,
            Jet::Right => RIGHT,
        }
    }
}

fn parse(input: &str) -> Vec<Jet> {
    input
        .chars()
        .map(|c| match c {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => unreachable!("invalid jet pattern"),
        })
        .collect()
}

// increases along positive x and y axis
const BAR: &[Coord] = &[
    Coord::new(0, 0),
    Coord::new(1, 0),
    Coord::new(2, 0),
    Coord::new(3, 0),
];

const CROSS: &[Coord] = &[
    Coord::new(0, 1),
    Coord::new(1, 0),
    Coord::new(1, 1),
    Coord::new(1, 2),
    Coord::new(2, 1),
];

const CORNER: &[Coord] = &[
    Coord::new(0, 0),
    Coord::new(1, 0),
    Coord::new(2, 0),
    Coord::new(2, 1),
    Coord::new(2, 2),
];

const PILLAR: &[Coord] = &[
    Coord::new(0, 0),
    Coord::new(0, 1),
    Coord::new(0, 2),
    Coord::new(0, 3),
];

const SQUARE: &[Coord] = &[
    Coord::new(0, 0),
    Coord::new(0, 1),
    Coord::new(1, 0),
    Coord::new(1, 1),
];

const ROCKS: [&[Coord]; 5] = [BAR, CROSS, CORNER, PILLAR, SQUARE];

#[derive(Debug, Clone, Copy)]
struct Piece {
    bottom_left: Coord,
    rock: &'static [Coord],
}

impl Piece {
    fn new(bottom_left: Coord, rock: &'static [Coord]) -> Self {
        Self { bottom_left, rock }
    }

    fn coords(self) -> impl Iterator<Item = Coord> {
        self.rock.iter().map(move |c| self.bottom_left + *c)
    }

    fn move_to(self, d: Coord) -> Self {
        Self::new(self.bottom_left + d, self.rock)
    }
}

#[derive(Clone)]
struct Chamber {
    stopped: Vec<BitSet>,
    highest: i64,
}

impl Chamber {
    const WIDTH: i64 = 7;
    const LEFT_BORDER: i64 = 0;
    const BOTTOM_BORDER: i64 = 0;

    fn new() -> Self {
        Self {
            stopped: Vec::new(),
            highest: Self::BOTTOM_BORDER,
        }
    }

    fn translate_coord(&self, c: Coord) -> (usize, usize) {
        (
            (c.x - Self::LEFT_BORDER - 1) as usize,
            (c.y - Self::BOTTOM_BORDER - 1) as usize,
        )
    }

    fn contains(&self, c: Coord) -> bool {
        let (x, y) = self.translate_coord(c);
        if let Some(r) = self.stopped.get(y) {
            r.contains(x)
        } else {
            false
        }
    }

    fn blocked(&self, piece: Piece) -> bool {
        piece.coords().any(|c| {
            c.x <= Self::LEFT_BORDER
                || c.x > Self::LEFT_BORDER + Self::WIDTH
                || c.y <= Self::BOTTOM_BORDER
                || self.contains(c)
        })
    }

    fn stop(&mut self, piece: Piece) {
        debug_assert!(!self.blocked(piece));
        for c in piece.coords() {
            let (x, y) = self.translate_coord(c);
            while self.stopped.len() <= y {
                self.stopped.push(BitSet::new());
            }
            self.stopped[y].insert(x);
            self.highest = self.highest.max(c.y);
        }
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for dy in 0..10 {
            let mut row = [b' '; Self::WIDTH as usize + 2];
            row[Self::LEFT_BORDER as usize] = b'|';
            row[(Self::WIDTH + Self::LEFT_BORDER + 1) as usize] = b'|';
            for x in Self::LEFT_BORDER + 1..=Self::LEFT_BORDER + Self::WIDTH {
                if self.contains(Coord::new(x, self.highest - dy)) {
                    row[x as usize] = b'#';
                }
            }
            f.write_str(from_utf8(row.as_slice()).unwrap())?;
            f.write_str("\n")?;
        }

        Ok(())
    }
}

fn one_piece<J>(chamber: &mut Chamber, rock: &'static [Coord], mut jets: J)
where
    J: Iterator<Item = Jet>,
{
    let mut piece = Piece::new(
        Coord::new(3 + Chamber::LEFT_BORDER, chamber.highest + 4),
        rock,
    );

    loop {
        let dir = jets.next().unwrap().to_dir();
        if !chamber.blocked(piece.move_to(dir)) {
            piece = piece.move_to(dir);
        }

        if chamber.blocked(piece.move_to(DOWN)) {
            break;
        } else {
            piece = piece.move_to(DOWN);
        }
    }

    chamber.stop(piece);
}

fn part_one(jets: &[Jet]) -> i64 {
    let mut jets = jets.iter().cycle().copied();
    let mut rocks = ROCKS.iter().cycle().copied();

    let mut chamber = Chamber::new();

    for _ in 0..2022 {
        one_piece(&mut chamber, rocks.next().unwrap(), &mut jets);
    }

    chamber.highest
}

fn pattern_search(jets: &[Jet]) -> (i64, i64) {
    // coprime lengths
    let lcm = jets.len() * ROCKS.len();
    let mut chamber = Chamber::new();

    let mut jets = jets.iter().cycle().copied();
    let mut rocks = ROCKS.iter().cycle().copied();

    let mut last_highest = chamber.highest;
    let mut records = Vec::with_capacity(1000);

    const TRIAL_LEN: usize = 1000;
    const MATCH_LEN: usize = 32;

    for _ in 0..TRIAL_LEN {
        for _ in 0..lcm {
            one_piece(&mut chamber, rocks.next().unwrap(), &mut jets);
        }

        records.push(chamber.highest - last_highest);
        last_highest = chamber.highest;

        if records.len() > MATCH_LEN {
            let last_match = records.len() - MATCH_LEN;

            let first_match = records
                .windows(MATCH_LEN)
                .position(|c| c == &records[last_match..])
                .unwrap();

            if first_match < last_match {
                return (first_match as i64 + 1, (last_match - first_match) as i64);
            }
        }
    }

    unreachable!("must be a pattern")

    // pattern from loop 2 repeats at loop 347
    // a loop of length 345
}

const PART_TWO_ROCKS: i64 = 1000000000000;

fn part_two(jets: &[Jet]) -> i64 {
    let (skip, pattern_len) = pattern_search(jets);
    // coprime lengths
    let lcm = (jets.len() * ROCKS.len()) as i64;
    let mut chamber = Chamber::new();

    let mut jets = jets.iter().cycle().copied();
    let mut rocks = ROCKS.iter().cycle().copied();

    let (mut skip_chamber, skip_height) = {
        for _ in 0..lcm * skip {
            one_piece(&mut chamber, rocks.next().unwrap(), &mut jets);
        }
        (chamber.clone(), chamber.highest)
    };

    let loop_height_growth = {
        for _ in 0..lcm * pattern_len {
            one_piece(&mut chamber, rocks.next().unwrap(), &mut jets);
        }

        chamber.highest - skip_height
    };

    let pattern_height = (PART_TWO_ROCKS - lcm * skip) / (lcm * pattern_len) * loop_height_growth;
    let remaining_rocks = (PART_TWO_ROCKS - lcm * skip) % (lcm * pattern_len);

    for _ in 0..remaining_rocks {
        one_piece(&mut skip_chamber, rocks.next().unwrap(), &mut jets);
    }

    pattern_height + skip_chamber.highest
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn example_part_one() {
        let jets = parse(INPUT);
        assert_eq!(part_one(&jets), 3068);
    }

    #[test]
    fn example_part_two() {
        let jets = parse(INPUT);
        assert_eq!(part_two(&jets), 1514285714288);
    }
}
