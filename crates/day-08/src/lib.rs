use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending, one_of},
    multi::many1,
    sequence::{delimited, pair, separated_pair, terminated},
    Parser,
};
use nom_supreme::final_parser::final_parser;

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> u32 {
    let (path, network) = parse(input).unwrap();

    let mut pos = network
        .index
        .iter()
        .position(|&name| name == "AAA")
        .unwrap();
    let mut steps = 0;
    for (idx, direction) in path.iter().cycle().enumerate() {
        pos = network.adjacency.get(pos).unwrap().get(*direction);
        if network.index[pos] == "ZZZ" {
            steps = idx + 1;
            break;
        }
    }

    steps as u32
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> u64 {
    let (path, network) = parse(input).unwrap();

    network
        .index
        .iter()
        .enumerate()
        .filter_map(
            |(idx, name)| {
                if name.ends_with('A') {
                    Some(idx)
                } else {
                    None
                }
            },
        )
        .map(|start| {
            let mut pos = start;
            let mut steps = 0;
            for (idx, direction) in path.iter().cycle().enumerate() {
                pos = network.adjacency.get(pos).unwrap().get(*direction);
                if network.index[pos].ends_with('Z') {
                    steps = idx + 1;
                    break;
                }
            }

            steps as u64
        })
        .reduce(aoc_util::numerics::least_common_multiple)
        .unwrap()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Network<T>
where
    T: Copy,
{
    index: Vec<T>,
    adjacency: Vec<Node<usize>>,
}

impl<T> FromIterator<(T, Node<T>)> for Network<T>
where
    T: Copy + std::hash::Hash + Eq,
{
    fn from_iter<I: IntoIterator<Item = (T, Node<T>)>>(iter: I) -> Self {
        let mut map = HashMap::new();
        let mut index = Vec::new();
        let mut nodes = Vec::new();
        for (idx, (name, node)) in iter.into_iter().enumerate() {
            map.insert(name, idx);
            index.push(name);
            nodes.push(node);
        }

        let adjacency = nodes
            .iter()
            .map(|node| Node {
                left: *map.get(&node.left).unwrap(), //index.iter().position(|&name| name == node.left).unwrap(),
                right: *map.get(&node.right).unwrap(), //index.iter().position(|&name| name == node.right).unwrap(),
            })
            .collect_vec();

        Self { index, adjacency }
    }
}

#[derive(Debug)]
struct Node<T>
where
    T: Copy,
{
    left: T,
    right: T,
}

impl<T> Node<T>
where
    T: Copy,
{
    pub fn get(&self, direction: Direction) -> T {
        match direction {
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }
}

fn parse(input: &str) -> Result<(Vec<Direction>, Network<&str>), nom::error::Error<&str>> {
    let direction_parser = one_of("LR").map(|c| match c {
        'L' => Direction::Left,
        'R' => Direction::Right,
        _ => unreachable!(),
    });

    let path_parser = many1(direction_parser);

    let node_parser = separated_pair(
        alphanumeric1,
        tag(" = "),
        delimited(
            tag("("),
            separated_pair(alphanumeric1, tag(", "), alphanumeric1),
            tag(")"),
        ),
    )
    .map(|(name, (left, right))| (name, Node { left, right }));

    let network_parser = many1(terminated(node_parser, line_ending)).map(Network::from_iter);

    let parser = separated_pair(path_parser, pair(line_ending, line_ending), network_parser);

    final_parser::<
        &str,
        (Vec<Direction>, Network<&str>),
        nom::error::Error<&str>,
        nom::error::Error<&str>,
    >(parser)(input)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const TEST_INPUT1: &str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)
";

    const TEST_INPUT2: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)
";

    const TEST_INPUT3: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)
";

    #[rstest]
    #[case(TEST_INPUT1, 2)]
    #[case(TEST_INPUT2, 6)]
    fn test_part1(#[case] input: &str, #[case] expected: u32) {
        let solution = solve_part1(input);
        assert_eq!(solution, expected);
    }

    #[rstest]
    #[case(TEST_INPUT3, 6)]
    fn test_part2(#[case] input: &str, #[case] expected: u64) {
        let solution = solve_part2(input);
        assert_eq!(solution, expected);
    }
}
