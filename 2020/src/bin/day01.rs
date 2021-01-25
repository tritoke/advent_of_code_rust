#![feature(test)]
extern crate test;

use std::cmp::Ordering;

fn main() {
    let input = get_input();

    let part_1 = part1(&input);
    let part_2 = part2(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn get_input() -> Vec<i32> {
    let mut input: Vec<i32> = include_str!("../inputs/day01.inp")
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.parse::<i32>().unwrap())
        .collect();

    input.sort_unstable();

    input
}

fn part1(input: &[i32]) -> i32 {
    let mut start = 0;
    let mut end = input.len() - 1;

    loop {
        let sum = input[start] + input[end];

        match sum.cmp(&2020) {
            Ordering::Less => start += 1,
            Ordering::Greater => end -= 1,
            Ordering::Equal => break,
        };
    }

    input[start] * input[end]
}

fn part2(input: &[i32]) -> i32 {
    let mut start = 0;
    let mut mid = 0;
    let mut end = input.len() - 1;

    loop {
        let sum = input[start] + input[mid] + input[end];

        match sum.cmp(&2020) {
            Ordering::Greater => {
                mid = start;
                end -= 1;
            }
            Ordering::Less => {
                if mid == end {
                    start += 1;
                    mid = start;
                } else {
                    mid += 1;
                }
            }
            Ordering::Equal => break,
        }
    }

    input[start] * input[mid] * input[end]
}

#[bench]
fn bench_part1_solution(b: &mut test::Bencher) {
    let input = get_input();
    b.iter(|| part1(&input))
}

#[bench]
fn bench_part2_solution(b: &mut test::Bencher) {
    let input = get_input();
    b.iter(|| part2(&input))
}

#[bench]
fn bench_get_input(b: &mut test::Bencher) {
    b.iter(|| get_input());
}
