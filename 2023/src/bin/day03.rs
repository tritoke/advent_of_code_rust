use std::{collections::BTreeMap, str::FromStr};

use anyhow::Context;

fn main() {
    let inp = parse_input();

    let p1 = part1(&inp);
    let p2 = part2(&inp);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn parse_input() -> Engine {
    include_str!("../../inputs/day03.txt")
        .parse()
        .expect("Failed to parse input file.")
}

#[derive(Debug, Default)]
struct Engine {
    numbers: Vec<u32>,
    gear_adjacents: BTreeMap<(usize, usize), Vec<u32>>,
}

impl FromStr for Engine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut numbers = Vec::new();
        let mut gear_adjacents: BTreeMap<_, Vec<u32>> = BTreeMap::new();
        let lines: Vec<_> = s.lines().collect();

        let detect_neighbours = |i: usize, j: usize, num_start: usize| {
            let above = i.saturating_sub(1);
            let below = (i + 1).min(lines.len() - 1);
            let left = num_start.saturating_sub(1);
            let right = j;

            let mut is_symbol = false;
            let mut gears = vec![];
            for (row, col) in itertools::iproduct!(above..=below, left..=right) {
                let b = lines[row].as_bytes()[col];
                if !b.is_ascii_digit() && b != b'.' {
                    is_symbol = true;
                }
                if b == b'*' {
                    gears.push((row, col))
                }
            }

            (is_symbol, gears)
        };

        let mut num = String::new();
        let mut num_start = 0;
        for (i, line) in lines.iter().enumerate() {
            for (j, c) in line.chars().enumerate() {
                if c.is_ascii_digit() {
                    if num.is_empty() {
                        num_start = j;
                    }
                    num.push(c);
                    continue;
                }

                if !num.is_empty() {
                    let n = num
                        .parse()
                        .with_context(|| format!("Line {i}, col {j}: num={num:?}"))?;
                    num.clear();

                    let (is_symbol, gears) = detect_neighbours(i, j, num_start);
                    if is_symbol {
                        numbers.push(n);
                    }

                    for gear in gears {
                        gear_adjacents.entry(gear).or_default().push(n);
                    }
                }
                num.clear();
            }

            if !num.is_empty() {
                let n = num
                    .parse()
                    .with_context(|| format!("Line {i}, col {}: num={num:?}", line.len()))?;

                let (is_symbol, gears) = detect_neighbours(i, line.len() - 1, num_start);
                if is_symbol {
                    numbers.push(n);
                }
                for gear in gears {
                    gear_adjacents.entry(gear).or_default().push(n);
                }
            }
            num.clear();
        }

        Ok(Engine {
            numbers,
            gear_adjacents,
        })
    }
}

fn part1(inp: &Engine) -> u32 {
    inp.numbers.iter().sum()
}

fn part2(inp: &Engine) -> u32 {
    inp.gear_adjacents
        .values()
        .filter(|adjacents| adjacents.len() == 2)
        .map(|adjacents| adjacents.iter().product::<u32>())
        .sum()
}
