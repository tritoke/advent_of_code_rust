#![feature(test, iter_intersperse)]
extern crate test;

use anyhow::Result;
use std::collections::HashMap;

const INPUT_NUM: usize = 0;

fn main() -> Result<()> {
    let input = get_input()?;

    let (part_1, part_2) = solve(input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);

    Ok(())
}

fn get_input() -> Result<Graph> {
    [
        include_str!("../inputs/day12.inp"),
        include_str!("../test_inputs/day12.inp1"),
        include_str!("../test_inputs/day12.inp2"),
        include_str!("../test_inputs/day12.inp3"),
    ][INPUT_NUM]
        .parse()
}

fn solve(graph: Graph) -> (usize, usize) {
    let start = graph.node_index("start").unwrap();
    let end = graph.node_index("end").unwrap();

    let mut path_finder = DfsPathfinder::new(&graph);
    path_finder.find_all_paths(start, end);

    let mut part_1 = 0;
    let mut two_path_count = 0;
    'path_loop: for path in path_finder
        .small_two_paths
        .iter()
        .chain(path_finder.small_single_paths.iter())
    {
        let mut counts = vec![0; graph.nodes.len()];
        let mut two = None;
        for &node in path.iter() {
            counts[node] += 1;
            if counts[node] == 2 && graph.is_small(node).unwrap() {
                match two {
                    None => two = Some(node),
                    Some(_) => continue 'path_loop,
                }
            }
        }

        match two {
            None => part_1 += 1,
            Some(_) => two_path_count += 1,
        }
    }

    let part_2 = part_1 + two_path_count;

    (part_1, part_2)
}

#[derive(Debug, Clone)]
struct DfsPathfinder<'a> {
    graph: &'a Graph,
    start: usize,
    end: usize,
    small_single_paths: Vec<Vec<usize>>,
    small_two_paths: Vec<Vec<usize>>,
}

impl<'a> DfsPathfinder<'a> {
    fn new(graph: &'a Graph) -> Self {
        Self {
            graph,
            small_single_paths: Default::default(),
            small_two_paths: Default::default(),
            start: Default::default(),
            end: Default::default(),
        }
    }

    fn find_all_paths(&mut self, start: usize, end: usize) {
        self.start = start;
        self.end = end;

        let mut path = vec![start];
        let mut path_node_counts = vec![0; self.graph.nodes.len()];

        self.dfs(&mut path, path_node_counts.as_mut_slice(), false);
    }

    fn dfs(
        &mut self,
        path: &mut Vec<usize>,
        path_node_counts: &mut [usize],
        mut path_contains_two: bool,
    ) {
        let node = *path.last().unwrap();

        // check if we are the end node
        if node == self.end {
            if path_contains_two {
                self.small_two_paths.push(path.clone());
            } else {
                self.small_single_paths.push(path.clone());
            }

            return;
        }

        // otherwise visit all children
        for &child in self.graph.succs(&node).into_iter().flatten() {
            if child == self.start {
                continue;
            }

            let count = path_node_counts[child];
            if self.graph.is_small(child).unwrap() {
                if path_contains_two {
                    if count >= 1 {
                        continue;
                    }
                } else if count == 1 {
                    // valid, we now have a path with two of the same element - this element
                    path_contains_two = true;
                }
            }

            path.push(child);
            path_node_counts[child] += 1;

            self.dfs(path, path_node_counts, path_contains_two);

            if self.graph.is_small(child).unwrap() && path_node_counts[child] == 2 {
                path_contains_two = false;
            }
            path.pop();
            path_node_counts[child] -= 1;
        }
    }
}

#[derive(Debug, Default, Clone)]
struct Graph {
    nodes: Vec<String>,
    edges: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn new() -> Self {
        Default::default()
    }

    fn node_index(&self, needle: &str) -> Option<usize> {
        self.nodes.iter().position(|s| *s == needle)
    }

    fn add_node(&mut self, node: String) -> usize {
        self.nodes.push(node);
        self.nodes.len() - 1
    }

    fn get_or_insert_node(&mut self, node: &str) -> usize {
        self.node_index(node)
            .unwrap_or_else(|| self.add_node(node.to_string()))
    }

    fn add_edge(&mut self, start: usize, end: usize) {
        let outbound: &mut Vec<_> = self.edges.entry(start).or_default();

        if !outbound.iter().any(|&x| x == end) {
            outbound.push(end);
        }
    }

    fn succs<'a>(&'a self, node: &usize) -> Option<&'a [usize]> {
        self.edges.get(node).map(Vec::as_slice)
    }

    fn is_big(&self, node: usize) -> Option<bool> {
        self.nodes
            .get(node)
            .map(|s| s.find(char::is_uppercase).is_some())
    }

    fn is_small(&self, node: usize) -> Option<bool> {
        self.is_big(node).map(std::ops::Not::not)
    }

    #[allow(dead_code)]
    fn path_to_string(&self, path: &[usize]) -> String {
        path.iter()
            .map(|&n| self.nodes[n].as_str())
            .intersperse(",")
            .collect()
    }
}

impl std::str::FromStr for Graph {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut graph = Graph::new();

        for line in s.lines() {
            let (a, b) = line.split_once('-').unwrap();

            let a_index = graph.get_or_insert_node(a);
            let b_index = graph.get_or_insert_node(b);

            graph.add_edge(a_index, b_index);
            graph.add_edge(b_index, a_index);
        }

        Ok(graph)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[bench]
    fn bench_solution(b: &mut test::Bencher) {
        let input = get_input().unwrap();
        b.iter(|| solve(input.clone()))
    }

    #[bench]
    fn bench_get_input(b: &mut test::Bencher) {
        b.iter(|| get_input());
    }
}
