use std::collections::VecDeque;

use ahash::{AHashMap as HashMap, AHashSet as HashSet};
use itertools::Itertools;

pub fn depth_first_search<P, N, T>(
    mut neighbors: impl FnMut(P, usize) -> N,
    mut visit: impl FnMut(P, usize) -> Option<T>,
    start: impl IntoIterator<Item = P>,
) -> Option<T>
where
    P: Copy + std::hash::Hash + Eq,
    N: IntoIterator<Item = P>,
{
    let mut stack = Vec::from_iter(start.into_iter().map(|p| (p, 0)));
    let mut visited = HashSet::new();
    let mut result = None;

    while let Some((position, depth)) = stack.pop() {
        result = visit(position, depth);

        if result.is_some() {
            break;
        }

        visited.insert(position);
        stack.extend(
            neighbors(position, depth)
                .into_iter()
                .filter(|n| !visited.contains(n))
                .map(|n| (n, depth + 1)),
        );
    }

    result
}

pub fn depth_first_path<P, N, T>(
    mut neighbors: impl FnMut(P, usize) -> N,
    mut visit: impl FnMut(P, usize) -> Option<T>,
    start: impl IntoIterator<Item = P>,
) -> Option<(Vec<P>, T)>
where
    P: Copy + std::hash::Hash + Eq,
    N: IntoIterator<Item = P>,
{
    let mut stack = Vec::from_iter(start.into_iter().map(|p| (p, p, 0)));
    let mut visited = HashMap::new();
    let mut result = None;

    while let Some((position, previous, depth)) = stack.pop() {
        result = visit(position, depth).map(|result| (result, position));
        visited.insert(position, previous);

        if result.is_some() {
            break;
        }

        stack.extend(
            neighbors(position, depth)
                .into_iter()
                .filter(|n| !visited.contains_key(n))
                .map(|n| (n, position, depth + 1)),
        );
    }

    result.map(|(result, position)| {
        let path = std::iter::successors(Some(position), |position| {
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
    start: impl IntoIterator<Item = P>,
) -> Option<T>
where
    P: Copy + std::hash::Hash + Eq,
    N: IntoIterator<Item = P>,
{
    let mut queue = VecDeque::from_iter(start.into_iter().map(|p| (p, 0)));
    let mut visited = HashSet::new();
    let mut result = None;

    while let Some((position, depth)) = queue.pop_front() {
        result = visit(position, depth);

        if result.is_some() {
            break;
        }

        visited.insert(position);
        neighbors(position, depth)
            .into_iter()
            .filter(|n| !visited.contains(n))
            .for_each(|n| {
                queue.push_back((n, depth + 1));
            });
    }

    result
}

pub fn breadth_first_path<P, N, T>(
    mut neighbors: impl FnMut(P, usize) -> N,
    mut visit: impl FnMut(P, usize) -> Option<T>,
    start: impl IntoIterator<Item = P>,
) -> Option<(Vec<P>, T)>
where
    P: Copy + std::hash::Hash + Eq,
    N: IntoIterator<Item = P>,
{
    let mut queue = VecDeque::from_iter(start.into_iter().map(|p| (p, p, 0)));
    let mut visited = HashMap::new();
    let mut result = None;

    while let Some((position, previous, depth)) = queue.pop_front() {
        result = visit(position, depth).map(|result| (result, position));
        visited.insert(position, previous);

        if result.is_some() {
            break;
        }

        neighbors(position, depth)
            .into_iter()
            .filter(|n| !visited.contains_key(n))
            .for_each(|n| {
                queue.push_back((n, position, depth + 1));
            })
    }

    result.map(|(result, position)| {
        let path = std::iter::successors(Some(position), |position| {
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

pub use astar::astar;

mod astar {
    use std::collections::{BinaryHeap, HashMap};

    use itertools::Itertools;

    #[derive(PartialEq, Eq)]
    struct Entry<P, C>(P, C)
    where
        P: Eq,
        C: Ord + Eq;

    impl<P, C> PartialOrd for Entry<P, C>
    where
        P: Eq,
        C: Ord + Eq,
    {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<P, C> Ord for Entry<P, C>
    where
        P: Eq,
        C: Ord + Eq,
    {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.1.cmp(&self.1)
        }
    }

    pub fn astar<P, N, C, T>(
        mut neighbors: impl FnMut(P) -> N,
        mut visit: impl FnMut(P) -> Option<T>,
        heuristic: impl Fn(P) -> C,
        start: impl IntoIterator<Item = P>,
    ) -> Option<(Vec<P>, T)>
    where
        P: Copy + Eq + std::hash::Hash,
        N: IntoIterator<Item = (P, C)>,
        C: Copy + Ord + std::ops::Add<C, Output = C> + num::Zero,
    {
        // A* search algorithm
        // Adapted from https://en.wikipedia.org/wiki/A*_search_algorithm#Pseudocode

        let mut result = None;
        let mut open_set = BinaryHeap::new();
        let mut visited = HashMap::new();

        for p in start.into_iter() {
            open_set.push(Entry(p, C::zero()));
            visited.insert(p, (p, C::zero()));
        }

        while let Some(current) = open_set.pop() {
            let current = current.0;
            result = visit(current).map(|result| (result, current));
            if result.is_some() {
                break;
            }

            let current_cost = visited.get(&current).map(|&(_, cost)| cost).unwrap();

            for (neighbor, cost) in neighbors(current) {
                let neighbor_cost = current_cost + cost;

                if visited
                    .get(&neighbor)
                    .map_or(true, |&(_, cost)| neighbor_cost < cost)
                {
                    // This path to neighbor is better than any previous one. Record it!
                    visited.insert(neighbor, (current, neighbor_cost));
                    let neighbor_priority = neighbor_cost + heuristic(neighbor);
                    open_set.push(Entry(neighbor, neighbor_priority));
                }
            }
        }

        result.map(|(result, position)| {
            let path = std::iter::successors(Some(position), |position| {
                visited.get(position).and_then(|(previous, _)| {
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
}
