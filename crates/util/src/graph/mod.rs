use std::collections::VecDeque;

use ahash::{AHashMap as HashMap, AHashSet as HashSet};
use itertools::Itertools;

mod astar;

pub use astar::astar;

pub fn depth_first_search<P, N, T>(
    mut neighbors: impl FnMut(&P, usize) -> N,
    mut visit: impl FnMut(&P, usize) -> Option<T>,
    start: impl IntoIterator<Item = P>,
) -> Option<T>
where
    P: Clone + std::hash::Hash + Eq,
    N: IntoIterator<Item = P>,
{
    let mut stack: Vec<(P, usize)> = Vec::from_iter(start.into_iter().map(|p| (p, 0)));
    let mut visited = HashSet::new();
    let mut result = None;

    while let Some((position, depth)) = stack.pop() {
        result = visit(&position, depth);

        if result.is_some() {
            break;
        }

        visited.insert(position.clone());
        stack.extend(
            neighbors(&position, depth)
                .into_iter()
                .filter(|n| !visited.contains(n))
                .map(|n| (n, depth + 1)),
        );
    }

    result
}

pub fn depth_first_path<P, N, T>(
    mut neighbors: impl FnMut(&P, usize) -> N,
    mut visit: impl FnMut(&P, usize) -> Option<T>,
    start: impl IntoIterator<Item = P>,
) -> Option<(Vec<P>, T)>
where
    P: Clone + std::hash::Hash + Eq,
    N: IntoIterator<Item = P>,
{
    let mut stack: Vec<(P, P, usize)> =
        Vec::from_iter(start.into_iter().map(|p| (p.clone(), p, 0)));
    let mut visited = HashMap::new();
    let mut result = None;

    while let Some((position, previous, depth)) = stack.pop() {
        result = visit(&position, depth).map(|result| (result, position.clone()));
        visited.insert(position.clone(), previous);

        if result.is_some() {
            break;
        }

        stack.extend(
            neighbors(&position, depth)
                .into_iter()
                .filter(|n| !visited.contains_key(n))
                .map(|n| (n, position.clone(), depth + 1)),
        );
    }

    result.map(|(result, position)| {
        let path = std::iter::successors(Some(position), |position| {
            visited.get(position).and_then(|previous| {
                if position == previous {
                    None
                } else {
                    Some(previous.clone())
                }
            })
        })
        .collect_vec();
        (path, result)
    })
}

pub fn breadth_first_search<P, N, T>(
    mut neighbors: impl FnMut(&P, usize) -> N,
    mut visit: impl FnMut(&P, usize) -> Option<T>,
    start: impl IntoIterator<Item = P>,
) -> Option<T>
where
    P: Clone + std::hash::Hash + Eq,
    N: IntoIterator<Item = P>,
{
    let mut queue: VecDeque<(P, usize)> = VecDeque::from_iter(start.into_iter().map(|p| (p, 0)));
    let mut visited = HashSet::new();
    let mut result = None;

    while let Some((position, depth)) = queue.pop_front() {
        result = visit(&position, depth);

        if result.is_some() {
            break;
        }

        visited.insert(position.clone());
        neighbors(&position, depth)
            .into_iter()
            .filter(|n| !visited.contains(n))
            .for_each(|n| {
                queue.push_back((n, depth + 1));
            });
    }

    result
}

pub fn breadth_first_path<P, N, T>(
    mut neighbors: impl FnMut(&P, usize) -> N,
    mut visit: impl FnMut(&P, usize) -> Option<T>,
    start: impl IntoIterator<Item = P>,
) -> Option<(Vec<P>, T)>
where
    P: Clone + std::hash::Hash + Eq,
    N: IntoIterator<Item = P>,
{
    let mut queue: VecDeque<(P, P, usize)> =
        VecDeque::from_iter(start.into_iter().map(|p| (p.clone(), p, 0)));
    let mut visited = HashMap::new();
    let mut result = None;

    while let Some((position, previous, depth)) = queue.pop_front() {
        result = visit(&position, depth).map(|result| (result, position.clone()));
        visited.insert(position.clone(), previous);

        if result.is_some() {
            break;
        }

        neighbors(&position, depth)
            .into_iter()
            .filter(|n| !visited.contains_key(n))
            .for_each(|n| {
                queue.push_back((n, position.clone(), depth + 1));
            })
    }

    result.map(|(result, position)| {
        let path = std::iter::successors(Some(position), |position| {
            visited.get(position).and_then(|previous| {
                if position == previous {
                    None
                } else {
                    Some(previous.clone())
                }
            })
        })
        .collect_vec();
        (path, result)
    })
}
