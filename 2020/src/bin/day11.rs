#![feature(test)]
extern crate test;

use grid::Grid;

type Input = Grid<Cell>;
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
        0 => include_str!("../inputs/day11.inp"),
        1 => include_str!("../test_inputs/day11.inp1"),
        _ => panic!("Unknown input number: {:?}", INPUT_NUM),
    };

    Grid::from_vec(
        in_str.lines()
              .flat_map(|line| line.chars().map(Cell::from_char))
              .collect(),
        in_str.find('\n').unwrap()
    )
}

fn part1(input: &Input) -> usize {
    let ruleset = 1;

    // make two mutable copies of the state to evolve the state between
    let board_a = &mut input.clone();
    let board_b = &mut input.clone();

    evolve_till_stable(board_a, board_b, ruleset);

    count_occupied(board_a)
}

fn part2(input: &Input) -> usize {
    let ruleset = 2;

    // make two mutable copies of the state to evolve the state between
    let board_a = &mut input.clone();
    let board_b = &mut input.clone();

    evolve_till_stable(board_a, board_b, ruleset);

    count_occupied(&board_a)
}

fn evolve_till_stable(board: &mut Input, copy_board: &mut Input, ruleset: usize) {
    let mut iter_no = 0;

    loop {
        let changes = match iter_no % 2 {
            0 => evolve_board(board, copy_board, ruleset),
            1 => evolve_board(copy_board, board, ruleset),
            _ => panic!("Bitch how did you break modulo??"),
        };

        if changes == 0 {
            break;
        }

        iter_no += 1;
    }
}

fn evolve_board(before_state: &Input, after_state: &mut Input, ruleset: usize) -> usize {
    let mut num_changes = 0;

    let occupied_limit = match ruleset {
        1 => 4,
        2 => 5,
        _ => panic!("Unknown ruleset: {}", ruleset),
    };

    let non_floor_iter = before_state.iter()
                                     .enumerate()
                                     .zip(after_state.iter_mut())
                                     .filter(|((_, cell), _)| !cell.is_floor());

    'cell_loop:
    for ((cell_no, cell), after_cell) in non_floor_iter {
        let cols = before_state.cols();
        let (cell_x, cell_y) = div_rem(cell_no, cols);

        let mut num_occupied = 0;
        for i in 0..9 {
            // i == 4 maps to an x and y offset of 0
            if i == 4 { continue };

            let (x_off, y_off) = {
                let (i, j) = div_rem(i, 3);
                
                (i as isize - 1, j as isize - 1)
            };

            match ruleset {
                1 => {
                    let (x, y) = (cell_x as isize + x_off, cell_y as isize + y_off);

                    if let Some(Cell::Occupied) = before_state.get(x as usize, y as usize) {
                        num_occupied += 1;
                    }
                },
                2 => {
                    let (mut x, mut y) = (cell_x as isize + x_off, cell_y as isize + y_off);
                    loop {
                        match before_state.get(x as usize, y as usize) {
                            Some(Cell::Occupied) => {
                                num_occupied += 1;
                                break;
                            },
                            Some(Cell::Floor) => (/* do nothing */),
                            Some(Cell::Empty) | None => break,
                        }

                        x += x_off;
                        y += y_off;
                    }
                }
                _ => panic!("Unrecognised ruleset: {}", ruleset)
            }

            // early exit if four or more seats are occupied
            if cell.is_occupied() && num_occupied >= occupied_limit {
                *after_cell = Cell::Empty;
                num_changes += 1;
                continue 'cell_loop;
            }
        }

        *after_cell = if cell.is_occupied() && num_occupied >= occupied_limit {
            num_changes += 1;
            Cell::Empty
        } else if cell.is_empty() && num_occupied == 0 {
            num_changes += 1;
            Cell::Occupied
        } else {
            before_state.get(cell_x, cell_y).unwrap().clone()
        };
    }

    num_changes
}

fn count_occupied(board: &Input) -> usize {
    board.iter()
         .filter(|cell| cell.is_occupied())
         .count()
}

#[derive(Debug, Clone)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

impl Cell {
    fn from_char(c: char) -> Cell {
        match c {
            '.' => Cell::Floor,
            '#' => Cell::Occupied,
            'L' => Cell::Empty,
            _ => panic!("Cannot convert {:?} to Cell.", c),
        }
    }

    fn to_char(&self) -> char {
        match self {
            Cell::Floor => '.',
            Cell::Empty => 'L',
            Cell::Occupied => '#',
        }
    }

    fn is_floor(&self)    -> bool { matches!(self, Cell::Floor)    } 
    fn is_empty(&self)    -> bool { matches!(self, Cell::Empty)    }
    fn is_occupied(&self) -> bool { matches!(self, Cell::Occupied) }
}

fn div_rem(lhs: usize, rhs: usize) -> (usize, usize) {
    let div = lhs / rhs;
    let rem = lhs % rhs;

    (div, rem)
}

#[allow(dead_code)]
fn print_board(input: &Input) {
    for row in 0..input.rows() {
        println!(
            "{}",
            input.iter_row(row).map(|cell| cell.to_char()).collect::<String>()
        );
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
