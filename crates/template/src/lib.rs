pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(_input: &str) -> &'static str {
    "TODO"
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(_input: &str) -> &'static str {
    "TODO"
}

#[cfg(test)]
mod tests {
    use aoc_util::test::setup_tracing;

    use super::*;

    const TEST_INPUT: &str = "";

    #[test]
    fn test_part1() {
        setup_tracing();
        let solution = solve_part1(TEST_INPUT);
        assert_eq!(solution, "TODO");
    }

    #[test]
    fn test_part2() {
        setup_tracing();
        let solution = solve_part2(TEST_INPUT);
        assert_eq!(solution, "TODO");
    }
}
