const INPUT: &str = include_str!("input.txt");

pub fn part1() -> impl std::fmt::Display {
    solve_part1(INPUT)
}

fn solve_part1(_input: &str) -> &'static str {
    "TODO"
}

pub fn part2() -> impl std::fmt::Display {
    solve_part2(INPUT)
}

fn solve_part2(_input: &str) -> &'static str {
    "TODO"
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_part1() {
        let solution = solve_part1(TEST_INPUT);
        assert_eq!(solution, "TODO");
    }

    #[test]
    fn test_part2() {
        let solution = solve_part2(TEST_INPUT);
        assert_eq!(solution, "TODO");
    }
}
