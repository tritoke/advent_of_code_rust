use rustc_hash::FxHashMap;
use std::cell::RefCell;

fn main() {
    let inp = parse_input();

    let p1 = part1(&inp);
    let p2 = part2(&inp);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn parse_input() -> Vec<(&'static [u8], Vec<usize>)> {
    include_str!("../../inputs/day12.txt")
        .lines()
        .map(|line| {
            let (record, nums) = line.split_once(' ').expect("Failed to split record");
            (
                record.as_bytes(),
                nums.split(',')
                    .map(str::parse)
                    .collect::<Result<_, _>>()
                    .expect("Failed to parse numbers"),
            )
        })
        .collect()
}

fn enumerate_possible_block_placements(blocks: Vec<&'static [u8]>, groups: &[usize]) -> u64 {
    type FuncInput = (Vec<&'static [u8]>, Vec<usize>);
    thread_local! {
        static MEMO: RefCell<FxHashMap<FuncInput, u64>> = RefCell::new(FxHashMap::default());
    };

    // if there are no blocks and no groups then there is only one way
    if blocks.is_empty() {
        return groups.is_empty().into();
    }

    // if there are no groups then there is just one way to organise that
    if groups.is_empty() {
        // we must also check that we don't count invalid ones
        let valid = blocks.iter().all(|block| !block.contains(&b'#'));
        return valid.into();
    }

    // check the memoisation cache after the trivial cases
    if let Some(stored) =
        MEMO.with_borrow(|memo| memo.get(&(blocks.clone(), groups.to_owned())).copied())
    {
        return stored;
    }

    // enumerate starting positions until they enumerate to zero remaining
    let fixing_group_len = groups[0];
    let remaining_groups = &groups[1..];
    let mut total = 0;

    for (idx, &block) in blocks.iter().enumerate() {
        let can_continue = !block.contains(&b'#');
        if block.len() < fixing_group_len {
            if can_continue {
                continue;
            } else {
                break;
            }
        }

        for offset in 0..=block.len() - fixing_group_len {
            let left_valid = offset
                .checked_sub(1)
                .map(|i| !block[..=i].contains(&b'#'))
                .unwrap_or(true);
            let right_valid = block
                .get(offset + fixing_group_len)
                .map(|&b| b != b'#')
                .unwrap_or(true);

            // if this is a valid position to fix the group at then recurse with the blocks adjusted
            if left_valid && right_valid {
                let mut sub_blocks = blocks[idx..].to_owned();

                // if the group we just placed ends the block then remove it
                let fixed_space = offset + fixing_group_len + 1;
                if sub_blocks[0].len() <= fixed_space {
                    sub_blocks.remove(0);
                } else {
                    // otherwise cut off the "fixed" portion of the first block
                    sub_blocks[0] = &sub_blocks[0][fixed_space..];
                }

                total += enumerate_possible_block_placements(sub_blocks, remaining_groups);
            }
        }

        // if the block contains a hashtag the fixed block MUST go there
        if !can_continue {
            break;
        }
    }

    // store the result in the cache
    MEMO.with_borrow_mut(|memo| {
        memo.insert((blocks.clone(), groups.to_owned()), total);
    });

    total
}

fn count_arrangements(s: &'static [u8], mut groups: &[usize]) -> u64 {
    // first determine any fixed blocks
    let mut blocks: Vec<_> = s
        .split(|&x| x == b'.')
        .filter(|block| !block.is_empty())
        .collect();

    loop {
        if blocks.is_empty() || groups.is_empty() {
            break;
        }

        let first_group = *groups.first().unwrap();
        let last_group = *groups.last().unwrap();
        let first_block = blocks.first().unwrap();
        let last_block = blocks.last().unwrap();
        if first_block.len() == first_group && first_block.contains(&b'#') {
            blocks.remove(0);
            groups = &groups[1..];
        } else if last_block.len() == last_group && last_block.contains(&b'#') {
            blocks.pop();
            groups = &groups[..groups.len() - 1];
        } else {
            break;
        }
    }

    // if there are no unfixed blocks we are already done :)
    if blocks.is_empty() {
        return 1;
    }

    enumerate_possible_block_placements(blocks, groups)
}

fn part1(inp: &[(&'static [u8], Vec<usize>)]) -> u64 {
    inp.iter()
        .map(|(record, nums)| count_arrangements(record, nums))
        .sum()
}

fn part2(inp: &[(&'static [u8], Vec<usize>)]) -> u64 {
    inp.iter()
        .map(|(record, nums)| {
            // memory is cheap :)
            let unfolded_record: &'static [u8] =
                Box::new([*record].repeat(5).join(&b"?"[..])).leak();
            let unfolded_nums = nums.repeat(5);
            count_arrangements(unfolded_record, &unfolded_nums)
        })
        .sum()
}
