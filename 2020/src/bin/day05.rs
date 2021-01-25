#![feature(test)]
extern crate test;

use std::num::ParseIntError;
use std::str::FromStr;

type Input = Vec<BSPTicket>;
type PartInput = [BSPTicket];

fn main() {
    let input = get_input();

    let part_1 = part1(&input);
    let part_2 = part2(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn get_input() -> Input {
    let mut input: Input = include_str!("../inputs/day05.inp")
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.parse::<BSPTicket>().unwrap())
        .collect();

    input.sort();

    input
}

fn part1(input: &PartInput) -> usize {
    input.last().unwrap().seat_id
}

fn part2(input: &PartInput) -> usize {
    for item in input.windows(2) {
        if let [ticket1, ticket2] = item {
            if ticket1.seat_id + 2 == ticket2.seat_id {
                return ticket1.seat_id + 1;
            }
        }
    }

    unreachable!()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct BSPTicket {
    seat_id: usize,
}

impl FromStr for BSPTicket {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // parse first 7 as binary number
        let mut seat_id: usize = 0;

        for c in s.chars() {
            let bit = match c {
                'F' | 'L' => 0,
                'B' | 'R' => 1,
                _ => panic!("Unrecognised element in bit pattern match: {:?}", c),
            };

            seat_id <<= 1;
            seat_id |= bit;
        }

        Ok(BSPTicket { seat_id })
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
