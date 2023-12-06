use nom::{
    bytes::complete::tag,
    character::complete::{self, digit1, line_ending, space1},
    multi::separated_list1,
    sequence::{delimited, pair},
    Parser,
};
use nom_supreme::final_parser::final_parser;

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> i64 {
    let sheet = parse_part1(input).unwrap();
    sheet
        .iter()
        .map(|(total_time, target_distance)| winning_charge_time_count(total_time, target_distance))
        .product()
}

fn parse_part1(input: &str) -> Result<Sheet, nom::error::Error<&str>> {
    let line_parser = |prefix| {
        delimited(
            pair(tag(prefix), space1),
            separated_list1(space1, complete::i64),
            line_ending,
        )
    };

    let sheet_parser = pair(line_parser("Time:"), line_parser("Distance:"))
        .map(|(times, distances)| Sheet::new(times, distances));

    final_parser::<&str, Sheet, nom::error::Error<&str>, nom::error::Error<&str>>(sheet_parser)(
        input,
    )
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> i64 {
    let sheet = parse_part2(input).unwrap();
    sheet
        .iter()
        .map(|(total_time, target_distance)| winning_charge_time_count(total_time, target_distance))
        .product()
}

fn parse_part2(input: &str) -> Result<Sheet, nom::error::Error<&str>> {
    let line_parser = |prefix| {
        delimited(
            pair(tag(prefix), space1),
            separated_list1(space1, digit1),
            line_ending,
        )
        .map(|digits: Vec<&str>| {
            digits
                .iter()
                .flat_map(|d| d.chars())
                .fold(0, |acc, c| 10 * acc + c.to_digit(10).unwrap() as i64)
        })
        .map(|value| vec![value])
    };

    let sheet_parser = pair(line_parser("Time:"), line_parser("Distance:"))
        .map(|(times, distances)| Sheet::new(times, distances));

    final_parser::<&str, Sheet, nom::error::Error<&str>, nom::error::Error<&str>>(sheet_parser)(
        input,
    )
}

fn winning_charge_time_count(total_time: i64, target_distance: i64) -> i64 {
    // f(t) = t * (T - t) - D
    // f(t) = (t * T - t * t) - D
    // f(t) = -t^2 + T t - D
    // therefore
    // f'(t) = -2t + T
    let f = |t: i64| (t * (total_time - t)) - target_distance;
    let f_ = |t: i64| (-2 * t) + total_time;

    // Use newton's method to obtain the minimum winning charge time
    let minimum_charge_time = {
        let mut t = 0;
        let mut value = f(t);
        while value <= 0 {
            let derivative = f_(t);
            assert_ne!(derivative, 0);
            let correction = (value / derivative).min(-1);
            t -= correction;
            value = f(t);
        }

        t
    };

    // `f(t)` is always (as far as I can tell) symmetric in the range `0..total_time`
    total_time - 2 * minimum_charge_time + 1
}

#[derive(Debug)]
struct Sheet {
    times: Vec<i64>,
    distances: Vec<i64>,
}

impl Sheet {
    pub fn new(times: Vec<i64>, distances: Vec<i64>) -> Self {
        assert_eq!(times.len(), distances.len());
        Self { times, distances }
    }

    pub fn iter(&self) -> impl Iterator<Item = (i64, i64)> + '_ {
        self.times
            .iter()
            .copied()
            .zip(self.distances.iter().copied())
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    const TEST_INPUT: &str = "Time:      7  15   30
Distance:  9  40  200
";

    #[test]
    fn test_part1() {
        let solution = solve_part1(TEST_INPUT);
        assert_eq!(solution, 288);
    }

    #[rstest]
    #[case(7, 9, 4)]
    #[case(15, 40, 8)]
    #[case(30, 200, 9)]
    fn test_count_winning_charge_times(
        #[case] total_time: i64,
        #[case] target_distance: i64,
        #[case] expected: i64,
    ) {
        let result = winning_charge_time_count(total_time, target_distance);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part2() {
        let solution = solve_part2(TEST_INPUT);
        assert_eq!(solution, 71503);
    }
}
