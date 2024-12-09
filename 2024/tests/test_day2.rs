use advent_of_code_2024::day2::*;
use std::str::FromStr;

#[test]
fn test_day02_part1() {
    let input = Input::from_str(include_str!("../inputs/day2.input")).unwrap();
    for row in input.rows() {
        dbg!(row);
    }
    assert_eq!(part1(&input), 670);
}

#[test]
fn test_day02_part2() {
    let input = Input::from_str(include_str!("../inputs/day2.input")).unwrap();
    assert_eq!(part2(&input), 700);
}
