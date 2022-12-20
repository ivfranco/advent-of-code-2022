pub fn solution(input: &str) -> String {
    let sequence = parse(input);
    format!("{}, {}", part_one(&sequence), part_two(&sequence))
}

fn parse(input: &str) -> Vec<isize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn shift(i: isize, len: usize) -> isize {
    if i <= 0 {
        // +0 => +0
        // -1 => +5
        // -2 => +4
        // -6 => +0
        i.rem_euclid((len - 1) as isize)
    } else {
        // +1 => +1
        // +6 => +6
        // +7 => +1
        (i - 1).rem_euclid((len - 1) as isize) + 1
    }
}

fn mix(sequence: &[isize], indices: &mut [isize]) {
    for (i, n) in sequence.iter().copied().enumerate() {
        let curr = indices[i];
        let next = shift(curr + n, sequence.len());

        for j in indices.iter_mut() {
            if *j < curr && *j < next {
                // not affected
            }

            if *j < curr && *j >= next {
                // n moved to its left
                *j = shift(*j + 1, sequence.len());
            }

            if *j > curr && *j <= next {
                // n moved to its right
                *j = shift(*j - 1, sequence.len());
            }

            if *j > curr && *j > next {
                // not affected
            }
        }

        indices[i] = next;
    }
}

fn shuffle(sequence: &[isize], indices: &[isize]) -> Vec<isize> {
    let mut shuffled = vec![0; sequence.len()];
    for (before, after) in indices.iter().copied().enumerate() {
        shuffled[after as usize] = sequence[before];
    }
    shuffled
}

fn coordinates(shuffled: &[isize]) -> isize {
    let z = shuffled.iter().position(|n| *n == 0).unwrap();
    [1000, 2000, 3000]
        .into_iter()
        .map(|d| {
            let after_idx = (z + d).rem_euclid(shuffled.len());
            shuffled[after_idx]
        })
        .sum()
}

fn part_one(sequence: &[isize]) -> isize {
    let mut indices: Vec<_> = (0..sequence.len()).map(|i| i as isize).collect();
    mix(sequence, &mut indices);
    let shuffled = shuffle(sequence, &indices);
    coordinates(&shuffled)
}

const DECRYPTION_KEY: isize = 811589153;

fn part_two(sequence: &[isize]) -> isize {
    let mut indices: Vec<_> = (0..sequence.len()).map(|i| i as isize).collect();
    let mut multiplied = sequence.to_vec();
    for n in &mut multiplied {
        *n *= DECRYPTION_KEY;
    }

    for _ in 0..10 {
        mix(&multiplied, &mut indices);
    }

    coordinates(&shuffle(&multiplied, &indices))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn example_part_one() {
        let sequence = parse(INPUT);
        assert_eq!(part_one(&sequence), 3);
    }

    #[test]
    fn example_part_two_one_round() {
        todo!()
    }

    #[test]
    fn example_part_two() {
        let sequence = parse(INPUT);
        assert_eq!(part_two(&sequence), 1623178306);
    }
}
