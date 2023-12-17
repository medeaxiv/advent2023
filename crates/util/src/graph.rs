use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

pub fn depth_first_search<P, N, T>(
    mut neighbors: impl FnMut(P, usize) -> N,
    mut visit: impl FnMut(P, usize) -> Option<T>,
    start: P,
) -> Option<T>
where
    P: Copy + std::hash::Hash + Eq,
    N: IntoIterator<Item = P>,
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
                    .into_iter()
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
    N: IntoIterator<Item = P>,
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
                    .into_iter()
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
    N: IntoIterator<Item = P>,
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
                .into_iter()
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
    N: IntoIterator<Item = P>,
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
                .into_iter()
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

pub use astar::astar;

mod astar {
    use std::collections::{BinaryHeap, HashMap};

    use itertools::Itertools;

    #[derive(PartialEq, Eq)]
    struct Entry<P, D>(P, D)
    where
        P: Eq,
        D: Ord + Eq;

    impl<P, D> PartialOrd for Entry<P, D>
    where
        P: Eq,
        D: Ord + Eq,
    {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl<P, D> Ord for Entry<P, D>
    where
        P: Eq,
        D: Ord + Eq,
    {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.1.cmp(&self.1)
        }
    }

    pub fn astar<P, N, D, T>(
        mut neighbors: impl FnMut(P) -> N,
        mut visit: impl FnMut(P) -> Option<T>,
        heuristic: impl Fn(P) -> D,
        distance: impl Fn(P, P) -> D,
        start: P,
    ) -> Option<(Vec<P>, T)>
    where
        P: Copy + Eq + std::hash::Hash,
        N: IntoIterator<Item = P>,
        D: Copy + Ord + std::ops::Add<D, Output = D> + num::Zero,
    {
        // A* search algorithm
        // Adapted from https://en.wikipedia.org/wiki/A*_search_algorithm#Pseudocode

        let mut result = None;
        // The set of discovered nodes that may need to be (re-)expanded.
        // Initially, only the start node is known.
        // This is usually implemented as a min-heap or priority queue rather than a hash-set.
        let mut open_set = BinaryHeap::new();
        open_set.push(Entry(start, D::zero()));

        // For node n, visited[n] is the node immediately preceding it on the cheapest path from the start
        // to n currently known.
        let mut visited = HashMap::new();

        // For node n, node_costs[n] is the cost of the cheapest path from start to n currently known.
        let mut node_costs = HashMap::new();
        node_costs.insert(start, D::zero());

        while result.is_none() && !open_set.is_empty() {
            // This operation can occur in O(Log(N)) time if open_set is a min-heap or a priority queue
            let current = open_set.pop().unwrap().0;
            result = visit(current).map(|result| (result, current));
            if result.is_some() {
                break;
            }

            for neighbor in neighbors(current) {
                // distance(current,neighbor) is the weight of the edge from current to neighbor
                // neighbor_cost is the distance from start to the neighbor through current
                let neighbor_cost =
                    *node_costs.get(&current).unwrap() + distance(current, neighbor);

                if node_costs
                    .get(&neighbor)
                    .map(|&cost| neighbor_cost < cost)
                    .unwrap_or(true)
                {
                    // This path to neighbor is better than any previous one. Record it!
                    visited.insert(neighbor, current);
                    node_costs.insert(neighbor, neighbor_cost);
                    let neighbor_priority = neighbor_cost + heuristic(neighbor);
                    open_set.push(Entry(neighbor, neighbor_priority));
                }
            }
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
}
