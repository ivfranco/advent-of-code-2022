/// WARNING: This solution is not generic.
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i64, line_ending, one_of},
    combinator::all_consuming,
    multi::{many1, separated_list1},
    sequence::{separated_pair, tuple},
    IResult, Parser,
};

pub fn solution(input: &str) -> String {
    let (map, path) = parse(input);
    format!(
        "{}, {}",
        part_one(&map, &path),
        part_two(&map, &path, CONNECTED_SIDES)
    )
}

#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Open,
    Wall,
    Void,
}
use Tile::*;

#[derive(Clone, Copy)]
enum Turn {
    L,
    R,
}
use Turn::*;

use crate::utils::{Coord, DOWN, LEFT, RIGHT, UP};

#[derive(Clone, Copy)]
enum Step {
    Forward(i64),
    Turn(Turn),
}

fn parse(input: &str) -> (Map, Vec<Step>) {
    let (_, (tiles, path)) = all_consuming(p_input)(input).expect("valid complete parse");
    (Map { tiles }, path)
}

fn p_input(input: &str) -> IResult<&str, (Vec<Vec<Tile>>, Vec<Step>)> {
    separated_pair(p_map, tuple((line_ending, line_ending)), p_path)(input)
}

fn p_map(input: &str) -> IResult<&str, Vec<Vec<Tile>>> {
    separated_list1(line_ending, p_map_line)(input)
}

fn p_map_line(input: &str) -> IResult<&str, Vec<Tile>> {
    many1(p_tile)(input)
}

fn p_tile(input: &str) -> IResult<&str, Tile> {
    one_of(" .#")
        .map(|t| match t {
            ' ' => Void,
            '.' => Open,
            '#' => Wall,
            _ => unreachable!("filtered by parser"),
        })
        .parse(input)
}

fn p_path(input: &str) -> IResult<&str, Vec<Step>> {
    many1(p_step)(input)
}

fn p_step(input: &str) -> IResult<&str, Step> {
    alt((
        i64.map(Step::Forward),
        tag("L").map(|_| Step::Turn(L)),
        tag("R").map(|_| Step::Turn(R)),
    ))(input)
}

struct Map {
    tiles: Vec<Vec<Tile>>,
}

impl Map {
    fn start(&self) -> Actor {
        let x = self.tiles[0]
            .iter()
            .position(|t| *t == Open)
            .expect("valid start condition");

        Actor {
            coord: Coord::new(x as i64, 0),
            facing: Right,
        }
    }

    fn get(&self, coord: Coord) -> Tile {
        self.tiles
            .get(coord.y as usize)
            .and_then(|row| row.get(coord.x as usize))
            .copied()
            .unwrap_or(Void)
    }

