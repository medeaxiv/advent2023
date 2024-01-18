use ahash::AHashMap as HashMap;

mod parser;

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> usize {
    let graph = parser::parse(input).unwrap();
    assert!(!graph.is_empty());

    let (minimum_cut, partition) =
        stoer_wagner::minimum_cut(&graph, 3).expect("Graph should have a cut");
    assert_eq!(minimum_cut, 3);
    partition.len() * (graph.len() - partition.len())
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(_input: &str) -> &'static str {
    "()"
}

#[derive(Default, Clone)]
struct Graph<'a> {
    nodes: Vec<&'a str>,
    index: HashMap<&'a str, usize>,
    adjacency_matrix: Vec<Vec<i64>>,
}

impl<'a> Graph<'a> {
    fn len(&self) -> usize {
        self.nodes.len()
    }

    fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    fn insert_node(&mut self, node: &'a str) -> usize {
        if let Some(index) = self.index.get(node) {
            *index
        } else {
            let index = self.len();
            self.nodes.push(node);
            self.index.insert(node, index);

            // Expand existing rows
            self.adjacency_matrix.iter_mut().for_each(|v| {
                v.push(0);
            });

            // Add new row
            self.adjacency_matrix.push(vec![0; self.len()]);

            index
        }
    }

    fn insert_edge(&mut self, from: &'a str, to: &'a str) {
        let from_idx = self.insert_node(from);
        let to_idx = self.insert_node(to);
        self.adjacency_matrix[from_idx][to_idx] += 1;
        self.adjacency_matrix[to_idx][from_idx] += 1;
    }
}

impl<'a> FromIterator<(&'a str, Vec<&'a str>)> for Graph<'a> {
    fn from_iter<T: IntoIterator<Item = (&'a str, Vec<&'a str>)>>(iter: T) -> Self {
        let mut graph = Graph::default();

        for (node, neighbors) in iter.into_iter() {
            for neighbor in neighbors {
                graph.insert_edge(node, neighbor);
            }
        }

        graph
    }
}

mod stoer_wagner {
    use aoc_util::slice::SliceExt;
    use itertools::Itertools;

    use super::*;

    #[derive(Clone, PartialEq, Eq)]
    struct Edge {
        neighbor: String,
        weight: usize,
    }

    impl PartialOrd for Edge {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Edge {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.weight.cmp(&other.weight)
        }
    }

    pub fn minimum_cut(graph: &Graph, target_cut: i64) -> Option<(i64, Vec<usize>)> {
        let mut matrix = graph.adjacency_matrix.clone();
        let mut collapsed = (0..graph.len()).map(|index| vec![index]).collect_vec();

        let mut minimum = None;
        for phase in 1..graph.len() {
            let (cut, t) = minimum_cut_phase(graph, &mut matrix, &mut collapsed, phase);

            if let Some((minimum_cut, partition)) = minimum.as_mut() {
                if cut < *minimum_cut {
                    *minimum_cut = cut;
                    *partition = collapsed[t].clone();
                }
            } else {
                minimum = Some((cut, collapsed[t].clone()));
            }

            if cut < target_cut {
                break;
            }
        }

        minimum
    }

    fn minimum_cut_phase(
        graph: &Graph,
        matrix: &mut [Vec<i64>],
        collapsed: &mut [Vec<usize>],
        phase: usize,
    ) -> (i64, usize) {
        let mut weights = matrix[0].clone();
        let mut s = 0;
        let mut t = 0;

        for _ in 0..graph.len() - phase {
            weights[t] = i64::MIN;
            s = t;
            t = weights.iter().position_max().unwrap();

            for (i, weight) in weights.iter_mut().enumerate() {
                *weight += matrix[t][i];
            }
        }

        let cut = weights[t] - matrix[t][t];

        if let Some((cs, ct)) = collapsed.multi_index_mut(s, t) {
            cs.extend_from_slice(ct);
        }

        for i in 0..graph.len() {
            matrix[s][i] += matrix[t][i];
        }

        for i in 0..graph.len() {
            matrix[i][s] = matrix[s][i];
        }

        matrix[0][t] = i64::MIN;

        (cut, t)
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_simple_graph() {
            let graph = Graph {
                nodes: vec!["a", "b", "c", "d", "e", "f"],
                index: HashMap::from_iter([
                    ("a", 0),
                    ("b", 1),
                    ("c", 2),
                    ("d", 3),
                    ("e", 4),
                    ("f", 5),
                ]),
                adjacency_matrix: vec![
                    vec![0, 5, 0, 0, 1, 4],
                    vec![5, 0, 2, 0, 0, 0],
                    vec![0, 2, 0, 6, 1, 1],
                    vec![0, 0, 6, 0, 3, 0],
                    vec![1, 0, 1, 3, 0, 0],
                    vec![4, 0, 1, 0, 0, 0],
                ],
            };

            let (minimum_cut, partition) = minimum_cut(&graph, 0).expect("Graph should have a cut");
            assert_eq!(minimum_cut, 4);
            assert_eq!(partition.len(), 3);
            assert!(partition.contains(&2));
            assert!(partition.contains(&3));
            assert!(partition.contains(&4));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
";

    #[test]
    fn test_part1() {
        let solution = solve_part1(TEST_INPUT);
        assert_eq!(solution, 54);
    }

    #[test]
    fn test_part2() {
        let solution = solve_part2(TEST_INPUT);
        assert_eq!(solution, "TODO");
    }
}
