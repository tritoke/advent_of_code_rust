use std::collections::{BinaryHeap, HashSet};

fn main() {
    let inp = parse_input();

    let p1 = part1(&inp);
    println!("Part 1: {p1}");

    let p2 = part2(&inp);
    println!("Part 2: {p2}");
}

fn parse_input() -> Vec<Vec<u8>> {
    include_str!("../../inputs/day17.txt")
        .lines()
        .map(|line| line.bytes().map(|c| c - b'0').collect())
        .collect()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turns(self) -> [Direction; 2] {
        use Direction::*;

        match self {
            Up | Down => [Left, Right],
            Left | Right => [Up, Down],
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct HeapNode {
    pos: (usize, usize),
    loss: u32,
    cannot_move_in: Direction,
}

impl PartialEq for HeapNode {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other).is_eq()
    }
}

impl Eq for HeapNode {}

impl PartialOrd for HeapNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.loss.cmp(&other.loss).reverse()
    }
}

struct Graph<'a> {
    grid: &'a [Vec<u8>],
}

impl<'a> Graph<'a> {
    fn neighbours(
        &'a self,
        node: HeapNode,
        min_move_len: usize,
        max_move_len: usize,
    ) -> impl Iterator<Item = HeapNode> + 'a {
        node.cannot_move_in
            .turns()
            .into_iter()
            .flat_map(move |dir| {
                let mut state = node;
                std::iter::from_fn(move || {
                    state = self.move_to(state, dir)?;
                    Some(state)
                })
                .skip(min_move_len - 1)
                .take(max_move_len - min_move_len + 1)
            })
    }

    fn move_to(&self, node: HeapNode, dir: Direction) -> Option<HeapNode> {
        let (x, y) = node.pos;
        let max_x = self.grid[0].len();
        let max_y = self.grid.len();

        let next_pos = match dir {
            Direction::Up => (x, y.checked_sub(1)?),
            Direction::Down => (x, (y + 1 < max_y).then_some(y + 1)?),
            Direction::Left => (x.checked_sub(1)?, y),
            Direction::Right => ((x + 1 < max_x).then_some(x + 1)?, y),
        };

        let (nx, ny) = next_pos;
        Some(HeapNode {
            pos: next_pos,
            loss: node.loss + self.grid[ny][nx] as u32,
            cannot_move_in: dir,
        })
    }
}

fn minimum_heat_loss(grid: &[Vec<u8>], min_move_len: usize, max_move_len: usize) -> u32 {
    let graph = Graph { grid };

    let mut to_explore = BinaryHeap::from([
        HeapNode {
            pos: (0, 0),
            loss: 0,
            cannot_move_in: Direction::Right,
        },
        HeapNode {
            pos: (0, 0),
            loss: 0,
            cannot_move_in: Direction::Down,
        },
    ]);
    let mut seen = HashSet::new();

    let dst = (grid[0].len() - 1, grid.len() - 1);
    while let Some(node) = to_explore.pop() {
        let HeapNode {
            pos,
            loss,
            cannot_move_in,
        } = node;

        if !seen.insert((pos, cannot_move_in)) {
            continue;
        }

        if pos == dst {
            return loss;
        }

        to_explore.extend(
            graph
                .neighbours(node, min_move_len, max_move_len)
                .filter(|node| !seen.contains(&(node.pos, node.cannot_move_in))),
        )
    }

    unreachable!("Failed to find a path");
}

fn part1(inp: &[Vec<u8>]) -> u32 {
    minimum_heat_loss(inp, 1, 3)
}

fn part2(inp: &[Vec<u8>]) -> u32 {
    minimum_heat_loss(inp, 4, 10)
}
