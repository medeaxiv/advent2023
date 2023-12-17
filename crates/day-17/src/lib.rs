use nalgebra::Vector2;

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> u32 {
    let map = parse(input);
    let goal = Pos::new(map.width as i32 - 1, map.height as i32 - 1);

    let result = aoc_util::graph::astar(
        |pos| {
            pos.neighbors(1, 3)
                .filter(|TraversalPosition { position, .. }| map.contains(*position))
        },
        |TraversalPosition { position, .. }| if position == goal { Some(()) } else { None },
        |TraversalPosition { position, .. }| aoc_util::geometry::manhattan_distance(position, goal),
        |_, new| map.get(new.position).unwrap() as i32,
        [
            TraversalPosition {
                position: Pos::new(1, 0),
                direction: Direction::Right,
                steps: 1,
            },
            TraversalPosition {
                position: Pos::new(0, 1),
                direction: Direction::Down,
                steps: 1,
            },
        ],
    );

    result
        .iter()
        .flat_map(|(path, _)| path)
        .map(|TraversalPosition { position, .. }| map.get(*position).unwrap_or(0))
        .sum()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> u32 {
    let map = parse(input);
    let goal = Pos::new(map.width as i32 - 1, map.height as i32 - 1);

    let result = aoc_util::graph::astar(
        |pos| {
            pos.neighbors(4, 10)
                .filter(|TraversalPosition { position, .. }| map.contains(*position))
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
        |_, new| map.get(new.position).unwrap() as i32,
        [
            TraversalPosition {
                position: Pos::new(1, 0),
                direction: Direction::Right,
                steps: 1,
            },
            TraversalPosition {
                position: Pos::new(0, 1),
                direction: Direction::Down,
                steps: 1,
            },
        ],
    );

    result
        .iter()
        .flat_map(|(path, _)| path)
        .map(|TraversalPosition { position, .. }| map.get(*position).unwrap_or(0))
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct TraversalPosition {
    position: Pos,
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
                        position: direction.next(from.position),
                        direction,
                        steps: from.steps + 1,
                    };
                    Some(neighbor)
                } else {
                    None
                }
            } else if direction == from.direction.reverse() {
                None
            } else if from.steps >= min_steps {
                let neighbor = Self {
                    position: direction.next(from.position),
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

type Pos = Vector2<i32>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub const ALL: [Self; 4] = [Self::Up, Self::Down, Self::Left, Self::Right];

    pub fn next(&self, position: Pos) -> Pos {
        match self {
            Self::Up => position + Pos::new(0, -1),
            Self::Down => position + Pos::new(0, 1),
            Self::Left => position + Pos::new(-1, 0),
            Self::Right => position + Pos::new(1, 0),
        }
    }

    pub fn reverse(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }
}

struct Map {
    width: usize,
    height: usize,
    blocks: Vec<u32>,
}

impl Map {
    pub fn contains(&self, position: Pos) -> bool {
        (0..self.width as i32).contains(&position.x)
            && (0..self.height as i32).contains(&position.y)
    }

    pub fn index(&self, position: Pos) -> Option<usize> {
        if self.contains(position) {
            Some(position.y as usize * self.width + position.x as usize)
        } else {
            None
        }
    }

    pub fn get(&self, position: Pos) -> Option<u32> {
        self.index(position).map(|idx| self.blocks[idx])
    }
}

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
            blocks.push(c.to_digit(10).unwrap());
        }
    }

    Map {
        width,
        height,
        blocks,
    }
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
