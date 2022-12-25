use std::{fmt::Display, str::from_utf8};

pub fn solution(input: &str) -> String {
    let snafus = parse(input);
    part_one(&snafus)
}

const SNAFU_RADIX: i64 = 5;
const SNAFU_OFFSET: i64 = 2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Snafu(i64);

impl Snafu {
    fn parse(raw: &str) -> Self {
        let mut n = match raw.as_bytes()[0] {
            b'1' => 1,
            b'2' => 2,
            _ => unreachable!("valid input"),
        };

        n = raw.bytes().skip(1).fold(n, |mut n, c| {
            n *= SNAFU_RADIX;
            n -= SNAFU_OFFSET;
            n += match c {
                b'=' => 0,
                b'-' => 1,
                b'0' => 2,
                b'1' => 3,
                b'2' => 4,
                _ => unreachable!("valid input"),
            };
            n
        });

        Snafu(n)
    }
}

impl Display for Snafu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buffer = vec![];
        let mut n = self.0;
        while n > 0 {
            n += SNAFU_OFFSET;
            let c = match n % SNAFU_RADIX {
                0 => b'=',
                1 => b'-',
                2 => b'0',
                3 => b'1',
                4 => b'2',
                _ => unreachable!("by mod"),
            };

            buffer.push(c);

            n /= SNAFU_RADIX;
        }

        buffer.reverse();
        f.write_str(from_utf8(&buffer).unwrap())
    }
}

fn parse(input: &str) -> Vec<Snafu> {
    input.lines().map(Snafu::parse).collect()
}

fn part_one(snafus: &[Snafu]) -> String {
    let n = snafus.iter().map(|s| s.0).sum::<i64>();
    format!("{}", Snafu(n))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn example_number_to_snafu() {
        assert_eq!(format!("{}", Snafu(1)), "1");
        assert_eq!(format!("{}", Snafu(2)), "2");
        assert_eq!(format!("{}", Snafu(3)), "1=");
        assert_eq!(format!("{}", Snafu(2022)), "1=11-2");
        assert_eq!(format!("{}", Snafu(314159265)), "1121-1110-1=0");
    }

    #[test]
    fn example_snafu_to_number() {
        assert_eq!(Snafu::parse("1"), Snafu(1));
        assert_eq!(Snafu::parse("2"), Snafu(2));
        assert_eq!(Snafu::parse("1="), Snafu(3));
        assert_eq!(Snafu::parse("1=11-2"), Snafu(2022));
        assert_eq!(Snafu::parse("1121-1110-1=0"), Snafu(314159265));
    }

    #[test]
    fn example_part_one() {
        let snafus = parse(INPUT);
        assert_eq!(part_one(&snafus), "2=-1=0");
    }
}
