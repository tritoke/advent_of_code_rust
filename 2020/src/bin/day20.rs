#![feature(test)]
extern crate test;

use std::num::ParseIntError;
use std::str::FromStr;

use fnv::FnvHashMap;

const INPUT_NUM: i32 = 1;

type Tiles = Vec<Tile>;

fn main() {
    let input = get_input();

    let (part_1, part_2) = solve(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn get_input() -> Tiles {
    let mut tiles: Tiles = match INPUT_NUM {
        0 => include_str!("../inputs/day20.inp"),
        1 => include_str!("../test_inputs/day20.inp1"),
        _ => panic!("Unknown input number: {:?}", INPUT_NUM),
    }
    .split("\n\n")
    .map(|tile| tile.parse().unwrap())
    .collect();

    // sort by id so we can then use
    // tiles.binary_search_by_key with tile.id
    tiles.sort_by_key(|tile| tile.id);

    tiles
}

fn solve(tiles: &Tiles) -> (usize, usize) {
    // map from edge to list of tile ids
    let mut edge_map: FnvHashMap<usize, Vec<usize>> = Default::default();

    // build the edge map
    for tile in tiles {
        for side in SideIter::new() {
            let edge = tile.get_edge(side);
            edge_map
                .entry(edge)
                .or_default()
                .push(tile.id);

            edge_map
                .entry(rev_n_bits(edge, 10))
                .or_default()
                .push(tile.id);
        }
    }

    // build a map from tile.id to the edges that they match
    let side_map: FnvHashMap<usize, Vec<(Side, usize)>> = tiles
        .iter()
        .map(|tile| {
            let matching_sides: Vec<(Side, usize)> = SideIter::new()
                .map(|side| (side, tile.get_edge(side)))
                .filter(|(_, edge)| edge_map[edge].len() > 1)
                .collect();

            (tile.id, matching_sides)
        })
        .collect();

    let corner_ids: Vec<usize> = side_map
        .iter()
        .filter(|(_, matching)| matching.len() == 2)
        .map(|(id, _)| *id)
        .collect();

    let part_1 = corner_ids.iter().product();

    // build out the full image from here
    // corners are known, build edges from corners until
    // we know what orientation each one is in
    // i.e. we have corner 1 -> a -> b -> c -> corner 2
    // and have that for all corners
    // might be worth having a wrapper struct which
    // holds tile id and the orientation of the tile, flipped rotated etc
    // and the neighbourning tild references

    // I want to generate a sequence of indexes
    let corner_tile = get_tile_with_id(tiles, corner_ids[0]).unwrap();

    // because this tile is a corner tile we know that each direction this tile has edges in
    // the rest of the tiles will also have edges in those directions
    // e.g.
    // a1 -> b1 -> c1 -> e1...
    // |     |     |     |
    // v     v     v     v
    // a2 -> b2 -> c2 -> e2...
    // |     |     |     |
    // v     v     v     v
    // a3 -> b3 -> c3 -> e3...
    // |     |     |     |
    // v     v     v     v
    // ...   ...   ...   ...

    // make the first corner the top left one, so edges going East and South
    // this is possible for all corners and just constitutes a rotation
    println!("{:?}", corner_tile);
    
    let top_left = get_tile_with_id(tiles, corner_ids[0]).unwrap();

    let (s_1, edge_1) = side_map[&corner_ids[0]][0];
    let (s_2, edge_2) = side_map[&corner_ids[0]][1];

    // current rotation
    let rot = match s_1.rotations_to(&s_2) {
        1 => s_1.rotations_to(&Side::East),
        3 => s_2.rotations_to(&Side::East),
        n => panic!("Shouldn't be possible to have {} rotations for a corner", n)
    };

    // make an array of, rotation / flip state to eventually build the thing from
    //let mut state_arr: Vec<(usize, bool, usize)> = Vec::new();


    let sides = vec![Side::North, Side::East, Side::South, Side::West];


    (part_1, 0)
}

fn get_tile_with_id(tiles: &Tiles, id: usize) -> Option<&Tile> {
    match tiles.binary_search_by_key(&id, |t| t.id) {
        Ok(idx) => tiles.get(idx),
        Err(_) => None
    }
}

// pre-compute west and east edges
#[derive(Debug)]
struct Tile {
    id: usize,
    data: Vec<usize>,
    // precompute edges which would otherwise be expensive
    west_edge: usize, // collected bit 0s from each row
    east_edge: usize, // collected bit 9s from each row
}

impl FromStr for Tile {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line_iter = s.lines();

        let id_line = line_iter.next().unwrap();
        let id_str = id_line.get(5..9).unwrap();
        let id_fromstr: usize = id_str.parse()?;

        let mut east_edge = 0;
        let mut west_edge = 0;

        // lowest bit is left hand side
        let tile_data: Vec<usize> = line_iter
            .enumerate()
            .map(|(line_no, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '#')
                    .fold(0, |mut acc, (n, _)| {
                        if n == 0 {
                            west_edge.set_bit(line_no as u32);
                        }
                        if n == 9 {
                            east_edge.set_bit(line_no as u32);
                        }
                        acc.set_bit(n as u32);
                        acc
                    })
            })
            .collect();

        Ok(Self {
            id: id_fromstr,
            data: tile_data,
            east_edge: east_edge,
            west_edge: west_edge
        })
    }
}

