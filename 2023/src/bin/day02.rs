fn main() {
    let inp = parse_input();

    let p1 = part1(&inp);
    let p2 = part2(&inp);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn parse_input() -> Vec<Game> {
    include_str!("../../inputs/day02.txt")
        .lines()
        .map(|line| {
            let mut game = Game {
                red: 0,
                green: 0,
                blue: 0,
            };

            let rounds = line.split_once(": ").expect("Line in invalid format").1;
            for block in rounds.split("; ") {
                for balls in block.split(", ") {
                    let (count, colour) = balls.split_once(' ').expect("Balls in invalid format.");
                    let count = count.parse().expect("Failed to parse count");
                    match colour {
                        "red" => game.red = game.red.max(count),
                        "green" => game.green = game.green.max(count),
                        "blue" => game.blue = game.blue.max(count),
                        x => unreachable!("A secret fourth colour appearerd! - {x:?}"),
                    }
                }
            }

            game
        })
        .collect()
}

#[derive(Debug)]
struct Game {
    red: u32,
    green: u32,
    blue: u32,
}

fn part1(inp: &[Game]) -> usize {
    inp.iter()
        .enumerate()
        .filter_map(|(i, game)| {
            let can_fit = game.red <= 12 && game.green <= 13 && game.blue <= 14;
            can_fit.then_some(i + 1)
        })
        .sum()
}

fn part2(inp: &[Game]) -> u32 {
    inp.iter()
        .map(|game| game.red * game.blue * game.green)
        .sum()
}
