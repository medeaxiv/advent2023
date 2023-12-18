use std::collections::BinaryHeap;

use ahash::AHashMap as HashMap;
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
    mut neighbors: impl FnMut(&P) -> N,
    mut visit: impl FnMut(&P) -> Option<T>,
    heuristic: impl Fn(&P) -> C,
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
    let mut queue = BinaryHeap::new();
    let mut visited = HashMap::new();

    for p in start.into_iter() {
        queue.push(Entry(p, C::zero()));
        visited.insert(p, (p, C::zero()));
    }

    while let Some(current) = queue.pop() {
        let current = current.0;
        result = visit(&current).map(|result| (result, current));
        if result.is_some() {
            break;
        }

        let current_cost = visited.get(&current).map(|&(_, cost)| cost).unwrap();

        for (neighbor, cost) in neighbors(&current) {
            let neighbor_cost = current_cost + cost;

            if visited
                .get(&neighbor)
                .map_or(true, |&(_, cost)| neighbor_cost < cost)
            {
                // This path to neighbor is better than any previous one. Record it!
                visited.insert(neighbor, (current, neighbor_cost));
                let neighbor_priority = neighbor_cost + heuristic(&neighbor);
                queue.push(Entry(neighbor, neighbor_priority));
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
