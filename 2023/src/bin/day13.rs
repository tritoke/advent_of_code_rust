#![feature(never_type, unwrap_infallible)]

use std::str::FromStr;

fn main() {
    let inp = parse_input();

    let p1 = solve(&inp, 0);
    let p2 = solve(&inp, 1);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

#[derive(Debug)]
struct Pattern {
    tiles: Vec<Vec<bool>>,
}

impl FromStr for Pattern {
    type Err = !;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles = s
            .lines()
            .map(|line| line.bytes().map(|x| x == b'#').collect())
            .collect();

        Ok(Self { tiles })
    }
}

impl Pattern {
    fn find_reflection(&self, error_target: usize) -> (bool, usize) {
        let mut errors_vert = vec![0; self.tiles[0].len()];
        let mut errors_horiz = vec![0; self.tiles.len()];

        // first should be impossible
        *errors_vert.first_mut().unwrap() = error_target + 1;
        *errors_horiz.first_mut().unwrap() = error_target + 1;

        for row in &self.tiles {
            for (i, errors) in errors_vert.iter_mut().enumerate() {
                *errors += (0..i)
                    .filter(|&j| {
                        let left = row[j];
                        let right = row.get(2 * i - j - 1).copied().unwrap_or(left);
                        left ^ right
                    })
                    .count();
            }
        }

        if let Some(idx) = errors_vert
            .into_iter()
            .enumerate()
            .find_map(|(i, errors)| (errors == error_target).then_some(i))
        {
            return (false, idx);
        }

        for (i, errors) in errors_horiz.iter_mut().enumerate() {
            *errors += (0..i)
                .map(|j| {
                    let left = &self.tiles[j];
                    let right = self.tiles.get(2 * i - j - 1).unwrap_or(left);
                    left.iter()
                        .zip(right.iter())
                        .filter(|(&l, &r)| l ^ r)
                        .count()
                })
                .sum::<usize>();
        }

        errors_horiz
            .into_iter()
            .enumerate()
            .find_map(|(i, errors)| (errors == error_target).then_some(i))
            .map(|i| (true, i))
            .expect("I'm a trash programmer")
    }
}

fn parse_input() -> Vec<Pattern> {
    include_str!("../../inputs/day13.txt")
        .split("\n\n")
        .map(|block| block.parse().into_ok())
        .collect()
}

fn solve(inp: &[Pattern], error_target: usize) -> u32 {
    inp.iter()
        .map(|pat| {
            let (is_horiz, pos) = pat.find_reflection(error_target);
            if is_horiz {
                pos as u32 * 100
            } else {
                pos as u32
            }
        })
        .sum()
}
