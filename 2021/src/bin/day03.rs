#![feature(test)]
extern crate test;

use anyhow::Result;
use aoc::bit_ops::BitOps;

const INPUT_NUM: usize = 0;
const WIDTH: usize = [12, 5][INPUT_NUM];

fn main() -> Result<()> {
    let input = get_input()?;

    let part_1 = part1(input.as_slice());
    let part_2 = part2(input.as_slice());

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);

    Ok(())
}

fn get_input() -> Result<Vec<u32>, std::num::ParseIntError> {
    [
        include_str!("../inputs/day03.inp"),
        include_str!("../test_inputs/day03.inp1"),
    ][INPUT_NUM]
        .lines()
        .map(|line| u32::from_str_radix(line, 2))
        .collect()
}

fn part1(inp: &[u32]) -> u32 {
    let mut counts = [0_u32; WIDTH];

    for x in inp.iter() {
        for (i, count) in counts.iter_mut().enumerate() {
            if x.test_bit(i as u32) {
                *count += 1;
            }
        }
    }

    let threshold = inp.len() as u32 / 2;

    let gamma_rate = counts
        .into_iter()
        .enumerate()
        .fold(0, |mut acc, (i, count)| {
            if count > threshold {
                acc.set_bit(i as u32);
            }
            acc
        });

    gamma_rate * (!gamma_rate & ((1 << WIDTH) - 1))
}

fn rating(inp: &[u32], invert: bool) -> u32 {
    let mut remaining = inp.to_vec();

    for bit in (0..WIDTH as u32).rev() {
        if remaining.len() == 1 {
            break;
        }

        // calculate the bit criteria
        let count: u32 = remaining
            .iter()
            .map(|x| {
                let set = x.test_bit(bit);
                (if invert { !set } else { set }) as u32
            })
            .sum();
        let threshold = remaining.len() as u32 / 2;
        let bc = count > threshold || (count == 1 && threshold == 1 && !invert);

        remaining.retain(|x| x.test_bit(bit) == bc);
    }

    remaining[0]
}

fn part2(inp: &[u32]) -> u32 {
    rating(inp, false) * rating(inp, true)
}

#[bench]
fn bench_part1_solution(b: &mut test::Bencher) {
    let input = get_input().unwrap();
    b.iter(|| part1(&input))
}

#[bench]
fn bench_part2_solution(b: &mut test::Bencher) {
    let input = get_input().unwrap();

    b.iter(|| part2(&input))
}

#[bench]
fn bench_get_input(b: &mut test::Bencher) {
    b.iter(|| get_input().unwrap());
}
