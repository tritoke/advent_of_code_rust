#![feature(test)]
extern crate test;

type Input = Vec<usize>;
type PartInput = [usize];
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
        0 => include_str!("../inputs/day10.inp"),
        1 => include_str!("../test_inputs/day10.inp1"),
        2 => include_str!("../test_inputs/day10.inp2"),
        _ => panic!("Unknown input number: {:?}", INPUT_NUM),
    };

    let mut inp: Input = in_str
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    inp.sort_unstable();

    inp
}

fn part1(input: &PartInput) -> usize {
    let mut diff_1 = 0;
    let mut diff_3 = 1;

    match input[0] {
        1 => diff_1 += 1,
        3 => diff_3 += 1,
        _ => (),
    }

    for w in input.windows(2) {
        let diff = w[1] - w[0];

        match diff {
            1 => diff_1 += 1,
            3 => diff_3 += 1,
            _ => (),
        }
    }

    diff_1 * diff_3
}

fn part2(input: &PartInput) -> usize {
    let mut perms = 1;
    let mut group_len = 1;

    // if we start with a 1 then we need add one to the initial group size
    if input[0] == 1 {
        group_len = 2;
    }

    // iterate through the list and count the number of elements with a difference of 1
    for w in input.windows(2) {
        let diff = w[1] - w[0];

        if diff == 1 {
            group_len += 1
        } else {
            // extend match if larger groups are found
            perms *= calc_perms(group_len);

            // reset the length of the group
            group_len = 1;
        }
    }

    perms * calc_perms(group_len)
}

fn calc_perms(group_size: usize) -> usize {
    match group_size {
        1 | 2 => 1,
        3 => 2,
        4 => 4,
        5 => 7,
        _ => panic!("Don't know how to handle groups of length {:?}", group_size),
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
