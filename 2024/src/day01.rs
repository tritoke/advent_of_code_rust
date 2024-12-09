use std::{collections::BTreeMap, str::FromStr};

use color_eyre::eyre::{bail, Report, Result};

pub struct Input {
    pub left: Vec<u32>,
    pub right: Vec<u32>,
}

impl FromStr for Input {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut left = vec![];
        let mut right = vec![];

        for line in s.lines() {
            let Some((l, r)) = line.split_once("   ") else {
                bail!("Failed to split line into left and right halves: {line:?}");
            };

            left.push(l.parse()?);
            right.push(r.parse()?);
        }

        Ok(Input { left, right })
    }
}

pub fn part1(input: &Input) -> u32 {
    let mut left = input.left.clone();
    let mut right = input.right.clone();

    left.sort();
    right.sort();

    left.into_iter()
        .zip(right)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

pub fn part2(input: &Input) -> u32 {
    let mut counts: BTreeMap<_, u32> = BTreeMap::new();
    for num in &input.right {
        *counts.entry(num).or_default() += 1;
    }

    input
        .left
        .iter()
        .map(|num| num * counts.get(num).unwrap_or(&0))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3\n";

    #[test]
    fn part1_example_parse() {
        let parsed = Input::from_str(EXAMPLE_INPUT).unwrap();
        assert_eq!(parsed.left, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(parsed.right, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn part1_example_logic() {
        let parsed = Input::from_str(EXAMPLE_INPUT).unwrap();
        assert_eq!(part1(&parsed), 11);
    }

    #[test]
    fn part2_example_logic() {
        let parsed = Input::from_str(EXAMPLE_INPUT).unwrap();
        assert_eq!(part2(&parsed), 31);
    }
}
