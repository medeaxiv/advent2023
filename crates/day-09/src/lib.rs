use itertools::Itertools;

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> i64 {
    input
        .lines()
        .map(parse)
        .map(|history| extrapolate(history.into_iter()))
        .sum()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> i64 {
    input
        .lines()
        .map(parse)
        .map(|history| extrapolate(history.into_iter().rev()))
        .sum()
}

fn extrapolate(history: impl Iterator<Item = i64>) -> i64 {
    delta_sequences(history.collect_vec())
        .map(|deltas| *deltas.last().unwrap())
        .sum()
}

fn delta_sequences(history: Vec<i64>) -> impl Iterator<Item = Vec<i64>> {
    std::iter::successors(Some(history), |seq| {
        let next = delta_sequence(seq);
        if next.iter().all(|&delta| delta == 0) {
            None
        } else {
            Some(next)
        }
    })
}

fn delta_sequence(range: &[i64]) -> Vec<i64> {
    range
        .iter()
        .tuple_windows()
        .map(|(a, b)| b - a)
        .collect_vec()
}

fn parse(line: &str) -> Vec<i64> {
    line.split_ascii_whitespace()
        .map(|e| e.parse().unwrap())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use aoc_util::test::setup_tracing;

    use super::*;

    const TEST_INPUT: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn test_part1() {
        setup_tracing();
        let solution = solve_part1(TEST_INPUT);
        assert_eq!(solution, 114);
    }

    #[test]
    fn test_part2() {
        setup_tracing();
        let solution = solve_part2(TEST_INPUT);
        assert_eq!(solution, 2);
    }
}
