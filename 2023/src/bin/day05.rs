use anyhow::anyhow;
use std::{mem, str::FromStr};

fn main() {
    let inp = parse_input();

    let p1 = part1(&inp);
    let p2 = part2(&inp);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn parse_input() -> Almanac {
    include_str!("../../inputs/day05.txt")
        .parse()
        .expect("Failed to parse input.")
}

#[derive(Debug)]
struct Mapping {
    src: u32,
    dst: u32,
    len: u32,
}

impl Mapping {
    #[allow(clippy::unnecessary_lazy_evaluations)]
    fn map_backward(&self, inp: u32) -> Option<u32> {
        (self.dst..self.dst + self.len)
            .contains(&inp)
            .then(|| inp - self.dst + self.src)
    }
}

impl FromStr for Mapping {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dst, src_len) = s
            .split_once(' ')
            .ok_or_else(|| anyhow!("Failed to split mapping."))?;

        let (src, len) = src_len
            .split_once(' ')
            .ok_or_else(|| anyhow!("Failed to split mapping."))?;

        Ok(Self {
            src: src.parse()?,
            dst: dst.parse()?,
            len: len.parse()?,
        })
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u32>,
    maps: Vec<Vec<Mapping>>,
}

impl FromStr for Almanac {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();

        let seeds = lines[0]
            .split_ascii_whitespace()
            .skip(1)
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        let mut maps = vec![];
        let mut curr_map = vec![];

        let mut i = 3;
        while i < lines.len() {
            let line = lines[i];
            i += 1;

            if line.is_empty() {
                maps.push(mem::take(&mut curr_map));
                i += 1;
                continue;
            }

            curr_map.push(line.parse()?);
        }

        if !curr_map.is_empty() {
            maps.push(curr_map);
        }

        Ok(Self { seeds, maps })
    }
}

impl Almanac {
    fn location_to_seed(&self, location: u32) -> u32 {
        self.maps.iter().rev().fold(location, |num, map| {
            map.iter()
                .find_map(|mapping| mapping.map_backward(num))
                .unwrap_or(num)
        })
    }

    fn in_seed_range(&self, seed: u32) -> bool {
        self.seeds
            .chunks(2)
            .any(|chunk| (chunk[0]..chunk[0] + chunk[1]).contains(&seed))
    }
}

fn part1(inp: &Almanac) -> u32 {
    (0..u32::MAX)
        .find(|&location| {
            let seed = inp.location_to_seed(location);
            inp.seeds.contains(&seed)
        })
        .unwrap()
}

fn part2(inp: &Almanac) -> u32 {
    (0..u32::MAX)
        .find(|&location| {
            let seed = inp.location_to_seed(location);
            inp.in_seed_range(seed)
        })
        .unwrap()
}
