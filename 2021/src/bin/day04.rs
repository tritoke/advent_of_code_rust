#![feature(test, array_chunks)]
extern crate test;

use anyhow::{anyhow, Result};

const INPUT_NUM: usize = 0;

#[derive(Debug, Clone)]
struct Bingo(Vec<u8>, Vec<Board>);

fn main() -> Result<()> {
    let input = get_input()?;

    let (part_1, part_2) = solve(input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);

    Ok(())
}

fn get_input() -> Result<Bingo> {
    let mut blocks = [
        include_str!("../inputs/day04.inp"),
        include_str!("../test_inputs/day04.inp1"),
    ][INPUT_NUM]
        .split("\n\n");

    let numbers = blocks
        .next()
        .unwrap()
        .split(',')
        .map(str::parse)
        .collect::<Result<_, _>>()?;

    let boards = blocks.map(str::parse).collect::<Result<_>>()?;

    Ok(Bingo(numbers, boards))
}

fn solve(inp: Bingo) -> (u32, u32) {
    let Bingo(numbers, mut boards) = inp;

    let mut part_1 = None;
    let mut part_2 = None;

    for n in numbers {
        let boards_remaining = boards.len();

        for board in boards.iter_mut() {
            board.mark(n);

            if part_1.is_none() && board.is_winning() {
                part_1 = Some(n as u32 * board.unmarked_sum());
            }

            if boards_remaining == 1 && board.is_winning() {
                part_2 = Some(n as u32 * board.unmarked_sum());
                break;
            }
        }

        boards.retain(|board| !board.is_winning());
    }

    (part_1.unwrap_or(0), part_2.unwrap_or(0))
}

#[derive(Debug, Copy, Clone)]
struct Board {
    numbers: [u8; 25],
}

impl Board {
    fn mark(&mut self, num: u8) {
        if let Some(n) = self.numbers.iter_mut().find(|x| **x == num) {
            *n = 0xFF;
        }
    }

    fn is_winning(&self) -> bool {
        let winning_row = self
            .numbers
            .array_chunks::<5>()
            .any(|chunk| *chunk == [0xFF; 5]);

        let winning_col =
            (0..5).any(|i| self.numbers.iter().skip(i).step_by(5).all(|n| *n == 0xFF));

        winning_row || winning_col
    }

    fn unmarked_sum(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|n| **n != 0xFF)
            .map(|n| *n as u32)
            .sum()
    }
}

impl std::str::FromStr for Board {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<u8> = s
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        Ok(Self {
            numbers: nums.try_into().map_err(|numbers: Vec<u8>| {
                anyhow!(
                    "Failed to parse board, found {} numbers where there should have been 25.",
                    numbers.len()
                )
            })?,
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
