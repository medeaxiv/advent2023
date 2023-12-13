use itertools::Itertools;

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> usize {
    let patterns = parse(input);

    patterns
        .iter()
        .filter_map(|pattern| {
            (1..pattern.height)
                .find(|row| pattern.folds_at_row(*row))
                .map(|row| row * 100)
                .or_else(|| (1..pattern.width).find(|column| pattern.folds_at_column(*column)))
        })
        .sum()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> usize {
    let patterns = parse(input);

    patterns
        .iter()
        .filter_map(|pattern| {
            (1..pattern.height)
                .find(|row| pattern.fold_at_row_error(*row) == 1)
                .map(|row| row * 100)
                .or_else(|| {
                    (1..pattern.width).find(|column| pattern.fold_at_column_error(*column) == 1)
                })
        })
        .sum()
}

struct Pattern {
    width: usize,
    height: usize,
    rows: Vec<u64>,
}

impl Pattern {
    pub fn folds_at_row(&self, row: usize) -> bool {
        self.accumulate_row_fold(row, true, |acc, (a, b)| acc && a == b)
            .unwrap_or(false)
    }

    pub fn fold_at_row_error(&self, row: usize) -> usize {
        self.accumulate_row_fold(row, 0, |acc, (a, b)| {
            let error = a ^ b;
            acc + error.count_ones() as usize
        })
        .unwrap_or(usize::MAX)
    }

    fn accumulate_row_fold<T>(
        &self,
        row: usize,
        init: T,
        f: impl Fn(T, (u64, u64)) -> T,
    ) -> Option<T> {
        if row < 1 || row >= self.height {
            return None;
        }

        let checks = row.min(self.height - row);
        let result = (0..checks)
            .map(|check| (row - check - 1, row + check))
            .map(|(a, b)| (self.rows[a], self.rows[b]))
            .fold(init, f);
        Some(result)
    }

    pub fn folds_at_column(&self, column: usize) -> bool {
        self.accumulate_column_fold(column, true, |acc, (a, b)| acc && a == b)
            .unwrap_or(false)
    }

    pub fn fold_at_column_error(&self, column: usize) -> usize {
        self.accumulate_column_fold(column, 0, |acc, (a, b)| {
            let error = a ^ b;
            acc + error.count_ones() as usize
        })
        .unwrap_or(usize::MAX)
    }

    fn accumulate_column_fold<T>(
        &self,
        column: usize,
        init: T,
        f: impl Fn(T, (u64, u64)) -> T,
    ) -> Option<T> {
        if column < 1 || column >= self.width {
            return None;
        }

        let column = self.width - column;

        let checks = column.min(self.width - column);
        let result = (0..checks)
            .map(|check| (column - check - 1, column + check))
            .map(|(a, b)| (self.column(a), self.column(b)))
            .fold(init, f);
        Some(result)
    }

    fn column(&self, index: usize) -> u64 {
        assert!(index < self.width);
        self.rows
            .iter()
            .fold(0, |acc, row| acc << 1 | ((row >> (index)) & 1))
    }
}

fn parse(input: &str) -> Vec<Pattern> {
    input
        .lines()
        .group_by(|line| line.is_empty())
        .into_iter()
        .filter_map(|(empty, group)| (!empty).then_some(group))
        .map(|group| parse_one(group))
        .collect_vec()
}

fn parse_one<'a>(input: impl Iterator<Item = &'a str>) -> Pattern {
    let mut width = 0;
    let mut height = 0;
    let rows = input
        .enumerate()
        .map(|(idx, line)| {
            assert!(idx < 64);
            assert!(line.len() <= 64);
            height += 1;

            if idx == 0 {
                width = line.len();
            }

            line.chars().fold(0u64, |acc, c| match c {
                '.' => acc << 1,
                '#' => (acc << 1) | 1,
                _ => unreachable!(),
            })
        })
        .collect_vec();

    Pattern {
        width,
        height,
        rows,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    #[test]
    fn test_part1() {
        let solution = solve_part1(TEST_INPUT);
        assert_eq!(solution, 405);
    }

    #[test]
    fn test_part2() {
        let solution = solve_part2(TEST_INPUT);
        assert_eq!(solution, 400);
    }
}
