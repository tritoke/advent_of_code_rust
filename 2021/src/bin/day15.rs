#![feature(test)]
extern crate test;

use std::cmp::Ordering;
use std::collections::{HashMap, BinaryHeap};
use std::fmt::Debug;
use std::collections::hash_map::Entry;

const INPUT_NUM: usize = 0;

fn main() {
    let input = get_input();

    let (part_1, part_2) = solve(input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn get_input() -> Vec<Vec<u8>> {
    [
        include_str!("../inputs/day15.inp"),
        include_str!("../test_inputs/day15.inp1"),
    ][INPUT_NUM]
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect()
        })
        .collect()
}

fn solve(input: Vec<Vec<u8>>) -> (usize, usize) {
    let mut path_finder = PathFinder::new(input);
    let part_1 = path_finder.find_lowest_risk_path();

    path_finder.extend(5);
    let part_2 = path_finder.find_lowest_risk_path();

    (part_1, part_2)
}

#[derive(Debug)]
struct IndexExtender {
    risk_levels: Vec<Vec<u8>>,
    extension: usize,
}

impl IndexExtender {
    fn new(risk_levels: Vec<Vec<u8>>, extension: usize) -> Self {
        Self { risk_levels, extension }
    }

    fn extension_mut(&mut self) -> &mut usize {
        &mut self.extension
    }

    fn get(&self, x: usize, y: usize) -> Option<usize> {
        let x_idx = x % self.risk_levels[0].len();
        let x_ext = x / self.risk_levels[0].len();
        let y_idx = y % self.risk_levels.len();
        let y_ext = y / self.risk_levels.len();

        let mut val = self.risk_levels[y_idx][x_idx] as usize + x_ext + y_ext;

        if val > 9 {
            val -= 9
        }

        if x_ext < self.extension && y_ext < self.extension {
            Some(val)
        } else {
            None
        }
    }

    fn width(&self) -> usize {
        self.risk_levels[0].len() * self.extension
    }

    fn height(&self) -> usize {
        self.risk_levels.len() * self.extension
    }
}

#[derive(Debug)]
struct PathFinder {
    risk_levels: IndexExtender,
}

impl PathFinder {
    fn new(risk_levels: Vec<Vec<u8>>) -> Self {
        Self { risk_levels: IndexExtender::new(risk_levels, 1) }
    }

    fn node_from_coords(
        &self,
        x: usize,
        y: usize,
        dist_from_start: usize,
    ) -> Option<MinHeapNode<(usize, usize)>> {
        self.risk_levels
            .get(x, y)
            .map(|risk| MinHeapNode::new(risk as usize + dist_from_start, (x, y)))
    }

    fn extend(&mut self, val: usize) {
        *self.risk_levels.extension_mut() = val;
    }

    fn find_lowest_risk_path(&self) -> usize {
        let mut queue: BinaryHeap<MinHeapNode<(usize, usize)>> =
            BinaryHeap::from([self.node_from_coords(0, 0, 0).unwrap()]);
        let mut distances: HashMap<(usize, usize), usize> = HashMap::from([((0, 0), self.risk_levels.get(0, 0).unwrap())]);

        let mut risk = 0;
        while let Some(MinHeapNode {
            weight: dist,
            item: (x, y),
        }) = queue.pop()
        {
            // if we have reached the node then YEET
            if x == self.risk_levels.width() - 1 && y == self.risk_levels.height() - 1 {
                risk = dist;
                break;
            }

            if dist != distances[&(x, y)] {
                continue;
            }

            #[rustfmt::skip]
            let neighbours = [
                Some((x + 1, y)),
                x.checked_sub(1).zip(Some(y)),
                Some((x, y + 1)),
                Some(x).zip(y.checked_sub(1)),
            ].map(|pos| pos.and_then(|(x, y)| self.node_from_coords(x, y, dist)));

            for node in neighbours.into_iter().flatten() {
                let closer = match distances.entry(node.item) {
                    Entry::Vacant(v) => {
                        v.insert(node.weight);
                        true
                    }
                    Entry::Occupied(mut o) => {
                        if *o.get() > node.weight {
                            *o.get_mut() = node.weight;
                            true
                        } else {
                            false
                        }
                    }
                };

                if closer {
                    queue.push(node);
                }
            }
        }

        risk - self.risk_levels.get(0, 0).unwrap()
    }
}

#[derive(Debug, Copy, Clone)]
struct MinHeapNode<T: Debug + Copy + Clone> {
    weight: usize,
    item: T,
}

impl<T: Debug + Copy + Clone> MinHeapNode<T> {
    fn new(weight: usize, item: T) -> Self {
        Self { weight, item }
    }
}

impl<T: Debug + Copy + Clone> PartialEq for MinHeapNode<T> {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl<T: Debug + Copy + Clone> Eq for MinHeapNode<T> {}

impl<T: Debug + Copy + Clone> PartialOrd for MinHeapNode<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Debug + Copy + Clone> Ord for MinHeapNode<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight).reverse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[bench]
    fn bench_solution(b: &mut test::Bencher) {
        let input = get_input();
        b.iter(|| solve(input.clone()))
    }

    #[bench]
    fn bench_get_input(b: &mut test::Bencher) {
        b.iter(|| get_input());
    }
}
