#![feature(test)]
extern crate test;

use either::{Either, Left, Right};
use rustc_hash::FxHashSet;

type Coord3d = (i32, i32, i32);
type Coord4d = (i32, i32, i32, i32);
type Position = Either<Coord3d, Coord4d>;
type Input = FxHashSet<Position>;

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
        0 => include_str!("../inputs/day17.inp"),
        1 => include_str!("../test_inputs/day17.inp1"),
        _ => panic!("Unknown input number: {:?}", INPUT_NUM),
    };

    in_str
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter(|(_, c)| *c == '#')
                .map(move |(x, _)| Left((x as i32, y as i32, 0i32)))
        })
        .collect()
}

fn part1(input: &Input) -> usize {
    let mut grid = cycle(input);

    for _ in 0..5 {
        grid = cycle(&grid);
    }

    grid.len()
}

// use either on the pos to get the nice overrides
fn part2(input: &Input) -> usize {
    let mut grid: Input = Default::default();

    // add the w thing
    grid.extend(
        input
            .iter()
            .map(|pos| pos.left_and_then(|(x, y, z)| Right((x, y, z, 0i32)))),
    );

    for _ in 0..6 {
        grid = cycle(&grid);
    }

    grid.len()
}

fn cycle(space: &Input) -> Input {
    let mut new_space: Input = Default::default();

    for &point in space {
        let num_neighbours = count_neighbours(space, point);

        if num_neighbours == 2 || num_neighbours == 3 {
            new_space.insert(point);
        }

        for neighbour in iter_neighbours(point) {
            if count_neighbours(space, neighbour) == 3 {
                new_space.insert(neighbour);
            }
        }
    }

    new_space
}

fn count_neighbours(space: &Input, pos: Position) -> usize {
    let mut num_neighbours: usize = 0;

    for neighbour in iter_neighbours(pos) {
        if space.contains(&neighbour) {
            num_neighbours += 1;
        }

        // we have no need to count past 3
        if num_neighbours >= 4 {
            break;
        }
    }

    num_neighbours
}

#[allow(dead_code)]
fn print_grid(grid: &Input) {
    let is_coord3d = grid.iter().next().unwrap().is_left();

    let mut x_bounds = (0, 0);
    let mut y_bounds = (0, 0);
    let mut z_bounds = (0, 0);
    let mut w_bounds = (0, 0);

    for coord in grid.iter() {
        let (x, y, z, w) = match coord {
            Left((x, y, z)) => (x, y, z, &0i32),
            Right((x, y, z, w)) => (x, y, z, w),
        };

        if x < &x_bounds.0 {
            x_bounds.0 = *x
        }
        if x > &x_bounds.1 {
            x_bounds.1 = *x
        }

        if y < &y_bounds.0 {
            y_bounds.0 = *y
        }
        if y > &y_bounds.1 {
            y_bounds.1 = *y
        }

        if z < &z_bounds.0 {
            z_bounds.0 = *z
        }
        if z > &z_bounds.1 {
            z_bounds.1 = *z
        }

        if !is_coord3d {
            if w < &w_bounds.0 {
                w_bounds.0 = *w
            }
            if w > &w_bounds.1 {
                w_bounds.1 = *w
            }
        }
    }

    for w in w_bounds.0..=w_bounds.1 {
        for z in z_bounds.0..=z_bounds.1 {
            if is_coord3d {
                println!("z={}", z);
            } else {
                println!("z={}, w={}", z, w);
            }

            for y in y_bounds.0..=y_bounds.1 {
                for x in x_bounds.0..=x_bounds.1 {
                    let point = if is_coord3d {
                        Left((x, y, z))
                    } else {
                        Right((x, y, z, w))
                    };

                    if grid.contains(&point) {
                        print!("#");
                    } else {
                        print!(".");
                    }
                }
                println!();
            }
            println!();
        }
    }
}

fn iter_neighbours(pos: Position) -> Neighbours {
    Neighbours { pos, iter_no: 0 }
}

struct Neighbours {
    pos: Position,
    iter_no: i32,
}

impl Iterator for Neighbours {
    type Item = Position;

    fn next(&mut self) -> Option<Self::Item> {
        let is_coord3d = self.pos.is_left();

        let max = if is_coord3d { 27 } else { 81 };

        let (x, y, z, w) = match self.pos {
            Left((x, y, z)) => (x, y, z, 0),
            Right((x, y, z, w)) => (x, y, z, w),
        };

        if self.iter_no < max {
            // when iter_no is half of max we get identity position
            // which maps all diffs to 0 so skip one
            if self.iter_no == max / 2 {
                self.iter_no += 1;
            }

            let x_diff = self.iter_no % 3 - 1;
            let y_diff = self.iter_no / 3 % 3 - 1;
            let z_diff = self.iter_no / 9 % 3 - 1;
            let w_diff = self.iter_no / 27 % 3 - 1;

            self.iter_no += 1;

            if is_coord3d {
                Some(Left((x + x_diff, y + y_diff, z + z_diff)))
            } else {
                Some(Right((x + x_diff, y + y_diff, z + z_diff, w + w_diff)))
            }
        } else {
            None
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
fn bench_cycle(b: &mut test::Bencher) {
    let input = get_input();

    b.iter(|| cycle(&input))
}

#[bench]
fn bench_get_input(b: &mut test::Bencher) {
    b.iter(|| get_input());
}
