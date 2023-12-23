use std::collections::VecDeque;

use ahash::{AHashMap as HashMap, AHashSet as HashSet};
use itertools::Itertools;
use nalgebra::{vector, Vector3};
use rayon::prelude::*;

mod parser;

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> u64 {
    let mut bricks = parser::parse(input).unwrap();
    let support_structure = collapse_to_support_structure(bricks.as_mut_slice());

    (0..bricks.len())
        .filter(|&brick_idx| {
            let support = &support_structure[brick_idx];
            for &supported_idx in support.supports.iter() {
                if support_structure[supported_idx].supported_by.len() == 1 {
                    return false;
                }
            }

            true
        })
        .count() as u64
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> u64 {
    let mut bricks = parser::parse(input).unwrap();
    let support_structure = collapse_to_support_structure(bricks.as_mut_slice());

    (0..bricks.len())
        .filter(|&brick_idx| {
            let support = &support_structure[brick_idx];
            for &supported_idx in support.supports.iter() {
                if support_structure[supported_idx].supported_by.len() == 1 {
                    return true;
                }
            }

            false
        })
        .par_bridge()
        .map(|brick_idx| count_supported_bricks(brick_idx, &support_structure))
        .sum()
}

fn collapse_to_support_structure(bricks: &mut [Brick]) -> Vec<Support> {
    bricks.sort_unstable_by_key(|brick| brick.a.z);

    let mut support_structure = vec![Support::default(); bricks.len()];
    let mut stack = HashMap::new();

    for (brick_idx, brick) in bricks.iter_mut().enumerate() {
        loop {
            if brick.a.z <= 1 {
                stack.extend(brick.positions().map(|position| (position, brick_idx)));
                break;
            }

            let mut is_supported = false;
            for support_idx in brick
                .positions()
                .map(|position| position - vector![0, 0, 1])
                .filter_map(|dropped| stack.get(&dropped).copied())
                .unique()
            {
                is_supported = true;
                support_structure[brick_idx].supported_by.push(support_idx);
                support_structure[support_idx].supports.push(brick_idx);
            }

            if is_supported {
                stack.extend(brick.positions().map(|position| (position, brick_idx)));
                break;
            } else {
                brick.a.z -= 1;
                brick.b.z -= 1;
            }
        }
    }

    support_structure
}

fn count_supported_bricks(start: usize, support_structure: &[Support]) -> u64 {
    let mut visited = HashSet::from([start]);
    let mut queue = VecDeque::from([start]);

    while let Some(brick_idx) = queue.pop_front() {
        let support = &support_structure[brick_idx];

        for &supported_idx in support.supports.iter() {
            if support_structure[supported_idx]
                .supported_by
                .iter()
                .all(|support_idx| visited.contains(support_idx))
            {
                visited.insert(supported_idx);
                queue.push_back(supported_idx);
            }
        }
    }

    visited.len() as u64 - 1
}

type Position = Vector3<i64>;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Brick {
    a: Position,
    b: Position,
    orientation: Orientation,
}

impl Brick {
    pub fn new(a: Position, b: Position) -> Self {
        assert!(a.x <= b.x);
        assert!(a.y <= b.y);
        assert!(a.z <= b.z);

        let orientation = if a.x != b.x {
            Orientation::X
        } else if a.y != b.y {
            Orientation::Y
        } else {
            Orientation::Z
        };

        Self { a, b, orientation }
    }

    pub fn positions(&self) -> impl Iterator<Item = Position> + '_ {
        let a = self.a;
        let index = match self.orientation {
            Orientation::X => 0,
            Orientation::Y => 1,
            Orientation::Z => 2,
        };

        let position = move |coord| {
            let mut position = a;
            position[index] = coord;
            position
        };

        match self.orientation {
            Orientation::X => (self.a.x..=self.b.x).map(position),
            Orientation::Y => (self.a.y..=self.b.y).map(position),
            Orientation::Z => (self.a.z..=self.b.z).map(position),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Orientation {
    X,
    Y,
    Z,
}

#[derive(Default, Debug, Clone)]
struct Support {
    supported_by: Vec<usize>,
    supports: Vec<usize>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";

    #[test]
    fn test_part1() {
        let solution = solve_part1(TEST_INPUT);
        assert_eq!(solution, 5);
    }

    #[test]
    fn test_part2() {
        let solution = solve_part2(TEST_INPUT);
        assert_eq!(solution, 7);
    }
}
