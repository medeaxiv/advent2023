use std::collections::HashSet;

use nalgebra::{vector, Vector2};

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> u32 {
    let sketch = parse(input);

    let mut steps = 0;
    sketch.visit_path(|_| {
        steps += 1;
    });

    steps / 2
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> u32 {
    #[derive(PartialEq, Eq)]
    enum State {
        Out,
        UpEdge,
        DownEdge,
        In,
    }

    impl State {
        fn advance(&mut self, edge_direction: Direction) {
            if edge_direction.contains(Direction::North) {
                match self {
                    Self::Out => {
                        *self = Self::UpEdge;
                    }
                    Self::UpEdge => {
                        *self = Self::Out;
                    }
                    Self::DownEdge => {
                        *self = Self::In;
                    }
                    Self::In => {
                        *self = Self::DownEdge;
                    }
                }
            }

            if edge_direction.contains(Direction::South) {
                match self {
                    Self::Out => {
                        *self = Self::DownEdge;
                    }
                    Self::UpEdge => {
                        *self = Self::In;
                    }
                    Self::DownEdge => {
                        *self = Self::Out;
                    }
                    Self::In => {
                        *self = Self::UpEdge;
                    }
                }
            }
        }
    }

    let sketch = parse(input);
    let mut path = HashSet::new();
    sketch.visit_path(|idx| {
        path.insert(idx);
    });

    let mut counter = 0;
    for y in 0..sketch.height {
        let mut state = State::Out;

        for x in 0..sketch.width {
            let idx = sketch.index_of(Pos::new(x as i64, y as i64));
            let on_edge = path.contains(&idx);

            if on_edge {
                state.advance(sketch.tile(idx));
            } else if state == State::In {
                counter += 1;
            }
        }
    }

    counter
}

type Pos = Vector2<i64>;

struct Sketch {
    width: usize,
    height: usize,
    start: usize,
    tiles: Vec<Direction>,
}

impl Sketch {
    pub fn new(width: usize, height: usize, start: Pos, tiles: Vec<Direction>) -> Self {
        assert_eq!(width * height, tiles.len());
        assert!(start.x >= 0 && (start.x as usize) < width);
        assert!(start.y >= 0 && (start.y as usize) < height);

        Self {
            width,
            height,
            start: start.y as usize * width + start.x as usize,
            tiles,
        }
    }

    pub fn index_of(&self, position: Pos) -> usize {
        assert!(position.x >= 0 && (position.x as usize) < self.width);
        assert!(position.y >= 0 && (position.y as usize) < self.height);
        position.y as usize * self.width + position.x as usize
    }

    pub fn visit_path(&self, mut visit: impl FnMut(usize)) {
        let mut previous = self.start;
        let mut current = self.neighbors(self.start).next().unwrap();

        loop {
            visit(current);

            if current == self.start {
                break;
            }

            let next = self
                .neighbors(current)
                .find(|&next| next != previous)
                .unwrap();
            previous = current;
            current = next;
        }
    }

    fn start_direction(&self) -> Direction {
        let mut direction = Direction::empty();

        let x = self.start % self.width;
        let y = self.start / self.width;

        if y > 0 && self.tiles[self.start - self.width].contains(Direction::South) {
            direction |= Direction::North;
        }

        if y + 1 < self.height && self.tiles[self.start + self.width].contains(Direction::North) {
            direction |= Direction::South;
        }

        if x > 0 && self.tiles[self.start - 1].contains(Direction::East) {
            direction |= Direction::West
        }

        if x + 1 < self.width && self.tiles[self.start + 1].contains(Direction::West) {
            direction |= Direction::East
        }

        direction
    }

    fn tile(&self, idx: usize) -> Direction {
        if idx == self.start {
            self.start_direction()
        } else {
            self.tiles[idx]
        }
    }

    fn neighbors(&self, idx: usize) -> impl Iterator<Item = usize> {
        let mut neighbors = [0; 4];
        let mut count = 0;

        let x = idx % self.width;
        let y = idx / self.width;

        let direction = self.tile(idx);
        if direction.contains(Direction::North) && y > 0 {
            neighbors[count] = idx - self.width;
            count += 1;
        }

        if direction.contains(Direction::South) && y + 1 < self.height {
            neighbors[count] = idx + self.width;
            count += 1;
        }

        if direction.contains(Direction::West) && x > 0 {
            neighbors[count] = idx - 1;
            count += 1;
        }

        if direction.contains(Direction::East) && x + 1 < self.width {
            neighbors[count] = idx + 1;
            count += 1;
        }

        neighbors.into_iter().take(count)
    }
}

bitflags::bitflags! {
    #[derive(Clone, Copy)]
    struct Direction: u8 {
        const North = 1<<0;
        const South = 1<<1;
        const West = 1<<2;
        const East = 1<<3;
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '|' => Self::North | Self::South,
            '-' => Self::West | Self::East,
            'L' => Self::North | Self::East,
            'J' => Self::North | Self::West,
            '7' => Self::South | Self::West,
            'F' => Self::South | Self::East,
            _ => Self::empty(),
        }
    }
}

fn parse(input: &str) -> Sketch {
    let mut start = Pos::zeros();
    let mut width = 0;
    let mut height = 0;
    let mut tiles = Vec::new();

    for (y, line) in input.lines().enumerate() {
        height = height.max(y + 1);
        for (x, tile) in line.chars().enumerate() {
            width = width.max(x + 1);
            tiles.push(Direction::from(tile));

            if tile == 'S' {
                start = vector![x as i64, y as i64];
            }
        }
    }

    Sketch::new(width, height, start, tiles)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const TEST_INPUT1: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";

    const TEST_INPUT2: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

    const TEST_INPUT3: &str = "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
";

    const TEST_INPUT4: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

    const TEST_INPUT5: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    #[rstest]
    #[case(TEST_INPUT1, 4)]
    #[case(TEST_INPUT2, 8)]
    fn test_part1(#[case] input: &str, #[case] expected: u32) {
        let solution = solve_part1(input);
        assert_eq!(solution, expected);
    }

    #[rstest]
    #[case(TEST_INPUT3, 4)]
    #[case(TEST_INPUT4, 8)]
    #[case(TEST_INPUT5, 10)]
    fn test_part2(#[case] input: &str, #[case] expected: u32) {
        let solution = solve_part2(input);
        assert_eq!(solution, expected);
    }
}
