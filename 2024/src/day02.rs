use std::str::FromStr;

use color_eyre::eyre::{Report, Result};

pub struct Input {
    pub(crate) numbers: Vec<u8>,
    pub(crate) row_widths: Vec<usize>,
}

impl Input {
    pub fn rows(&self) -> Rows<'_> {
        Rows {
            numbers: &self.numbers,
            row_widths: &self.row_widths,
            number_index: 0,
            row: 0,
        }
    }
}

pub struct Rows<'a> {
    numbers: &'a [u8],
    row_widths: &'a [usize],
    number_index: usize,
    row: usize,
}

impl<'a> Iterator for Rows<'a> {
    type Item = &'a [u8];

    fn next(&mut self) -> Option<Self::Item> {
        let width = self.row_widths.get(self.row)?;
        let start = self.number_index;

        self.row += 1;
        self.number_index += width;

        self.numbers.get(start..start + width)
    }
}

impl FromStr for Input {
    type Err = Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<_> = s
            .split_ascii_whitespace()
            .map(str::parse)
            .collect::<Result<_, _>>()?;

        let row_widths = s
            .lines()
            .map(|line| line.chars().filter(|c| c.is_ascii_whitespace()).count() + 1)
            .collect();

        Ok(Input {
            numbers,
            row_widths,
        })
    }
}

fn row_is_safe(row: &[u8]) -> bool {
    let monotonic = row.is_sorted_by(|a, b| a < b) || row.is_sorted_by(|a, b| a > b);
    if !monotonic {
        return false;
    }

    row.windows(2).all(|pair| {
        let diff = pair[0].abs_diff(pair[1]);
        (1..=3).contains(&diff)
    })
}

pub fn part1(input: &Input) -> u32 {
    let mut count = 0;

    for row in input.rows() {
        if row_is_safe(row) {
            count += 1;
        }
    }

    count
}

fn row_is_safe_skipping(row: &[u8], skip: usize) -> bool {
    let base_iter = row
        .iter()
        .enumerate()
        .filter_map(|(i, value)| (i != skip).then_some(value));
    let first_skipped = base_iter.clone().skip(1);
    let mut ascending = 0;
    let mut descending = 0;
    for (a, b) in base_iter.zip(first_skipped) {
        if !(1..=3).contains(&a.abs_diff(*b)) {
            return false;
        }

        if a > b {
            descending += 1;
        } else {
            ascending += 1;
        }
    }

    ascending == 0 || descending == 0
}

pub fn part2(input: &Input) -> u32 {
    let mut count = 0;

    'outer: for row in input.rows() {
        if row_is_safe(row) {
            count += 1;
            continue;
        }

        for i in 0..row.len() {
            if row_is_safe_skipping(row, i) {
                count += 1;
                continue 'outer;
            }
        }
    }

    count
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &str = "\
        7 6 4 2 1\n\
        1 2 7 8 9\n\
        9 7 6 2 1\n\
        1 3 2 4 5\n\
        8 6 4 4 1\n\
        1 3 6 7 9\n\
    ";

    #[test]
    fn example_parse() {
        let parsed = Input::from_str(EXAMPLE_INPUT).unwrap();

        assert!(parsed.row_widths.iter().all(|x| *x == 5));

        #[rustfmt::skip]
        assert_eq!(
            parsed.numbers,
            [
                7, 6, 4, 2, 1, 1, 2, 7, 8, 9, 9, 7, 6, 2, 1, 
                1, 3, 2, 4, 5, 8, 6, 4, 4, 1, 1, 3, 6, 7, 9,
            ]
        );
    }

    #[test]
    fn input_recover_rows() {
        let parsed = Input::from_str(EXAMPLE_INPUT).unwrap();

        let correct = [
            &[7, 6, 4, 2, 1],
            &[1, 2, 7, 8, 9],
            &[9, 7, 6, 2, 1],
            &[1, 3, 2, 4, 5],
            &[8, 6, 4, 4, 1],
            &[1, 3, 6, 7, 9],
        ];

        for (a, b) in parsed.rows().zip(correct) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn input_recover_rows_wonky() {
        let parsed = Input::from_str("1 2 3\n1 2\n1 2 3 4\n").unwrap();

        let mut rows = parsed.rows();
        assert_eq!(rows.next(), Some(&[1u8, 2, 3] as &[_]));
        assert_eq!(rows.next(), Some(&[1u8, 2] as &[_]));
        assert_eq!(rows.next(), Some(&[1u8, 2, 3, 4] as &[_]));
        assert_eq!(rows.next(), None);
    }

    #[test]
    fn part1_example_logic() {
        let parsed = Input::from_str(EXAMPLE_INPUT).unwrap();
        assert_eq!(part1(&parsed), 2);
    }

    #[test]
    fn part2_example_logic() {
        let parsed = Input::from_str(EXAMPLE_INPUT).unwrap();
        assert_eq!(part2(&parsed), 4);
    }
}
