#![feature(test)]
extern crate test;

use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;

type Colour = String;
type Input = HashMap<Colour, Backpack>;

fn main() {
    let input = get_input();

    let part_1 = part1(&input);
    let part_2 = part2(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn get_input() -> Input {
    let mut backpacks: Input = include_str!("../inputs/day07.inp")
        .lines()
        .map(|line| {
            let sep = " bags contain ";
            let index = line.find(sep).unwrap();

            let colour = line.get(..index).unwrap();
            let backpack_str = line.get(index + sep.len()..).unwrap();

            let backpack = backpack_str.parse().unwrap();

            (colour.to_string(), backpack)
        })
        .collect();

    // link the backpacks to those which can reach it
    let backwards_connections: Vec<(Colour, Colour)> = backpacks
        .iter()
        .flat_map(|(backpack_name, backpack)| {
            backpack
                .can_contain
                .iter()
                .map(move |(contained, _)| (backpack_name.clone(), contained.clone()))
        })
        .collect();

    for (container, containee) in backwards_connections.iter() {
        backpacks
            .entry(containee.to_string())
            .and_modify(|b| b.contained_by.push(container.to_string()));
    }

    backpacks
}

fn part1(input: &Input) -> usize {
    let mut could_contain: HashSet<&str> = HashSet::new();

    let mut stack: Vec<&str> = vec!["shiny gold"];
    while !stack.is_empty() {
        let backpack = stack.pop().unwrap();
        for container in input[backpack].contained_by.iter() {
            stack.push(container.as_str());
            if !could_contain.contains(container.as_str()) {
                could_contain.insert(container.as_str());
            }
        }
    }

    could_contain.len()
}

fn part2(input: &Input) -> usize {
    must_contain(input, "shiny gold")
}

fn must_contain(input: &Input, backpack: &str) -> usize {
    let contains = &input[backpack].can_contain;

    contains.iter().fold(0, |acc, (colour, num)| {
        acc + num * (1 + must_contain(input, colour))
    })
}

#[derive(Debug)]
struct Backpack {
    contained_by: Vec<Colour>,
    can_contain: Vec<(Colour, usize)>,
}

impl FromStr for Backpack {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let can_contain_fromstr = if s.starts_with("no") {
            Vec::new()
        } else {
            s.split(", ")
                .map(|bp_str| {
                    let first_space = bp_str.find(' ').unwrap();
                    let last_space = bp_str.rfind(' ').unwrap();

                    let number = bp_str.get(..first_space).unwrap();
                    let bag = bp_str.get(first_space + 1..last_space).unwrap();

                    let number_fromstr = number.parse().unwrap();

                    (bag.to_string(), number_fromstr)
                })
                .collect()
        };

        Ok(Backpack {
            contained_by: Vec::new(),
            can_contain: can_contain_fromstr,
        })
    }
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
