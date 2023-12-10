use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

pub fn depth_first_search<P, N, T>(
    mut neighbors: impl FnMut(P, usize) -> N,
    mut visit: impl FnMut(P, usize) -> Option<T>,
    start: P,
) -> Option<T>
where
    P: Copy + std::hash::Hash + Eq,
    N: Iterator<Item = P>,
{
    let mut stack = vec![(start, 0)];
    let mut visited = HashSet::new();
    let mut result = None;

    while result.is_none() && !stack.is_empty() {
        let (position, depth) = stack.pop().unwrap();
        result = visit(position, depth);

        if result.is_none() {
            visited.insert(position);
            stack.extend(
                neighbors(position, depth)
                    .filter(|n| !visited.contains(n))
                    .map(|n| (n, depth + 1)),
            );
        }
    }

    result
}

pub fn depth_first_path<P, N, T>(
    mut neighbors: impl FnMut(P, usize) -> N,
    mut visit: impl FnMut(P, usize) -> Option<T>,
    start: P,
) -> Option<(Vec<P>, T)>
where
    P: Copy + std::hash::Hash + Eq,
    N: Iterator<Item = P>,
{
    let mut stack = vec![(start, start, 0)];
    let mut visited = HashMap::new();
    let mut result_position = None;
    let mut result = None;

    while result.is_none() && !stack.is_empty() {
        let (position, previous, depth) = stack.pop().unwrap();
        result = visit(position, depth);
        visited.insert(position, previous);

        if result.is_some() {
            result_position = Some(position);
        } else {
            stack.extend(
                neighbors(position, depth)
                    .filter(|n| !visited.contains_key(n))
                    .map(|n| (n, position, depth + 1)),
            );
        }
    }

    result.map(|result| {
        let path = std::iter::successors(result_position, |position| {
            visited.get(position).and_then(|previous| {
                if position == previous {
                    None
                } else {
                    Some(*previous)
                }
            })
        })
        .collect_vec();
        (path, result)
    })
}

pub fn breadth_first_search<P, N, T>(
    mut neighbors: impl FnMut(P, usize) -> N,
    mut visit: impl FnMut(P, usize) -> Option<T>,
    start: P,
) -> Option<T>
where
    P: Copy + std::hash::Hash + Eq,
    N: Iterator<Item = P>,
{
    let mut queue = VecDeque::from_iter([(start, 0)]);
    let mut visited = HashSet::new();
    let mut result = None;

    while result.is_none() && !queue.is_empty() {
        let (position, depth) = queue.pop_front().unwrap();
        result = visit(position, depth);

        if result.is_none() {
            visited.insert(position);

            neighbors(position, depth)
                .filter(|n| !visited.contains(n))
                .for_each(|n| {
                    queue.push_back((n, depth + 1));
                });
        }
    }

    result
}

pub fn breadth_first_path<P, N, T>(
    mut neighbors: impl FnMut(P, usize) -> N,
    mut visit: impl FnMut(P, usize) -> Option<T>,
    start: P,
) -> Option<(Vec<P>, T)>
where
    P: Copy + std::hash::Hash + Eq,
    N: Iterator<Item = P>,
{
    let mut queue = VecDeque::from_iter([(start, start, 0)]);
    let mut visited = HashMap::new();
    let mut result_position = None;
    let mut result = None;

    while result.is_none() && !queue.is_empty() {
        let (position, previous, depth) = queue.pop_front().unwrap();
        result = visit(position, depth);
        visited.insert(position, previous);

        if result.is_some() {
            result_position = Some(position);
        } else {
            neighbors(position, depth)
                .filter(|n| !visited.contains_key(n))
                .for_each(|n| {
                    queue.push_back((n, position, depth + 1));
                })
        }
    }

    result.map(|result| {
        let path = std::iter::successors(result_position, |position| {
            visited.get(position).and_then(|previous| {
                if position == previous {
                    None
                } else {
                    Some(*previous)
                }
            })
        })
        .collect_vec();
        (path, result)
    })
}
