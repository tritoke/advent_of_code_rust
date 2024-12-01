use advent_of_code_2024::day1::*;
use std::str::FromStr;

#[test]
fn test_part1() {
    let input = Input::from_str(include_str!("../inputs/day1.input")).unwrap();
    assert_eq!(part1(&input), 2430334);
}

#[test]
fn test_part2() {
    let input = Input::from_str(include_str!("../inputs/day1.input")).unwrap();
    assert_eq!(part2(&input), 28786472);
}
