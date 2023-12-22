use aoc_util::grid::{Direction, Position};
use nom::{
    bytes::complete::{tag, take},
    character::complete::{self, one_of, space1},
    combinator::map,
    sequence::{delimited, pair, separated_pair},
    IResult, Parser,
};
use nom_supreme::final_parser::final_parser;

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> u64 {
    let plan = input
        .lines()
        .map(parse)
        .map(Result::unwrap)
        .map(|(incorrect, _)| incorrect);

    calculate_area(plan)
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> u64 {
    let plan = input
        .lines()
        .map(parse)
        .map(Result::unwrap)
        .map(|(_, correct)| correct);

    calculate_area(plan)
}

fn calculate_area(plan: impl IntoIterator<Item = (Direction, usize)>) -> u64 {
    let mut position = Position::zeros();
    let mut perimeter = 0;
    let mut area = 0;

    for (direction, distance) in plan.into_iter() {
        let next = position + direction * distance;
        perimeter += distance as u64;
        area += determinant(position, next);
        position = next;
    }

    area.unsigned_abs() / 2 + perimeter / 2 + 1
}

fn determinant(a: Position, b: Position) -> i64 {
    a.x * b.y - b.x * a.y
}

#[allow(clippy::type_complexity)]
fn parse(input: &str) -> Result<((Direction, usize), (Direction, usize)), nom::error::Error<&str>> {
    fn direction_parser(input: &str) -> IResult<&str, Direction> {
        map(one_of("UDLR0123"), |c| match c {
            'U' | '3' => Direction::Up,
            'D' | '1' => Direction::Down,
            'L' | '2' => Direction::Left,
            'R' | '0' => Direction::Right,
            _ => unreachable!(),
        })(input)
    }

    let color_parser = delimited(
        tag("(#"),
        pair(
            take(5usize).map(|s| usize::from_str_radix(s, 16).unwrap()),
            direction_parser,
        ),
        tag(")"),
    )
    .map(|(distance, direction)| (direction, distance));

    let parser = separated_pair(
        direction_parser,
        space1,
        separated_pair(complete::u32.map(|v| v as usize), space1, color_parser),
    )
    .map(|(direction, (distance, color))| ((direction, distance), color));

    final_parser::<
        &str,
        ((Direction, usize), (Direction, usize)),
        nom::error::Error<&str>,
        nom::error::Error<&str>,
    >(parser)(input)
}

#[cfg(test)]
mod tests {
    use aoc_util::test::setup_tracing;
    use rstest::rstest;

    use super::*;

    const TEST_INPUT: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    #[test]
    fn test_part1() {
        setup_tracing();
        let solution = solve_part1(TEST_INPUT);
        assert_eq!(solution, 62);
    }

    #[rstest]
    #[case(
        "R 4 (#000000)
D 2 (#000000)
L 4 (#000000)
U 2 (#000000)
",
        15
    )]
    #[case(
        "L 4 (#000000)
D 2 (#000000)
R 4 (#000000)
U 2 (#000000)
",
        15
    )]
    fn extras_part1(#[case] input: &str, #[case] expected: u64) {
        setup_tracing();
        let solution = solve_part1(input);
        assert_eq!(solution, expected);
    }

    #[test]
    fn test_part2() {
        setup_tracing();
        let solution = solve_part2(TEST_INPUT);
        assert_eq!(solution, 952408144115);
    }
}
