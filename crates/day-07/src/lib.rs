use std::cmp::Reverse;

use itertools::Itertools;
use nom::{
    character::complete::{self, one_of, space1},
    combinator::map,
    multi::fill,
    sequence::separated_pair,
    IResult,
};
use nom_supreme::final_parser::final_parser;

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> u64 {
    let mut hands = input.lines().map(|line| parse(line).unwrap()).collect_vec();

    hands.sort_by_cached_key(|(hand, _)| {
        (
            HandType::from_cards(&hand.cards, Ruleset::Standard),
            hand.cards,
        )
    });

    hands
        .iter()
        .enumerate()
        .map(|(idx, (_, bid))| (idx + 1) as u64 * bid)
        .sum()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> u64 {
    let mut hands = input.lines().map(|line| parse(line).unwrap()).collect_vec();

    hands.sort_by_cached_key(|(hand, _)| {
        let mut cards = hand.cards;
        for card in cards.iter_mut() {
            if card.0 == 'J' {
                card.0 = ' ';
            }
        }

        (HandType::from_cards(&hand.cards, Ruleset::Joker), cards)
    });

    hands
        .iter()
        .enumerate()
        .map(|(idx, (_, bid))| (idx + 1) as u64 * bid)
        .sum()
}

struct Hand {
    cards: [Card; 5],
}

impl Hand {
    pub fn new(cards: [Card; 5]) -> Self {
        Self { cards }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 7,
    FourOfAKind = 6,
    FullHouse = 5,
    ThreeOfAKind = 4,
    TwoPair = 3,
    OnePair = 2,
    HighCard = 1,
}

impl HandType {
    pub fn from_cards(cards: &[Card; 5], ruleset: Ruleset) -> Self {
        let mut map = Vec::with_capacity(5);

        for card in cards.iter() {
            if let Some(idx) = map.iter().position(|(c, _)| c == card) {
                map[idx].1 += 1;
            } else {
                map.push((*card, 1));
            }
        }

        map.sort_unstable_by_key(|(_, count)| Reverse(*count));
        if ruleset == Ruleset::Joker && map.len() > 1 {
            if let Some(idx) = map.iter().position(|(c, _)| c == &Card('J')) {
                let jokers = map.remove(idx);
                map[0].1 += jokers.1
            }
        }

        match map.len() {
            1 => Self::FiveOfAKind,
            2 => match map[0].1 {
                4 => Self::FourOfAKind,
                3 => Self::FullHouse,
                n => unreachable!("2 {n}"),
            },
            3 => match map[0].1 {
                3 => Self::ThreeOfAKind,
                2 => Self::TwoPair,
                n => unreachable!("3 {n}"),
            },
            4 => Self::OnePair,
            5 => Self::HighCard,
            n => unreachable!("{n}"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Ruleset {
    Standard,
    Joker,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Card(char);

impl Card {
    pub const fn value(&self) -> u32 {
        match self.0 {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => 11,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            _ => 0,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.value().cmp(&other.value())
    }
}

fn parse(line: &str) -> Result<(Hand, u64), nom::error::Error<&str>> {
    fn card_parser(input: &str) -> IResult<&str, Card> {
        map(one_of("AKQJT98765432"), Card)(input)
    }

    fn hand_parser(input: &str) -> IResult<&str, Hand> {
        let mut buf = [Card(' '); 5];
        let (rest, ()) = fill(card_parser, &mut buf)(input)?;
        Ok((rest, Hand::new(buf)))
    }

    let parser = separated_pair(hand_parser, space1, complete::u64);

    final_parser::<&str, (Hand, u64), nom::error::Error<&str>, nom::error::Error<&str>>(parser)(
        line,
    )
}

#[cfg(test)]
mod tests {
    use aoc_util::test::setup_tracing;
    use rstest::rstest;

    use super::*;

    const TEST_INPUT: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn test_part1() {
        setup_tracing();
        let solution = solve_part1(TEST_INPUT);
        assert_eq!(solution, 6440);
    }

    #[test]
    fn test_part2() {
        setup_tracing();
        let solution = solve_part2(TEST_INPUT);
        assert_eq!(solution, 5905);
    }

    #[rstest]
    #[case(
        "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41",
        6839
    )]
    fn extras_part2(#[case] input: &str, #[case] expected: u64) {
        setup_tracing();
        let solution = solve_part2(input);
        assert_eq!(solution, expected);
    }
}
