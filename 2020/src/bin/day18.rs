#![feature(test)]
extern crate test;

use std::iter::Peekable;

const INPUT_NUM: i32 = 0;

type Input = Vec<String>;
type PartInput = [String];

fn main() {
    let input = get_input();

    let part_1 = part1(&input);
    let part_2 = part2(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn get_input() -> Input {
    match INPUT_NUM {
        0 => include_str!("../inputs/day18.inp"),
        1 => include_str!("../test_inputs/day18.inp1"),
        _ => panic!("Unknown input number: {:?}", INPUT_NUM),
    }
    .lines()
    .map(|line| {
        line.chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>()
    })
    .collect()
}

fn part1(input: &PartInput) -> usize {
    input
        .iter()
        .map(|line| {
            let line_copy = line.clone();

            let iter = &mut line_copy.chars().peekable();

            eval(iter, false)
        })
        .sum()
}

fn part2(input: &PartInput) -> usize {
    input
        .iter()
        .map(|line| {
            let line_copy = line.clone();

            let iter = &mut line_copy.chars().peekable();

            eval(iter, true)
        })
        .sum()
}

fn eval<T: Iterator<Item = char>>(iter: &mut Peekable<T>, plus_higher: bool) -> usize {
    let mut value = 0;

    let mut sums: Vec<usize> = Vec::new();

    while let Some(tok) = iter.peek() {
        match tok {
            // number at the start of an expression
            '0'..='9' => value = iter.next().unwrap() as usize - '0' as usize,

            // operation
            '+' | '*' => {
                let op = iter.next().unwrap();

                let num = match iter.peek().unwrap() {
                    '(' => {
                        iter.next();
                        eval(iter, plus_higher)
                    }
                    '0'..='9' => iter.next().unwrap() as usize - '0' as usize,
                    _ => panic!("I don't know what I'm doing"),
                };

                value = match op {
                    '+' => value + num,
                    '*' => {
                        if plus_higher {
                            // store value in sums if non zero
                            if value != 0 {
                                sums.push(value);
                            }

                            num
                        } else {
                            value * num
                        }
                    }
                    _ => panic!("magic shit bruv"),
                };
            }

            // end expr
            ')' => {
                iter.next();
                break;
            }

            // parens as first expression
            '(' => {
                iter.next();
                value = eval(iter, plus_higher);
            }
            _ => panic!("I don't know what I'm doing with {}", tok),
        }
    }

    if plus_higher {
        if value != 0 {
            sums.push(value);
        }

        sums.iter().product()
    } else {
        value
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
