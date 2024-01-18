use std::collections::VecDeque;

use ahash::{AHashMap as HashMap, AHashSet as HashSet};
use itertools::Itertools;

pub use super::astar::astar;

pub fn depth_first_search<P, N, T>(
    mut neighbors: impl FnMut(&P, usize) -> N,
    mut visit: impl FnMut(&P, usize) -> Option<T>,
    start: impl IntoIterator<Item = P>,
) -> Option<T>
where
    P: Clone + std::hash::Hash + Eq,
    N: IntoIterator<Item = P>,
{
    let mut stack = Vec::new();
    let mut visited = HashSet::new();
    let mut result = None;

    for position in start.into_iter() {
        stack.push((position.clone(), 0));
        visited.insert(position);
    }

    while let Some((position, depth)) = stack.pop() {
        result = visit(&position, depth);

        if result.is_some() {
            break;
        }

        for neighbor in neighbors(&position, depth).into_iter() {
            if visited.contains(&neighbor) {
                continue;
            }

            visited.insert(neighbor.clone());
            stack.push((neighbor, depth + 1));
        }
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
    let mut stack = Vec::new();
    let mut visited = HashMap::new();
    let mut result = None;

    for position in start.into_iter() {
        stack.push((position.clone(), position.clone(), 0));
        visited.insert(position.clone(), position);
    }

    while let Some((position, previous, depth)) = stack.pop() {
        result = visit(&position, depth).map(|result| (result, position.clone()));
        visited.insert(position.clone(), previous);

        if result.is_some() {
            break;
        }

        for neighbor in neighbors(&position, depth).into_iter() {
            if visited.contains_key(&neighbor) {
                continue;
            }

            visited.insert(neighbor.clone(), position.clone());
            stack.push((neighbor, position.clone(), depth + 1));
        }
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
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut result = None;

    for position in start.into_iter() {
        queue.push_back((position.clone(), 0));
        visited.insert(position);
    }

    while let Some((position, depth)) = queue.pop_front() {
        result = visit(&position, depth);

        if result.is_some() {
            break;
        }

        for neighbor in neighbors(&position, depth).into_iter() {
            if visited.contains(&neighbor) {
                continue;
            }

            visited.insert(neighbor.clone());
            queue.push_back((neighbor, depth + 1));
        }
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
    let mut queue = VecDeque::new();
    let mut visited = HashMap::new();
    let mut result = None;

    for position in start.into_iter() {
        queue.push_back((position.clone(), position.clone(), 0));
        visited.insert(position.clone(), position);
    }

    while let Some((position, previous, depth)) = queue.pop_front() {
        result = visit(&position, depth).map(|result| (result, position.clone()));
        visited.insert(position.clone(), previous);

        if result.is_some() {
            break;
        }

        for neighbor in neighbors(&position, depth).into_iter() {
            if visited.contains_key(&neighbor) {
                continue;
            }

            visited.insert(neighbor.clone(), position.clone());
            queue.push_back((neighbor, position.clone(), depth + 1));
        }
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
