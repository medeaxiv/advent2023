use std::collections::VecDeque;

use ahash::{AHashMap as HashMap, AHashSet as HashSet};
use aoc_util::{
    cache::Cache,
    grid::{Direction, Grid, Position, TileChar},
};
use itertools::Itertools;

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> u64 {
    let map = parse(input);
    let start = Position::new(1, 0);
    let graph = build_trail_graph1(start, &map);

    fn max_length<C>(
        position: Position,
        graph: &HashMap<Position, Vec<(Position, u64)>>,
        cache: &mut C,
    ) -> u64
    where
        C: Cache<Position, u64>,
    {
        if let Some(&length) = cache.get(&position) {
            return length;
        }

        let length = graph
            .get(&position)
            .iter()
            .flat_map(|edges| edges.iter())
            .map(|edge| edge.1 + max_length(edge.0, graph, cache))
            .max()
            .unwrap_or(0);

        cache.insert(position, length);
        length
    }

    let mut cache = HashMap::new();
    max_length(start, &graph, &mut cache)
}

fn build_trail_graph1(start: Position, map: &Map) -> HashMap<Position, Vec<(Position, u64)>> {
    let mut graph = HashMap::new();

    let mut visited = HashSet::from([start]);
    let mut queue = VecDeque::from([start]);

    while let Some(position) = queue.pop_front() {
        if position.y == map.height() - 1 {
            continue;
        }

        let neighbors = Direction::ALL
            .into_iter()
            .filter(|&d| {
                let neighbor = position + d;
                match map.get(&neighbor) {
                    None | Some(Tile::Forest) => false,
                    Some(Tile::Path) => true,
                    Some(Tile::Slope(slope)) => *slope == d,
                }
            })
            .collect_vec();

        for (neighbor, distance) in neighbors
            .iter()
            .map(|&direction| walk_along_path(position, direction, map))
        {
            let adjacency = graph.entry(position).or_insert_with(Vec::new);
            adjacency.push((neighbor, distance));
            if visited.insert(neighbor) {
                queue.push_back(neighbor);
            }
        }
    }

    graph
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> u64 {
    let map = parse(input);
    let start = Position::new(1, 0);
    let end = Position::new(map.width() - 2, map.height() - 1);
    let graph = build_trail_graph2(start, &map);

    fn max_length(
        position: Position,
        end: Position,
        visited: &mut HashSet<Position>,
        graph: &HashMap<Position, Vec<(Position, u64)>>,
    ) -> Option<u64> {
        if position == end {
            return Some(0);
        }

        visited.insert(position);
        let length = graph
            .get(&position)
            .iter()
            .flat_map(|edges| edges.iter())
            .filter_map(|edge| {
                if visited.contains(&edge.0) {
                    None
                } else {
                    max_length(edge.0, end, visited, graph).map(|l| l + edge.1)
                }
            })
            .max();
        visited.remove(&position);

        length
    }

    let mut visited = HashSet::new();
    max_length(start, end, &mut visited, &graph).unwrap_or(0)
}

fn build_trail_graph2(start: Position, map: &Map) -> HashMap<Position, Vec<(Position, u64)>> {
    let mut graph = HashMap::new();

    let mut visited = HashSet::from([start]);
    let mut queue = VecDeque::from([start]);

    while let Some(position) = queue.pop_front() {
        if position.y == map.height() - 1 {
            continue;
        }

        let neighbors = Direction::ALL
            .into_iter()
            .filter(|&d| {
                let neighbor = position + d;
                !matches!(map.get(&neighbor), None | Some(Tile::Forest))
            })
            .collect_vec();

        for (neighbor, distance) in neighbors
            .iter()
            .map(|&direction| walk_along_path(position, direction, map))
        {
            if neighbor.y == 0 {
                continue;
            }

            let adjacency = graph.entry(position).or_insert_with(Vec::new);
            adjacency.push((neighbor, distance));
            if visited.insert(neighbor) {
                queue.push_back(neighbor);
            }
        }
    }

    graph
}

fn walk_along_path(mut position: Position, mut direction: Direction, map: &Map) -> (Position, u64) {
    let mut steps_taken = 1;
    position += direction;

    loop {
        let mut iter = [direction.turn_left(), direction, direction.turn_right()]
            .into_iter()
            .filter_map(|d| map.get(&(position + d)).map(|t| (d, t)))
            .filter(|(_, t)| !matches!(t, Tile::Forest))
            .map(|(d, _)| d);
        let next = iter.next();
        let overflow = iter.next();

        if next.is_none() || overflow.is_some() {
            break;
        }

        let next = next.unwrap();
        direction = next;
        position += next;
        steps_taken += 1;
    }

    (position, steps_taken)
}

type Map = Grid<Tile>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Path,
    Forest,
    Slope(Direction),
}

impl TileChar for Tile {
    fn to_char(&self) -> char {
        match self {
            Self::Path => '.',
            Self::Forest => '#',
            Self::Slope(direction) => direction.to_char(),
        }
    }
}

fn parse(input: &str) -> Map {
    let mut width = 0;
    let mut height = 0;
    let mut tiles = Vec::new();

    for (y, line) in input.lines().enumerate() {
        if y == 0 {
            width = line.len();
        }
        height += 1;

        for c in line.chars() {
            let tile = match c {
                '.' => Tile::Path,
                '^' => Tile::Slope(Direction::Up),
                'v' => Tile::Slope(Direction::Down),
                '<' => Tile::Slope(Direction::Left),
                '>' => Tile::Slope(Direction::Right),
                _ => Tile::Forest,
            };

            tiles.push(tile);
        }
    }

    Map::new(width, height, tiles)
}

#[allow(dead_code)]
fn print_graph(graph: &HashMap<Position, Vec<(Position, u64)>>) {
    let mut rendered_edges: HashMap<(Position, Position), u32> = HashMap::new();

    for (node, edges) in graph.iter() {
        for edge in edges.iter() {
            if let Some(count) = rendered_edges.get_mut(&(edge.0, *node)) {
                *count += 1;
            } else {
                rendered_edges.insert((*node, edge.0), 1);
            }
        }
    }

    for ((a, b), count) in rendered_edges {
        if count == 1 {
            println!("\"{:?}\" -> \"{:?}\"", a, b);
        } else {
            println!("\"{:?}\" -> \"{:?}\" [dir=both]", a, b);
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_util::test::setup_tracing;

    use super::*;

    const TEST_INPUT: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

    #[test]
    fn test_part1() {
        setup_tracing();
        let solution = solve_part1(TEST_INPUT);
        assert_eq!(solution, 94);
    }

    #[test]
    fn test_part2() {
        let solution = solve_part2(TEST_INPUT);
        assert_eq!(solution, 154);
    }
}
