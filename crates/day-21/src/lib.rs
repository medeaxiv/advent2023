use aoc_util::grid::{Direction, Grid, Position, TileChar};

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input, 64)
}

fn solve_part1(input: &str, steps: usize) -> u64 {
    let (map, start) = parse(input);

    let mut even_counter = 0;
    let mut odd_counter = 0;
    aoc_util::graph::breadth_first_search(
        |pos, _| {
            let pos = *pos;
            Direction::ALL
                .into_iter()
                .map(move |direction| pos + direction)
                .filter(|neighbor| matches!(map.get(neighbor), Some(Tile::Garden)))
        },
        |_, depth| {
            if depth > steps {
                return Some(());
            }

            if depth % 2 == 0 {
                even_counter += 1;
            } else {
                odd_counter += 1;
            }

            None
        },
        [start],
    );

    if steps % 2 == 0 {
        even_counter
    } else {
        odd_counter
    }
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input, 26501365)
}

fn solve_part2(input: &str, steps: usize) -> u64 {
    let (map, start) = parse(input);

    assert_eq!(map.width(), map.height(), "Map must be squared");
    let size = map.width();

    assert!(size % 2 == 1, "Map must have an odd size");

    assert_eq!(start.x, size / 2, "Start must be centered in the map");
    assert_eq!(start.y, size / 2, "Start must be centered in the map");
    assert_eq!(steps as i64 % size, size / 2);

    for i in 0..size {
        assert_eq!(
            map[(start.y * size + i) as usize],
            Tile::Garden,
            "Map must be clear along the start row"
        );

        assert_eq!(
            map[(start.x + i * size) as usize],
            Tile::Garden,
            "Map must be clear along the start column"
        );
    }

    let grid_size = steps as i64 / size - 1;

    let mut even_distance_positions = Vec::with_capacity(map.len() / 2 + 1);
    let mut odd_distance_positions = Vec::with_capacity(map.len() / 2 + 1);
    aoc_util::graph::breadth_first_search(
        |pos, _| {
            let pos = *pos;
            Direction::ALL
                .into_iter()
                .map(move |direction| pos + direction)
                .filter(|neighbor| matches!(map.get(neighbor), Some(Tile::Garden)))
        },
        |pos, depth| {
            if depth % 2 == 0 {
                even_distance_positions.push(*pos);
            } else {
                odd_distance_positions.push(*pos);
            }

            None as Option<()>
        },
        [start],
    );

    let (even_grid_positions, odd_grid_positions) = if steps % 2 == 0 {
        (even_distance_positions, odd_distance_positions)
    } else {
        (odd_distance_positions, even_distance_positions)
    };

    let (even_grid, odd_grid) = (
        even_grid_positions.len() as u64,
        odd_grid_positions.len() as u64,
    );
    let (corners, small_edges, large_edges) = if grid_size % 2 == 0 {
        (
            count_corners(start, size, &odd_grid_positions),
            count_small_edges(start, size, &even_grid_positions),
            count_large_edges(start, size, &odd_grid_positions),
        )
    } else {
        (
            count_corners(start, size, &even_grid_positions),
            count_small_edges(start, size, &odd_grid_positions),
            count_large_edges(start, size, &even_grid_positions),
        )
    };

    let even_grid_count = {
        let tmp = (grid_size as u64 / 2) * 2 + 1;
        tmp * tmp
    };

    let odd_grid_count = {
        let tmp = ((grid_size as u64 + 1) / 2) * 2;
        tmp * tmp
    };

    let small_edge_count = grid_size as u64 + 1;
    let large_edge_count = grid_size as u64;

    (even_grid_count * even_grid)
        + (odd_grid_count * odd_grid)
        + corners
        + (small_edge_count * small_edges)
        + (large_edge_count * large_edges)
}

fn count_from(start: Position, steps: i64, candidates: &[Position]) -> u64 {
    candidates
        .iter()
        .map(|&p| aoc_util::geometry::manhattan_distance(p, start))
        .filter(|&d| d <= steps)
        .count() as u64
}

fn count_corners(start: Position, size: i64, candidates: &[Position]) -> u64 {
    let starts = [
        Position::new(start.x, size - 1),
        Position::new(start.x, 0),
        Position::new(size - 1, start.y),
        Position::new(0, start.y),
    ];

    starts
        .into_iter()
        .map(|pos| count_from(pos, size - 1, candidates))
        .sum()
}

fn count_small_edges(_start: Position, size: i64, candidates: &[Position]) -> u64 {
    let starts = [
        Position::new(0, 0),
        Position::new(0, size - 1),
        Position::new(size - 1, 0),
        Position::new(size - 1, size - 1),
    ];

    starts
        .into_iter()
        .map(|pos| count_from(pos, size / 2 - 1, candidates))
        .sum()
}

fn count_large_edges(_start: Position, size: i64, candidates: &[Position]) -> u64 {
    let starts = [
        Position::new(0, 0),
        Position::new(0, size - 1),
        Position::new(size - 1, 0),
        Position::new(size - 1, size - 1),
    ];

    starts
        .into_iter()
        .map(|pos| count_from(pos, 3 * size / 2 - 1, candidates))
        .sum()
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

    const TEST_INPUT1: &str = "...........
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
    #[case(TEST_INPUT1, 6, 16)]
    fn test_part1(#[case] input: &str, #[case] steps: usize, #[case] expected: u64) {
        let solution = solve_part1(input, steps);
        assert_eq!(solution, expected);
    }

    const TEST_INPUT2: &str = "...
.S.
...";

    #[rstest]
    #[case(TEST_INPUT2, 4, 25)]
    #[case(TEST_INPUT2, 7, 64)]
    #[case(TEST_INPUT2, 10, 121)]
    #[case(TEST_INPUT2, 13, 196)]
    fn test_part2(#[case] input: &str, #[case] steps: usize, #[case] expected: u64) {
        let solution = solve_part2(input, steps);
        assert_eq!(solution, expected);
    }
}
