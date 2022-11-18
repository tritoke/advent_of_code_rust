#![feature(test)]
extern crate test;

use std::cmp::Ordering;
use std::collections::{HashSet, VecDeque};

const INPUT_NUM: usize = 0;

fn main() {
    let (player_1, player_2) = get_input();

    let part_1 = part1(&player_1, &player_2);
    let part_2 = part2(&player_1, &player_2);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn get_input() -> (Vec<usize>, Vec<usize>) {
    let (plr1, plr2) = [
        include_str!("../inputs/day22.inp"),
        include_str!("../test_inputs/day22.inp1"),
        include_str!("../test_inputs/day22.inp2"),
    ][INPUT_NUM]
        .split_once("\n\n")
        .expect("Invalid input file.");

    (
        plr1.lines()
            .skip(1)
            .map(|line| {
                line.parse()
                    .expect("Found non numeric value in player data.")
            })
            .collect(),
        plr2.lines()
            .skip(1)
            .map(|line| {
                line.parse()
                    .expect("Found non numeric value in player data.")
            })
            .collect(),
    )
}

fn part1(p1_deck: &[usize], p2_deck: &[usize]) -> usize {
    let mut game = GameState::new(p1_deck, p2_deck);

    while !game.is_over() {
        let (p1, p2) = game.draw_cards_unchecked();

        match p1.cmp(&p2) {
            Ordering::Greater => {
                game.player_1.push_back(p1);
                game.player_1.push_back(p2);
            }
            Ordering::Less => {
                game.player_2.push_back(p2);
                game.player_2.push_back(p1);
            }
            Ordering::Equal => unreachable!("Draws are impossible."),
        }
    }

    let winner = game.decide_winner();

    winner
        .iter()
        .zip((1..=winner.len()).rev())
        .fold(0, |acc, (a, b)| acc + (a * b))
}

fn part2(p1_deck: &[usize], p2_deck: &[usize]) -> usize {
    let mut game = GameState::new(p1_deck, p2_deck);

    let mut prev_states: HashSet<GameState> = Default::default();
    let mut game_stack: Vec<GameState> = Default::default();

    let winner = loop {
        if prev_states.contains(&game) {
            break &game.player_1;
        }

        match game.draw_cards() {
            (Some(p1), Some(p2)) => {
                let (p1_cards, p2_cards) = game.num_cards();
            }
            (Some(_), None) => (), // player 2 win
            (None, Some(_)) => (), // player 2 win
            (None, None) => unreachable!("Both players cannot run out of cards."),
        }
    };

    0
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct GameState {
    player_1: VecDeque<usize>,
    player_2: VecDeque<usize>,
}

impl GameState {
    fn new(p1_deck: &[usize], p2_deck: &[usize]) -> Self {
        Self {
            player_1: p1_deck.iter().copied().collect(),
            player_2: p2_deck.iter().copied().collect(),
        }
    }

    fn is_over(&self) -> bool {
        self.player_1.is_empty() || self.player_2.is_empty()
    }

    fn draw_cards(&mut self) -> (Option<usize>, Option<usize>) {
        (self.player_1.pop_front(), self.player_2.pop_front())
    }

    fn draw_cards_unchecked(&mut self) -> (usize, usize) {
        (
            self.player_1.pop_front().unwrap(),
            self.player_2.pop_front().unwrap(),
        )
    }

    fn num_cards(&mut self) -> (usize, usize) {
        (self.player_1.len(), self.player_2.len())
    }

    fn decide_winner(&self) -> &VecDeque<usize> {
        if self.player_1.is_empty() {
            &self.player_2
        } else {
            &self.player_1
        }
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
