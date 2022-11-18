#![feature(test)]
extern crate test;

use anyhow::Result;

const INPUT_NUM: usize = 0;

fn main() -> Result<()> {
    let input = get_input()?;

    let (part_1, part_2) = solve(input.as_slice());

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);

    Ok(())
}

fn get_input() -> Result<Vec<Command>> {
    [
        include_str!("../inputs/day02.inp"),
        include_str!("../test_inputs/day02.inp1"),
    ][INPUT_NUM]
        .lines()
        .map(str::parse)
        .collect()
}

fn solve(inp: &[Command]) -> (u32, u32) {
    let mut horiz: u32 = 0;
    let mut aim: u32 = 0;
    let mut depth: u32 = 0;

    for cmd in inp.iter() {
        match cmd {
            Command::Forward(x) => {
                horiz += x;
                depth += aim * x;
            }
            Command::Down(x) => {
                aim += x;
            }
            Command::Up(x) => {
                aim -= x;
            }
        }
    }

    (horiz * aim, horiz * depth)
}

#[derive(Debug, Copy, Clone)]
enum Command {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl std::str::FromStr for Command {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Command::*;

        let (cmd, d) = s.split_once(' ').unwrap();
        let dist: u32 = d.parse()?;
        let c = cmd.chars().next().unwrap();

        Ok(match c {
            'f' => Forward(dist),
            'd' => Down(dist),
            'u' => Up(dist),
            _ => unreachable!("Only 'f','d' and 'u' are valid first letters of commands."),
        })
    }
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = get_input().unwrap();
    b.iter(|| solve(&input))
}

#[bench]
fn bench_get_input(b: &mut test::Bencher) {
    b.iter(|| get_input().unwrap());
}
