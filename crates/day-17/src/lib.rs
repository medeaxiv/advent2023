use aoc_util::grid::*;

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> u32 {
    let map = parse(input);
    let goal = Position::new(map.width() - 1, map.height() - 1);

    let result = aoc_util::graph::search::astar(
        |pos| pos.neighbors(1, 3, &map),
        |pos| if pos.position == goal { Some(()) } else { None },
        |pos| aoc_util::geometry::manhattan_distance(pos.position, goal) as usize,
        [
            TraversalPosition {
                position: Position::zeros(),
                direction: Direction::Right,
                cost: 0,
            },
            TraversalPosition {
                position: Position::zeros(),
                direction: Direction::Down,
                cost: 0,
            },
        ],
    );

    result
        .iter()
        .flat_map(|(path, _)| path)
        .map(|TraversalPosition { cost, .. }| *cost as u32)
        .sum()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> u32 {
    let map = parse(input);
    let goal = Position::new(map.width() - 1, map.height() - 1);

    let result = aoc_util::graph::search::astar(
        |pos| pos.neighbors(4, 10, &map),
        |pos| if pos.position == goal { Some(()) } else { None },
        |pos| aoc_util::geometry::manhattan_distance(pos.position, goal) as usize,
        [
            TraversalPosition {
                position: Position::zeros(),
                direction: Direction::Right,
                cost: 0,
            },
            TraversalPosition {
                position: Position::zeros(),
                direction: Direction::Down,
                cost: 0,
            },
        ],
    );

    result
        .iter()
        .flat_map(|(path, _)| path)
        .map(|TraversalPosition { cost, .. }| *cost as u32)
        .sum()
}

#[derive(Debug, Clone, Copy)]
struct TraversalPosition {
    position: Position,
    direction: Direction,
    cost: usize,
}

impl std::hash::Hash for TraversalPosition {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.position.hash(state);
        self.direction.orientation().hash(state);
    }
}

impl PartialEq for TraversalPosition {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
            && self.direction.orientation() == other.direction.orientation()
    }
}

impl Eq for TraversalPosition {}

impl TraversalPosition {
    pub fn neighbors(
        &self,
        min_steps: usize,
        max_steps: usize,
        map: &Map,
    ) -> impl IntoIterator<Item = (Self, usize)> {
        let mut neighbors = Vec::new();

        let left_direction = self.direction.turn_left();
        let right_direction = self.direction.turn_right();

        let mut left_position = self.position;
        let mut right_position = self.position;
        let mut left_cost = 0;
        let mut right_cost = 0;

        for distance in 1..=max_steps {
            left_position += left_direction;
            if let Some(&value) = map.get(&left_position) {
                left_cost += value as usize;
                if distance >= min_steps {
                    neighbors.push((
                        Self {
                            position: left_position,
                            direction: left_direction,
                            cost: left_cost,
                        },
                        left_cost,
                    ));
                }
            }

            right_position += right_direction;
            if let Some(&value) = map.get(&right_position) {
                right_cost += value as usize;
                if distance >= min_steps {
                    neighbors.push((
                        Self {
                            position: right_position,
                            direction: right_direction,
                            cost: right_cost,
                        },
                        right_cost,
                    ));
                }
            }
        }

        neighbors
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
