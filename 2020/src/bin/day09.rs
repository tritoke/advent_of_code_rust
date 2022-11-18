#![feature(test)]
extern crate test;

use std::cmp::Ordering;
use std::collections::BTreeSet;

type Input = Vec<usize>;
type PartInput = [usize];
const INPUT_NUM: i32 = 0;

fn main() {
    let input = get_input();

    let part_1 = part1(&input);
    let part_2 = part2(&input, part_1);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn get_input() -> Input {
    let in_str = match INPUT_NUM {
        0 => include_str!("../inputs/day09.inp"),
        1 => include_str!("../test_inputs/day09.inp1"),
        _ => panic!("Unknown input number: {:?}", INPUT_NUM),
    };

    in_str.lines().map(|line| line.parse().unwrap()).collect()
}

fn part1(input: &PartInput) -> usize {
    let preamble_length: usize = match INPUT_NUM {
        0 => 25,
        1 => 5,
        _ => panic!("Unknown input number: {:?}", INPUT_NUM),
    };

    let mut prev_window: BTreeSet<usize> = BTreeSet::new();

    for &i in input.iter().take(preamble_length) {
        prev_window.insert(i);
    }

    for window in input.windows(preamble_length + 1) {
        let target = window[preamble_length];
        let to_remove = window[0];

        let mut small_end_iter = prev_window.iter();
        let mut big_end_iter = prev_window.iter().rev();

        let mut small_end = small_end_iter.next().unwrap();
        let mut big_end = big_end_iter.next().unwrap();

        // see if this one satisfies the property
        loop {
            // if we hit the same number this means there are none
            // which sum to the right value so break
            if small_end == big_end {
                return target;
            }

            let pair = small_end + big_end;
            match pair.cmp(&target) {
                Ordering::Less => small_end = small_end_iter.next().unwrap(),
                Ordering::Greater => big_end = big_end_iter.next().unwrap(),
                Ordering::Equal => break,
            }
        }

        // finally remove the old end and insert the new one
        prev_window.remove(&to_remove);
        prev_window.insert(target);
    }

    unreachable!();
}

fn part2(input: &PartInput, part_1_ans: usize) -> usize {
    let mut start: usize = 0;
    let mut end: usize = 1;
    let mut sum = input[start] + input[end];

    loop {
        match sum.cmp(&part_1_ans) {
            Ordering::Less => {
                end += 1;
                sum += input[end];
            }
            Ordering::Greater => {
                sum -= input[start];
                start += 1;
            }
            Ordering::Equal => break,
        }
    }

    let mut min = input[end];
    let mut max = input[end];

    for &elem in input.iter().take(end).skip(start) {
        if elem > max {
            max = elem
        };
        if elem < min {
            min = elem
        };
    }

    max + min
}

#[bench]
fn bench_part1_solution(b: &mut test::Bencher) {
    let input = get_input();
    b.iter(|| part1(&input))
}

#[bench]
fn bench_part2_solution(b: &mut test::Bencher) {
    let input = get_input();
    let part_1_answer = part1(&input);

    b.iter(|| part2(&input, part_1_answer))
}

#[bench]
fn bench_get_input(b: &mut test::Bencher) {
    b.iter(|| get_input());
}
