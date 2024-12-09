use std::str::FromStr;

use advent_of_code_2024::*;
use concat_idents::concat_idents;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

macro_rules! bench_day {
    ($day:literal, $module:path, $input:literal) => {
        concat_idents!(part1_bench = bench_, $day, _part1 {
            pub fn part1_bench(c: &mut Criterion) {
                let input = $module::Input::from_str(include_str!($input)).unwrap();
                c.bench_function(concat!($day, "_part1"), |b| b.iter(|| $module::part1(black_box(&input))));
            }
        });

        concat_idents!(part2_bench = bench_, $day, _part2 {
            pub fn part2_bench(c: &mut Criterion) {
                let input = $module::Input::from_str(include_str!($input)).unwrap();
                c.bench_function(concat!($day, "_part2"), |b| b.iter(|| $module::part2(black_box(&input))));
            }
        });
    };
}

bench_day!("day01", day01, "../inputs/day1.input");
bench_day!("day02", day02, "../inputs/day2.input");

fn bench_day03_part1(c: &mut Criterion) {
    let input = include_str!("../inputs/day2.input");
    c.bench_function("day03_part1", |b| b.iter(|| day03::part1(black_box(input))));
}

fn bench_day03_part2(c: &mut Criterion) {
    let input = include_str!("../inputs/day2.input");
    c.bench_function("day03_part2", |b| b.iter(|| day03::part2(black_box(input))));
}

criterion_group!(
    benches,
    bench_day01_part1,
    bench_day01_part2,
    bench_day02_part1,
    bench_day02_part2,
    bench_day03_part1,
    bench_day03_part2,
);

criterion_main!(benches,);
