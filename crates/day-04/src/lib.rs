use std::num::NonZeroU32;

use aoc_util::cache::Cache;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{self, space1},
    multi::many1,
    sequence::{preceded, separated_pair, tuple},
    IResult, Parser,
};
use nom_supreme::final_parser::final_parser;

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| parse(line).unwrap())
        .map(|card| card.score())
        .sum()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> u32 {
    let dependencies = input
        .lines()
        .map(|line| parse(line).unwrap())
        .map(|card| card.winning_number_count())
        .enumerate()
        .map(|(idx, count)| ((idx + 1)..=(idx + count as usize)).collect_vec())
        .collect_vec();

    fn count_internal<C>(idx: usize, dependencies: &Vec<Vec<usize>>, cache: &mut C) -> u32
    where
        C: Cache<usize, NonZeroU32>,
    {
        if let Some(&value) = cache.get(&idx) {
            return value.get();
        }

        let count = dependencies
            .get(idx)
            .expect("out of bounds")
            .iter()
            .map(|idx| count_internal(*idx, dependencies, cache))
            .fold(1, |count, el| count + el);
        cache.insert(idx, NonZeroU32::new(count).unwrap());

        count
    }

    let mut cache = vec![None; dependencies.len()];
    (0..dependencies.len())
        .map(|idx| count_internal(idx, &dependencies, &mut cache))
        .sum()
}

struct Card {
    winning: Vec<u32>,
    numbers: Vec<u32>,
}

impl Card {
    pub fn winning_number_count(&self) -> u32 {
        self.numbers
            .iter()
            .filter(|n| self.winning.contains(n))
            .count() as u32
    }

    pub fn score(&self) -> u32 {
        let count = self.winning_number_count();

        if count > 0 {
            2u32.pow(count - 1)
        } else {
            0
        }
    }
}

fn parse(input: &str) -> Result<Card, nom::error::Error<&str>> {
    fn number_parser(input: &str) -> IResult<&str, u32> {
        preceded(space1, complete::u32)(input)
    }

    fn numbers_parser(input: &str) -> IResult<&str, Vec<u32>> {
        many1(number_parser)(input)
    }

    let id_parser = preceded(tag("Card"), number_parser);
    let card_parser = preceded(
        tuple((id_parser, tag(":"))),
        separated_pair(numbers_parser, tag(" |"), numbers_parser),
    )
    .map(|(winning, numbers)| Card { winning, numbers });

    final_parser(card_parser)(input)
}

#[cfg(test)]
mod tests {
    use aoc_util::test::setup_tracing;
    use rstest::rstest;

    use super::*;

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn test_part1() {
        setup_tracing();
        let solution = solve_part1(TEST_INPUT);
        assert_eq!(solution, 13);
    }

    #[rstest]
    #[case("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53", 8)]
    #[case("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19", 2)]
    #[case("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1", 2)]
    #[case("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83", 1)]
    #[case("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36", 0)]
    #[case("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11", 0)]
    fn score_part1(#[case] card: &str, #[case] expected: u32) {
        setup_tracing();
        let card = parse(card).unwrap();
        let score = card.score();
        assert_eq!(score, expected);
    }

    #[test]
    fn test_part2() {
        setup_tracing();
        let solution = solve_part2(TEST_INPUT);
        assert_eq!(solution, 30);
    }
}
