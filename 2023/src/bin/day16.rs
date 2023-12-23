use std::collections::BTreeSet;

fn main() {
    let inp = parse_input();

    let p1 = part1(&inp);
    let p2 = part2(&inp);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn parse_input() -> Vec<&'static [u8]> {
    include_str!("../../inputs/day16.txt")
        .lines()
        .map(|line| line.as_bytes())
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    #[rustfmt::skip]
    fn step(&self, grid: &[&[u8]], x: usize, y: usize) -> Option<(usize, usize)> {
        let max_x = grid.len() - 1;
        let max_y = grid[0].len() - 1;
        match self {
            Self::Up    if y != 0    => Some((x, y - 1)),
            Self::Down  if y < max_y => Some((x, y + 1)),
            Self::Left  if x != 0    => Some((x - 1, y)),
            Self::Right if x < max_x => Some((x + 1, y)),
            _ => None
        }
    }

    #[rustfmt::skip]
    fn encounter(self, tile: u8) -> [Option<Direction>; 2] {
        match (self, tile) {
            (dir, b'.')
            | (dir @ (Self::Up | Self::Down), b'|')
            | (dir @ (Self::Left | Self::Right), b'-') => [Some(dir), None],
            (Self::Up, b'/')    | (Self::Down, b'\\')  => [Some(Self::Right), None],
            (Self::Up, b'\\')   | (Self::Down, b'/')   => [Some(Self::Left),  None],
            (Self::Left, b'/')  | (Self::Right, b'\\') => [Some(Self::Down),  None],
            (Self::Left, b'\\') | (Self::Right, b'/')  => [Some(Self::Up),    None],
            (Self::Up | Self::Down, b'-')              => [Some(Self::Left),  Some(Self::Right)],
            (Self::Left | Self::Right, b'|')           => [Some(Self::Up),    Some(Self::Down)],
            (dir, _) => panic!("Unhandled combination: self={dir:?}, tile={}", tile as char),
        }
    }
}

fn count_energised(inp: &[&[u8]], init: (usize, usize, Direction)) -> u32 {
    let mut energised = Vec::new();
    energised.resize_with(inp.len(), || vec![false; inp[0].len()]);

    let mut to_explore = vec![init];
    let mut explored = BTreeSet::new();
    while let Some((x, y, dir)) = to_explore.pop() {
        energised[y][x] = true;
        if !explored.insert((x, y, dir.clone())) {
            continue;
        }

        let next_states = dir
            .encounter(inp[y][x])
            .into_iter()
            .flatten()
            .filter_map(|dir| dir.step(inp, x, y).map(|(x, y)| (x, y, dir)));

        to_explore.extend(next_states);
    }

    energised.into_iter().flatten().filter(|x| *x).count() as u32
}

fn part1(inp: &[&[u8]]) -> u32 {
    count_energised(inp, (0, 0, Direction::Right))
}

fn part2(inp: &[&[u8]]) -> u32 {
    let max_x = inp[0].len() - 1;
    let max_y = inp.len() - 1;

    let top = (0..=max_x).map(|x| (x, 0, Direction::Down));
    let bottom = (0..=max_x).map(|x| (x, max_y, Direction::Up));
    let left = (0..=max_y).map(|y| (0, y, Direction::Right));
    let right = (0..=max_y).map(|y| (max_x, y, Direction::Left));

    top.chain(bottom)
        .chain(left)
        .chain(right)
        .map(|init| count_energised(inp, init))
        .max()
        .unwrap()
}
