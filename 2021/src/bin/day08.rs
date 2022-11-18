#![feature(test)]
extern crate test;

use anyhow::Result;

const INPUT_NUM: usize = 0;

#[derive(Debug, Clone)]
struct Line([&'static str; 10], [&'static str; 4]);

fn main() -> Result<()> {
    let input = get_input();

    let (part_1, part_2) = solve(input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);

    Ok(())
}

fn get_input() -> Vec<Line> {
    [
        include_str!("../inputs/day08.inp"),
        include_str!("../test_inputs/day08.inp1"),
        include_str!("../test_inputs/day08.inp2"),
    ][INPUT_NUM]
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" | ").unwrap();
            let l: Vec<_> = left.split(' ').collect();
            let r: Vec<_> = right.split(' ').collect();
            Line(l.try_into().unwrap(), r.try_into().unwrap())
        })
        .collect()
}

fn solve(inp: Vec<Line>) -> (usize, usize) {
    let solved: Vec<_> = inp.iter().map(Line::solve).collect();

    let mut part_1 = 0;
    let mut part_2 = 0;

    for s in solved {
        let mut acc = 0;
        for d in s {
            if matches!(d, 1 | 4 | 7 | 8) {
                part_1 += 1;
            }

            acc *= 10;
            acc += d as usize;
        }

        part_2 += acc;
    }

    (part_1, part_2)
}

impl Line {
    fn solve(&self) -> [u32; 4] {
        let mut counts = [0_u8; 7];
        for s in self.0 {
            for b in s.bytes() {
                let idx = (b - b'a') as usize;
                counts[idx] += 1;
            }
        }

        let mut transform = [None; 7];
        for (i, c) in counts.iter().enumerate() {
            match c {
                4 => transform[i] = Some(4),
                6 => transform[i] = Some(1),
                9 => transform[i] = Some(5),
                _ => {}
            }
        }

        for (len, dig) in [(2, 2), (3, 0), (4, 3), (7, 6)] {
            'outer: for num in self.0.iter() {
                if num.len() == len {
                    for b in num.bytes() {
                        let d = (b - b'a') as usize;
                        if transform[d].is_none() {
                            transform[d] = Some(dig);
                            break 'outer;
                        }
                    }
                }
            }
        }

        self.1.map(|s| {
            let mut digits: Vec<u32> = s
                .bytes()
                .map(|b| transform[(b - b'a') as usize].unwrap())
                .collect();
            digits.sort_unstable();
            match digits[..] {
                [0, 1, 2, 4, 5, 6] => 0,
                [2, 5] => 1,
                [0, 2, 3, 4, 6] => 2,
                [0, 2, 3, 5, 6] => 3,
                [1, 2, 3, 5] => 4,
                [0, 1, 3, 5, 6] => 5,
                [0, 1, 3, 4, 5, 6] => 6,
                [0, 2, 5] => 7,
                [0, 1, 2, 3, 4, 5, 6] => 8,
                [0, 1, 2, 3, 5, 6] => 9,
                _ => unreachable!(),
            }
        })
    }
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = get_input();
    b.iter(|| solve(input.clone()))
}

#[bench]
fn bench_get_input(b: &mut test::Bencher) {
    b.iter(|| get_input());
}