    fn wrap(&self, mut coord: Coord, facing: Facing) -> Coord {
        let back = facing.turn(L).turn(L).dir();
        while self.get(coord + back) != Void {
            coord = coord + back;
        }
        coord
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Facing {
    Up,
    Left,
    Down,
    Right,
}
use Facing::*;

impl Facing {
    fn turn(self, turn: Turn) -> Self {
        match turn {
            L => match self {
                Up => Left,
                Left => Down,
                Down => Right,
                Right => Up,
            },
            R => match self {
                Up => Right,
                Left => Up,
                Down => Left,
                Right => Down,
            },
        }
    }

    fn number(self) -> i64 {
        match self {
            Up => 3,
            Left => 2,
            Down => 1,
            Right => 0,
        }
    }

    fn dir(self) -> Coord {
        match self {
            Up => UP,
            Left => LEFT,
            Down => DOWN,
            Right => RIGHT,
        }
    }
}

#[derive(Debug)]
struct Actor {
    coord: Coord,
    facing: Facing,
}

impl Actor {
    fn follow(&mut self, step: Step, map: &Map) {
        match step {
            Step::Turn(t) => self.facing = self.facing.turn(t),
            Step::Forward(n) => self.forward(n, map),
        }
    }

    fn forward(&mut self, n: i64, map: &Map) {
        let dir = self.facing.dir();
        for _ in 0..n {
            match map.get(self.coord + dir) {
                Open => self.coord = self.coord + dir,
                Wall => break,
                Void => {
                    let wrap = map.wrap(self.coord, self.facing);
                    match map.get(wrap) {
                        Open => self.coord = wrap,
                        Wall => break,
                        Void => unreachable!("by the definition of wrap"),
                    }
                }
            }
        }
    }

    fn follow_cube(&mut self, step: Step, map: &Map, connected: &[Connected]) {
        match step {
            Step::Turn(t) => self.facing = self.facing.turn(t),
            Step::Forward(n) => self.forward_cube(n, map, connected),
        }
    }

    fn forward_cube(&mut self, n: i64, map: &Map, connected: &[Connected]) {
        for _ in 0..n {
            let dir = self.facing.dir();
            match map.get(self.coord + dir) {
                Open => self.coord = self.coord + dir,
                Wall => break,
                Void => {
                    let wrapped = connected
                        .iter()
                        .find_map(|conn| conn.wrap(self))
                        .expect("cube is connected");

                    match map.get(wrapped.coord) {
                        Open => *self = wrapped,
                        Wall => break,
                        Void => unreachable!("by the definition of wrap"),
                    }
                }
            }
        }
    }

    fn password(self) -> i64 {
        1000 * (self.coord.y + 1) + 4 * (self.coord.x + 1) + self.facing.number()
    }
}

fn part_one(map: &Map, path: &[Step]) -> i64 {
    let mut actor = map.start();
    for step in path {
        actor.follow(*step, map);
    }
    actor.password()
}

fn part_two(map: &Map, path: &[Step], connected: &[Connected]) -> i64 {
    let mut actor = map.start();
    for step in path {
        actor.follow_cube(*step, map, connected);
    }
    actor.password()
}

#[derive(Clone, Copy)]
struct Side {
    from: Coord,
    to: Coord,
    exit: Facing,
}

impl Side {
    const fn new_horizontal(y: i64, x0: i64, length: i64, exit: Facing) -> Self {
        Self {
            from: Coord::new(x0, y),
            to: Coord::new(x0 + length - 1, y),
            exit,
        }
    }

    const fn new_vertical(x: i64, y0: i64, length: i64, exit: Facing) -> Self {
        Self {
            from: Coord::new(x, y0),
            to: Coord::new(x, y0 + length - 1),
            exit,
        }
    }

    fn is_horizontal(&self) -> bool {
        self.from.y == self.to.y
    }

    fn len(&self) -> i64 {
        if self.is_horizontal() {
            self.to.x - self.from.x + 1
        } else {
            self.to.y - self.from.y + 1
        }
    }

    fn contains(&self, coord: Coord) -> bool {
        if self.is_horizontal() {
            self.from.y == coord.y && self.from.x <= coord.x && self.to.x >= coord.x
        } else {
            self.from.x == coord.x && self.from.y <= coord.y && self.to.y >= coord.y
        }
    }

    fn crossing(&self, actor: &Actor) -> bool {
        self.contains(actor.coord) && actor.facing.turn(L).turn(L) == self.exit
    }
}

#[derive(Clone, Copy)]
struct Connected {
    fst: Side,
    snd: Side,
    forward: bool,
}

impl Connected {
    const fn new_forward(fst: Side, snd: Side) -> Self {
        Self {
            fst,
            snd,
            forward: true,
        }
    }

    const fn new_backward(fst: Side, snd: Side) -> Self {
        Self {
            fst,
            snd,
            forward: false,
        }
    }

    fn wrap(&self, actor: &Actor) -> Option<Actor> {
        let (wrap_from, wrap_to) = if self.fst.crossing(actor) {
            (self.fst, self.snd)
        } else if self.snd.crossing(actor) {
            (self.snd, self.fst)
        } else {
            return None;
        };

        let d = if wrap_from.is_horizontal() {
            actor.coord.x - wrap_from.from.x
        } else {
            actor.coord.y - wrap_from.from.y
        };

        let d = if self.forward {
            d
        } else {
            wrap_to.len() - d - 1
        };

        let coord = if wrap_to.is_horizontal() {
            Coord::new(wrap_to.from.x + d, wrap_to.from.y)
        } else {
            Coord::new(wrap_to.from.x, wrap_to.from.y + d)
        };

        Some(Actor {
            coord,
            facing: wrap_to.exit,
        })
    }
}

const SIDE_LEN: i64 = 50;

#[allow(clippy::identity_op)]
const CONNECTED_SIDES: &[Connected] = &[
    // 3f
    Connected::new_forward(
        Side::new_horizontal(0, 1 * SIDE_LEN, SIDE_LEN, Down),
        Side::new_vertical(0, 3 * SIDE_LEN, SIDE_LEN, Right),
    ),
    // 7f
    Connected::new_forward(
        Side::new_horizontal(0, 2 * SIDE_LEN, SIDE_LEN, Down),
        Side::new_horizontal(4 * SIDE_LEN - 1, 0, SIDE_LEN, Up),
    ),
    // 2b
    Connected::new_backward(
        Side::new_vertical(1 * SIDE_LEN, 0, SIDE_LEN, Right),
        Side::new_vertical(0, 2 * SIDE_LEN, SIDE_LEN, Right),
    ),
    // 6b
    Connected::new_backward(
        Side::new_vertical(3 * SIDE_LEN - 1, 0, SIDE_LEN, Left),
        Side::new_vertical(2 * SIDE_LEN - 1, 2 * SIDE_LEN, SIDE_LEN, Left),
    ),
    // 5f
    Connected::new_forward(
        Side::new_horizontal(1 * SIDE_LEN - 1, 2 * SIDE_LEN, SIDE_LEN, Up),
        Side::new_vertical(2 * SIDE_LEN - 1, 1 * SIDE_LEN, SIDE_LEN, Left),
    ),
    // 4f
    Connected::new_forward(
        Side::new_horizontal(3 * SIDE_LEN - 1, 1 * SIDE_LEN, SIDE_LEN, Up),
        Side::new_vertical(1 * SIDE_LEN - 1, 3 * SIDE_LEN, SIDE_LEN, Left),
    ),
    // 1f
    Connected::new_forward(
        Side::new_vertical(1 * SIDE_LEN, 1 * SIDE_LEN, SIDE_LEN, Right),
        Side::new_horizontal(2 * SIDE_LEN, 0, SIDE_LEN, Down),
    ),
];

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    const SIDE_LEN: i64 = 4;

    #[allow(clippy::identity_op)]
    const EXAMPLE_CONNECTED_SIDES: &[Connected] = &[
        // 3b
        Connected::new_backward(
            Side::new_horizontal(0, 2 * SIDE_LEN, SIDE_LEN, Down),
            Side::new_horizontal(1 * SIDE_LEN, 0, SIDE_LEN, Down),
        ),
        // 5f
        Connected::new_forward(
            Side::new_vertical(2 * SIDE_LEN, 0, SIDE_LEN, Right),
            Side::new_horizontal(1 * SIDE_LEN, 1 * SIDE_LEN, SIDE_LEN, Down),
        ),
        // 7b
        Connected::new_backward(
            Side::new_vertical(3 * SIDE_LEN - 1, 0, SIDE_LEN, Left),
            Side::new_vertical(4 * SIDE_LEN - 1, 2 * SIDE_LEN, SIDE_LEN, Left),
        ),
        // 1f
        Connected::new_forward(
            Side::new_vertical(0, 1 * SIDE_LEN, SIDE_LEN, Right),
            Side::new_horizontal(3 * SIDE_LEN - 1, 3 * SIDE_LEN, SIDE_LEN, Up),
        ),
        // 6b
        Connected::new_backward(
            Side::new_vertical(3 * SIDE_LEN - 1, 1 * SIDE_LEN, SIDE_LEN, Left),
            Side::new_horizontal(2 * SIDE_LEN, 3 * SIDE_LEN, SIDE_LEN, Down),
        ),
        // 2b
        Connected::new_backward(
            Side::new_horizontal(2 * SIDE_LEN - 1, 0, SIDE_LEN, Up),
            Side::new_horizontal(3 * SIDE_LEN - 1, 2 * SIDE_LEN, SIDE_LEN, Up),
        ),
        // 4f
        Connected::new_forward(
            Side::new_horizontal(2 * SIDE_LEN - 1, 1 * SIDE_LEN, SIDE_LEN, Up),
            Side::new_vertical(2 * SIDE_LEN, 2 * SIDE_LEN, SIDE_LEN, Right),
        ),
    ];

    #[test]
    fn example_part_one() {
        let (map, path) = parse(INPUT);
        assert_eq!(part_one(&map, &path), 6032);
    }

    #[test]
    fn example_part_two() {
        let (map, path) = parse(INPUT);
        assert_eq!(part_two(&map, &path, EXAMPLE_CONNECTED_SIDES), 5031);
    }
}
