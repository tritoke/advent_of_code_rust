#![feature(test)]
extern crate test;

type Input = [Vec<char>];

fn main() {
    let input = get_input();

    let part_1 = part1(&input);
    let part_2 = part2(&input, part_1);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn get_input() -> Vec<Vec<char>> {
    include_str!("../inputs/day03.inp")
        .lines()
        .take_while(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

#[derive(Debug)]
struct Slope {
    right: usize,
    down: usize,
}

impl Slope {
    fn calc_index(&self, depth: usize, slope_width: usize) -> usize {
        ((depth / self.down) * self.right) % slope_width
    }

    fn take_line(&self, depth: usize) -> bool {
        depth % self.down == 0
    }

    fn hits(&self, trees: &Input) -> usize {
        let slope_width = trees[0].len();

        trees
            .iter()
            .enumerate()
            .filter(|(index, _)| self.take_line(*index))
            .map(|(index, tree_line)| {
                let hits = tree_line[self.calc_index(index, slope_width)];
                match hits {
                    '#' => 1,
                    _ => 0,
                }
            })
            .sum()
    }
}

fn part1(input: &Input) -> usize {
    let slope = Slope { right: 3, down: 1 };

    slope.hits(input)
}

fn part2(input: &Input, part_1_ans: usize) -> usize {
    let slopes = vec![
        Slope { right: 1, down: 1 },
        Slope { right: 5, down: 1 },
        Slope { right: 7, down: 1 },
        Slope { right: 1, down: 2 },
    ];

    slopes
        .iter()
        .map(|slope| slope.hits(input))
        .product::<usize>()
        * part_1_ans
}

#[bench]
fn bench_part1_solution(b: &mut test::Bencher) {
    let input = get_input();
    b.iter(|| part1(&input))
}

#[bench]
fn bench_part2_solution(b: &mut test::Bencher) {
    let input = get_input();
    b.iter(|| part2(&input, 184))
}

#[bench]
fn bench_get_input(b: &mut test::Bencher) {
    b.iter(|| get_input());
}
