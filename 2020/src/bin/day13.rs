#![feature(test)]
extern crate test;

use std::num::ParseIntError;
use std::str::FromStr;

type Input = (usize, Vec<Route>);
const INPUT_NUM: i32 = 0;

fn main() {
    let input = get_input().unwrap();

    let part_1 = part1(&input);
    let part_2 = part2(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn get_input() -> Option<Input> {
    let in_str = match INPUT_NUM {
        0 => include_str!("../inputs/day13.inp"),
        1 => include_str!("../test_inputs/day13.inp1"),
        _ => panic!("Unknown input number: {:?}", INPUT_NUM),
    };

    let mut lines = in_str.lines();

    let earliest_ts = lines.next()?.parse().unwrap();

    let routes = lines
        .next()?
        .split(',')
        .map(|line| line.parse().unwrap())
        .collect();

    Some((earliest_ts, routes))
}

fn part1(input: &Input) -> usize {
    let (earliest_ts, routes) = input;

    let min_bus = routes
        .iter()
        .filter(|route| route.in_service())
        .map(|route| route.get_number())
        .min_by_key(|n| n - (earliest_ts % n))
        .unwrap();

    let time_till_bus = min_bus - (earliest_ts % min_bus);

    time_till_bus * min_bus
}

fn part2(input: &Input) -> i64 {
    let (_, routes) = input;
    let in_service = routes
        .iter()
        .enumerate()
        .filter(|(_, route)| route.in_service());

    let mut route_numbers: Vec<i64> = Vec::new();
    let mut offsets: Vec<i64> = Vec::new();

    for (i, route) in in_service {
        offsets.push(i as i64);
        route_numbers.push(route.get_number() as i64);
    }

    crt(&offsets[..], &route_numbers[..]).abs()
}

#[derive(Debug)]
enum Route {
    Bus(usize),
    OutOutService,
}

impl FromStr for Route {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if s == "x" {
            Route::OutOutService
        } else {
            Route::Bus(s.parse().unwrap())
        })
    }
}

impl Route {
    fn in_service(&self) -> bool { matches!(self, Route::Bus(_)) }

    fn get_number(&self) -> usize {
        match self {
            Route::Bus(n) => *n,
            _ => panic!("Cannot get bus number for route: {:?}", self),
        }
    }
}

// adapted from https://docs.rs/ring-algorithm/0.2.2/ring_algorithm/
fn crt(u: &[i64], m: &[i64]) -> i64 {
    let mut v = Vec::with_capacity(u.len());

    for (i, (u_i, m_i)) in u.iter().zip(m.iter()).enumerate() {
        let coef_i = mod_inv(m[0..i].iter().fold(1, |p, v| (p * v) % m_i), *m_i);

        let t = v
            .iter()
            .zip(m.iter())
            .rev()
            .fold(0, |t, (v_j, m_j)| ((m_j * t) + v_j) % m_i);
        v.push(((u_i - t) * coef_i) % m_i);
    }

    let mut ret = v.pop().unwrap();
    for (v_i, m_i) in v.iter().zip(m.iter()).rev() {
        ret = (ret * m_i) + v_i;
    }

    ret
}

fn mod_inv(a: i64, m: i64) -> i64 {
    let (gcd, inv_a, _) = egcd(a, m);
    match gcd {
        1 => inv_a,
        _ => panic!("Unable to find inverse of {} % {}", a, m),
    }
}

fn egcd(x: i64, y: i64) -> (i64, i64, i64) {
    let mut old = (x, 1, 0);
    let mut now = (y, 0, 1);

    while now.0 != 0 {
        let q = old.0 / now.0;
        let new = (old.0 - q * now.0, old.1 - q * now.1, old.2 - q * now.2);
        old = now;
        now = new;
    }
    old
}

#[bench]
fn bench_part1_solution(b: &mut test::Bencher) {
    let input = get_input().unwrap();
    b.iter(|| part1(&input))
}

#[bench]
fn bench_part2_solution(b: &mut test::Bencher) {
    let input = get_input().unwrap();

    b.iter(|| part2(&input))
}

#[bench]
fn bench_get_input(b: &mut test::Bencher) {
    b.iter(|| get_input());
}
