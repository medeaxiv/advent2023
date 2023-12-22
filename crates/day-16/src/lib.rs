use std::collections::HashSet;

use aoc_util::grid::*;
use rayon::prelude::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> usize {
    let contraption = parse(input);
    count_energized_tiles_from(Position::zeros(), Direction::Right, &contraption)
}

fn count_energized_tiles_from(
    position: Position,
    direction: Direction,
    contraption: &Contraption,
) -> usize {
    let mut energy = vec![false; contraption.len()];

    let mut beam = Beam {
        position,
        direction,
    };

    let mut visited = HashSet::new();
    let mut stack = Vec::new();

    loop {
        let stop = beam.advance_to_splitter(contraption, |b| {
            if let Some(idx) = contraption.index(&b.position) {
                energy[idx] = true;
            }
        });

        let position = beam.position;

        if let BeamStop::Splitter(directions) = stop {
            if visited.insert(position) {
                beam.direction = directions[0];
                stack.push((position, directions[1]));
                continue;
            }
        }

        if let Some((position, direction)) = stack.pop() {
            beam.position = position;
            beam.direction = direction;
            continue;
        } else {
            break;
        }
    }

    energy.iter().filter(|energy| **energy).count()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> usize {
    let contraption = parse(input);

    let top = (0..contraption.width()).map(|x| (Position::new(x, 0), Direction::Down));
    let bottom = (0..contraption.width())
        .map(|x| (Position::new(x, contraption.height() - 1), Direction::Up));
    let left = (0..contraption.height()).map(|y| (Position::new(0, y), Direction::Right));
    let right = (0..contraption.height())
        .map(|y| (Position::new(contraption.width() - 1, y), Direction::Left));

    top.chain(bottom)
        .chain(left)
        .chain(right)
        .par_bridge()
        .map(|(position, direction)| count_energized_tiles_from(position, direction, &contraption))
        .max()
        .unwrap()
}

#[derive(Debug)]
struct Beam {
    position: Position,
    direction: Direction,
}

impl Beam {
    fn advance(
        &mut self,
        contraption: &Contraption,
        mut visitor: impl FnMut(&Self),
    ) -> Result<(), BeamStop> {
        let tile = contraption
            .get(&self.position)
            .ok_or(BeamStop::OutOfBounds)?;

        visitor(self);

        match tile {
            Tile::Empty => {
                self.position += self.direction;
                Ok(())
            }
            Tile::Mirror(mirror) => {
                self.direction = mirror.reflect(self.direction);
                self.position += self.direction;
                Ok(())
            }
            Tile::Splitter(splitter) => {
                if let Some(directions) = splitter.split(self.direction) {
                    Err(BeamStop::Splitter(directions))
                } else {
                    self.position += self.direction;
                    Ok(())
                }
            }
        }
    }

    pub fn advance_to_splitter(
        &mut self,
        contraption: &Contraption,
        mut visitor: impl FnMut(&Self),
    ) -> BeamStop {
        loop {
            match self.advance(contraption, &mut visitor) {
                Ok(_) => {}
                Err(stop) => {
                    return stop;
                }
            }
        }
    }
}

enum BeamStop {
    OutOfBounds,
    Splitter([Direction; 2]),
}

type Contraption = Grid<Tile>;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Splitter(Splitter),
    Mirror(Mirror),
}

impl TileChar for Tile {
    fn to_char(&self) -> char {
        match self {
            Self::Empty => '.',
            Self::Splitter(Splitter::Horizontal) => '-',
            Self::Splitter(Splitter::Vertical) => '|',
            Self::Mirror(Mirror::Down) => '\\',
            Self::Mirror(Mirror::Up) => '/',
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Splitter {
    Vertical,
    Horizontal,
}

impl Splitter {
    pub fn split(&self, direction: Direction) -> Option<[Direction; 2]> {
        match (*self, direction) {
            (Self::Vertical, Direction::Up) => None,
            (Self::Horizontal, Direction::Up) => Some([Direction::Left, Direction::Right]),
            (Self::Vertical, Direction::Down) => None,
            (Self::Horizontal, Direction::Down) => Some([Direction::Left, Direction::Right]),
            (Self::Vertical, Direction::Left) => Some([Direction::Up, Direction::Down]),
            (Self::Horizontal, Direction::Left) => None,
            (Self::Vertical, Direction::Right) => Some([Direction::Up, Direction::Down]),
            (Self::Horizontal, Direction::Right) => None,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mirror {
    Down,
    Up,
}

impl Mirror {
    pub fn reflect(&self, direction: Direction) -> Direction {
        match (*self, direction) {
            (Self::Down, Direction::Up) => Direction::Left,
            (Self::Up, Direction::Up) => Direction::Right,
            (Self::Down, Direction::Down) => Direction::Right,
            (Self::Up, Direction::Down) => Direction::Left,
            (Self::Down, Direction::Left) => Direction::Up,
            (Self::Up, Direction::Left) => Direction::Down,
            (Self::Down, Direction::Right) => Direction::Down,
            (Self::Up, Direction::Right) => Direction::Up,
        }
    }
}

fn parse(input: &str) -> Contraption {
    let mut width = 0;
    let mut height = 0;
    let mut tiles = Vec::new();

    for (y, line) in input.lines().enumerate() {
        if y == 0 {
            width = line.len();
        }
        height += 1;

        for tile in line.chars().map(|c| match c {
            '-' => Tile::Splitter(Splitter::Horizontal),
            '|' => Tile::Splitter(Splitter::Vertical),
            '\\' => Tile::Mirror(Mirror::Down),
            '/' => Tile::Mirror(Mirror::Up),
            _ => Tile::Empty,
        }) {
            tiles.push(tile);
        }
    }

    Contraption::new(width, height, tiles)
}

#[cfg(test)]
mod tests {
    use aoc_util::test::setup_tracing;

    use super::*;

    const TEST_INPUT: &str = r".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
";

    #[test]
    fn test_part1() {
        setup_tracing();
        let solution = solve_part1(TEST_INPUT);
        assert_eq!(solution, 46);
    }

    #[test]
    fn test_part2() {
        setup_tracing();
        let solution = solve_part2(TEST_INPUT);
        assert_eq!(solution, 51);
    }
}
