#![feature(test)]
#![allow(dead_code, unused_variables)]
extern crate test;

use aoc::bit_ops::BitOps;

use anyhow::Result;
use std::str::FromStr;

use std::collections::hash_map::Entry;
use fnv::FnvHashMap;

const INPUT_NUM: i32 = 0;

fn main() -> Result<()> {
    let input = get_input()?;

    let (part_1, part_2) = solve(&input)?;

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);

    Ok(())
}

fn get_input() -> Result<Tiles> {
    match INPUT_NUM {
        0 => include_str!("../inputs/day20.inp"),
        1 => include_str!("../test_inputs/day20.inp1"),
        _ => panic!("Unknown input number: {:?}", INPUT_NUM),
    }
    .parse()
}

fn solve(tiles: &Tiles) -> Result<(usize, usize)> {
    // map from edge to list of tile ids
    let mut edge_map: FnvHashMap<usize, Vec<usize>> = Default::default();

    // build the edge map from each edge to the list of IDs which can produce it
    for tile in tiles.inner.iter() {
        for side in [Side::North, Side::East, Side::South, Side::West] {
            let edge = tile.get_edge(side);

            edge_map.entry(edge).or_default().push(tile.id);

            edge_map
                .entry(rev_n_bits(edge, 10))
                .or_default()
                .push(tile.id);
        }
    }

    // build a map from tile.id to the edges that they match
    let side_map: FnvHashMap<usize, Vec<(Side, usize)>> = tiles
        .inner
        .iter()
        .map(|tile| {
            let matching_sides: Vec<(Side, usize)> =
                [Side::North, Side::East, Side::South, Side::West]
                    .into_iter()
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

    // make the first corner the top left one - edges going South and East
    let top_left = corner_ids[1];
    let (s_1, edge_1) = side_map[&top_left][0];
    let (s_2, edge_2) = side_map[&top_left][1];

    // current rotation
    let rot = match s_1.rotations_to(&s_2) {
        1 => s_1.rotations_to(&Side::East),
        3 => s_2.rotations_to(&Side::East),
        n => unreachable!("{} rotations should not be possible for a corner.", n),
    };

    // map from tile id to rotation / flip
    let mut state_map: FnvHashMap<usize, (usize, Flip)> = Default::default();
    state_map.insert(top_left, (rot, Flip::NoFlip));

    let side_length = (tiles.inner.len() as f64).sqrt() as usize;

    // build a mapping from (row, col), to the tile id at that position
    let mut position_map: FnvHashMap<(usize, usize), usize> = Default::default();
    position_map.insert((0, 0), top_left);

    for row in 0..side_length - 1 {
        for col in 0..side_length {
            let id = *position_map.get(&(row, col)).unwrap();
            let tile = tiles.get_by_id(&id).unwrap();
            let (rot, flip) = *state_map.get(&id).unwrap();

            // find east edge connection
            let east_edge = tile.get_edge(Side::East.rotate_neg(rot));
            let east_neighbour = edge_map[&east_edge]
                .iter()
                .find(|tile_id| **tile_id != id)
                .and_then(|id| tiles.get_by_id(id));

            // find south edge connection
            let south_edge = tile.get_edge(Side::South.rotate_neg(rot));
            let south_neighbour = edge_map[&south_edge]
                .iter()
                .find(|tile_id| **tile_id != id)
                .and_then(|id| tiles.get_by_id(id));

            println!("{:?}", (row, col, id, east_neighbour.map(|t|t.id), south_neighbour.map(|t|t.id)));

            // if there is an east neighbour, put its info in the maps
            if let Some(en) = east_neighbour {
                position_map.insert((row, col + 1), en.id);

                let (side, flipped) = en.find_connection_state(east_edge).unwrap();
                let rotations = side.rotations_to(&Side::West);
                let flip_type = if flipped ^ (rotations >= 2) { Flip::Horizontal } else { Flip::NoFlip };

                if en.id == 2473 {
                    println!("{:010b}", tile.get_edge(Side::East.rotate_neg(rot)));
                    println!("{:010b}", en.get_edge(side));
                    println!("{:010b}", east_edge);
                    println!("{:010b}", rev_n_bits(east_edge, 10));
                    dbg!(side, flip, flipped);
                    dbg!(rotations);
                    dbg!(flip_type);
                }

                state_map.insert(en.id, (rotations, flip_type));
            }

            // if there is a south neighbour, put its info in the maps
            if let Some(sn) = south_neighbour {
                position_map.insert((row + 1, col), sn.id);

                let (side, flipped) = sn.find_connection_state(south_edge).unwrap();
                let rotations = side.rotations_to(&Side::North);
                let flip_type = if flipped ^ (rotations >= 2) { Flip::Vertical } else { Flip::NoFlip };

                if sn.id == 2473 {
                    println!("{:010b}", tile.get_edge(Side::South.rotate_neg(rot)));
                    println!("{:010b}", sn.get_edge(side));
                    println!("{:010b}", south_edge);
                    println!("{:010b}", rev_n_bits(south_edge, 10));
                    dbg!(side, flip, flipped);
                    dbg!(rotations);
                    dbg!(flip_type);
                }
                
                state_map.insert(sn.id, (rotations, flip_type));
            }
        }
    }

    //*state_map.get_mut(&2473).unwrap() = (2, Flip::Vertical);
    //*state_map.get_mut(&1171).unwrap() = (3, Flip::NoFlip);

    dbg!(&position_map);
    dbg!(&state_map);

    // build the final search grids
    let mut rows: Vec<u128> = std::iter::repeat(0_u128).take(side_length * 8).collect();
    let mut cols = rows.clone();
    let mut monster_bits = rows.clone();

    for row in 0..side_length {
        for col in 0..side_length {
            let id = position_map[&(row, col)];
            let (rot, flip) = state_map[&id];

            let to_change: Vec<(&mut u128, usize)> = rows.iter_mut().skip(row * 8).zip(tiles.get_by_id(&id).unwrap().iter_rows(rot)).collect();

            match flip {
                Flip::NoFlip => {
                    for (row, cell_row) in to_change {
                        *row <<= 8;
                        *row |= cell_row as u128;
                    }
                }
                Flip::Horizontal => {
                    for (row, cell_row) in to_change.into_iter().rev() {
                        *row <<= 8;
                        *row |= cell_row as u128;
                    }
                }
                Flip::Vertical => {
                    for (row, cell_row) in to_change {
                        *row <<= 8;
                        *row |= rev_n_bits(cell_row, 8) as u128;
                    }
                }
            }
        }
    }

    for (i, col) in cols.iter_mut().enumerate() {
        for bit in 0..side_length * 8 {
            if rows[bit].test_bit(i as u32) {
                col.set_bit(bit as u32);
            }
        }
    }

    // hardcode the monster variations
    let monster          = 0b000000000000000000101000011000011000011101001001001001001000_u128;
    let monster_rev      = 0b010000000000000000001110000110000110000100010010010010010010_u128;
    let monster_flip     = 0b010010010010010010001000011000011000011100000000000000000010_u128;
    let monster_flip_rev = 0b000100100100100100101110000110000110000101000000000000000000_u128;

    let monsters = [
        monster,
        monster_rev,
        monster_flip,
        monster_flip_rev,
    ];

    for (j, &r) in rows.iter().enumerate() {
        for i in (0..8 * side_length).rev() {
            print!("{}", if r.test_bit(i as u32) { "#" } else { "." });
            if i % 8 == 0 && i != 0 {
                //print!("|");
            }
        }
        println!("");
        if j % 8 == 7 { //&& j < 21 {
            //println!("--------------------------");
        }
    }

    /*
    println!();

    for (j, &r) in cols.iter().enumerate() {
        for i in (0..8 * side_length).rev() {
            print!("{}", if r.test_bit(i as u32) { "#" } else { "." });
            if i % 8 == 0 && i != 0 {
                //print!("|");
            }
        }
        println!("");
        if j % 8 == 7 { //&& j < 21 {
            //println!("--------------------------");
        }
    }
    */

    // perform the search
    let mut count = 0;

    for grid in [&rows, &cols] {
        for w in grid.windows(3) {
            let a = w[0];
            let b = w[1];
            let c = w[2];

            for offset in 0..8 * side_length - 20 {
                let pot_monster = ((a >> offset) & ((1 << 20) - 1)) << 40
                                | ((b >> offset) & ((1 << 20) - 1)) << 20
                                | ((c >> offset) & ((1 << 20) - 1));

                if monsters.iter().find(|m| (pot_monster & **m) == **m).is_some() {
                    count += 1;
                    //println!("{:060b}", pot_monster);
                    //println!("{:020b}", a & ((1 << 20) - 1));
                    //println!("{:020b}", b & ((1 << 20) - 1));
                    //println!("{:020b}", c & ((1 << 20) - 1));
                }
            }
        }
    }

    println!("{}", count);
    let monster_bits: u32 = count * monster.count_ones();
    let all_bits: u32 = rows.into_iter().map(u128::count_ones).sum();

    Ok((part_1, (all_bits - monster_bits) as usize))
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
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut line_iter = s.lines();

        let id: usize = line_iter
            .next()
            .and_then(|line| line.get(5..9))
            .unwrap()
            .parse()?;

        let mut east_edge = 0;
        let mut west_edge = 0;

        // lowest bit is left hand side
        let data: Vec<usize> = line_iter
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
            id,
            data,
            east_edge,
            west_edge,
        })
    }
}

