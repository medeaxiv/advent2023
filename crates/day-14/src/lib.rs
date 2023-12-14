use std::{cell::RefCell, rc::Rc};

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

    let store = Rc::new(RefCell::new(vec![(
        platform.field.clone(),
        platform.load(),
    )]));
    let next_store = store.clone();
    let ne_store = store.clone();

    let (cycle_length, cycle_offset) = aoc_util::sequence::detect_cycle(
        move |&current| {
            let mut store = next_store.borrow_mut();

            let next = current + 1;
            while store.len() <= next {
                platform.cycle();
                store.push((platform.field.clone(), platform.load()));
            }

            next
        },
        move |&tortoise, &hare| {
            let store = ne_store.borrow();
            store[tortoise] != store[hare]
        },
        0usize,
    );

    let idx = cycle_offset + (1_000_000_000 - cycle_offset) % cycle_length;
    let store_borrow = store.borrow();
    store_borrow[idx].1
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
        let index_north = |column: usize, row: usize| row * self.width + column;
        let index_south = |column: usize, row: usize| (self.height - row - 1) * self.width + column;
        let index_west = |column: usize, row: usize| column * self.width + row;
        let index_east = |column: usize, row: usize| column * self.width + (self.width - row - 1);

        let (index, column_range, row_range) = match direction {
            Direction::North => (
                &index_north as &dyn Fn(usize, usize) -> usize,
                0..self.width,
                0..self.height,
            ),
            Direction::South => (
                &index_south as &dyn Fn(usize, usize) -> usize,
                0..self.width,
                0..self.height,
            ),
            Direction::West => (
                &index_west as &dyn Fn(usize, usize) -> usize,
                0..self.height,
                0..self.width,
            ),
            Direction::East => (
                &index_east as &dyn Fn(usize, usize) -> usize,
                0..self.height,
                0..self.width,
            ),
        };

        for column in column_range {
            let mut drop_row = 0;
            for row in row_range.clone() {
                let idx = index(column, row);
                match self.field[idx] {
                    Tile::Cube => {
                        drop_row = row + 1;
                    }
                    Tile::Round => {
                        if row != drop_row {
                            self.field[index(column, drop_row)] = Tile::Round;
                            self.field[idx] = Tile::Empty;
                        }

                        drop_row += 1;
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn load(&self) -> usize {
        self.field
            .iter()
            .enumerate()
            .filter(|(_, tile)| matches!(tile, Tile::Round))
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
    Cube,
    Round,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Cube => write!(f, "#"),
            Self::Round => write!(f, "O"),
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
                '#' => Tile::Cube,
                'O' => Tile::Round,
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
