#![feature(test)]
extern crate test;

use std::num::ParseIntError;
use std::str::FromStr;

fn main() {
    let input = get_input();

    let part_1 = part1(&input);
    let part_2 = part2(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn get_input() -> Vec<Passport> {
    include_str!("../inputs/day04.inp")
        .split("\n\n")
        .take_while(|line| !line.is_empty())
        .map(|line| line.parse::<Passport>().unwrap())
        .collect()
}

fn part1(input: &[Passport]) -> usize {
    input.iter().filter(|passport| passport.is_valid()).count()
}

fn part2(input: &[Passport]) -> usize {
    input
        .iter()
        .filter(|passport| passport.is_valid_strict())
        .count()
}

#[derive(Default, Debug)]
struct Passport {
    birth_year: u32,
    issue_year: u32,
    expiration_year: u32,
    height: String,
    hair_color: String,
    eye_color: String,
    passport_id: String,
    country_id: u32,
}

impl FromStr for Passport {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pairs = s.split_whitespace().map(|pair| {
            let (key, _) = pair.split_at(3);
            let (_, val) = pair.split_at(4);
            (key, val)
        });

        let mut passport: Passport = Default::default();

        for (key, val) in pairs {
            match key {
                "byr" => passport.birth_year = val.parse::<u32>().unwrap_or(0),
                "iyr" => passport.issue_year = val.parse::<u32>().unwrap_or(0),
                "eyr" => passport.expiration_year = val.parse::<u32>().unwrap_or(0),
                "hgt" => passport.height = val.to_string(),
                "hcl" => passport.hair_color = val.to_string(),
                "ecl" => passport.eye_color = val.to_string(),
                "pid" => passport.passport_id = val.to_string(),
                "cid" => passport.country_id = val.parse::<u32>().unwrap_or(0),

                &_ => panic!("Got unexpected key: {:?} while parsing.", key),
            }
        }

        Ok(passport)
    }
}

impl Passport {
    fn is_valid(&self) -> bool {
        self.birth_year != 0
            && self.issue_year != 0
            && self.expiration_year != 0
            && !self.height.is_empty()
            && !self.hair_color.is_empty()
            && !self.eye_color.is_empty()
            && !self.passport_id.is_empty()
    }

    fn is_valid_strict(&self) -> bool {
        let birth_year_check = self.birth_year >= 1920 && self.birth_year <= 2002;
        let issue_year_check = self.issue_year >= 2010 && self.issue_year <= 2020;
        let expiration_year_check = self.expiration_year >= 2020 && self.expiration_year <= 2030;
        let height_check = self.check_height();
        let hair_color_check = self.check_hair_color();
        let eye_color_check = self.check_eye_color();
        let passport_id_check = self.check_passport_id();

        birth_year_check
            && issue_year_check
            && expiration_year_check
            && height_check
            && hair_color_check
            && eye_color_check
            && passport_id_check
    }

    fn check_height(&self) -> bool {
        // assert the string is not empty
        if self.height.is_empty() {
            return false;
        }

        let height_str = &self.height;

        if let Some(in_pos) = height_str.find("in") {
            let (num, _) = height_str.split_at(in_pos);
            let height: i32 = num.parse().unwrap();

            // check the height is a valid height in inches
            (59..=76).contains(&height)
        } else if let Some(cm_pos) = height_str.find("cm") {
            let (num, _) = height_str.split_at(cm_pos);
            let height: i32 = num.parse().unwrap();

            // check the height is a valid height in cm
            (150..=193).contains(&height)
        } else {
            false
        }
    }

    fn check_hair_color(&self) -> bool {
        // all colors have length 7
        if self.hair_color.len() != 7 {
            return false;
        }

        let hair_color_str = self.hair_color.as_str();

        let (prefix, number) = hair_color_str.split_at(1);

        let prefix_check = prefix == "#";
        let hex_check = u32::from_str_radix(number, 16).is_ok();

        prefix_check && hex_check
    }

    fn check_eye_color(&self) -> bool {
        // all colors are length 3
        if self.eye_color.len() != 3 {
            return false;
        }

        matches!(
            self.eye_color.as_str(),
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
        )
    }

    fn check_passport_id(&self) -> bool {
        if self.passport_id.len() != 9 {
            return false;
        }

        u32::from_str_radix(self.passport_id.as_str(), 10).is_ok()
    }
}

#[bench]
fn bench_part1_solution(b: &mut test::Bencher) {
    let input = get_input();
    b.iter(|| part1(&input))
}

#[bench]
fn bench_part2_solution(b: &mut test::Bencher) {
    let input = get_input();
    b.iter(|| part2(&input))
}

#[bench]
fn bench_get_input(b: &mut test::Bencher) {
    b.iter(|| get_input());
}
