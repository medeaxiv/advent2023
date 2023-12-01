use regex::Regex;

const INPUT: &str = include_str!("input.txt");

pub fn part1() -> impl std::fmt::Display {
    solve_part1(INPUT)
}

fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut digits = line.chars().filter_map(|c| c.to_digit(10));
            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            (first, last)
        })
        .map(|(a, b)| a * 10 + b)
        .sum()
}

pub fn part2() -> impl std::fmt::Display {
    solve_part2(INPUT)
}

fn solve_part2(input: &str) -> u32 {
    let re = Regex::new(r"^([0-9]|one|two|three|four|five|six|seven|eight|nine)").unwrap();
    input
        .lines()
        .map(|line| {
            let mut digits = (0..line.len())
                .map(|offset| &line[offset..])
                .filter_map(|slice| re.captures(slice)?.get(1).map(|m| m.as_str()))
                .map(digit);

            let first = digits.next().unwrap();
            let last = digits.last().unwrap_or(first);
            (first, last)
        })
        .map(|(a, b)| a * 10 + b)
        .sum()
}

fn digit(s: &str) -> u32 {
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
        let solution = solve_part1(TEST_INPUT1);
        assert_eq!(solution, 142);
    }

    #[test]
    fn test_part2() {
        let solution = solve_part2(TEST_INPUT2);
        assert_eq!(solution, 281);
    }
}
