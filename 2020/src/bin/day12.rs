#![feature(test)]
extern crate test;

use std::num::ParseIntError;
use std::str::FromStr;

type Input = Vec<(Direction, i64)>;
type PartInput = [(Direction, i64)];
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
        0 => include_str!("../inputs/day12.inp"),
        1 => include_str!("../test_inputs/day12.inp1"),
        _ => panic!("Unknown input number: {:?}", INPUT_NUM),
    };

    in_str
        .lines()
        .map(|line| {
            let (dir, num) = line.split_at(1);

            (dir.parse().unwrap(), num.parse().unwrap())
        })
        .collect()
}

fn part1(input: &PartInput) -> usize {
    let mut ship = Ship::new();

    for (action, value) in input.iter() {
        if action.is_turn() {
            ship.rotate(*action, *value);
        } else if action.is_forward() {
            ship.travel(None, *value);
        } else {
            ship.travel(Some(*action), *value);
        }
    }

    ship.position.manhattan_distance()
}

fn part2(input: &PartInput) -> usize {
    let mut ship = Ship::new();
    let mut waypoint = Point::new_at(10, 1);

    for (action, value) in input.iter() {
        match action {
            Direction::North | Direction::South | Direction::East | Direction::West => {
                waypoint.translate(*action, *value)
            }
            Direction::Left => waypoint.rotate_counter_clockwise(*value),
            Direction::Right => waypoint.rotate_counter_clockwise(-*value),
            Direction::Forward => ship.travel_towards_waypoint(&waypoint, *value),
        }
    }

    ship.position.manhattan_distance()
}

#[derive(Debug)]
struct Ship {
    position: Point,
    facing: Direction,
}

impl Ship {
    fn new() -> Self {
        Self {
            position: Point::new(),
            facing: Direction::East,
        }
    }

    fn travel(&mut self, direction: Option<Direction>, distance: i64) {
        self.position
            .translate(direction.unwrap_or(self.facing), distance);
    }

    fn travel_towards_waypoint(&mut self, waypoint: &Point, distance: i64) {
        self.position.x += waypoint.x * distance;
        self.position.y += waypoint.y * distance;
    }

    fn rotate(&mut self, direction: Direction, degrees: i64) {
        self.facing = match degrees {
            90 => {
                if direction.is_left() {
                    self.facing.rotate_left()
                } else {
                    self.facing.rotate_right()
                }
            }
            180 => self.facing.rotate_180(),
            270 => {
                if direction.is_right() {
                    self.facing.rotate_left()
                } else {
                    self.facing.rotate_right()
                }
            }
            _ => panic!("Don't know how to rotate by {} degrees", degrees),
        }
    }
}

#[derive(Debug)]
struct Point {
    x: i64,
    y: i64,
}

impl Point {
    fn new() -> Self {
        Self { x: 0, y: 0 }
    }

    fn new_at(x: i64, y: i64) -> Self {
        Self { x, y }
    }

    fn translate(&mut self, direction: Direction, distance: i64) {
        match direction {
            Direction::North => self.y += distance,
            Direction::South => self.y -= distance,
            Direction::East => self.x += distance,
            Direction::West => self.x -= distance,
            _ => panic!("Can't go in this direction: {:?}", direction),
        }
    }

    fn rotate_counter_clockwise(&mut self, degrees: i64) {
        let x = self.x;
        let y = self.y;

        // anti-clockwise rotation by theta
        // x = x cos(theta) - y sin(theta)
        // y = x sin(theta) + y cos(theta)
        // cos 90, 180, 270 = 0, -1, 0
        // sin 90, 180, 270 = 1, 0, -1
        let (new_x, new_y) = match degrees {
            90 | -270 => (-y, x),
            180 | -180 => (-x, -y),
            270 | -90 => (y, -x),
            _ => panic!("Don't know how to rotate by {} degrees :(", degrees),
        };

        self.x = new_x;
        self.y = new_y;
    }

    fn manhattan_distance(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

#[derive(Debug, Copy, Clone)]
enum Direction {
    North,
    South,
    East,
    West,
    Left,
    Right,
    Forward,
}

impl Direction {
    fn is_turn(&self) -> bool {
        matches!(self, Direction::Left | Direction::Right)
    }
    fn is_forward(&self) -> bool {
        matches!(self, Direction::Forward)
    }
    fn is_left(&self) -> bool {
        matches!(self, Direction::Left)
    }
    fn is_right(&self) -> bool {
        matches!(self, Direction::Right)
    }

    fn rotate_left(&self) -> Self {
        match self {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
            _ => panic!("Can't rotate direction {:?}", self),
        }
    }

    fn rotate_right(&self) -> Self {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
            _ => panic!("Can't rotate direction {:?}", self),
        }
    }

    fn rotate_180(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
            _ => panic!("Can't rotate direction {:?}", self),
        }
    }
}

impl FromStr for Direction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "N" => Direction::North,
            "S" => Direction::South,
            "E" => Direction::East,
            "W" => Direction::West,
            "L" => Direction::Left,
            "R" => Direction::Right,
            "F" => Direction::Forward,
            _ => panic!("Invalid direction: {:?}", s),
        })
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
