#![feature(test)]
extern crate test;

use anyhow::Result;
use std::collections::VecDeque;
use std::fmt;

const INPUT_NUM: usize = 0;

fn main() -> Result<()> {
    let input = get_input()?;

    let (part_1, part_2) = solve(input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);

    Ok(())
}

fn get_input() -> Result<Cavern> {
    [
        include_str!("../inputs/day11.inp"),
        include_str!("../test_inputs/day11.inp1"),
        include_str!("../test_inputs/day11.inp2"),
    ][INPUT_NUM]
        .parse()
}

fn solve(mut cavern: Cavern) -> (usize, usize) {
    let part_1 = (0..100).map(|_| cavern.step()).sum();

    let mut part_2 = 100;
    let num_octopuses = cavern.num_octopuses();
    loop {
        part_2 += 1;
        let flashed = cavern.step();
        if flashed == num_octopuses {
            break;
        }
    }

    (part_1, part_2)
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct Cavern {
    octopuses: Vec<Vec<u8>>,
}

impl Cavern {
    fn step(&mut self) -> usize {
        // First, the energy level of each octopus increases by 1.
        self.octopuses
            .iter_mut()
            .flat_map(|line| line.iter_mut())
            .for_each(|energy| *energy += 1);

        // Then, any octopus with an energy level greater than 9 flashes.
        let mut flash_queue: VecDeque<(usize, usize)> = self
            .octopuses
            .iter()
            .enumerate()
            .flat_map(|(y, line)| {
                line.iter()
                    .enumerate()
                    .filter(|&(_, energy)| *energy == 10)
                    .map(move |(x, _)| (x, y))
            })
            .collect();

        let mut flash_count = 0;
        while let Some((x, y)) = flash_queue.pop_front() {
            flash_count += 1;
            let offsets = [
                y.checked_sub(1).zip(x.checked_sub(1)),
                y.checked_sub(1).zip(Some(x)),
                y.checked_sub(1).zip(Some(x + 1)),
                Some(y).zip(x.checked_sub(1)),
                Some(y).zip(Some(x + 1)),
                Some(y + 1).zip(x.checked_sub(1)),
                Some(y + 1).zip(Some(x)),
                Some(y + 1).zip(Some(x + 1)),
            ];

            for (row, col) in offsets.into_iter().flatten() {
                if let Some(energy) = self
                    .octopuses
                    .get_mut(row)
                    .and_then(|line| line.get_mut(col))
                {
                    if *energy != 0 {
                        *energy += 1;
                    }

                    if *energy == 10 {
                        flash_queue.push_back((col, row));
                    }
                }

                // Finally, any octopus that flashed during this step has its energy level set to 0.
                self.octopuses[y][x] = 0;
            }
        }

        flash_count
    }

    fn num_octopuses(&self) -> usize {
        self.octopuses[0].len() * self.octopuses.len()
    }
}

impl std::str::FromStr for Cavern {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let octopuses = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap() as u8)
                    .collect()
            })
            .collect();

        Ok(Self { octopuses })
    }
}

impl fmt::Display for Cavern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.octopuses.iter() {
            for o in row.iter() {
                write!(f, "{}", o)?;
            }
            writeln!(f)?;
        }

        Ok(())
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
