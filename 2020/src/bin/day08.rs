#![feature(test)]
extern crate test;

use std::collections::BTreeSet;
use std::num::ParseIntError;
use std::str::FromStr;

type Input = Vec<Instruction>;
type PartInput = [Instruction];

fn main() {
    let input = get_input();

    let part_1 = part1(&input);
    let part_2 = part2(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn get_input() -> Input {
    include_str!("../inputs/day08.inp")
        //include_str!("../test_inputs/day08.inp1")
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn part1(input: &PartInput) -> i64 {
    let mut program = Program {
        instrs: input,
        ip: 0,
        acc: 0,
    };

    match program.eval_no_repeat() {
        ReturnState::Repeat(n) => n,
        ReturnState::Return(_) => panic!("Should repeat instruction not return."),
    }
}

fn part2(input: &PartInput) -> i64 {
    let mut instrs = input.to_vec();

    for i in 0..input.len() {
        let original_instr = instrs[i];

        if original_instr.is_acc() {
            continue;
        }

        let new_instr = match original_instr {
            Instruction::Nop(n) => Instruction::Jmp(n),
            Instruction::Jmp(n) => Instruction::Nop(n),
            instr => instr,
        };

        if new_instr.is_jump_loop() {
            continue;
        }

        instrs[i] = new_instr;

        let mut program = Program {
            instrs: &instrs,
            ip: 0,
            acc: 0,
        };

        match program.eval_no_repeat() {
            ReturnState::Repeat(_) => instrs[i] = original_instr,
            ReturnState::Return(n) => return n,
        }
    }

    unreachable!()
}

#[derive(Debug)]
struct Program<'a> {
    instrs: &'a [Instruction],
    ip: i64,
    acc: i64,
}

impl Program<'_> {
    fn eval_no_repeat(&mut self) -> ReturnState {
        let mut visited: BTreeSet<i64> = BTreeSet::new();

        while self.ip < self.instrs.len() as i64 {
            if visited.contains(&self.ip) {
                return ReturnState::Repeat(self.acc);
            }

            visited.insert(self.ip);

            match self.instrs[self.ip as usize] {
                Instruction::Acc(n) => self.acc += n,
                Instruction::Jmp(n) => self.ip += n - 1,
                Instruction::Nop(_) => (),
            }

            self.ip += 1;
        }

        ReturnState::Return(self.acc)
    }
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    Nop(i64),
    Jmp(i64),
    Acc(i64),
}

impl Instruction {
    fn is_acc(&self) -> bool {
        matches!(self, Instruction::Acc(_))
    }
    fn is_jump_loop(&self) -> bool {
        matches!(self, Instruction::Jmp(0))
    }
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let space = s.find(' ').unwrap();

        let instr = s.get(..space).unwrap();
        let num = s.get(space + 1..).unwrap();

        let num_fromstr = num.parse().unwrap();

        Ok(match instr {
            "nop" => Instruction::Nop(num_fromstr),
            "acc" => Instruction::Acc(num_fromstr),
            "jmp" => Instruction::Jmp(num_fromstr),
            _ => panic!("Unrecognised instruction: {:?}", s),
        })
    }
}

enum ReturnState {
    Repeat(i64),
    Return(i64),
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
