#![feature(test)]
extern crate test;

use rustc_hash::FxHashMap;

type Map = FxHashMap<usize, usize>;

type Input = Vec<usize>;
type PartInput = [usize];
const INPUT_NUM: usize = 0;

fn main() {
    let input = get_input(INPUT_NUM);

    let part_1 = part1(&input);
    let part_2 = part2(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn get_input(input_num: usize) -> Input {
    let in_str = match input_num {
        0 => include_str!("../inputs/day15.inp"),
        1 => include_str!("../test_inputs/day15.inp1"),
        2 => include_str!("../test_inputs/day15.inp2"),
        3 => include_str!("../test_inputs/day15.inp3"),
        4 => include_str!("../test_inputs/day15.inp4"),
        5 => include_str!("../test_inputs/day15.inp5"),
        6 => include_str!("../test_inputs/day15.inp6"),
        7 => include_str!("../test_inputs/day15.inp7"),
        _ => panic!("Unknown input number: {:?}", input_num),
    };

    in_str
        .split(',')
        .map(|num| num.trim_end().parse().unwrap())
        .collect()
}

fn part1(input: &PartInput) -> usize {
    let mut game = Game::new(input);

    game.nth(2020 - 1).unwrap()
}

fn part2(input: &PartInput) -> usize {
    let mut game = Game::new(input);

    game.nth(30000000 - 1).unwrap()
}

struct Game<'a> {
    starting_numbers: &'a PartInput,
    last_turns: Map,
    last_no: usize,
    iter_no: usize,
    last_is_new: bool,
}

impl<'a> Game<'a> {
    fn new(input: &'a PartInput) -> Self {
        Self {
            starting_numbers: input,
            last_turns: Default::default(),
            last_no: 0,
            iter_no: 0,
            last_is_new: false,
        }
    }
}

impl Iterator for Game<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        // inserting the starting number still
        let next_no;

        // in the starting numbers
        if self.iter_no < self.starting_numbers.len() {
            next_no = self.starting_numbers[self.iter_no];
        } else if self.last_is_new {
            next_no = 0;
        } else {
            let last_seen = self.last_turns[&self.last_no];
            next_no = self.iter_no - last_seen - 1;
            self.last_turns.insert(self.last_no, self.iter_no - 1);
        }

        // update map entry
        let last = self.last_turns.entry(next_no).or_insert(self.iter_no);

        self.last_is_new = *last == self.iter_no;

        // update last_no
        self.last_no = next_no;

        // update iter_no
        self.iter_no += 1;

        Some(next_no)
    }
}

#[bench]
fn bench_part1_solution(b: &mut test::Bencher) {
    let input = get_input(0);
    b.iter(|| part1(&input))
}

#[bench]
fn bench_part2_solution(b: &mut test::Bencher) {
    let input = get_input(0);

    b.iter(|| part2(&input))
}

#[bench]
fn bench_get_input(b: &mut test::Bencher) {
    b.iter(|| get_input(0));
}
