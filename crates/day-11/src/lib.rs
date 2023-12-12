use std::collections::HashSet;

use aoc_util::geometry::manhattan_distance;
use itertools::Itertools;
use nalgebra::Vector2;

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> u64 {
    let mut map = parse(input);
    map.expand(2);
    map.galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| manhattan_distance(*a, *b) as u64)
        .sum()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input, 1_000_000)
}

fn solve_part2(input: &str, expansion: i64) -> u64 {
    let mut map = parse(input);
    map.expand(expansion);
    map.galaxies
        .iter()
        .tuple_combinations()
        .map(|(a, b)| manhattan_distance(*a, *b) as u64)
        .sum()
}

type Pos = Vector2<i64>;

#[derive(Debug)]
struct Map {
    galaxies: Vec<Pos>,
    empty_rows: Vec<i64>,
    empty_columns: Vec<i64>,
}

impl Map {
    pub fn expand(&mut self, mut expansion: i64) {
        expansion = (expansion - 1).max(0);

        for galaxy in self.galaxies.iter_mut() {
            let x_offset = self.empty_columns.partition_point(|&x| x < galaxy.x) as i64;
            let y_offset = self.empty_rows.partition_point(|&y| y < galaxy.y) as i64;
            galaxy.x += x_offset * expansion;
            galaxy.y += y_offset * expansion;
        }
    }
}

fn parse(input: &str) -> Map {
    let mut galaxies = vec![];
    let mut empty_rows = Vec::new();
    let mut columns = HashSet::new();
    let mut max_column = 0;

    for (y, line) in input.lines().enumerate() {
        let y = y as i64;
        let mut empty = true;
        for (x, _) in line.chars().enumerate().filter(|&(_, c)| c == '#') {
            let x = x as i64;
            max_column = max_column.max(x);
            empty = false;

            galaxies.push(Pos::new(x, y));
            columns.insert(x);
        }

        if empty {
            empty_rows.push(y);
        }
    }

    let empty_columns = (0..=max_column)
        .filter(|col| !columns.contains(col))
        .collect_vec();

    Map {
        galaxies,
        empty_rows,
        empty_columns,
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const TEST_INPUT: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn test_part1() {
        let solution = solve_part1(TEST_INPUT);
        assert_eq!(solution, 374);
    }

    #[rstest]
    #[case(10, 1030)]
    #[case(100, 8410)]
    fn test_part2(#[case] expansion: i64, #[case] expected: u64) {
        let solution = solve_part2(TEST_INPUT, expansion);
        assert_eq!(solution, expected);
    }
}
