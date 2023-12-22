use std::ops::Range;

use itertools::Itertools;
use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{self, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated, tuple},
    Parser,
};
use nom_supreme::final_parser::final_parser;

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> u64 {
    let almanac = parse(input).unwrap();

    almanac
        .seeds
        .iter()
        .map(|&seed| almanac.maps.iter().fold(seed, |acc, map| map.map(acc)))
        .min()
        .unwrap()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> u64 {
    let almanac = parse(input).unwrap();

    almanac
        .seeds
        .iter()
        .tuples()
        .map(|(&start, &len)| (start..(start + len)))
        .flat_map(|range| {
            almanac
                .maps
                .iter()
                .fold(vec![range], |acc, map| map.map_ranges(acc.as_slice()))
        })
        .map(|range| range.start)
        .min()
        .unwrap()
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: Vec<AlmanacMap>,
}

#[derive(Debug)]
struct AlmanacMap {
    ranges: Vec<AlmanacRange>,
}

impl AlmanacMap {
    pub fn map(&self, entry: u64) -> u64 {
        self.ranges
            .iter()
            .find_map(|range| range.map(entry))
            .unwrap_or(entry)
    }

    pub fn map_ranges(&self, ranges: &[Range<u64>]) -> Vec<Range<u64>> {
        let mut mapped_ranges = Vec::with_capacity(ranges.len());
        let mut unmapped_ranges = Vec::from_iter(ranges.iter().cloned());
        let mut unmapped_buffer = Vec::new();

        for map in self.ranges.iter() {
            for range in unmapped_ranges.iter() {
                match map.map_range(range.clone()) {
                    RangeMap::Unmapped(range) => {
                        unmapped_buffer.push(range);
                    }
                    RangeMap::Mapped(range) => {
                        mapped_ranges.push(range);
                    }
                    RangeMap::Intersecting { mapped, unmapped } => {
                        mapped_ranges.push(mapped);
                        unmapped_buffer.push(unmapped);
                    }
                    RangeMap::Contained {
                        mapped,
                        unmapped_before,
                        unmapped_after,
                    } => {
                        mapped_ranges.push(mapped);
                        unmapped_buffer.push(unmapped_before);
                        unmapped_buffer.push(unmapped_after);
                    }
                }
            }

            unmapped_ranges.clear();
            std::mem::swap(&mut unmapped_ranges, &mut unmapped_buffer);
        }

        mapped_ranges.extend(unmapped_ranges);
        mapped_ranges
    }
}

#[derive(Debug)]
struct AlmanacRange {
    source: Range<u64>,
    destination: Range<u64>,
}

impl AlmanacRange {
    pub fn new(source: u64, destination: u64, len: u64) -> Self {
        Self {
            source: source..(source + len),
            destination: destination..(destination + len),
        }
    }

    fn unchecked_map(&self, entry: u64) -> u64 {
        let offset = entry - self.source.start;
        self.destination.start + offset
    }

    pub fn map(&self, entry: u64) -> Option<u64> {
        if !self.source.contains(&entry) {
            return None;
        }

        Some(self.unchecked_map(entry))
    }

    pub fn map_range(&self, range: Range<u64>) -> RangeMap {
        if range.end <= self.source.start || range.start >= self.source.end {
            return RangeMap::Unmapped(range);
        }

        let mut unmapped_len = 0;
        let mut unmapped = [0..0, 0..0];

        let start = if range.start < self.source.start {
            unmapped[unmapped_len] = range.start..self.source.start;
            unmapped_len += 1;
            self.destination.start
        } else {
            self.unchecked_map(range.start)
        };

        let end = if range.end > self.source.end {
            unmapped[unmapped_len] = self.source.end..range.end;
            unmapped_len += 1;
            self.destination.end
        } else {
            self.unchecked_map(range.end)
        };

        match unmapped_len {
            0 => RangeMap::Mapped(start..end),
            1 => RangeMap::Intersecting {
                mapped: start..end,
                unmapped: unmapped[0].clone(),
            },
            2 => RangeMap::Contained {
                mapped: start..end,
                unmapped_before: unmapped[0].clone(),
                unmapped_after: unmapped[1].clone(),
            },
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
enum RangeMap {
    Unmapped(Range<u64>),
    Mapped(Range<u64>),
    Intersecting {
        mapped: Range<u64>,
        unmapped: Range<u64>,
    },
    Contained {
        mapped: Range<u64>,
        unmapped_before: Range<u64>,
        unmapped_after: Range<u64>,
    },
}

fn parse(input: &str) -> Result<Almanac, nom::error::Error<&str>> {
    let seeds_parser = preceded(tag("seeds: "), separated_list1(space1, complete::u64));

    let map_header_parser = terminated(take_until(" "), pair(tag(" map:"), line_ending));

    let map_range_parser = tuple((complete::u64, space1, complete::u64, space1, complete::u64))
        .map(|(destination, _, source, _, len)| AlmanacRange::new(source, destination, len));

    let map_parser = preceded(
        map_header_parser,
        separated_list1(line_ending, map_range_parser),
    )
    .map(|ranges| AlmanacMap { ranges });

    let maps_parser = many1(delimited(line_ending, map_parser, line_ending));

    let almanac_parser = separated_pair(seeds_parser, line_ending, maps_parser)
        .map(|(seeds, maps)| Almanac { seeds, maps });

    final_parser::<&str, Almanac, nom::error::Error<&str>, nom::error::Error<&str>>(almanac_parser)(
        input,
    )
}

#[cfg(test)]
mod tests {
    use aoc_util::test::setup_tracing;
    use rstest::rstest;

    use super::*;

    const TEST_INPUT: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn test_parser() {
        setup_tracing();
        let solution = parse(TEST_INPUT);
        assert!(solution.is_ok());
    }

    #[test]
    fn test_part1() {
        setup_tracing();
        let solution = solve_part1(TEST_INPUT);
        assert_eq!(solution, 35);
    }

    #[test]
    fn test_part2() {
        setup_tracing();
        let solution = solve_part2(TEST_INPUT);
        assert_eq!(solution, 46);
    }

    #[rstest]
    #[case(AlmanacRange::new(10, 30, 5), 0..5, RangeMap::Unmapped(0..5))]
    #[case(AlmanacRange::new(10, 30, 5), 5..10, RangeMap::Unmapped(5..10))]
    #[case(AlmanacRange::new(10, 30, 5), 20..25, RangeMap::Unmapped(20..25))]
    #[case(AlmanacRange::new(10, 30, 5), 15..20, RangeMap::Unmapped(15..20))]
    #[case(AlmanacRange::new(10, 30, 5), 10..15, RangeMap::Mapped(30..35))]
    #[case(AlmanacRange::new(10, 30, 5), 10..14, RangeMap::Mapped(30..34))]
    #[case(AlmanacRange::new(10, 30, 5), 11..15, RangeMap::Mapped(31..35))]
    #[case(AlmanacRange::new(10, 30, 5), 5..15, RangeMap::Intersecting { mapped: 30..35, unmapped: 5..10 })]
    #[case(AlmanacRange::new(10, 30, 5), 5..14, RangeMap::Intersecting { mapped: 30..34, unmapped: 5..10 })]
    #[case(AlmanacRange::new(10, 30, 5), 10..20, RangeMap::Intersecting { mapped: 30..35, unmapped: 15..20 })]
    #[case(AlmanacRange::new(10, 30, 5), 11..20, RangeMap::Intersecting { mapped: 31..35, unmapped: 15..20 })]
    #[case(AlmanacRange::new(10, 30, 5), 5..20, RangeMap::Contained{ mapped: 30..35, unmapped_before: 5..10, unmapped_after: 15..20 })]
    fn test_map_range(
        #[case] mapper: AlmanacRange,
        #[case] range: Range<u64>,
        #[case] expected: RangeMap,
    ) {
        setup_tracing();
        let result = mapper.map_range(range);
        assert_eq!(result, expected)
    }

    #[rstest]
    #[case(AlmanacMap { ranges: vec![] }, vec![], vec![])]
    #[case(AlmanacMap { ranges: vec![AlmanacRange::new(10, 30, 5)] }, vec![0..5, 20..25], vec![0..5, 20..25])]
    #[case(AlmanacMap { ranges: vec![AlmanacRange::new(10, 30, 5)] }, vec![5..15], vec![30..35, 5..10])]
    #[case(AlmanacMap { ranges: vec![AlmanacRange::new(10, 30, 5)] }, vec![5..20], vec![30..35, 5..10, 15..20])]
    #[case(AlmanacMap { ranges: vec![AlmanacRange::new(10, 30, 5)] }, vec![5..20], vec![30..35, 5..10, 15..20])]
    #[case(AlmanacMap { ranges: vec![AlmanacRange::new(10, 30, 5), AlmanacRange::new(20, 40, 5)] }, vec![5..30], vec![30..35, 40..45, 5..10, 15..20, 25..30])]
    fn test_map_ranges(
        #[case] map: AlmanacMap,
        #[case] ranges: Vec<Range<u64>>,
        #[case] expected: Vec<Range<u64>>,
    ) {
        setup_tracing();
        let result = map.map_ranges(ranges.as_slice());
        assert_eq!(result, expected);
    }
}
