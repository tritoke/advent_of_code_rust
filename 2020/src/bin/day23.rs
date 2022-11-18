#![feature(test)]
extern crate test;

fn main() {
    let input: Vec<usize> = "389125467"
        // "219748365"
        // "962713854"
        // "123456789"
        .chars()
        .flat_map(|c| c.to_digit(10).map(|n| n as usize))
        .collect();

    let ans1 = part1(input.as_slice());

    println!("Part 1: {:?}", ans1);
}

fn part1(input: &[usize]) -> u64 {
    let board = game(input, 10, 100);

    let mut i = 1;
    let mut total = 0;

    loop {
        if board[i] == 1 {
            break;
        };

        if board[i] != 10 && board[i] != 0 {
            total = total * 10 + board[i];
        }

        i = board[i];
    }

    total as u64
}

fn print_ll(ll: &[usize], num: usize) {
    let mut v = vec![];
    let mut i = 0;
    for _ in 0..num {
        i = ll[i];
        v.push(i);
    }
    println!("{:?}", v);
}

fn populate_holding(cups: &[usize], holding: &mut [usize; 3], curr: usize) {
    for i in 0..3 {
        let mut c = if i == 0 {
            curr
        } else {
            holding[i - 1]
        };

        if cups[c] == 0 {
            holding[i] = cups[0];
        } else {
            holding[i] = cups[c];
        }
    }
}

fn game(input: &[usize], size: usize, iterations: u32) -> Vec<usize> {
    let mut cups: Vec<usize> = (1..=dbg!(size.max(dbg!(input.len() + 1)))).collect();

    for (i, j) in std::iter::once(&0)
        .chain(input.iter())
        .zip(input.iter().chain(std::iter::once(&(input.len() + 1))))
    {
        cups[*i] = *j;
    }

    // insert start link
    if size == input.len() + 1 {
        cups[*input.last().unwrap()] = 0;
    } else {
        *cups.last_mut().unwrap() = 0;
    }

    let mut curr = cups[0];
    let mut holding = [0; 3];

    for _ in 0..iterations {
        populate_holding(cups.as_slice(), &mut holding, curr);

        print_ll(&cups, 10);
        println!("{:?}", holding);
        println!("{}", curr);

        let mut dest = if curr == 1 { cups.len() - 1 } else { curr - 1 };
        while holding.contains(&dest) {
            dest = if dest == 1 { cups.len() - 2 } else { dest - 1 };
        }
        println!("{}", dest);

        let tmp = cups[dest];
        cups[dest] = holding[0];
        cups[curr] = cups[holding[2]];
        cups[holding[2]] = tmp;

        curr = cups[curr];
    }

    print_ll(cups.as_slice(), 9);

    cups
}
