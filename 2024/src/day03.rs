pub type Input = str;

pub fn part1(input: &Input) -> u32 {
    let mut sum = 0;

    for seg in input.split("mul(") {
        let Some((arguments, _)) = seg.split_once(')') else {
            continue;
        };

        let Some((left, right)) = arguments.split_once(',') else {
            continue;
        };

        let Some(left) = left.parse::<u32>().ok() else {
            continue;
        };

        let Some(right) = right.parse::<u32>().ok() else {
            continue;
        };

        sum += left * right;
    }

    sum
}

pub fn part2(input: &Input) -> u32 {
    let mut sum = 0;

    for seg in input.split("do()") {
        dbg!(seg);

        let enabled = match seg.split_once("don't()") {
            Some((left, _)) => left,
            None => seg,
        };

        dbg!(enabled);

        sum += part1(enabled);
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example_logic() {
        let parsed = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(parsed), 161);
    }

    #[test]
    fn part2_example_logic() {
        let parsed = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(parsed), 48);
    }
}
