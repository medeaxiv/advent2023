use aoc_util::grid::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> u32 {
    let map = parse(input);
    let goal = Position::new(map.width() - 1, map.height() - 1);

    let result = aoc_util::graph::astar(
        |pos| {
            pos.neighbors(1, 3)
                .filter(|TraversalPosition { position, .. }| map.contains(position))
        },
        |TraversalPosition { position, .. }| if position == goal { Some(()) } else { None },
        |TraversalPosition { position, .. }| aoc_util::geometry::manhattan_distance(position, goal),
        |_, new| map.get(&new.position).copied().unwrap() as usize,
        [
            TraversalPosition {
                position: Position::new(1, 0),
                direction: Direction::Right,
                steps: 1,
            },
            TraversalPosition {
                position: Position::new(0, 1),
                direction: Direction::Down,
                steps: 1,
            },
        ],
    );

    result
        .iter()
        .flat_map(|(path, _)| path)
        .map(|TraversalPosition { position, .. }| map.get(position).copied().unwrap_or(0) as u32)
        .sum()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> u32 {
    let map = parse(input);
    let goal = Position::new(map.width() - 1, map.height() - 1);

    let result = aoc_util::graph::astar(
        |pos| {
            pos.neighbors(4, 10)
                .filter(|TraversalPosition { position, .. }| map.contains(position))
        },
        |TraversalPosition {
             position, steps, ..
         }| {
            if position == goal && steps >= 4 {
                Some(())
            } else {
                None
            }
        },
        |TraversalPosition { position, .. }| aoc_util::geometry::manhattan_distance(position, goal),
        |_, new| map.get(&new.position).copied().unwrap() as usize,
        [
            TraversalPosition {
                position: Position::new(1, 0),
                direction: Direction::Right,
                steps: 1,
            },
            TraversalPosition {
                position: Position::new(0, 1),
                direction: Direction::Down,
                steps: 1,
            },
        ],
    );

    result
        .iter()
        .flat_map(|(path, _)| path)
        .map(|TraversalPosition { position, .. }| map.get(position).copied().unwrap_or(0) as u32)
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TraversalPosition {
    position: Position,
    direction: Direction,
    steps: usize,
}

impl TraversalPosition {
    pub fn neighbors(&self, min_steps: usize, max_steps: usize) -> impl Iterator<Item = Self> {
        let from = *self;
        Direction::ALL.iter().filter_map(move |&direction| {
            if direction == from.direction {
                if from.steps < max_steps {
                    let neighbor = Self {
                        position: from.position + direction,
                        direction,
                        steps: from.steps + 1,
                    };
                    Some(neighbor)
                } else {
                    None
                }
            } else if direction == from.direction.inverse() {
                None
            } else if from.steps >= min_steps {
                let neighbor = Self {
                    position: from.position + direction,
                    direction,
                    steps: 1,
                };
                Some(neighbor)
            } else {
                None
            }
        })
    }
}

type Map = Grid<u8>;

fn parse(input: &str) -> Map {
    let mut width = 0;
    let mut height = 0;
    let mut blocks = Vec::new();
    for (y, line) in input.lines().enumerate() {
        if y == 0 {
            width = line.len();
        }

        height += 1;

        for c in line.chars() {
            blocks.push(c.to_digit(10).unwrap() as u8);
        }
    }

    Map::new(width, height, blocks)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const TEST_INPUT1: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    const TEST_INPUT2: &str = "111111111111
999999999991
999999999991
999999999991
999999999991
";

    #[test]
    fn test_part1() {
        let solution = solve_part1(TEST_INPUT1);
        assert_eq!(solution, 102);
    }

    #[rstest]
    #[case(TEST_INPUT1, 94)]
    #[case(TEST_INPUT2, 71)]
    fn test_part2(#[case] input: &str, #[case] expected: u32) {
        let solution = solve_part2(input);
        assert_eq!(solution, expected);
    }
}
