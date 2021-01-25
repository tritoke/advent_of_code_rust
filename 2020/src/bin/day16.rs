#![feature(binary_heap_into_iter_sorted)]
#![feature(test)]
extern crate test;

use std::num::ParseIntError;
use std::ops::RangeInclusive;
use std::str::FromStr;

use std::cmp::Ordering;
use std::collections::BinaryHeap;

type Ticket = Vec<usize>;
type Input = (Vec<Rule>, Ticket, Vec<Ticket>);

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
        0 => include_str!("../inputs/day16.inp"),
        1 => include_str!("../test_inputs/day16.inp1"),
        _ => panic!("Unknown input number: {:?}", INPUT_NUM),
    };

    let mut blocks = in_str.split("\n\n");
    let rules: Vec<Rule> = blocks
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    let your_ticket_str = blocks.next().unwrap();
    let your_ticket_offset = your_ticket_str.find('\n').unwrap();
    let ticket_str = your_ticket_str.get(your_ticket_offset + 1..).unwrap();
    let your_ticket: Ticket = parse_ticket(ticket_str);

    let nearby_tickets: Vec<Ticket> = blocks
        .next()
        .unwrap()
        .lines()
        .skip(1)
        .map(|line| parse_ticket(line))
        .collect();

    (rules, your_ticket, nearby_tickets)
}

fn part1(input: &Input) -> usize {
    let (rules, _, nearby) = input;

    nearby
        .iter()
        .flat_map(|ticket| {
            ticket
                .iter()
                .filter(|num| !rules.iter().any(|rule| rule.validate(*num)))
        })
        .sum()
}

fn part2(input: &Input) -> usize {
    let (rules, my_ticket, nearby) = input;

    // iterator over valid nearby tickets
    let nearby_filt = nearby.iter().filter(|ticket| {
        // filter on first ticket number for each one
        ticket
            .iter()
            .all(|ref num| rules.iter().any(|rule| rule.validate(num)))
    });

    // represent a rule being applicable to a position by
    // bit <pos> being set in the usize at position <rule_num>
    let mut poss_rules = vec![usize::MAX; rules.len()];

    for ticket in nearby_filt {
        for (tic_idx, num) in ticket.iter().enumerate() {
            for (rul_idx, rule) in rules.iter().enumerate() {
                if poss_rules[rul_idx].test_bit(tic_idx as u32) && !rule.validate(num) {
                    poss_rules[rul_idx].clear_bit(tic_idx as u32);
                }
            }
        }
    }

    let mut minheap: BinaryHeap<RuleBits> = Default::default();

    let mask = usize::MAX >> (64 - rules.len());
    for (rule_no, rule) in poss_rules.iter().enumerate() {
        let rb = RuleBits {
            rule_no,
            bit_pat: *rule & mask,
        };
        minheap.push(rb);
    }

    let mut product = 1;
    let mut eliminated = mask;
    for rule in minheap.into_iter_sorted() {
        let masked = rule.bit_pat & eliminated;

        let field_no = masked.trailing_zeros();

        eliminated.clear_bit(field_no);

        if rules[rule.rule_no].rule_name.starts_with("departure") {
            product *= my_ticket[field_no as usize];
        }
    }

    product
}

fn parse_ticket(s: &str) -> Vec<usize> {
    s.split(',').map(|line| line.parse().unwrap()).collect()
}

#[derive(Debug)]
struct Rule {
    rule_name: String,
    range_1: RangeInclusive<usize>,
    range_2: RangeInclusive<usize>,
}

impl Rule {
    fn validate(&self, n: &usize) -> bool {
        self.range_1.contains(n) || self.range_2.contains(n)
    }
}

impl FromStr for Rule {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let colon_pos = s.find(':').unwrap();
        let rule_name = s.get(..colon_pos).unwrap();

        let o_pos = s.find(" o").unwrap();

        let rule_1_str = s.get(colon_pos + 2..o_pos).unwrap();
        let rule_2_str = s.get(o_pos + 4..).unwrap();

        let r1_dash_pos = rule_1_str.find('-').unwrap();
        let r2_dash_pos = rule_2_str.find('-').unwrap();

        let r1_lower = rule_1_str.get(..r1_dash_pos).unwrap();
        let r1_upper = rule_1_str.get(r1_dash_pos + 1..).unwrap();

        let r2_lower = rule_2_str.get(..r2_dash_pos).unwrap();
        let r2_upper = rule_2_str.get(r2_dash_pos + 1..).unwrap();

        let r1_low_fromstr: usize = r1_lower.parse()?;
        let r1_upp_fromstr: usize = r1_upper.parse()?;
        let r2_low_fromstr: usize = r2_lower.parse()?;
        let r2_upp_fromstr: usize = r2_upper.parse()?;

        Ok(Self {
            rule_name: rule_name.to_string(),
            range_1: (r1_low_fromstr..=r1_upp_fromstr),
            range_2: (r2_low_fromstr..=r2_upp_fromstr),
        })
    }
}

struct RuleBits {
    rule_no: usize,
    bit_pat: usize,
}

impl Ord for RuleBits {
    fn cmp(&self, other: &Self) -> Ordering {
        self.bit_pat
            .count_ones()
            .cmp(&other.bit_pat.count_ones())
            .reverse()
    }
}

impl PartialOrd for RuleBits {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for RuleBits {
    fn eq(&self, other: &Self) -> bool {
        self.bit_pat.count_ones() == other.bit_pat.count_ones()
    }
}

impl Eq for RuleBits {}

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
