use std::collections::HashMap;

use aoc_util::cache::{Cache, NoCache};
use itertools::Itertools;
use rayon::prelude::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> usize {
    input
        .par_lines()
        .map(parse)
        .map(|(springs, pattern)| {
            let mut cache = NoCache;
            count_fits(&springs, &pattern, &mut cache)
        })
        .sum()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

const PART2_EXPANSION: usize = 5;

fn solve_part2(input: &str) -> usize {
    input
        .par_lines()
        .map(parse)
        .map(|item| expand(item, PART2_EXPANSION))
        .map(|(springs, pattern)| {
            let mut cache = HashMap::new();
            count_fits(&springs, &pattern, &mut cache)
        })
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

fn parse(line: &str) -> (Vec<SpringState>, Vec<usize>) {
    let (springs, pattern) = line.split_once(' ').unwrap();

    let springs = springs
        .chars()
        .map(|c| match c {
            '.' => SpringState::Operational,
            '#' => SpringState::Damaged,
            '?' => SpringState::Unknown,
            _ => unreachable!(),
        })
        .collect_vec();

    let pattern = pattern.split(',').map(|s| s.parse().unwrap()).collect_vec();

    (springs, pattern)
}

fn expand(
    (springs, pattern): (Vec<SpringState>, Vec<usize>),
    n: usize,
) -> (Vec<SpringState>, Vec<usize>) {
    let mut expanded_springs = Vec::with_capacity(springs.len() * n + n - 1);
    for i in 0..n {
        if i > 0 {
            expanded_springs.push(SpringState::Unknown);
        }
        expanded_springs.extend_from_slice(&springs);
    }

    (expanded_springs, pattern.repeat(n))
}

fn can_fit_length(springs: &[SpringState], from: usize, length: usize) -> bool {
    if from + length > springs.len() {
        return false;
    }

    if let Some(&SpringState::Damaged) = from.checked_sub(1).and_then(|idx| springs.get(idx)) {
        return false;
    }

    if springs[from..(from + length)].contains(&SpringState::Operational) {
        return false;
    }

    if let Some(&SpringState::Damaged) = springs.get(from + length) {
        return false;
    }

    true
}

fn count_fits<C>(springs: &[SpringState], pattern: &[usize], cache: &mut C) -> usize
where
    C: Cache<(usize, usize), usize>,
{
    fn count_internal<C>(
        pos: (usize, usize),
        springs: &[SpringState],
        pattern: &[usize],
        cache: &mut C,
    ) -> usize
    where
        C: Cache<(usize, usize), usize>,
    {
        if let Some(&cached) = cache.get(&pos) {
            return cached;
        }

        if springs.is_empty() && !pattern.is_empty() {
            return 0;
        }

        if pattern.is_empty() {
            if springs.contains(&SpringState::Damaged) {
                return 0;
            } else {
                return 1;
            }
        }

        let len = pattern[0];
        let mut total = 0;

        if can_fit_length(springs, 0, len) {
            let next_pos = (pos.0 + len + 1, pos.1 + 1);
            total += count_internal(next_pos, slice_from(springs, len + 1), &pattern[1..], cache)
        }

        if springs[0] != SpringState::Damaged {
            total += count_internal((pos.0 + 1, pos.1), slice_from(springs, 1), pattern, cache);
        }

        cache.insert(pos, total);
        total
    }

    count_internal((0, 0), springs, pattern, cache)
}

fn slice_from<T>(slice: &[T], idx: usize) -> &[T] {
    let idx = idx.min(slice.len());
    &slice[idx..]
}

#[cfg(test)]
mod tests {
    use aoc_util::test::setup_tracing;
    use rstest::rstest;

    use super::*;

    const TEST_INPUT: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    fn test_part1() {
        setup_tracing();
        let solution = solve_part1(TEST_INPUT);
        assert_eq!(solution, 21);
    }

    #[rstest]
    #[case(". 1", false)]
    #[case(".? 1", false)]
    #[case(".# 1", false)]
    #[case("? 1", true)]
    #[case("?? 1", true)]
    #[case("?? 2", true)]
    #[case("?? 3", false)]
    #[case("??# 3", true)]
    #[case("# 1", true)]
    #[case("## 1", false)]
    #[case("## 2", true)]
    #[case("#? 1", true)]
    #[case("#? 2", true)]
    #[case("#?# 3", true)]
    #[case("#?#. 3", true)]
    #[case("#?#. 4", false)]
    fn test_can_fit_length(#[case] input: &str, #[case] expected: bool) {
        setup_tracing();
        let (springs, pattern) = parse(input);
        let length = pattern[0];
        let solution = can_fit_length(&springs, 0, length);
        assert_eq!(solution, expected);
    }

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 4)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 1)]
    #[case("????.######..#####. 1,6,5", 4)]
    #[case("?###???????? 3,2,1", 10)]
    fn test_part1_single(#[case] line: &str, #[case] expected: usize) {
        setup_tracing();
        let (springs, pattern) = parse(line);
        let mut cache = NoCache;
        let solution = count_fits(&springs, &pattern, &mut cache);
        assert_eq!(solution, expected);
    }

    #[test]
    fn test_part2() {
        setup_tracing();
        let solution = solve_part2(TEST_INPUT);
        assert_eq!(solution, 525152);
    }

    #[rstest]
    #[case("???.### 1,1,3", 1)]
    #[case(".??..??...?##. 1,1,3", 16384)]
    #[case("?#?#?#?#?#?#?#? 1,3,1,6", 1)]
    #[case("????.#...#... 4,1,1", 16)]
    #[case("????.######..#####. 1,6,5", 2500)]
    #[case("?###???????? 3,2,1", 506250)]
    fn test_part2_single(#[case] line: &str, #[case] expected: usize) {
        setup_tracing();
        let (springs, pattern) = expand(parse(line), PART2_EXPANSION);
        let mut cache = HashMap::new();
        let solution = count_fits(&springs, &pattern, &mut cache);
        assert_eq!(solution, expected);
    }
}
