use std::collections::HashSet;

use nalgebra::Vector2;
use rayon::prelude::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> usize {
    let contraption = parse(input);
    count_energized_tiles_from(Pos::zeros(), Direction::Right, &contraption)
}

fn count_energized_tiles_from(
    position: Pos,
    direction: Direction,
    contraption: &Contraption,
) -> usize {
    let mut energy = vec![false; contraption.tiles.len()];

    let mut beam = Beam {
        position,
        direction,
    };

    let mut visited = HashSet::new();
    let mut stack = Vec::new();

    loop {
        let stop = beam.advance_to_splitter(contraption, |b| {
            if let Some(idx) = contraption.index(b.position) {
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

    let top = (0..contraption.width as i32).map(|x| (Pos::new(x, 0), Direction::Down));
    let bottom = (0..contraption.width as i32)
        .map(|x| (Pos::new(x, contraption.height as i32 - 1), Direction::Up));
    let left = (0..contraption.height as i32).map(|y| (Pos::new(0, y), Direction::Right));
    let right = (0..contraption.height as i32)
        .map(|y| (Pos::new(contraption.width as i32 - 1, y), Direction::Left));

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
    position: Pos,
    direction: Direction,
}

impl Beam {
    fn advance(
        &mut self,
        contraption: &Contraption,
        mut visitor: impl FnMut(&Self),
    ) -> Result<(), BeamStop> {
        let tile = contraption
            .get(self.position)
            .ok_or(BeamStop::OutOfBounds)?;

        visitor(self);

        match tile {
            Tile::Empty => {
                self.position = self.direction.next(self.position);
                Ok(())
            }
            Tile::Mirror(mirror) => {
                self.direction = self.direction.reflect(mirror);
                self.position = self.direction.next(self.position);
                Ok(())
            }
            Tile::Splitter(splitter) => {
                if let Some(directions) = self.direction.split(splitter) {
                    Err(BeamStop::Splitter(directions))
                } else {
                    self.position = self.direction.next(self.position);
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

type Pos = Vector2<i32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn reflect(&self, mirror: Mirror) -> Self {
        match (*self, mirror) {
            (Self::Up, Mirror::Down) => Self::Left,
            (Self::Up, Mirror::Up) => Self::Right,
            (Self::Down, Mirror::Down) => Self::Right,
            (Self::Down, Mirror::Up) => Self::Left,
            (Self::Left, Mirror::Down) => Self::Up,
            (Self::Left, Mirror::Up) => Self::Down,
            (Self::Right, Mirror::Down) => Self::Down,
            (Self::Right, Mirror::Up) => Self::Up,
        }
    }

    pub fn split(&self, splitter: Splitter) -> Option<[Self; 2]> {
        match (*self, splitter) {
            (Self::Up, Splitter::Vertical) => None,
            (Self::Up, Splitter::Horizontal) => Some([Self::Left, Self::Right]),
            (Self::Down, Splitter::Vertical) => None,
            (Self::Down, Splitter::Horizontal) => Some([Self::Left, Self::Right]),
            (Self::Left, Splitter::Vertical) => Some([Self::Up, Self::Down]),
            (Self::Left, Splitter::Horizontal) => None,
            (Self::Right, Splitter::Vertical) => Some([Self::Up, Self::Down]),
            (Self::Right, Splitter::Horizontal) => None,
        }
    }

    pub fn next(&self, from: Pos) -> Pos {
        match self {
            Self::Up => from + Pos::new(0, -1),
            Self::Down => from + Pos::new(0, 1),
            Self::Left => from + Pos::new(-1, 0),
            Self::Right => from + Pos::new(1, 0),
        }
    }
}

struct Contraption {
    width: usize,
    height: usize,
    tiles: Vec<Tile>,
}

impl Contraption {
    pub fn index(&self, position: Pos) -> Option<usize> {
        if (0..self.width as i32).contains(&position.x)
            && (0..self.height as i32).contains(&position.y)
        {
            Some(position.y as usize * self.width + position.x as usize)
        } else {
            None
        }
    }

    pub fn get(&self, position: Pos) -> Option<Tile> {
        self.index(position).map(|idx| self.tiles[idx])
    }
}

impl std::fmt::Debug for Contraption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f)?;
        write!(f, "+")?;
        for _ in 0..self.width {
            write!(f, "-")?;
        }
        writeln!(f, "+")?;
        write!(f, "|")?;

        for (idx, tile) in self.tiles.iter().enumerate() {
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
enum Tile {
    Empty,
    Splitter(Splitter),
    Mirror(Mirror),
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Self::Empty => '.',
            Self::Splitter(Splitter::Horizontal) => '-',
            Self::Splitter(Splitter::Vertical) => '|',
            Self::Mirror(Mirror::Down) => '\\',
            Self::Mirror(Mirror::Up) => '/',
        };

        write!(f, "{c}")
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Splitter {
    Vertical,
    Horizontal,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mirror {
    Down,
    Up,
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

    Contraption {
        width,
        height,
        tiles,
    }
}

#[cfg(test)]
mod tests {
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
        let solution = solve_part1(TEST_INPUT);
        assert_eq!(solution, 46);
    }

    #[test]
    fn test_part2() {
        let solution = solve_part2(TEST_INPUT);
        assert_eq!(solution, 51);
    }
}
