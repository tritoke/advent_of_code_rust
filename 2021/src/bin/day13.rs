#![feature(test, iter_intersperse)]
extern crate test;

use anyhow::{anyhow, Result};
use std::collections::HashSet;

const INPUT_NUM: usize = 0;

fn main() -> Result<()> {
    let input = get_input()?;

    let (part_1, part_2) = solve(input);

    println!("Part 1: {}", part_1);
    println!("Part 2:\n{}", part_2);

    Ok(())
}

fn get_input() -> Result<Input> {
    [
        include_str!("../inputs/day13.inp"),
        include_str!("../test_inputs/day13.inp1"),
    ][INPUT_NUM]
        .parse()
}

fn solve(input: Input) -> (usize, String) {
    let Input {
        paper: mut points,
        instrs,
    } = input;

    let mut instr_iter = instrs.into_iter();
    let instr = instr_iter.next().unwrap();

    for p in points.iter_mut() {
        p.fold(instr);
    }

    let part_1 = points.iter().copied().collect::<HashSet<_>>().len();

    for instr in instr_iter {
        for p in points.iter_mut() {
            p.fold(instr);
        }
    }

    let word: HashSet<Point> = points.into_iter().collect();

    let mut max_x = 0;
    let mut max_y = 0;
    for p in word.iter() {
        max_x = usize::max(max_x, p.x);
        max_y = usize::max(max_y, p.y);
    }

    let mut table = vec![vec![b'.'; max_x + 1]; max_y + 1];

    for Point { x, y } in word {
        table[y][x] = b'#';
    }

    let part_2 = table
        .into_iter()
        .map(|row| String::from_utf8(row).unwrap())
        .intersperse("\n".into())
        .collect();

    (part_1, part_2)
}

#[derive(Debug, Clone)]
struct Input {
    paper: Vec<Point>,
    instrs: Vec<Instruction>,
}

impl std::str::FromStr for Input {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (points, instructions) = s.split_once("\n\n").unwrap();

        let paper = points.lines().map(str::parse).collect::<Result<_>>()?;
        let instrs = instructions
            .lines()
            .map(str::parse)
            .collect::<Result<_>>()?;

        Ok(Self { paper, instrs })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn fold(&mut self, Instruction { axis, line }: Instruction) {
        match axis {
            Axis::X if self.x >= line => self.x = 2 * line - self.x,
            Axis::Y if self.y >= line => self.y = 2 * line - self.y,
            _ => {}
        }
    }
}

impl std::str::FromStr for Point {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let (x, y) = s.split_once(',').unwrap();

        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    axis: Axis,
    line: usize,
}

impl std::str::FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let val = s.rsplit_once(' ').unwrap().1;
        let (axis, line) = val.split_once('=').unwrap();

        Ok(Self {
            axis: axis.parse()?,
            line: line.parse()?,
        })
    }
}

#[derive(Debug, Clone, Copy)]
enum Axis {
    X,
    Y,
}

impl std::str::FromStr for Axis {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s {
            "x" => Ok(Axis::X),
            "y" => Ok(Axis::Y),
            _ => Err(anyhow!("Invalid axis: {:?}", s)),
        }
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
