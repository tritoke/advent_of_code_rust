#![feature(test, array_windows, binary_heap_into_iter_sorted)]
extern crate test;

use std::collections::{BinaryHeap, VecDeque};

const INPUT_NUM: usize = 0;

fn main() {
    let input = get_input();

    let (part_1, part_2) = solve(input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn get_input() -> Vec<&'static str> {
    [
        include_str!("../inputs/day09.inp"),
        include_str!("../test_inputs/day09.inp1"),
    ][INPUT_NUM]
        .lines()
        .collect()
}

fn solve(inp: Vec<&'static str>) -> (usize, usize) {
    let top_border = vec![b'9'; inp[0].len() + 2];
    let mut modified: Vec<Vec<u8>> = std::iter::once(top_border.clone())
        .chain(inp.iter().map(|l| format!("9{}9", l).into_bytes()))
        .chain(std::iter::once(top_border))
        .collect();

    let basins: Vec<_> = modified
        .array_windows::<3>()
        .enumerate()
        .flat_map(|(y, rows)| {
            let [top, mid, bot] = rows;
            top.array_windows::<3>()
                .enumerate()
                .zip(mid.array_windows::<3>().zip(bot.array_windows::<3>()))
                .filter(|((_, t), (m, b))| {
                    let c = m[1];
                    c < t[1] && c < b[1] && c < m[0] && c < m[2]
                })
                .map(move |((x, _), _)| (x + 1, y + 1))
        })
        .collect();

    let mut part_1 = 0;
    let mut max_heap = BinaryHeap::new();
    for (x, y) in basins {
        part_1 += (modified[y][x] - b'0' + 1) as usize;

        let basin_size = floodfill(modified.as_mut_slice(), (x, y));
        max_heap.push(basin_size);
    }
    let part_2 = max_heap.into_iter_sorted().take(3).product();

    (part_1, part_2)
}

fn floodfill(caves: &mut [Vec<u8>], low_point: (usize, usize)) -> usize {
    let mut point_queue = VecDeque::from([low_point]);

    let mut count = 0;
    while let Some((x, y)) = point_queue.pop_front() {
        if caves[y][x] != b'9' {
            count += 1;
            caves[y][x] = b'9';
            point_queue.push_back((x - 1, y));
            point_queue.push_back((x + 1, y));
            point_queue.push_back((x, y - 1));
            point_queue.push_back((x, y + 1));
        }
    }

    count
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = get_input();
    b.iter(|| solve(input.clone()))
}

#[bench]
fn bench_get_input(b: &mut test::Bencher) {
    b.iter(|| get_input());
}
