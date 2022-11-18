#![feature(test, array_windows)]
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

fn get_input() -> Result<Input> {
    [
        include_str!("../inputs/day14.inp"),
        include_str!("../test_inputs/day14.inp1"),
    ][INPUT_NUM]
        .parse()
}

fn solve(Input { template, rules }: Input) -> (usize, usize) {
    let mut polymeriser = Polymeriser::new(template.clone(), rules.clone(), 10);
    let counts = polymeriser.polymerise();
    let part_1 = counts.max() - counts.min();

    let mut polymeriser = Polymeriser::new(template.clone(), rules.clone(), 40);
    let counts = polymeriser.polymerise();
    let part_2 = counts.max() - counts.min();

    (part_1, part_2)
}

#[derive(Debug, Clone)]
struct Polymeriser {
    template: Vec<u8>,
    rules: HashMap<(u8, u8), u8>,
    step_cache: HashMap<(u8, u8, usize), Counter>,
    max_step: usize,
}

impl Polymeriser {
    fn new(template: Vec<u8>, rules: HashMap<(u8, u8), u8>, max_step: usize) -> Self {
        Self {
            template,
            rules,
            max_step,
            step_cache: Default::default(),
        }
    }

    fn count(&mut self, left: u8, right: u8, mut step: usize) -> Counter {
        if let Some(cached_result) = self.step_cache.get(&(left, right, step)) {
            return cached_result.clone();
        }

        if step == self.max_step {
            Counter::empty()
        } else {
            step += 1;

            let next = *self.rules.get(&(left, right)).unwrap();
            let mut new_counter = Counter::new(next);
            new_counter.update(self.count(left, next, step));
            new_counter.update(self.count(next, right, step));

            self.step_cache
                .insert((left, right, step - 1), new_counter.clone());

            new_counter
        }
    }

    fn polymerise(&mut self) -> Counter {
        let mut counter = Counter::from(self.template.as_slice());
        for &[left, right] in self.template.clone().array_windows() {
            counter.update(self.count(left, right, 0));
        }

        counter
    }
}

#[derive(Debug, Default, Clone)]
struct Counter {
    counts: HashMap<u8, usize>,
}

impl Counter {
    fn empty() -> Self {
        Default::default()
    }

    fn new(elem: u8) -> Self {
        let mut counter = Self::empty();

        counter.counts.insert(elem, 1);

        counter
    }

    fn from(elems: &[u8]) -> Self {
        let mut counter = Self::empty();

        for &elem in elems {
            *counter.counts.entry(elem).or_default() += 1;
        }

        counter
    }

    fn update(&mut self, other: Self) {
        for (elem, count) in other.counts {
            *self.counts.entry(elem).or_default() += count;
        }
    }

    fn max(&self) -> usize {
        self.counts.iter().map(|(_, &count)| count).max().unwrap()
    }

    fn min(&self) -> usize {
        self.counts.iter().map(|(_, &count)| count).min().unwrap()
    }
}

#[derive(Debug, Clone)]
struct Input {
    template: Vec<u8>,
    rules: HashMap<(u8, u8), u8>,
}

impl std::str::FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (temp, pairs) = s.split_once("\n\n").unwrap();

        let template = temp.trim().to_string().into_bytes();
        let rules = pairs
            .lines()
            .map(|line| {
                let (pair, result) = line.split_once(" -> ").unwrap();
                let mut pair_bytes = pair.bytes();
                let left = pair_bytes.next().unwrap();
                let right = pair_bytes.next().unwrap();
                let dest = result.bytes().next().unwrap();

                ((left, right), dest)
            })
            .collect();

        Ok(Self { template, rules })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[bench]
    fn bench_solution(b: &mut test::Bencher) {
        let input = get_input().unwrap();
        b.iter(|| solve(input.clone()))
    }

    #[bench]
    fn bench_get_input(b: &mut test::Bencher) {
        b.iter(|| get_input());
    }
}
