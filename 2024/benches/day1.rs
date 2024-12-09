use std::str::FromStr;

use advent_of_code_2024::day01::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn bench_part1(c: &mut Criterion) {
    let input = Input::from_str(include_str!("../inputs/day1.input")).unwrap();
    c.bench_function("day01_part1", |b| b.iter(|| part1(black_box(&input))));
}

pub fn bench_part2(c: &mut Criterion) {
    let input = Input::from_str(include_str!("../inputs/day1.input")).unwrap();
    c.bench_function("day01_part2", |b| b.iter(|| part2(black_box(&input))));
}

criterion_group!(benches, bench_part1, bench_part2);
criterion_main!(benches);
