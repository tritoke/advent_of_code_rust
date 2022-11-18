#![feature(test)]
extern crate test;

const INPUT_NUM: usize = 0;

fn main() {
    let input = get_input();

    let (part_1, part_2) = solve(input);

    println!("Part 1: {}", part_1);
    println!("Part 2: {}", part_2);
}

fn get_input() -> Vec<&'static str> {
    [
        include_str!("../inputs/day10.inp"),
        include_str!("../test_inputs/day10.inp1"),
    ][INPUT_NUM]
        .lines()
        .collect()
}

fn solve(inp: Vec<&'static str>) -> (u32, u64) {
    use ParseChunkError::*;

    fn score_corrupt_bracket(bracket: u8) -> u32 {
        match bracket {
            b')' => 3,
            b']' => 57,
            b'}' => 1197,
            b'>' => 25137,
            _ => unreachable!(),
        }
    }

    let mut part_1 = 0;
    let mut incomplete_scores = Vec::new();

    for chunk in inp {
        match parse_chunk(chunk) {
            Err(CorruptedChunk(c)) => {
                part_1 += score_corrupt_bracket(c);
            }
            Err(IncompleteChunk(mut stack)) => {
                stack.reverse();
                incomplete_scores.push(score_incomplete_chunk(stack.as_slice()));
            }
            _ => {}
        }
    }

    let midpoint = incomplete_scores.len() / 2;
    let part_2 = *incomplete_scores.select_nth_unstable(midpoint).1;

    (part_1, part_2)
}

fn score_incomplete_chunk(stack: &[u8]) -> u64 {
    fn score_incomplete_bracket(bracket: u8) -> u64 {
        match bracket {
            b'(' | b')' => 1,
            b'[' | b']' => 2,
            b'{' | b'}' => 3,
            b'<' | b'>' => 4,
            _ => unreachable!(),
        }
    }

    stack
        .iter()
        .fold(0, |a, &b| a * 5 + score_incomplete_bracket(b))
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum ParseChunkError {
    CorruptedChunk(u8),
    IncompleteChunk(Vec<u8>),
}

fn parse_chunk(chunk: &str) -> Result<(), ParseChunkError> {
    fn is_opening(c: u8) -> bool {
        matches!(c, b'(' | b'[' | b'{' | b'<')
    }

    fn is_pair(l: u8, r: u8) -> bool {
        matches!(
            (l, r),
            (b'(', b')') | (b'[', b']') | (b'{', b'}') | (b'<', b'>')
        )
    }

    let mut stack = Vec::new();

    for c in chunk.bytes() {
        if is_opening(c) {
            stack.push(c);
        } else if let Some(top_c) = stack.pop() {
            if !is_pair(top_c, c) {
                return Err(ParseChunkError::CorruptedChunk(c));
            }
        } else {
            return Err(ParseChunkError::IncompleteChunk(stack));
        }
    }

    if !stack.is_empty() {
        return Err(ParseChunkError::IncompleteChunk(stack));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_chunk_valid_1() {
        let s = "()";
        assert_eq!(parse_chunk(s), Ok(()));
    }

    #[test]
    fn test_parse_chunk_valid_2() {
        let s = "([])";
        assert_eq!(parse_chunk(s), Ok(()));
    }

    #[test]
    fn test_parse_chunk_valid_3() {
        let s = "{()()()}";
        assert_eq!(parse_chunk(s), Ok(()));
    }

    #[test]
    fn test_parse_chunk_valid_4() {
        let s = "<([{}])>";
        assert_eq!(parse_chunk(s), Ok(()));
    }

    #[test]
    fn test_parse_chunk_valid_5() {
        let s = "[<>({}){}[([])<>]]";
        assert_eq!(parse_chunk(s), Ok(()));
    }

    #[test]
    fn test_parse_chunk_valid_6() {
        let s = "(((((((((())))))))))";
        assert_eq!(parse_chunk(s), Ok(()));
    }

    #[test]
    fn test_parse_chunk_corrupted_1() {
        let s = "(]";
        assert_eq!(parse_chunk(s), Err(ParseChunkError::CorruptedChunk(b']')));
    }

    #[test]
    fn test_parse_chunk_corrupted_2() {
        let s = "{()()()>";
        assert_eq!(parse_chunk(s), Err(ParseChunkError::CorruptedChunk(b'>')));
    }

    #[test]
    fn test_parse_chunk_corrupted_3() {
        let s = "(((()))}";
        assert_eq!(parse_chunk(s), Err(ParseChunkError::CorruptedChunk(b'}')));
    }

    #[test]
    fn test_parse_chunk_corrupted_4() {
        let s = "<([]){()}[{}])";
        assert_eq!(parse_chunk(s), Err(ParseChunkError::CorruptedChunk(b')')));
    }

    #[test]
    fn test_score_incomplete_chunk_correct_1() {
        let s = b"}}]])})]";
        assert_eq!(score_incomplete_chunk(s), 288957);
    }

    #[test]
    fn test_score_incomplete_chunk_correct_2() {
        let s = b")}>]})";
        assert_eq!(score_incomplete_chunk(s), 5566);
    }

    #[test]
    fn test_score_incomplete_chunk_correct_3() {
        let s = b"}}>}>))))";
        assert_eq!(score_incomplete_chunk(s), 1480781);
    }

    #[test]
    fn test_score_incomplete_chunk_correct_4() {
        let s = b"]]}}]}]}>";
        assert_eq!(score_incomplete_chunk(s), 995444);
    }

    #[test]
    fn test_score_incomplete_chunk_correct_5() {
        let s = b"])}>";
        assert_eq!(score_incomplete_chunk(s), 294);
    }

    #[bench]
    fn bench_solution(b: &mut test::Bencher) {
        let input = get_input();
        b.iter(|| solve(input.clone()))
    }

    #[bench]
    fn bench_get_input(b: &mut test::Bencher) {
        b.iter(|| get_input());
    }
}