impl Tile {
    fn get_edge(&self, side: Side) -> usize {
        match side {
            Side::North => self.data[0],
            Side::South => self.data[9],
            Side::East => self.east_edge,
            Side::West => self.west_edge,
        }
    }

    fn get_neighbour(
        &self,
        tiles: &Tiles,
        side: &Side,
        side_map: &FnvHashMap<usize, Vec<(Side, usize)>>,
    ) -> Option<usize> {
        side_map[&self.id]
            .iter()
            .find(|(s, _)| s == side)
            .and_then(|(_, id)| tiles.inner.binary_search_by_key(id, |t| t.id).ok())
    }

    fn find_connection_state(&self, needle: usize) -> Option<(Side, bool)> {
        let flipped_needle = rev_n_bits(needle, 10);

        let mut options = vec![];
        for side in [Side::North, Side::East, Side::South, Side::West] {
            let edge = self.get_edge(side);

            if needle == 0b0101011100 {
                dbg!(side);
                println!("edge = {:010b}", edge);
                println!("needle = {:010b}", needle);
                println!("flipped_needle = {:010b}", flipped_needle);
                print!("\n");
            }

            if edge == needle || edge == flipped_needle {
                options.push(
                    Some((side, edge == flipped_needle))
                )
                //return Some((side, edge == flipped_needle));
            }
        }

        if needle == 0b0101011100 {
            dbg!(&options);
        }
        options.first().cloned().flatten()
        //None
    }

