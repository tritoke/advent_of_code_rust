use core::fmt;
use core::fmt::Write;
use core::hash::Hash;
use std::collections::HashMap;

fn main() {
    let inp = parse_input();

    let p1 = part1(&inp);
    let p2 = part2(&inp);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn parse_input() -> Vec<&'static [u8]> {
    include_str!("../../inputs/day14.txt")
        .lines()
        .map(|line| line.as_bytes())
        .collect()
}

fn part1(inp: &[&[u8]]) -> u32 {
    let mut total = 0;

    for col in 0..inp[0].len() {
        let mut top_free = 0;
        for (i, row) in inp.iter().enumerate() {
            match row[col] {
                b'.' => (),
                // this isn't necessarily correct but we will hit the # and correct it before we
                // hit an o
                b'#' => top_free = i + 1,
                b'O' => {
                    total += inp.len() - top_free;
                    top_free += 1;
                }
                _ => unreachable!("other characters are cringe"),
            }
        }
    }

    total as u32
}

#[derive(Debug, Hash, Clone, PartialEq, Eq)]
struct Platform {
    tiles: Vec<u8>,
    row_width: usize,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (i, c) in self.tiles.iter().enumerate() {
            f.write_char(*c as char)?;
            if (i + 1) % self.row_width == 0 {
                f.write_char('\n')?;
            }
        }

        Ok(())
    }
}

impl From<&[&[u8]]> for Platform {
    fn from(value: &[&[u8]]) -> Self {
        Self {
            tiles: value.iter().flat_map(|row| row.iter().copied()).collect(),
            row_width: value[0].len(),
        }
    }
}

impl Platform {
    fn compute_idx(&self, (row, col): (usize, usize)) -> usize {
        row * self.row_width + col
    }

    fn swap(&mut self, a: (usize, usize), b: (usize, usize)) {
        let a = self.compute_idx(a);
        let b = self.compute_idx(b);

        if a == b {
            return;
        }

        debug_assert_eq!(self.tiles[a], b'O');
        debug_assert_eq!(self.tiles[b], b'.');
        self.tiles.swap(a, b);
    }

    fn spin(&mut self) {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East);
    }

    fn tilt(&mut self, direction: Direction) {
        let no_cols = self.tiles.len() / self.row_width;
        let no_rows = self.row_width;

        let (outer_lim, inner_lim, rev_inner) = match direction {
            Direction::North => (no_cols, no_rows, false),
            Direction::South => (no_cols, no_rows, true),
            Direction::East => (no_rows, no_cols, true),
            Direction::West => (no_rows, no_cols, false),
        };

        for o in 0..outer_lim {
            let mut top_free = if rev_inner { inner_lim - 1 } else { 0 };
            for mut i in 0..inner_lim {
                if rev_inner {
                    i = inner_lim - 1 - i;
                }

                #[rustfmt::skip]
                let tile_idx = match direction {
                    Direction::East  | Direction::West  => self.compute_idx((o, i)),
                    Direction::North | Direction::South => self.compute_idx((i, o)),
                };

                match self.tiles[tile_idx] {
                    b'.' => (),
                    b'#' => {
                        top_free = if rev_inner {
                            i.saturating_sub(1)
                        } else {
                            i + 1
                        }
                    }
                    b'O' => {
                        match direction {
                            Direction::East | Direction::West => self.swap((o, i), (o, top_free)),
                            Direction::North | Direction::South => self.swap((i, o), (top_free, o)),
                        }

                        top_free = if rev_inner {
                            // this panics if we are at the end but we don't actually care at that
                            // point so saturate it instead
                            top_free.saturating_sub(1)
                        } else {
                            top_free + 1
                        };
                    }
                    _ => unreachable!("other characters are cringe"),
                }
            }
        }
    }

    fn north_weight(&self) -> u32 {
        self.tiles
            .iter()
            .enumerate()
            .filter_map(|(i, c)| (*c == b'O').then_some(i))
            .map(|i| {
                let row = i / self.row_width;
                let rows = self.tiles.len() / self.row_width;
                (rows - row) as u32
            })
            .sum()
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    North,
    West,
    South,
    East,
}

fn part2(inp: &[&[u8]]) -> u32 {
    const LIMIT: usize = 1_000_000_000;
    let mut plat: Platform = inp.into();

    let mut seen = vec![plat.clone()];
    let mut state_to_last_seen = HashMap::new();
    for i in 0..LIMIT {
        plat.spin();

        if let Some(last_seen) = state_to_last_seen.insert(plat.clone(), i) {
            let period = i - last_seen;
            return seen[last_seen + ((LIMIT - last_seen) % period)].north_weight();
        }

        seen.push(plat.clone());
    }

    plat.north_weight()
}
