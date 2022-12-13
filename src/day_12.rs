use std::collections::{hash_map::Entry, HashMap, VecDeque};

pub fn solution(input: &str) -> String {
    let map = parse(input);
    format!("{}, {}", part_one(&map), part_two(&map))
}

type Pos = (isize, isize);

struct Map {
    grid: Vec<Vec<i8>>,
    start: Pos,
    end: Pos,
}

impl Map {
    fn get_height(&self, (x, y): Pos) -> Option<i8> {
        self.grid.get(y as usize)?.get(x as usize).copied()
    }

    fn reverse_next_square(&self, from @ (x, y): Pos) -> impl Iterator<Item = Pos> + '_ {
        [(1, 0), (-1, 0), (0, 1), (0, -1)]
            .into_iter()
            .map(move |(dx, dy)| (x + dx, y + dy))
            .filter(move |&pos| {
                let Some(h) = self.get_height(from) else {
                    return false;
                };
                let Some(h_next) = self.get_height(pos) else {
                    return false;
                };
                h_next - h >= -1
            })
    }
}

fn parse(input: &str) -> Map {
    let mut start = (0, 0);
    let mut end = (0, 0);

    let grid = input
        .lines()
        .enumerate()
        .map(|(col, l)| {
            l.as_bytes()
                .iter()
                .enumerate()
                .map(|(row, b)| match b {
                    b'S' => {
                        start = (row as isize, col as isize);
                        0
                    }
                    b'E' => {
                        end = (row as isize, col as isize);
                        (b'z' - b'a') as i8
                    }
                    b'a'..=b'z' => (b - b'a') as i8,
                    _ => unreachable!("invalid height"),
                })
                .collect()
        })
        .collect();

    Map { grid, start, end }
}

fn shortest_to_all(map: &Map) -> HashMap<Pos, (Pos, isize)> {
    let mut shortest = HashMap::new();
    shortest.insert(map.end, (map.end, 0));
    let mut queue = VecDeque::new();
    queue.push_back((map.end, 0));

    while let Some((node, cost)) = queue.pop_front() {
        for next_node in map.reverse_next_square(node) {
            if let Entry::Vacant(e) = shortest.entry(next_node) {
                let next_cost = cost + 1;
                e.insert((node, next_cost));
                queue.push_back((next_node, next_cost));
            }
        }
    }

    shortest
}

fn part_one(map: &Map) -> isize {
    let shortest = shortest_to_all(map);
    let (_, cost) = shortest[&map.start];
    cost
}

fn part_two(map: &Map) -> isize {
    let shortest = shortest_to_all(map);
    shortest
        .into_iter()
        .filter_map(|(p, (_, c))| {
            if map.get_height(p) == Some(0) {
                Some(c)
            } else {
                None
            }
        })
        .min()
        .expect("non-empty grid")
}
