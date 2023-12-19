use std::collections::BTreeSet;

type GridRef = (usize, usize);

fn main() {
    let inp = parse_input();

    let (path, p1) = part1(&inp);
    let p2 = part2(&inp, path);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn parse_input() -> Grid {
    let tiles = include_str!("../../inputs/day10.txt")
        .lines()
        .map(|line| line.as_bytes().to_owned())
        .collect();

    Grid { tiles }
}

struct Grid {
    tiles: Vec<Vec<u8>>,
}

impl Grid {
    fn find_start(&self) -> GridRef {
        self.tiles
            .iter()
            .enumerate()
            .find_map(|(row, line)| line.iter().position(|c| *c == b'S').map(|col| (row, col)))
            .unwrap()
    }

    fn rows(&self) -> usize {
        self.tiles.len()
    }

    fn cols(&self) -> usize {
        self.tiles[0].len()
    }

    fn cardinal_directions(&self, (row, col): GridRef) -> [Option<GridRef>; 4] {
        let up = row.checked_sub(1).map(|r| (r, col));
        let down = (row + 1 < self.rows()).then_some((row + 1, col));
        let left = col.checked_sub(1).map(|c| (row, c));
        let right = (col + 1 < self.cols()).then_some((row, col + 1));
        [up, down, left, right]
    }

    fn connections(&self, (row, col): GridRef) -> Option<[GridRef; 2]> {
        let [up, down, left, right] = self.cardinal_directions((row, col));

        match self.tiles[row][col] {
            b'|' => Some([up?, down?]),
            b'-' => Some([left?, right?]),
            b'L' => Some([up?, right?]),
            b'J' => Some([up?, left?]),
            b'7' => Some([left?, down?]),
            b'F' => Some([right?, down?]),
            b'S' => [up, down, left, right]
                .into_iter()
                .flatten()
                .filter(|neigh| {
                    self.connections(*neigh)
                        .into_iter()
                        .flatten()
                        .any(|gr| gr == (row, col))
                })
                .collect::<Vec<_>>()
                .try_into()
                .ok(),
            _ => None,
        }
    }

    fn categorise_start(&self, (row, col): GridRef) -> u8 {
        debug_assert_eq!(self.tiles[row][col], b'S');

        // first find connections
        let connections = self
            .connections((row, col))
            .expect("Failed to find connections to start node");

        let [up, down, left, right] = self.cardinal_directions((row, col));

        let conns = connections.map(Some);
        if conns == [up, down] {
            b'|'
        } else if conns == [left, right] {
            b'-'
        } else if conns == [up, left] {
            b'J'
        } else if conns == [up, right] {
            b'L'
        } else if conns == [down, left] {
            b'7'
        } else if conns == [down, right] {
            b'F'
        } else {
            unreachable!("I mean I hope it would match one of these lmao");
        }
    }
}

fn part1(inp: &Grid) -> (Vec<GridRef>, u32) {
    let start = inp.find_start();

    // build the entire loop
    let first_edge = inp.connections(start).expect("start has no conections")[0];
    let mut path = vec![start, first_edge];
    loop {
        let prev = *path.iter().rev().nth(1).expect("prev doesn't exist");
        let pos = *path.last().expect("path last doesn't exist");
        let conns = inp
            .connections(pos)
            .expect("path element has no connections");
        let next = if conns[0] == prev { conns[1] } else { conns[0] };
        if next == start {
            break;
        }
        path.push(next);
    }

    let ans = path.len() as u32 / 2;
    (path, ans)
}

fn part2(inp: &Grid, path: Vec<GridRef>) -> u32 {
    let path_set: BTreeSet<GridRef> = path.iter().copied().collect();

    let mut count = 0;
    for (row, row_tiles) in inp.tiles.iter().enumerate() {
        let mut in_region = false;
        let mut last_corner = None;
        for (col, mut tile) in row_tiles.iter().copied().enumerate() {
            if tile == b'S' {
                tile = inp.categorise_start((row, col));
            }
            if path_set.contains(&(row, col)) {
                match (last_corner, tile) {
                    (_, b'|') => in_region = !in_region,
                    (Some(b'L'), b'J') | (Some(b'F'), b'7') => last_corner = None,
                    (Some(b'F'), b'J') | (Some(b'L'), b'7') => {
                        in_region = !in_region;
                        last_corner = None;
                    }
                    (_, b'F' | b'L' | b'7' | b'J') => last_corner = Some(tile),
                    _ => (),
                }
            } else if in_region {
                count += 1;
            }
        }
    }

    count
}
