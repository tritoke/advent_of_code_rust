use std::fs::File;
use std::io::prelude::*;
use std::time::Instant;
use std::cmp::Ordering;

#[derive(Eq,Debug,Copy,Clone)]
struct Item {
    a: i32,
    b: i32,
    c: i32,
} 

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.a == other.a {
            self.b.cmp(&other.b)
        } else {
            self.a.cmp(&other.a)
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(&other))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a) & (self.b == other.b)
    }
}

fn results() -> (i32, i32) {
    let mut f = File::open("inputs/1").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("reading file failed");

    let mut total = 0;
    let mut items: Vec<Item> = Vec::with_capacity(1000);

    for shift in contents
            .lines()
            .map(|s| s.parse::<i32>().unwrap()) {
        total += shift;
        items.push(
                Item {
                    a: total,
                    b: 0,
                    c: items.len() as i32,
                }
            );
    }

    let part1 = total;

    for mut item in &mut items {
        item.b = item.a / part1;
        item.a %= part1;
        if item.a < 0 {
            item.a += part1;
            item.b -= 1;
        }
    }

    items.sort();

    let mut last = Item {
        a: -1,
        b: 0,
        c: 0,
    };

    for mut item in &mut items {
        let before = item.clone();

        if item.a == last.a {
            item.a = item.b - last.b;
            item.b = last.c;
            item.c = item.a + item.b * part1;
        } else {
            item.b = i32::max_value();
            item.a = i32::max_value();
            item.c = i32::max_value();
        }
        last = before;
    }

    items.sort();

    items.reverse();
    println!("{:?}", items);
    items.reverse();

    return (part1, items[0].c);
}

pub fn solve() {
    let start = Instant::now();
    let (part1, part2) = results();
    println!("part 1: {}", part1);
    println!("part 2: {}", part2);
    println!("took {} micro-seconds", start.elapsed().as_micros());
}
