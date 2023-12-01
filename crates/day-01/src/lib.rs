use regex::Regex;

const INPUT: &str = include_str!("input.txt");

pub struct Shared;

pub fn part1() -> (impl std::fmt::Display, Shared) {
    (solve_p1(INPUT), Shared)
}

fn solve_p1(input: &str) -> u32 {
    input
        .lines()
        .filter_map(|line| {
            let mut first = None;
            let mut last = None;
            for c in line.chars().filter(|c| c.is_ascii_digit()) {
                first = first.or(Some(c));
                last = Some(c);
            }

            first.and_then(|first| last.map(|last| (first, last)))
        })
        .map(|(a, b)| int_val(a) * 10 + int_val(b))
        .sum()
}

fn int_val(c: char) -> u32 {
    assert!(c.is_ascii_digit());
    c as u32 - '0' as u32
}

pub fn part2(_shared: Shared) -> impl std::fmt::Display {
    solve_p2(INPUT)
}

fn solve_p2(input: &str) -> u32 {
    let re = Regex::new(r"([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    input
        .lines()
        .filter_map(|line| {
            let mut first = None;
            let mut last = None;
            for start_idx in 0..line.len() {
                if let Some(capture) = re.captures_at(line, start_idx) {
                    first = first.or(capture.get(1).map(|m| m.as_str()));
                    last = capture.get(1).map(|m| m.as_str());
                }
            }

            first.and_then(|first| last.map(|last| int_val2(first) * 10 + int_val2(last)))
        })
        .sum()
}

fn int_val2(s: &str) -> u32 {
    match s {
        "0" => 0,
        "one" | "1" => 1,
        "two" | "2" => 2,
        "three" | "3" => 3,
        "four" | "4" => 4,
        "five" | "5" => 5,
        "six" | "6" => 6,
        "seven" | "7" => 7,
        "eight" | "8" => 8,
        "nine" | "9" => 9,
        _ => unreachable!("{s:?} should not be matched"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet
";

    const TEST_INPUT2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen
";

    #[test]
    fn test_part1() {
        let solution = solve_p1(TEST_INPUT1);
        assert_eq!(solution, 142);
    }

    #[test]
    fn test_part2() {
        let solution = solve_p2(TEST_INPUT2);
        assert_eq!(solution, 281);
    }
}
