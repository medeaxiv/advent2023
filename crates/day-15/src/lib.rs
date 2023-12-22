use std::ops::IndexMut;

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> usize {
    input
        .split([',', '\r', '\n'])
        .filter(|s| !s.is_empty())
        .map(hash)
        .sum()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> usize {
    let mut map = HashMap::new();

    for op in input
        .split([',', '\r', '\n'])
        .filter(|s| !s.is_empty())
        .map(parse)
    {
        match op {
            Operation::Insert(label, value) => {
                map.insert(label, value);
            }
            Operation::Remove(label) => {
                map.remove(label);
            }
        }
    }

    map.buckets
        .iter()
        .enumerate()
        .flat_map(|(bucket_idx, bucket)| {
            bucket
                .iter()
                .enumerate()
                .map(move |(slot_idx, (_, value))| (bucket_idx, slot_idx, *value))
        })
        .map(|(bucket_idx, slot_idx, value)| (bucket_idx + 1) * (slot_idx + 1) * value)
        .sum()
}

fn parse(input: &str) -> Operation {
    if let Some((label, value)) = input.split_once('=') {
        Operation::Insert(label, value.parse().unwrap())
    } else if let Some(label) = input.strip_suffix('-') {
        Operation::Remove(label)
    } else {
        unreachable!("{input}")
    }
}

enum Operation<'a> {
    Insert(&'a str, usize),
    Remove(&'a str),
}

fn hash(input: &str) -> usize {
    input
        .bytes()
        .fold(0, |acc, c| ((acc + c as usize) * 17) % 256)
}

struct HashMap<'a> {
    buckets: Vec<Vec<(&'a str, usize)>>,
}

impl<'a> HashMap<'a> {
    pub fn new() -> Self {
        Self {
            buckets: vec![Vec::new(); 256],
        }
    }

    pub fn insert(&mut self, key: &'a str, value: usize) {
        let idx = hash(key);
        let bucket = self.buckets.index_mut(idx);

        if let Some((_, stored)) = bucket.iter_mut().find(|(k, _)| k == &key) {
            *stored = value;
        } else {
            bucket.push((key, value));
        }
    }

    pub fn remove(&mut self, key: &'a str) {
        let idx = hash(key);
        let bucket = self.buckets.index_mut(idx);

        if let Some(entry_idx) = bucket.iter().position(|(k, _)| k == &key) {
            bucket.remove(entry_idx);
        }
    }
}

#[cfg(test)]
mod tests {
    use aoc_util::test::setup_tracing;

    use super::*;

    const TEST_INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn test_part1() {
        setup_tracing();
        let solution = solve_part1(TEST_INPUT);
        assert_eq!(solution, 1320);
    }

    #[test]
    fn test_part2() {
        setup_tracing();
        let solution = solve_part2(TEST_INPUT);
        assert_eq!(solution, 145);
    }
}
