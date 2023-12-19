use std::ops::Range;

use itertools::Itertools;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MultiRange<N>
where
    N: Eq + Ord + Clone,
{
    internal: MultiRangeInternal<N>,
}

impl<N> MultiRange<N>
where
    N: Eq + Ord + Clone,
{
    pub fn new(value: impl Into<MultiRange<N>>) -> Self {
        value.into()
    }

    pub fn empty() -> Self {
        Self {
            internal: MultiRangeInternal::Empty,
        }
    }

    pub fn split(&self, at: N) -> (Self, Self) {
        let (a, b) = self.internal.split(at);
        (Self { internal: a }, Self { internal: b })
    }

    pub fn merge(&self, other: &Self) -> Self {
        Self {
            internal: self.internal.merge(&other.internal),
        }
    }
}

impl<N, T> From<T> for MultiRange<N>
where
    N: Eq + Ord + Clone,
    T: Into<MultiRangeInternal<N>>,
{
    fn from(value: T) -> Self {
        Self {
            internal: value.into(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum MultiRangeInternal<N>
where
    N: Eq + Ord + Clone,
{
    Empty,
    Single(Range<N>),
    Multiple(Vec<Range<N>>),
}

impl<N> MultiRangeInternal<N>
where
    N: Eq + Ord + Clone,
{
    #[allow(dead_code)]
    pub fn len(&self) -> usize {
        match self {
            Self::Empty => 0,
            Self::Single(..) => 1,
            Self::Multiple(vec) => vec.len(),
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Self::Empty => true,
            Self::Single(range) => range.is_empty(),
            Self::Multiple(ranges) => ranges.is_empty(),
        }
    }

    pub fn split(&self, at: N) -> (Self, Self) {
        match self {
            Self::Empty => (Self::Empty, Self::Empty),
            Self::Single(range) => Self::split_single(range, at),
            Self::Multiple(ranges) => Self::split_multiple(ranges, at),
        }
    }

    pub fn merge(&self, other: &Self) -> Self {
        if self.is_empty() && other.is_empty() {
            return Self::Empty;
        } else if self.is_empty() {
            return other.clone();
        } else if other.is_empty() {
            return self.clone();
        }

        let mut steps = Vec::new();
        self.append_merge_steps(&mut steps);
        other.append_merge_steps(&mut steps);
        steps.sort_unstable_by_key(|(n, _)| n.clone());

        let mut ranges = Vec::new();
        let mut counter = 0usize;
        let mut start = None;

        for (n, group) in steps.into_iter().group_by(|(n, _)| n.clone()).into_iter() {
            for (_, step) in group {
                match step {
                    MergeStep::Start => {
                        counter += 1;
                    }
                    MergeStep::End => {
                        counter -= 1;
                    }
                }
            }

            if counter > 0 && start.is_none() {
                start = Some(n.clone());
            } else if counter == 0 {
                if let Some(start) = start.as_ref() {
                    ranges.push(start.clone()..n.clone());
                }

                start = None;
            }
        }

        ranges.into()
    }

    fn split_single(range: &Range<N>, at: N) -> (Self, Self) {
        if at <= range.start {
            (Self::Empty, Self::Single(range.clone()))
        } else if at >= range.end {
            (Self::Single(range.clone()), Self::Empty)
        } else {
            (
                Self::Single(range.start.clone()..at.clone()),
                Self::Single(at..range.end.clone()),
            )
        }
    }

    fn split_multiple(ranges: &[Range<N>], at: N) -> (Self, Self) {
        if at <= ranges[0].start {
            return (Self::Empty, Self::Multiple(Vec::from(ranges)));
        } else if at >= ranges[ranges.len()].end {
            return (Self::Multiple(Vec::from(ranges)), Self::Empty);
        }

        let mut lt = vec![];
        let mut gt = vec![];
        for range in ranges.iter() {
            if at <= range.start {
                gt.push(range.clone());
            } else if at >= range.end {
                lt.push(range.clone());
            } else {
                lt.push(range.start.clone()..at.clone());
                gt.push(at.clone()..range.end.clone());
            }
        }

        (lt.into(), gt.into())
    }

    fn append_merge_steps(&self, steps: &mut Vec<(N, MergeStep)>) {
        match self {
            Self::Empty => {}
            Self::Single(range) => {
                steps.reserve(2);
                steps.push((range.start.clone(), MergeStep::Start));
                steps.push((range.end.clone(), MergeStep::End));
            }
            Self::Multiple(ranges) => {
                steps.reserve(ranges.len() * 2);
                for range in ranges.iter() {
                    steps.push((range.start.clone(), MergeStep::Start));
                    steps.push((range.end.clone(), MergeStep::End));
                }
            }
        }
    }
}

impl<N> From<Range<N>> for MultiRangeInternal<N>
where
    N: Eq + Ord + Clone,
{
    fn from(value: Range<N>) -> Self {
        if value.is_empty() {
            Self::Empty
        } else {
            Self::Single(value)
        }
    }
}

impl<N> From<Vec<Range<N>>> for MultiRangeInternal<N>
where
    N: Eq + Ord + Clone,
{
    fn from(value: Vec<Range<N>>) -> Self {
        if value.is_empty() {
            Self::Empty
        } else if value.len() == 1 {
            Self::Single(value[0].clone())
        } else {
            Self::Multiple(value)
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum MergeStep {
    Start,
    End,
}

#[cfg(test)]
mod tests {}
