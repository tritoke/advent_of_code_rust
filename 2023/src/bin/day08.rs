use std::collections::BTreeMap;
use std::mem;

fn main() {
    let inp = parse_input();

    let p1 = part1(&inp);
    let p2 = part2(&inp);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn parse_input() -> Network {
    let lines: Vec<_> = include_str!("../../inputs/day08.txt").lines().collect();

    let directions = lines[0];

    // build the map in two passes, first find all keys
    let name_to_index: BTreeMap<&str, usize> = lines
        .iter()
        .skip(2)
        .map(|line| line.split_once(' ').unwrap().0)
        .enumerate()
        .map(|(a, b)| (b, a))
        .collect();

    let index_to_name = lines
        .iter()
        .skip(2)
        .map(|line| line.split_once(' ').unwrap().0)
        .collect();

    // now build the map
    let map = lines
        .iter()
        .skip(2)
        .map(|line| {
            let (l, r) = line[7..15].split_once(", ").unwrap();
            (name_to_index[l], name_to_index[r])
        })
        .collect();

    Network {
        directions,
        index_to_name,
        name_to_index,
        map,
    }
}

#[derive(Debug)]
struct Network {
    directions: &'static str,
    index_to_name: Vec<&'static str>,
    name_to_index: BTreeMap<&'static str, usize>,
    map: Vec<(usize, usize)>,
}

fn part1(inp: &Network) -> u32 {
    let Network {
        directions,
        index_to_name: _,
        name_to_index,
        map,
    } = inp;
    let mut count = 0;
    let mut pos = name_to_index["AAA"];
    let end = name_to_index["ZZZ"];
    for dir in directions.bytes().cycle() {
        count += 1;
        pos = if dir == b'L' { map[pos].0 } else { map[pos].1 };
        if pos == end {
            break;
        }
    }

    count
}

fn find_orbit(start: usize, directions: &str, map: &[(usize, usize)]) -> usize {
    let mut pos = start;
    let mut history = BTreeMap::new();
    let mut count = 0;

    for (i, dir) in directions.bytes().enumerate().cycle() {
        count += 1;
        let (l, r) = map[pos];
        pos = if dir == b'L' { l } else { r };

        if let Some(offset) = history.insert((i, pos), count) {
            return count - offset;
        }
    }

    unreachable!(".cycle() creates infinite iterators");
}

fn part2(inp: &Network) -> u64 {
    let Network {
        directions,
        index_to_name,
        name_to_index,
        map,
    } = inp;

    index_to_name
        .iter()
        .filter(|name| name.ends_with('A'))
        .map(|name| find_orbit(name_to_index[name], directions, map) as u64)
        .fold(1, lcm)
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    if a == b {
        return a;
    }
    if b > a {
        mem::swap(&mut a, &mut b);
    }
    loop {
        if b == 0 {
            break a;
        }

        let rmdr = a % b;
        a = b;
        b = rmdr;
    }
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}
