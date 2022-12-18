use std::{
    collections::{hash_map::Entry, HashMap},
    fmt::Display,
    str::from_utf8,
};

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

/// pieces are upside down

/// ####
const BAR: &[u64] = &[0b111100000];

/// .#.
/// ###
/// .#.
const CROSS: &[u64] = &[0b010000000, 0b111000000, 0b010000000];

/// ..#
/// ..#
/// ###
const CORNER: &[u64] = &[0b111000000, 0b001000000, 0b001000000];

/// #
/// #
/// #
/// #
const PILLAR: &[u64] = &[0b100000000, 0b100000000, 0b100000000, 0b100000000];

/// ##
/// ##
const SQUARE: &[u64] = &[0b110000000, 0b110000000];

const ROCKS: [&[u64]; 5] = [BAR, CROSS, CORNER, PILLAR, SQUARE];

#[derive(Debug, Clone, Copy)]
struct Piece {
    bottom_left: Coord,
    rock: &'static [u64],
}

impl Piece {
    fn new(bottom_left: Coord, rock: &'static [u64]) -> Self {
        Self { bottom_left, rock }
    }

    fn rows(self) -> impl Iterator<Item = BitSet> {
        self.rock
            .iter()
            .map(move |b| BitSet::from_bits(b >> self.bottom_left.x))
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
    const WIDTH: usize = 7;
    const WALL: BitSet = BitSet::from_bits((0b1 << (Self::WIDTH + 1)) | 0b1);
    const BOTTOM: BitSet = BitSet::from_bits((0b1 << (Self::WIDTH + 2)) - 1);

    fn new() -> Self {
        let stopped = vec![Self::BOTTOM];
        Self {
            stopped,
            highest: 0,
        }
    }

    fn blocked(&self, piece: Piece) -> bool {
        piece.rows().enumerate().any(|(i, r)| {
            let y = piece.bottom_left.y as usize + i;
            let c = self.stopped.get(y).copied().unwrap_or(Self::WALL);
            !r.intersection(&c).is_empty()
        })
    }

    fn stop(&mut self, piece: Piece) {
        debug_assert!(!self.blocked(piece));
        for (i, r) in piece.rows().enumerate() {
            let y = piece.bottom_left.y as usize + i;
            while self.stopped.len() <= y {
                self.stopped.push(Self::WALL);
            }
            self.stopped[y] = self.stopped[y].union(&r);
            self.highest = self.highest.max(y as i64);
        }
    }

    /// Should be good enough. A perfectly accurate pattern signature has to run a multi-start DFS
    /// from the top line of the tower to determine the lower reachable height from open areas.
    fn signature(&self) -> Vec<BitSet> {
        let start = self.stopped.len().saturating_sub(16);
        self.stopped[start..].to_vec()
    }
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for dy in 0..10 {
            let mut row = [b' '; Self::WIDTH as usize + 2];
            row[0] = b'|';
            row[Self::WIDTH + 1] = b'|';
            for x in 1..=Self::WIDTH {
                if self.stopped[(self.highest - dy) as usize].contains(x as usize) {
                    row[x as usize] = b'#';
                }
            }
            f.write_str(from_utf8(row.as_slice()).unwrap())?;
            f.write_str("\n")?;
        }

        Ok(())
    }
}

fn one_jet(chamber: &mut Chamber, piece: &mut Piece, jet: Jet) -> bool {
    let dir = jet.to_dir();
    if !chamber.blocked(piece.move_to(dir)) {
        *piece = piece.move_to(dir);
    }

    if chamber.blocked(piece.move_to(DOWN)) {
        chamber.stop(*piece);
        true
    } else {
        *piece = piece.move_to(DOWN);
        false
    }
}

fn one_piece<J>(chamber: &mut Chamber, rock: &'static [u64], mut jets: J)
where
    J: Iterator<Item = Jet>,
{
    let mut piece = Piece::new(Coord::new(3, chamber.highest + 4), rock);

    loop {
        let jet = jets.next().unwrap();
        let stopped = one_jet(chamber, &mut piece, jet);
        if stopped {
            break;
        }
    }
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
    let mut chamber = Chamber::new();

    let mut jets = jets.iter().copied().enumerate().cycle();
    let mut rocks = ROCKS.iter().copied().enumerate().cycle();

    const TRIAL_LEN: i64 = 10000;
    let mut records: HashMap<(usize, usize, Vec<BitSet>), i64> = HashMap::new();

    for i in 0..TRIAL_LEN {
        let (rock_idx, rock) = rocks.next().unwrap();
        let (mut jet_idx, mut jet) = jets.next().unwrap();
        let mut piece = Piece::new(Coord::new(3, chamber.highest + 4), rock);

        while !one_jet(&mut chamber, &mut piece, jet) {
            (jet_idx, jet) = jets.next().unwrap();
        }

        match records.entry((rock_idx, jet_idx, chamber.signature())) {
            Entry::Occupied(e) => {
                let skip = *e.get();
                return (skip + 1, i - skip);
            }
            Entry::Vacant(e) => {
                e.insert(i);
            }
        }
    }

    unreachable!("must be a pattern")
}

const PART_TWO_ROCKS: i64 = 1000000000000;

fn part_two(jets: &[Jet]) -> i64 {
    let (skip, pattern_len) = pattern_search(jets);
    // coprime lengths
    let mut chamber = Chamber::new();

    let mut jets = jets.iter().cycle().copied();
    let mut rocks = ROCKS.iter().cycle().copied();

    let (mut skip_chamber, skip_height) = {
        for _ in 0..skip {
            one_piece(&mut chamber, rocks.next().unwrap(), &mut jets);
        }
        (chamber.clone(), chamber.highest)
    };

    let loop_height_growth = {
        for _ in 0..pattern_len {
            one_piece(&mut chamber, rocks.next().unwrap(), &mut jets);
        }

        chamber.highest - skip_height
    };

    let pattern_height = (PART_TWO_ROCKS - skip) / pattern_len * loop_height_growth;
    let remaining_rocks = (PART_TWO_ROCKS - skip) % pattern_len;

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