    fn iter_rows(&self, rot: usize) -> Box<dyn Iterator<Item = usize> + '_> {
        // individual arms have to be boxed as their types aren't the same
        match rot % 4 {
            0 => Box::new(
                self.data
                    .iter()
                    .skip(1)
                    .take(8)
                    .map(|row| row >> 1 & 0b1111_1111),
            ),

            2 => Box::new(
                self.data
                    .iter()
                    .skip(1)
                    .take(8)
                    .rev()
                    .map(|row| rev_n_bits(row >> 1, 8)),
            ),

            1 => Box::new((1..=8).map(|bit| {
                self.data
                    .iter()
                    .skip(1)
                    .take(8)
                    .enumerate()
                    .fold(0, |mut acc, (i, x)| {
                        if x.test_bit(bit) {
                            acc.set_bit(i as u32);
                        }
                        acc
                    })
            })),

            3 => Box::new((1..=8).rev().map(|bit| {
                self.data
                    .iter()
                    .skip(1)
                    .take(8)
                    .enumerate()
                    .fold(0, |mut acc, (i, x)| {
                        if x.test_bit(bit) {
                            acc.set_bit((7 - i) as u32);
                        }
                        acc
                    })
            })),

            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
struct Tiles {
    inner: Vec<Tile>,
}

impl Tiles {
    fn get_by_id(&self, id: &usize) -> Option<&Tile> {
        self.inner
            .binary_search_by_key(id, |t| t.id)
            .ok()
            .and_then(|idx| self.inner.get(idx))
    }
}

impl FromStr for Tiles {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut tiles: Vec<Tile> = s.split("\n\n").map(str::parse).collect::<Result<_>>()?;

        // sort by id so we can then use
        // tiles.binary_search_by_key with tile.id
        tiles.sort_by_key(|tile| tile.id);

        Ok(Self { inner: tiles })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Side {
    North,
    East,
    South,
    West,
}

impl Side {
    fn get_sideno(&self) -> usize {
        match self {
            Side::North => 0,
            Side::East => 1,
            Side::South => 2,
            Side::West => 3,
        }
    }

    fn from_sideno(sideno: usize) -> Self {
        match sideno {
            0 => Side::North,
            1 => Side::East,
            2 => Side::South,
            3 => Side::West,
            _ => panic!("Cannot create Side from sideno: {}.", sideno),
        }
    }

    fn rotations_to(&self, other: &Self) -> usize {
        let s1 = self.get_sideno();
        let s2 = other.get_sideno();

        (s2 - s1) % 4
    }

    fn rotate_pos(&self, rotations: usize) -> Self {
        let sideno = self.get_sideno();

        let new_sideno = (sideno + rotations) % 4;

        Side::from_sideno(new_sideno)
    }

    fn rotate_neg(&self, rotations: usize) -> Self {
        let sideno = self.get_sideno();

        let new_sideno = (sideno - rotations) % 4;

        Side::from_sideno(new_sideno)
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Flip {
    NoFlip,
    Vertical,
    Horizontal,
}

/// returns the reverse of the first `bits` in `n`
fn rev_n_bits(n: usize, bits: u32) -> usize {
    (0..bits).filter(|b| n.test_bit(*b)).fold(0, |mut acc, b| {
        acc.set_bit(bits - 1 - b);
        acc
    })
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = get_input().unwrap();

    b.iter(|| solve(&input))
}

#[bench]
fn bench_get_input(b: &mut test::Bencher) {
    b.iter(|| get_input());
}
