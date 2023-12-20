use itertools::Itertools;

fn main() {
    let (inp1, inp2) = parse_input();

    let p1 = solve(&inp1);
    let p2 = solve(&inp2);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn parse_input() -> (Vec<(u64, u64)>, Vec<(u64, u64)>) {
    let rows: Vec<_> = include_str!("../../inputs/day11.txt").lines().collect();

    // iterate all cells tracking whether any cell was taken in each col/row
    let mut row_taken = vec![false; rows.len()];
    let mut col_taken = vec![false; rows[0].len()];

    // collect all galaxies as well
    let mut galaxies = vec![];

    for (row, row_chars) in rows.iter().enumerate() {
        for (col, tile) in row_chars.bytes().enumerate() {
            if tile == b'#' {
                row_taken[row] = true;
                col_taken[col] = true;
                galaxies.push((row as u64, col as u64));
            }
        }
    }

    // we can now compute a vector of offsets for both the row and column
    let row_offsets: Vec<_> = row_taken
        .into_iter()
        .scan(0, |state, taken| {
            *state += (!taken) as u64;
            Some(*state)
        })
        .collect();

    let col_offsets: Vec<_> = col_taken
        .into_iter()
        .scan(0, |state, taken| {
            *state += (!taken) as u64;
            Some(*state)
        })
        .collect();

    // we can now adjust the position of each galaxy using these offsets
    let mut gal1 = galaxies.clone();
    let mut gal2 = galaxies;

    for (row, col) in gal1.iter_mut() {
        *row += row_offsets[*row as usize];
        *col += col_offsets[*col as usize];
    }

    for (row, col) in gal2.iter_mut() {
        *row += row_offsets[*row as usize] * 999_999;
        *col += col_offsets[*col as usize] * 999_999;
    }

    (gal1, gal2)
}

fn solve(inp: &[(u64, u64)]) -> u64 {
    inp.iter()
        .combinations(2)
        .map(|galaxies| {
            let (x1, y1) = *galaxies[0];
            let (x2, y2) = *galaxies[1];
            x1.abs_diff(x2) + y1.abs_diff(y2)
        })
        .sum()
}
