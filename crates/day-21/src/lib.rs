use aoc_util::grid::{Direction, Grid, Position, TileChar};

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input, 64)
}

fn solve_part1(input: &str, steps: usize) -> u64 {
    let (map, start) = parse(input);

    let even_steps = steps % 2 == 0;
    let start_remainder = (start.x + start.y) % 2;

    let mut counter = 0;
    aoc_util::graph::breadth_first_search(
        |pos, _| {
            let pos = *pos;
            Direction::ALL
                .into_iter()
                .map(move |direction| pos + direction)
                .filter(|neighbor| matches!(map.get(neighbor), Some(Tile::Garden)))
        },
        |pos, depth| {
            if even_steps == ((pos.x + pos.y) % 2 == start_remainder) {
                counter += 1;
            }

            if depth > steps {
                Some(())
            } else {
                None
            }
        },
        [start],
    );

    counter
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input, 26501365)
}

fn solve_part2(input: &str, _steps: usize) -> u64 {
    let (_map, _start) = parse(input);

    0
}

type Map = Grid<Tile>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Garden,
    Rock,
}

impl TileChar for Tile {
    fn to_char(&self) -> char {
        match self {
            Self::Garden => '.',
            Self::Rock => '#',
        }
    }
}

fn parse(input: &str) -> (Map, Position) {
    let mut width = 0;
    let mut height = 0;
    let mut tiles = Vec::new();
    let mut start = None;

    for (y, line) in input.lines().enumerate() {
        if y == 0 {
            width = line.len();
        }
        height += 1;

        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {
                    tiles.push(Tile::Garden);
                }
                '#' => {
                    tiles.push(Tile::Rock);
                }
                'S' => {
                    tiles.push(Tile::Garden);
                    start = Some(Position::new(x as i64, y as i64));
                }
                _ => unreachable!(),
            }
        }
    }

    (
        Map::new(width, height, tiles),
        start.expect("No start position"),
    )
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const TEST_INPUT: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

    #[rstest]
    #[case(TEST_INPUT, 6, 16)]
    fn test_part1(#[case] input: &str, #[case] steps: usize, #[case] expected: u64) {
        let solution = solve_part1(input, steps);
        assert_eq!(solution, expected);
    }

    #[rstest]
    #[case(TEST_INPUT, 6, 16)]
    #[case(TEST_INPUT, 10, 50)]
    #[case(TEST_INPUT, 50, 1594)]
    #[case(TEST_INPUT, 100, 6536)]
    #[case(TEST_INPUT, 500, 167004)]
    #[case(TEST_INPUT, 1000, 668697)]
    #[case(TEST_INPUT, 5000, 16733044)]
    fn test_part2(#[case] input: &str, #[case] steps: usize, #[case] expected: u64) {
        let solution = solve_part2(input, steps);
        assert_eq!(solution, expected);
    }
}