impl Tile {
    fn get_edge(&self, side: Side) -> usize {
        match side {
            Side::North => self.data[0],
            Side::South => self.data[9],
            Side::East  => self.east_edge,
            Side::West  => self.west_edge,
        }
    }

    fn iter_neighbours<'a>(&self, tiles: &'a Tiles, edge_map: &'a FnvHashMap<usize, Vec<usize>>,
                       side_map: &'a FnvHashMap<usize, Vec<(Side, usize)>>) -> NeighbourIter<'a> {
        NeighbourIter {
            tiles: tiles,
            edge_map: edge_map,
            side_map: side_map,
            id: self.id,
            iter_no: 0
        }
    }

    fn get_neighbour(&self, tiles: &Tiles, side: &Side, edge_map: &FnvHashMap<usize, Vec<usize>>,
                     side_map: &FnvHashMap<usize, Vec<(Side, usize)>>) -> Option<usize> {
        if let Some((_, id)) = side_map[&self.id].iter().find(|(s,_)| s == side) {
            if let Ok(idx) = tiles.binary_search_by_key(id, |t| t.id) {
                Some(idx)
            } else {
                None
            }
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct NeighbourIter<'a> {
    tiles: &'a Tiles,
    edge_map: &'a FnvHashMap<usize, Vec<usize>>,
    side_map: &'a FnvHashMap<usize, Vec<(Side, usize)>>,
    id: usize,
    iter_no: usize
}

impl Iterator for NeighbourIter<'_> {
    type Item = (Side, usize);

    fn next(&mut self) -> Option<Self::Item> {
        self.iter_no += 1;

        if let Some((side, edge)) = self.side_map[&self.id].get(self.iter_no - 1) {
            if let Some(id) = self.edge_map[&edge].iter().filter(|id| **id != self.id).next() {
                if let Ok(idx) = self.tiles.binary_search_by_key(id, |t| t.id) {
                    Some((*side, idx))
                } else {
                    None
                }
            } else {
                eprintln!("Couldn't find tile with edge {}.", edge);
                None
            }
        } else {
            None
        }
    }
}

#[derive(Debug,Copy,Clone,Eq,PartialEq)]
enum Side {
    North,
    East,
    South,
    West
}

impl Side {
    fn get_sideno(&self) -> usize {
        match self {
            Side::North => 0,
            Side::East  => 1,
            Side::South => 2,
            Side::West  => 3,
        }
    }

    fn from_sideno(sideno: usize) -> Self {
        match sideno {
            0 => Side::North,
            1 => Side::East,
            2 => Side::South,
            3 => Side::West,
            _ => panic!("Cannot create Side from sideno: {}.", sideno)
        }
    }

    fn rotations_to(&self, other: &Self) -> usize {
        let s1 = self.get_sideno();
        let s2 = other.get_sideno();

        return s2 - s1;
    }

    fn rotate_by(&self, rotations: usize) -> Self {
        let sideno = self.get_sideno();

        let new_sideno = (sideno + rotations) % 4;
        
        Side::from_sideno(new_sideno)
    }
}

struct SideIter {
    iter_no: usize
}

impl Iterator for SideIter {
    type Item = Side;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter_no += 1;

        match self.iter_no {
            1 => Some(Side::North),
            2 => Some(Side::East),
            3 => Some(Side::South),
            4 => Some(Side::West),
            _ => None
        }
    }
}

impl SideIter {
    fn new() -> Self {
        Self {
            iter_no: 0
        }
    }
}

trait BitOps {
    fn set_bit(&mut self, n: u32);
    fn test_bit(&self, n: u32) -> bool;
    fn clear_bit(&mut self, n: u32);
}

impl BitOps for usize {
    fn set_bit(&mut self, n: u32) {
        *self |= 1 << n;
    }

    fn test_bit(&self, n: u32) -> bool {
        (*self & (1 << n)) == (1 << n)
    }

    fn clear_bit(&mut self, n: u32) {
        *self &= !(1 << n);
    }
}

/// returns the reverse of the first `bits` in `n`
fn rev_n_bits(n: usize, bits: u32) -> usize {
    (0..bits)
        .filter(|b| n.test_bit(*b))
        .fold(0, |mut acc, b| {
            acc.set_bit(bits - 1 - b);
            acc
        })
}

#[allow(dead_code)]
fn print_tile(tile: &Tile) {
    println!("Tile {}:", tile.id);
    for row in tile.data.iter() {
        let row_string = (0..10)
            .map(|i| if row.test_bit(i) { '#' } else { '.' })
            .collect::<String>();

        println!("{}", row_string);
    }
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = get_input();

    b.iter(|| solve(&input))
}

#[bench]
fn bench_get_input(b: &mut test::Bencher) {
    b.iter(|| get_input());
}
