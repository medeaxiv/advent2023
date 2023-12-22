use nalgebra::{vector, Vector2};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> u32 {
    let schematic = parse(input);

    schematic
        .numbers
        .par_iter()
        .filter(|(pos, slice, _)| {
            let y_range = (pos.y.saturating_sub(1))..=(pos.y + 1);
            let x_range = (pos.x.saturating_sub(1))..=(pos.x + slice.len());
            schematic
                .symbols
                .iter()
                .any(|(pos, _)| y_range.contains(&pos.y) && x_range.contains(&pos.x))
        })
        .map(|(_, _, value)| *value)
        .sum()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> u32 {
    let schematic = parse(input);

    schematic
        .symbols
        .par_iter()
        .filter(|(_, symbol)| *symbol == "*")
        .filter_map(|(gear_pos, _)| {
            let (count, ratio) = schematic
                .numbers
                .iter()
                .filter(|(pos, slice, _)| {
                    let y_range = (pos.y.saturating_sub(1))..=(pos.y + 1);
                    let x_range = (pos.x.saturating_sub(1))..=(pos.x + slice.len());
                    y_range.contains(&gear_pos.y) && x_range.contains(&gear_pos.x)
                })
                .fold((0, 1), |(count, acc), (_, _, value)| {
                    (count + 1, acc * value)
                });

            match count {
                2 => Some(ratio),
                _ => None,
            }
        })
        .sum()
}

type Pos = Vector2<usize>;

#[derive(Default, Debug)]
struct Schematic<'a> {
    symbols: Vec<(Pos, &'a str)>,
    numbers: Vec<(Pos, &'a str, u32)>,
}

fn parse(input: &str) -> Schematic {
    let mut schematic = Schematic::default();

    enum State {
        Default,
        Number(usize),
    }

    for (y, line) in input.lines().enumerate() {
        let mut state = State::Default;
        for (x, cur) in line.chars().enumerate() {
            if let State::Default = state {
                if cur.is_ascii_digit() {
                    state = State::Number(x);
                }
            } else if let State::Number(start) = state {
                if !cur.is_ascii_digit() {
                    let pos = vector![start, y];
                    let number = &line[start..x];
                    let value = number.parse().unwrap();
                    schematic.numbers.push((pos, number, value));
                    state = State::Default;
                }
            }

            if cur == '.' || cur.is_ascii_digit() {
                continue;
            }

            let pos = vector![x, y];
            let symbol = &line[x..=x];
            assert!(symbol.is_ascii());
            schematic.symbols.push((pos, symbol));
        }

        if let State::Number(start) = state {
            let pos = vector![start, y];
            let number = &line[start..];
            let value = number.parse().unwrap();
            schematic.numbers.push((pos, number, value));
        }
    }

    schematic
}

#[cfg(test)]
mod tests {
    use aoc_util::test::setup_tracing;
    use rstest::rstest;

    use super::*;

    const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn test_part1() {
        setup_tracing();
        let solution = solve_part1(TEST_INPUT);
        assert_eq!(solution, 4361);
    }

    #[rstest]
    #[case(
        "........
.24..4..
......*.",
        4
    )]
    #[case(
        "12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56",
        925
    )]
    fn extras_part1(#[case] input: &str, #[case] expected: u32) {
        setup_tracing();
        let solution = solve_part1(input);
        assert_eq!(solution, expected);
    }

    #[test]
    fn test_part2() {
        setup_tracing();
        let solution = solve_part2(TEST_INPUT);
        assert_eq!(solution, 467835);
    }

    #[rstest]
    #[case(
        "........
.24..4..
......*.",
        0
    )]
    #[case(
        "12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56",
        6756
    )]
    fn extras_part2(#[case] input: &str, #[case] expected: u32) {
        setup_tracing();
        let solution = solve_part2(input);
        assert_eq!(solution, expected);
    }
}
