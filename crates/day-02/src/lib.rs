use std::collections::HashMap;

use itertools::Itertools;
use rayon::{iter::ParallelIterator, str::ParallelString};

const INPUT: &str = include_str!("input.txt");

pub fn part1() -> impl std::fmt::Display {
    solve_part1(INPUT)
}

fn parse(line: &str) -> Game {
    let (prefix, line) = line.split_once(": ").unwrap();
    let id = prefix[5..].parse::<u32>().unwrap();
    let sets = line
        .split("; ")
        .map(|part| {
            part.split(", ")
                .map(|entry| {
                    let (count, color) = entry.split_once(' ').unwrap();
                    let count = count.parse::<u32>().unwrap();
                    (color.to_string(), count)
                })
                .collect_vec()
        })
        .collect_vec();
    Game { id, sets }
}

fn solve_part1(input: &str) -> u32 {
    let maximums: HashMap<&str, u32> = HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    input
        .par_lines()
        .map(parse)
        .filter(|game| {
            for (color, count) in game.sets.iter().flat_map(|set| set.iter()) {
                if *count > maximums.get(color.as_str()).copied().unwrap_or(0) {
                    return false;
                }
            }

            true
        })
        .map(|game| game.id)
        .sum()
}

pub fn part2() -> impl std::fmt::Display {
    solve_part2(INPUT)
}

fn solve_part2(input: &str) -> u32 {
    input
        .par_lines()
        .map(parse)
        .map(|game| {
            let mut maximums: HashMap<&str, u32> = HashMap::new();

            for (color, count) in game.sets.iter().flat_map(|set| set.iter()) {
                maximums
                    .entry(color.as_str())
                    .and_modify(|val| *val = u32::max(*val, *count))
                    .or_insert(*count);
            }

            maximums.values().product::<u32>()
        })
        .sum()
}

struct Game {
    id: u32,
    sets: Vec<Vec<(String, u32)>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn test_part1() {
        let solution = solve_part1(TEST_INPUT);
        assert_eq!(solution, 8);
    }

    #[test]
    fn test_part2() {
        let solution = solve_part2(TEST_INPUT);
        assert_eq!(solution, 2286);
    }
}
