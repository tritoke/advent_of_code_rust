#![feature(test, try_trait)]
extern crate test;

use rustc_hash::{FxHashMap, FxHashSet};

const INPUT_NUM: usize = 0;

type Input = Vec<Food>;
type PartInput = [Food];

fn main() {
    let input = get_input();

    let (part_1, part_2) = solve(&input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn get_input() -> Input {
    [
        include_str!("../inputs/day21.inp"),
        include_str!("../test_inputs/day21.inp1"),
    ][INPUT_NUM]
        .lines()
        .map(|line| line.parse().unwrap())
        .collect()
}

fn solve<'a>(input: &'a PartInput) -> (usize, String) {
    // counts of safe ingredients
    let mut safe_ingredients: FxHashMap<&'a str, usize> = Default::default();

    // allergen K could be from any ingredient in V
    let mut could: FxHashMap<&'a str, FxHashSet<&'a str>> = Default::default();

    // allergen K is known to be contained by ingredient V
    let mut known: FxHashMap<&'a str, &'a str> = Default::default();

    // stack to push the new known allergens onto
    let mut remove_stack: Vec<&'a str> = Vec::new();

    for food in input {
        // update the counts of the each ingredient
        for ingredient in food.ingredients.iter() {
            *safe_ingredients.entry(&ingredient).or_insert(0) += 1;
        }

        // update could for each allergen
        for allergen in food.allergens.iter() {
            // if we know which ingredient has this allergen then just continue
            if known.contains_key(allergen.as_str()) {
                continue;
            };

            let ingredient_set: FxHashSet<&'a str> =
                food.ingredients.iter().map(|s| s.as_str()).collect();

            let potential = could
                .entry(allergen)
                .and_modify(|set| set.retain(|i| ingredient_set.contains(i)))
                .or_insert(ingredient_set);

            if potential.len() == 1 {
                let ingredient = potential.drain().next().unwrap();
                known.insert(allergen, ingredient);
                remove_stack.push(ingredient);
            }
        }

        // remove all known ingredients from other sets
        while let Some(to_remove) = remove_stack.pop() {
            for (allergen, set) in could.iter_mut() {
                set.remove(to_remove);
                if set.len() == 1 {
                    let ingredient = set.drain().next().unwrap();
                    known.insert(allergen, ingredient);
                    remove_stack.push(ingredient);
                }
            }
        }
    }

    // remove all the known allergens from the safe ingredients list
    for (_, ingredient) in known.iter() {
        safe_ingredients.remove(ingredient);
    }

    let part_1 = safe_ingredients.iter().map(|(_, n)| n).sum::<usize>();

    let mut known_sorted: Vec<(&'a str, &'a str)> = known.into_iter().collect();
    known_sorted.sort_unstable_by_key(|(a, _)| *a);

    let (_, first) = known_sorted.first().unwrap();

    let part_2: String = known_sorted
        .iter()
        .skip(1)
        .fold(first.to_string(), |mut a, (_, i)| {
            a.push(',');
            a.push_str(i);
            a
        });

    (part_1, part_2)
}

#[derive(Debug)]
struct Food {
    ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl std::str::FromStr for Food {
    type Err = std::option::NoneError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let open_brac_pos = s.find('(')?;

        let ingredient_str = s.get(0..open_brac_pos - 1)?;
        let allergen_str = s.get(open_brac_pos + 10..s.len() - 1)?;

        let ingredients = ingredient_str.split(' ').map(String::from).collect();
        let allergens = allergen_str.split(", ").map(String::from).collect();

        Ok(Food {
            ingredients,
            allergens,
        })
    }
}

#[bench]
fn bench_solution(b: &mut test::Bencher) {
    let input = get_input();

    b.iter(|| solve(&input))
}

#[bench]
fn bench_get_input(b: &mut test::Bencher) {
    b.iter(|| get_input());
}
