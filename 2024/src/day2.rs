use std::{collections::BTreeMap, str::FromStr};

use color_eyre::eyre::{bail, Report, Result};

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

pub fn part1(input: &Input) -> u32 {
    let mut count = 0;

    for row in input.rows() {
        let monotonic = row.is_sorted_by(|a, b| a < b) || row.is_sorted_by(|a, b| a > b);
        if !monotonic {
            continue;
        }

        let limited_diff = row.windows(2).all(|pair| {
            let diff = pair[0].abs_diff(pair[1]);
            (1..=3).contains(&diff)
        });

        if limited_diff {
            count += 1;
        }
    }

    count
}

pub fn part2(input: &Input) -> u32 {
    let mut count = 0;

    'outer: for row in input.rows() {
        let mut element_removed = None;
        let mut ascending = 0;
        let mut descending = 0;
        for (i, triplet) in row.windows(3).enumerate() {
            let &[a, b, c] = triplet else { unreachable!() };

            let bc_diff_valid = (1..=3).contains(&b.abs_diff(c));
            if let Some(removed) = element_removed {
                if i == removed + 1 {
                    if bc_diff_valid {
                        if b < c {
                            ascending += 1;
                        } else {
                            descending += 1;
                        }

                        continue;
                    } else {
                        continue 'outer;
                    }
                }
            }

            // happy path, this triplet meets the conditions without removal
            let ab_diff_valid = (1..=3).contains(&a.abs_diff(b));
            if ab_diff_valid && bc_diff_valid {
                if a < b && b < c {
                    // all are ascending and the differences are okay
                    ascending += 1;
                    continue;
                } else if a > b && b > c {
                    // all are descending and the differences are okay
                    descending += 1;
                    continue;
                }
            }

            // we need to remove b
            if element_removed.is_some() {
                continue 'outer;
            } else {
                element_removed = Some(i);
            }

            let ac_diff_valid = (1..=3).contains(&a.abs_diff(c));
            if ac_diff_valid {
                if a < c {
                    ascending += 1;
                } else {
                    descending += 1;
                }
            } else {
                // no configuration was valid
                continue 'outer;
            }
        }

        if descending <= 1 || ascending <= 1 {
            count += 1;
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
