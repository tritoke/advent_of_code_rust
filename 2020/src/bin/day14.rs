#![feature(test)]
extern crate test;

use either::{Either, Left, Right};
use std::num::ParseIntError;
use std::str::FromStr;

use ahash::AHashMap;

type Input = Vec<Either<Mask, MemAction>>;
type PartInput = [Either<Mask, MemAction>];
const INPUT_NUM: i32 = 0;

fn main() {
    let input = get_input();

    let part_1 = part1(&input);
    let part_2 = part2(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn get_input() -> Input {
    let in_str = match INPUT_NUM {
        0 => include_str!("../inputs/day14.inp"),
        1 => include_str!("../test_inputs/day14.inp1"),
        2 => include_str!("../test_inputs/day14.inp2"),
        _ => panic!("Unknown input number: {:?}", INPUT_NUM),
    };

    in_str
        .lines()
        .map(|line| match line.chars().nth(1) {
            Some('a') => Left(line.parse::<Mask>().unwrap()),
            _ => Right(line.parse::<MemAction>().unwrap()),
        })
        .collect()
}

fn part1(input: &PartInput) -> usize {
    let mut mask: Mask = Default::default();

    let mut mem = AHashMap::default();

    for item in input {
        match item {
            Left(new_mask) => mask = *new_mask,
            Right(memaction) => {
                mem.insert(memaction.addr, mask.mask(memaction.value));
            }
        }
    }

    mem.values().sum()
}

fn part2(input: &PartInput) -> usize {
    let mut mask: Mask = Default::default();

    let mut mem = AHashMap::default();

    for item in input {
        match item {
            Left(new_mask) => mask = *new_mask,
            Right(memaction) => {
                for addr in mask.iter_floating_addrs(memaction.addr) {
                    //println!("mem[{}] = {}", addr, memaction.value);
                    mem.insert(addr, memaction.value);
                }
            }
        }
    }

    mem.values().sum()
}

#[derive(Debug, Copy, Clone)]
struct Mask {
    one_mask: usize,
    zero_mask: usize,
}

impl Mask {
    fn mask(&self, val: usize) -> usize {
        (val | self.one_mask) & self.zero_mask
    }

    fn iter_floating_addrs(&self, addr: usize) -> FloatingMask {
        FloatingMask {
            addr: addr | self.one_mask,
            floating_mask: self.one_mask ^ self.zero_mask,
            iter_no: 0,
        }
    }
}

impl Default for Mask {
    fn default() -> Self {
        Self {
            one_mask: usize::MIN,
            zero_mask: usize::MAX >> (64 - 36),
        }
    }
}

impl FromStr for Mask {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mask_bits = 36;

        let mut mask: Mask = Default::default();

        for (i, c) in s.chars().skip(7).enumerate() {
            match c {
                '1' => mask.one_mask.set_bit(mask_bits - 1 - i as u32),
                '0' => mask.zero_mask.clear_bit(mask_bits - 1 - i as u32),
                'X' => (),
                _ => panic!("Invalid character {:?} in mask.", c),
            }
        }

        Ok(mask)
    }
}

#[derive(Debug, Copy, Clone)]
struct MemAction {
    addr: usize,
    value: usize,
}

impl FromStr for MemAction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let end_brac = s.find(']').unwrap();
        let last_space = s.rfind(' ').unwrap();

        let addr_fromstr = s.get(4..end_brac).unwrap().parse::<usize>()?;
        let value_fromstr = s.get(last_space + 1..).unwrap().parse::<usize>()?;

        Ok(MemAction {
            addr: addr_fromstr,
            value: value_fromstr,
        })
    }
}

#[derive(Debug)]
struct FloatingMask {
    addr: usize,
    floating_mask: usize,
    iter_no: usize,
}

impl Iterator for FloatingMask {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        let num_set_bits = self.floating_mask.count_ones();
        let max_iter_no = 1 << num_set_bits;

        let mut masked_addr = self.addr;

        if self.iter_no < max_iter_no {
            let mut masks_applied = 0;

            for i in 0..36 {
                if self.floating_mask.test_bit(i) {
                    if self.iter_no.test_bit(masks_applied) {
                        masked_addr.set_bit(i);
                    } else {
                        masked_addr.clear_bit(i);
                    }

                    masks_applied += 1;

                    if masks_applied >= num_set_bits {
                        break;
                    }
                }
            }

            self.iter_no += 1;

            Some(masked_addr)
        } else {
            None
        }
    }
}

trait BitOps {
    fn set_bit(&mut self, n: u32);
    fn test_bit(&self, n: u32) -> bool;
    fn clear_bit(&mut self, n: u32);
}

impl BitOps for usize {
    fn set_bit(&mut self, n: u32) {
        *self |= 1 << n;
    }

    fn test_bit(&self, n: u32) -> bool {
        (*self & (1 << n)) == (1 << n)
    }

    fn clear_bit(&mut self, n: u32) {
        *self &= !(1 << n);
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
