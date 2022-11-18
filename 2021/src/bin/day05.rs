#![feature(test)]
extern crate test;

use anyhow::Result;
use std::collections::HashMap;

const INPUT_NUM: usize = 0;

fn main() -> Result<()> {
    let input = get_input()?;

    let (part_1, part_2) = solve(input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);

    Ok(())
}

fn get_input() -> Result<Vec<Range2d>> {
    [
        include_str!("../inputs/day05.inp"),
        include_str!("../test_inputs/day05.inp1"),
    ][INPUT_NUM]
        .lines()
        .map(str::parse)
        .collect()
}

fn solve(inp: Vec<Range2d>) -> (usize, usize) {
    let (diag, flat): (Vec<Range2d>, Vec<Range2d>) =
        inp.into_iter().partition(Range2d::is_diagonal);

    let mut counts: HashMap<(u32, u32), u32> = Default::default();

    for point in flat.into_iter().flat_map(Range2d::into_iter) {
        *counts.entry(point).or_insert(0) += 1;
    }

    let part_1 = counts.iter().filter(|(_, c)| **c > 1).count();

    for point in diag.into_iter().flat_map(Range2d::into_iter) {
        *counts.entry(point).or_insert(0) += 1;
    }

    let part_2 = counts.iter().filter(|(_, c)| **c > 1).count();

    (part_1, part_2)
}

#[derive(Debug, Copy, Clone)]
struct Range2d {
    start: (u32, u32),
    end: (u32, u32),
}

impl Range2d {
    fn is_diagonal(&self) -> bool {
        self.start.0 != self.end.0 && self.start.1 != self.end.1
    }

    fn into_iter(self) -> impl Iterator<Item = (u32, u32)> {
        use std::cmp::Ordering::*;

        let x_iter: Box<dyn Iterator<Item = u32>> = match self.start.0.cmp(&self.end.0) {
            Equal => Box::new(std::iter::repeat(self.start.0)),
            Less => Box::new(self.start.0..=self.end.0),
            Greater => Box::new((self.end.0..=self.start.0).rev()),
        };

        let y_iter: Box<dyn Iterator<Item = u32>> = match self.start.1.cmp(&self.end.1) {
            Equal => Box::new(std::iter::repeat(self.start.1)),
            Less => Box::new(self.start.1..=self.end.1),
            Greater => Box::new((self.end.1..=self.start.1).rev()),
        };

        x_iter.zip(y_iter)
    }
}

impl std::str::FromStr for Range2d {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (s, e) = s.split_once(" -> ").unwrap();
        let (x1, y1) = s.split_once(',').unwrap();
        let (x2, y2) = e.split_once(',').unwrap();

        Ok(Self {
            start: (x1.parse()?, y1.parse()?),
            end: (x2.parse()?, y2.parse()?),
        })
    }
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = get_input().unwrap();
    b.iter(|| solve(input.clone()))
}

#[bench]
fn bench_get_input(b: &mut test::Bencher) {
    b.iter(|| get_input().unwrap());
}
