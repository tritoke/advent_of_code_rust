use std::str::FromStr;

fn main() {
    let inp = parse_input();

    let p1 = part1(&inp);
    let p2 = part2(&inp);

    println!("Part 1: {p1}");
    println!("Part 2: {p2}");
}

fn parse_input() -> Vec<(Hand, u32)> {
    include_str!("../../inputs/day07.txt")
        .lines()
        .map(|line| {
            let (hand, bid) = line.split_once(' ').expect("Line in invalid format");
            (
                hand.parse().expect("Failed to parse hand"),
                bid.parse().expect("Failed to parse bid"),
            )
        })
        .collect()
}

#[derive(Debug, Clone)]
struct Hand {
    cards: [u8; 5],
}

impl Hand {
    fn classify_kind(longest: u8, next_longest: u8) -> u8 {
        match (longest, next_longest) {
            // Five of a kind
            (5, 0) => 6,
            // Four of a kind
            (4, 1) => 5,
            // Full House
            (3, 2) => 4,
            // Three of a kind
            (3, 1) => 3,
            // Two pair
            (2, 2) => 2,
            // One pair
            (2, 1) => 1,
            // High card
            (1, 1) => 0,
            _ => unreachable!("No other runs are possible."),
        }
    }

    fn kind(&self) -> u8 {
        let mut sorted_cards = self.cards;
        sorted_cards.sort();

        let mut runs = [1, 0, 0, 0, 0];
        let mut run = 0;
        let mut last_card = sorted_cards[0];
        for card in sorted_cards.into_iter().skip(1) {
            if card != last_card {
                run += 1;
                last_card = card;
            }
            runs[run] += 1;
        }

        runs.sort_by(|a, b| a.cmp(b).reverse());
        Self::classify_kind(runs[0], runs[1])
    }

    fn joker_kind(&self) -> u8 {
        let mut sorted_cards = self.cards;
        sorted_cards.sort();

        let mut runs = [0, 0, 0, 0, 0, 0];
        let mut run = 0;
        let mut last_card = u8::MAX;
        let mut joker_count = 0;
        for card in sorted_cards {
            if card == 11 {
                joker_count += 1;
                continue;
            }

            if card != last_card {
                run += 1;
                last_card = card;
            }
            runs[run] += 1;
        }

        runs.sort_by(|a, b| a.cmp(b).reverse());
        Self::classify_kind(runs[0] + joker_count, runs[1])
    }
}

impl FromStr for Hand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: [u8; 5] = <[u8; 5]>::try_from(s.as_bytes())?.map(|c| match c {
            b'2'..=b'9' => c - b'0',
            b'T' => 10,
            b'J' => 11,
            b'Q' => 12,
            b'K' => 13,
            b'A' => 14,
            _ => unreachable!("These are the only valid cards."),
        });

        Ok(Hand { cards })
    }
}

fn part1(inp: &[(Hand, u32)]) -> u32 {
    let mut inputs = inp.to_vec();
    inputs.sort_by_cached_key(|(hand, _)| (hand.kind(), hand.cards));
    inputs
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_hand, bid))| acc + (i as u32 + 1) * bid)
}

fn part2(inp: &[(Hand, u32)]) -> u32 {
    let mut inputs = inp.to_vec();
    inputs.sort_by_cached_key(|(hand, _)| {
        (
            hand.joker_kind(),
            hand.cards.map(|c| if c == 11 { 0 } else { c }),
        )
    });
    inputs
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_hand, bid))| acc + (i as u32 + 1) * bid)
}
