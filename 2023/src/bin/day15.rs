fn main() {
    let inp = parse_input();

    let p1 = part1(&inp);
    let p2 = part2(&inp);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn parse_input() -> Vec<&'static str> {
    include_str!("../../inputs/day15.txt")
        .trim()
        .split(',')
        .collect()
}

fn hash(data: &str) -> usize {
    data.bytes()
        .fold(0u8, |state, c| (state.wrapping_add(c)).wrapping_mul(17))
        .into()
}

fn part1(inp: &[&str]) -> u32 {
    inp.iter().map(|step| hash(step) as u32).sum()
}

fn part2(inp: &[&str]) -> u32 {
    let mut map = vec![];
    map.resize_with(256, Vec::new);

    for step in inp {
        if let Some((label, focal_len)) = step.split_once('=') {
            let r#box = &mut map[hash(label)];
            let focal_len = focal_len.parse::<u8>().unwrap();

            if let Some((_, fl)) = r#box.iter_mut().find(|(label_, _)| *label_ == label) {
                *fl = focal_len;
            } else {
                r#box.push((label, focal_len));
            }
        } else {
            let label = &step[..step.len() - 1];
            map[hash(label)].retain(|(label_, _)| *label_ != label);
        }
    }

    map.into_iter()
        .enumerate()
        .map(|(i, r#box)| {
            r#box
                .into_iter()
                .enumerate()
                .map(|(j, (_, focal_len))| (i as u32 + 1) * (j as u32 + 1) * focal_len as u32)
                .sum::<u32>()
        })
        .sum()
}
