use std::{collections::HashSet, iter::from_fn};

use itertools::iproduct;

pub fn solution(input: &str) -> String {
    let map = parse(input);

    format!("{}, {}", part_one(&map), part_two(&map))
}

fn add((x0, y0): (isize, isize), (x1, y1): (isize, isize)) -> (isize, isize) {
    (x0 + x1, y0 + y1)
}

struct Map {
    trees: Vec<Vec<u8>>,
}

impl Map {
    fn width(&self) -> isize {
        self.trees[0].len() as isize
    }

    fn height(&self) -> isize {
        self.trees.len() as isize
    }

    fn get(&self, (x, y): (isize, isize)) -> Option<u8> {
        if x < 0 || y < 0 {
            return None;
        }

        self.trees.get(y as usize)?.get(x as usize).copied()
    }

    fn visible_trees(
        &self,
        from: (isize, isize),
        step: (isize, isize),
    ) -> impl Iterator<Item = (isize, isize)> + '_ {
        let mut highest = None::<u8>;
        let mut curr = from;

        from_fn(move || loop {
            let h = self.get(curr)?;
            if highest < Some(h) {
                highest = Some(h);
                let ret = curr;
                curr = add(curr, step);
                return Some(ret);
            } else {
                curr = add(curr, step);
            }
        })
    }
}

fn parse(input: &str) -> Map {
    let trees = input
        .lines()
        .map(|l| l.as_bytes().iter().map(|h| h - b'0').collect())
        .collect();
    Map { trees }
}

fn part_one(map: &Map) -> usize {
    let mut visible = HashSet::new();

    visible.extend((0..map.width()).flat_map(|x| map.visible_trees((x, 0), (0, 1))));
    visible
        .extend((0..map.width()).flat_map(|x| map.visible_trees((x, map.height() - 1), (0, -1))));
    visible.extend((0..map.height()).flat_map(|y| map.visible_trees((0, y), (1, 0))));
    visible
        .extend((0..map.height()).flat_map(|y| map.visible_trees((map.width() - 1, y), (-1, 0))));

    // let mut sorted = visible.iter().collect::<Vec<_>>();
    // sorted.sort_unstable();
    // println!("{:?}", sorted);

    visible.len()
}

fn view_to<I>(map: &Map, step: (isize, isize), iter: I) -> Vec<Vec<u32>>
where
    I: IntoIterator<Item = (isize, isize)>,
{
    let mut view = vec![vec![0; map.width() as usize]; map.height() as usize];
    for start in iter.into_iter() {
        let (mut x, mut y) = start;
        let mut idx = 0u32;
        let mut nearest = [0u32; 10];

        while let Some(h) = map.get((x, y)) {
            let blocked = *nearest[h as usize..].iter().max().unwrap();
            view[y as usize][x as usize] = idx - blocked;
            nearest[h as usize] = idx;

            (x, y) = add((x, y), step);
            idx += 1;
        }
    }

    view
}

#[rustfmt::skip]
fn part_two(map: &Map) -> u32 {
    let view_to_north = view_to(map, (0, 1), (0..map.width()).map(|x| (x, 0)));
    let view_to_south = view_to( map, (0, -1), (0..map.width()).map(|x| (x, map.height() - 1)));
    let view_to_west = view_to(map, (1, 0), (0..map.height()).map(|y| (0, y)));
    let view_to_east = view_to( map, (-1, 0), (0..map.height()).map(|y| (map.width() - 1, y)));

    iproduct!(0 .. map.width(), 0 .. map.height())
        .map(|(x, y)| {
            let (x, y) = (x as usize, y as usize);
            view_to_north[y][x] * view_to_south[y][x] * view_to_west[y][x] * view_to_east[y][x]
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn example_part_one() {
        let map = parse(INPUT);
        assert_eq!(part_one(&map), 21);
    }

    #[test]
    fn example_part_two() {
        let map = parse(INPUT);
        assert_eq!(part_two(&map), 8)
    }
}
