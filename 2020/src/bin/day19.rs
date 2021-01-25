#![feature(box_syntax)]
#![feature(box_patterns)]
#![feature(test)]
extern crate test;

use fnv::FnvHashMap;

use std::str::Chars;

use std::num::ParseIntError;
use std::str::FromStr;

type Rules = FnvHashMap<usize, Rule>;
type Cases = Vec<&'static str>;
type Input = (Rules, Cases);

const INPUT_NUM: i32 = 0;

fn main() {
    let input = get_input();

    let part_1 = part1(&input);
    let part_2 = part2(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn get_input() -> Input {
    let input = match INPUT_NUM {
        0 => include_str!("../inputs/day19.inp"),
        1 => include_str!("../test_inputs/day19.inp1"),
        2 => include_str!("../test_inputs/day19.inp2"),
        _ => panic!("Unknown input number: {:?}", INPUT_NUM),
    };

    let mut groups = input.split("\n\n");
    let rules_str = groups.next().unwrap();
    let tests_str = groups.next().unwrap();

    let rules: Rules = rules_str
        .lines()
        .map(|line| {
            let colon_pos = line.find(':').unwrap();
            let num_str = line.get(..colon_pos).unwrap();
            let rule_no = num_str.parse::<usize>().unwrap();

            let space_pos = line.find(' ').unwrap();
            let rulestr = line.get(space_pos + 1..).unwrap();
            let rule: Rule = rulestr.parse().unwrap();

            (rule_no, rule)
        })
        .collect();

    let tests: Cases = tests_str.lines().collect();

    (rules, tests)
}

fn part1(input: &Input) -> usize {
    let (rules, cases) = input;

    cases
        .iter()
        .filter(|case| {
            let mut char_iter = case.chars();
            let rule_matches = rules[&0].matches(&rules, &mut char_iter);
            let is_full_match = char_iter.next().is_none();

            rule_matches && is_full_match
        })
        .count()
}

fn part2(input: &Input) -> usize {
    let (rules, cases) = input;

    cases
        .iter()
        .filter(|case| {
            let mut iter = case.chars();

            // try and match as many 42s as we can
            let mut num_42s = 0;
            loop {
                let save_iter = iter.clone();

                if !rules[&42].matches(&rules, &mut iter) {
                    iter = save_iter;
                    break;
                } else {
                    num_42s += 1;
                }
            }

            let mut num_31s = 0;
            loop {
                let save_iter = iter.clone();

                if !rules[&31].matches(&rules, &mut iter) {
                    iter = save_iter;
                    break;
                } else {
                    num_31s += 1;
                }
            }

            iter.next().is_none() && num_42s > num_31s && num_31s > 0
        })
        .count()
}

#[derive(Debug)]
enum Rule {
    Sequence(Vec<usize>),
    Alternation(Box<(Rule, Rule)>),
    Atom(char),
}

impl FromStr for Rule {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s.starts_with('"') {
            Rule::Atom(s.chars().nth(1).unwrap())
        } else if s.contains('|') {
            let split: Vec<&str> = s.split(" | ").collect();

            Rule::Alternation(box (split[0].parse()?, split[1].parse()?))
        } else {
            Rule::Sequence(s.split_whitespace().map(|i| i.parse().unwrap()).collect())
        })
    }
}

impl Rule {
    fn matches(&self, rules: &Rules, mut iter: &mut Chars) -> bool {
        match self {
            Rule::Atom(c) => match iter.next() {
                Some(chr) => chr == *c,
                None => false,
            },
            Rule::Alternation(box (rule_1, rule_2)) => {
                // first clone the iterator so we can try the second alternative from the current
                // position - this is a shallow clone so it is cheap :)
                let mut cloned_iter = iter.clone();

                if rule_1.matches(rules, &mut cloned_iter) {
                    *iter = cloned_iter;
                    true
                } else {
                    rule_2.matches(rules, &mut iter)
                }
            }
            Rule::Sequence(rule_seq) => rule_seq
                .iter()
                .map(|rule_no| rules[rule_no].matches(rules, &mut iter))
                .all(|b| b),
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
