use std::{collections::VecDeque, marker::PhantomData};

pub trait DimensionCollection<D, B> {
    fn get_dimension(&self, dimension: &D) -> B;
}

#[derive(Debug, Clone)]
pub struct DimensionRange<B>
where
    B: std::fmt::Debug + Clone,
{
    pub start: Option<B>,
    pub end: Option<B>,
}

impl<B> DimensionRange<B>
where
    B: std::fmt::Debug + Clone,
{
    fn lesser(boundary: B) -> Self {
        Self {
            start: None,
            end: Some(boundary),
        }
    }
    fn greater(boundary: B) -> Self {
        Self {
            start: Some(boundary),
            end: None,
        }
    }
}

#[derive(Debug)]
pub struct KdTree<D, B, V>
where
    D: std::fmt::Debug + Eq,
    B: std::fmt::Debug + Ord + Clone,
    V: std::fmt::Debug,
{
    nodes: Vec<KdTreeNode<D, B, V>>,
}

impl<D, B, V> KdTree<D, B, V>
where
    D: std::fmt::Debug + Eq,
    B: std::fmt::Debug + Ord + Clone,
    V: std::fmt::Debug,
{
    pub fn build<P>(f: impl Fn(P) -> KdTreeBuilderNode<D, B, V, P>, root: P) -> Self {
        let builder = KdTreeBuilder::new(f);

        let nodes = builder.build(root);

        Self { nodes }
    }

    pub fn find(&self, dimensions: &impl DimensionCollection<D, B>) -> &V {
        let mut idx = 0;

        loop {
            match &self.nodes[idx] {
                KdTreeNode::Leaf(value) => {
                    return value;
                }
                KdTreeNode::Split {
                    dimension,
                    boundary,
                    lesser,
                    greater,
                } => {
                    let dimension = dimensions.get_dimension(dimension);

                    idx = if dimension < *boundary {
                        *lesser
                    } else {
                        *greater
                    };
                }
            }
        }
    }

    pub fn traverse<T>(
        &self,
        mut visit: impl FnMut(&V, &[(&D, DimensionRange<B>)]) -> Option<T>,
    ) -> Option<T> {
        let mut queue = VecDeque::from_iter([(0, Vec::new())]);

        while let Some((idx, ranges)) = queue.pop_front() {
            match &self.nodes[idx] {
                KdTreeNode::Leaf(value) => {
                    if let Some(result) = visit(value, ranges.as_slice()) {
                        return Some(result);
                    }
                }
                KdTreeNode::Split {
                    dimension,
                    boundary,
                    lesser,
                    greater,
                } => {
                    let mut lesser_ranges = ranges.clone();
                    if let Some((_, range)) =
                        lesser_ranges.iter_mut().find(|(d, _)| **d == *dimension)
                    {
                        range.end = Some(boundary.clone());
                    } else {
                        lesser_ranges.push((dimension, DimensionRange::lesser(boundary.clone())))
                    }

                    queue.push_back((*lesser, lesser_ranges));

                    let mut greater_ranges = ranges;
                    if let Some((_, range)) =
                        greater_ranges.iter_mut().find(|(d, _)| **d == *dimension)
                    {
                        range.start = Some(boundary.clone());
                    } else {
                        greater_ranges.push((dimension, DimensionRange::greater(boundary.clone())))
                    }

                    queue.push_back((*greater, greater_ranges));
                }
            }
        }

        None
    }
}

#[derive(Debug)]
enum KdTreeNode<D, B, V>
where
    D: std::fmt::Debug + Eq,
    B: std::fmt::Debug + Ord + Clone,
    V: std::fmt::Debug,
{
    Split {
        dimension: D,
        boundary: B,
        lesser: usize,
        greater: usize,
    },
    Leaf(V),
}

impl<D, B, V> KdTreeNode<D, B, V>
where
    D: std::fmt::Debug + Eq,
    B: std::fmt::Debug + Ord + Clone,
    V: std::fmt::Debug,
{
}

struct KdTreeBuilder<D, B, V, P, F>
where
    D: std::fmt::Debug + Eq,
    B: std::fmt::Debug + Ord + Clone,
    V: std::fmt::Debug,
    F: Fn(P) -> KdTreeBuilderNode<D, B, V, P>,
{
    f: F,
    _p: PhantomData<(D, B, V, P)>,
}

impl<D, B, V, P, F> KdTreeBuilder<D, B, V, P, F>
where
    D: std::fmt::Debug + Eq,
    B: std::fmt::Debug + Ord + Clone,
    V: std::fmt::Debug,
    F: Fn(P) -> KdTreeBuilderNode<D, B, V, P>,
{
    pub fn new(f: F) -> Self {
        Self {
            f,
            _p: Default::default(),
        }
    }

    pub fn build(&self, root: P) -> Vec<KdTreeNode<D, B, V>> {
        let mut nodes = Vec::new();
        let mut queue = VecDeque::from_iter([root]);
        let mut idx = 0;

        while let Some(position) = queue.pop_front() {
            match (self.f)(position) {
                KdTreeBuilderNode::Leaf(value) => {
                    nodes.push(KdTreeNode::Leaf(value));
                }
                KdTreeBuilderNode::Split {
                    dimension,
                    boundary,
                    lesser,
                    greater,
                } => {
                    let node = KdTreeNode::Split {
                        dimension,
                        boundary,
                        lesser: idx + 1,
                        greater: idx + 2,
                    };
                    idx += 2;

                    queue.push_back(lesser);
                    queue.push_back(greater);

                    nodes.push(node);
                }
            }
        }

        nodes
    }
}

pub enum KdTreeBuilderNode<D, B, V, P>
where
    D: Eq,
    B: Ord,
{
    Split {
        dimension: D,
        boundary: B,
        lesser: P,
        greater: P,
    },
    Leaf(V),
}
