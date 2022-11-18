#![feature(test, array_windows)]
extern crate test;

use anyhow::Result;

const INPUT_NUM: usize = 0;

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
        include_str!("../inputs/day01.inp"),
        include_str!("../test_inputs/day01.inp1"),
    ][INPUT_NUM]
        .lines()
        .map(str::parse)
        .collect()
}

fn num_increasing(slice: &[u32]) -> usize {
    slice.array_windows().filter(|[a, b]| a < b).count()
}

fn part1(inp: &[u32]) -> usize {
    num_increasing(inp)
}

fn part2(inp: &[u32]) -> usize {
    let slice_sums: Vec<u32> = inp.windows(3).map(|w| w.iter().sum()).collect();

    num_increasing(slice_sums.as_slice())
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
