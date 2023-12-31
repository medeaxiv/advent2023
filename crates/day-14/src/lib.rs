use std::{collections::BTreeMap, ops::Range};

use itertools::Itertools;

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> usize {
    let mut platform = parse(input);
    platform.tilt(Direction::North);
    platform.load()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> usize {
    let mut platform = parse(input);

    let mut loads = Vec::new();
    let (cycle_length, cycle_offset) = aoc_util::sequence::detect_cycle_cached(
        || {
            let next = platform.boulders().map(|idx| idx as u16).collect_vec();
            loads.push(platform.load());
            platform.cycle();
            next
        },
        &mut BTreeMap::new(),
    );

    let idx = cycle_offset + (1_000_000_000 - cycle_offset) % cycle_length;
    loads[idx]
}

struct Platform {
    width: usize,
    height: usize,
    field: Vec<Tile>,
}

impl Platform {
    pub fn cycle(&mut self) {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East);
    }

    pub fn tilt(&mut self, direction: Direction) {
        let width = self.width;
        let height = self.height;

        match direction {
            Direction::North => self.apply_tilt(
                |column: usize, row: usize| row * width + column,
                0..width,
                0..height,
            ),
            Direction::South => self.apply_tilt(
                |column: usize, row: usize| (height - row - 1) * width + column,
                0..width,
                0..height,
            ),
            Direction::West => self.apply_tilt(
                |column: usize, row: usize| column * width + row,
                0..height,
                0..width,
            ),
            Direction::East => self.apply_tilt(
                |column: usize, row: usize| column * width + (width - row - 1),
                0..height,
                0..width,
            ),
        };
    }

    fn apply_tilt(
        &mut self,
        index: impl Fn(usize, usize) -> usize,
        column_range: Range<usize>,
        row_range: Range<usize>,
    ) {
        for column in column_range {
            let mut drop_row = 0;
            for row in row_range.clone() {
                let idx = index(column, row);
                match self.field[idx] {
                    Tile::Block => {
                        drop_row = row + 1;
                    }
                    Tile::Boulder => {
                        if row != drop_row {
                            self.field[index(column, drop_row)] = Tile::Boulder;
                            self.field[idx] = Tile::Empty;
                        }

                        drop_row += 1;
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn boulders(&self) -> impl Iterator<Item = usize> + '_ {
        self.field
            .iter()
            .enumerate()
            .filter_map(|(idx, tile)| match tile {
                Tile::Boulder => Some(idx),
                _ => None,
            })
    }

    pub fn load(&self) -> usize {
        self.field
            .iter()
            .enumerate()
            .filter(|(_, tile)| matches!(tile, Tile::Boulder))
            .map(|(idx, _)| self.height - idx / self.width)
            .sum()
    }
}

impl std::fmt::Debug for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        write!(f, "+")?;
        for _ in 0..self.width {
            write!(f, "-")?;
        }
        writeln!(f, "+")?;
        write!(f, "|")?;

        for (idx, tile) in self.field.iter().enumerate() {
            if idx > 0 && idx % self.width == 0 {
                writeln!(f, "|")?;
                write!(f, "|")?;
            }

            write!(f, "{tile:?}")?;
        }

        writeln!(f, "|")?;
        write!(f, "+")?;
        for _ in 0..self.width {
            write!(f, "-")?;
        }
        writeln!(f, "+")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Block,
    Boulder,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Block => write!(f, "#"),
            Self::Boulder => write!(f, "O"),
        }
    }
}

fn parse(input: &str) -> Platform {
    let mut width = 0;
    let mut height = 0;

    let mut field = Vec::with_capacity(input.len());
    for (y, line) in input.lines().enumerate() {
        if y == 0 {
            width = line.len();
        }
        height += 1;

        for c in line.chars() {
            let tile = match c {
                '#' => Tile::Block,
                'O' => Tile::Boulder,
                _ => Tile::Empty,
            };

            field.push(tile);
        }
    }

    assert_eq!(field.len(), width * height);

    Platform {
        width,
        height,
        field,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    #[test]
    fn test_part1() {
        let solution = solve_part1(TEST_INPUT);
        assert_eq!(solution, 136);
    }

    #[test]
    fn test_part2() {
        let solution = solve_part2(TEST_INPUT);
        assert_eq!(solution, 64);
    }
}
