use core::ops::Index;

fn main() {
    let inp = parse_input();

    dbg!(inp);
    // let p1 = part1(&inp);
    // let p2 = part2(&inp);

    // println!("Part 1: {p1}");
    // println!("Part 2: {p2}");
}

fn parse_input() -> Engine {
    let s = include_str!("../../inputs/day03.txt");
    let mut engine = Engine::new();

    for row in s.lines() {
        engine.add_row(row);
    }

    engine
}

#[derive(Debug, Default)]
struct Engine {
    parts: Vec<u32>,
    row_width: usize,
}

impl Engine {
    fn new() -> Self {
        Default::default()
    }

    fn add_row(&mut self, row: &str) {
        if self.parts.is_empty() {
            self.row_width = row.len();
        } else {
            assert!(row.len() == self.row_width);
        }
        let nums = row.chars().map(|c| c.to_digit(10).unwrap_or(0));
        self.parts.extend(nums);
    }

    fn compute_index(&self, idx: (isize, isize)) -> isize {
        let (col, row) = idx;
        row * self.row_width + col
    }

    fn get(&self, idx: (isize, isize)) -> Option<&u32> {
        self.compute_index(idx)
            .try_into()
            .and_then(|pos| self.parts.get(pos))
    }
}

// fn part1(inp: &[Game]) -> u32 { }
//
// fn part2(inp: &[Game]) -> u32 { }
