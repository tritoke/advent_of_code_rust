use advent_of_code_2024::day03::*;

#[test]
fn test_day03_part1() {
    let input = include_str!("../inputs/day3.input");
    assert_eq!(part1(&input), 178538786);
}

#[test]
fn test_day03_part2() {
    let input = include_str!("../inputs/day3.input");
    assert_eq!(part2(&input), 102467299);
}
