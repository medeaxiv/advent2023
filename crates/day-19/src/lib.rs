use ahash::AHashMap as HashMap;
use aoc_util::{
    range::MultiRange,
    tree::kdtree::{DimensionCollection, KdTree, KdTreeBuilderNode},
};
use rayon::prelude::*;

mod parser;

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(input)
}

fn solve_part1(input: &str) -> u64 {
    let (workflows, parts) = parser::parse(input).unwrap();
    let workflows = Workflows::from_iter(workflows);

    parts
        .into_par_iter()
        .filter(|part| workflows.accepts(part))
        .map(|part| part.x + part.m + part.a + part.s)
        .sum()
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(input)
}

fn solve_part2(input: &str) -> u64 {
    let workflows = parser::parse_workflows(input).unwrap();
    let workflows = Workflows::from_iter(workflows);

    let mut count = 0;
    let _: Option<()> = workflows.tree.traverse(|&accepted, ranges| {
        if !accepted {
            return None;
        }

        count += Category::ALL
            .iter()
            .map(|category| {
                ranges
                    .iter()
                    .find(|(c, _)| *c == category)
                    .map(|(_, range)| range.end.unwrap_or(4001) - range.start.unwrap_or(1))
                    .unwrap_or(4000)
            })
            .product::<u64>();

        None
    });

    count
}

struct Workflows {
    tree: KdTree<Category, u64, bool>,
}

impl Workflows {
    pub fn accepts(&self, part: &Part) -> bool {
        *self.tree.find(part)
    }
}

impl<'a> FromIterator<Workflow<'a>> for Workflows {
    fn from_iter<T: IntoIterator<Item = Workflow<'a>>>(iter: T) -> Self {
        let graph = HashMap::from_iter(iter.into_iter().map(|workflow| (workflow.name, workflow)));

        let tree = KdTree::build(
            |p| match p.0 {
                Destination::Terminal(accepted) => KdTreeBuilderNode::Leaf(accepted),
                Destination::Workflow(name) => graph
                    .get(name)
                    .map(|workflow| {
                        let (filter, destination) = workflow.filters[p.1];

                        let branch = (destination, 0);
                        let next = if p.1 + 1 >= workflow.filters.len() {
                            (workflow.fallback, 0)
                        } else {
                            (p.0, p.1 + 1)
                        };

                        let (lesser, greater) = match filter.condition {
                            Condition::GreaterThan(..) => (next, branch),
                            Condition::LessThan(..) => (branch, next),
                        };

                        KdTreeBuilderNode::Split {
                            dimension: filter.category,
                            boundary: filter.condition.boundary(),
                            lesser,
                            greater,
                        }
                    })
                    .unwrap_or(KdTreeBuilderNode::Leaf(false)),
            },
            (Destination::Workflow("in"), 0),
        );

        Self { tree }
    }
}

#[derive(Debug)]
struct Workflow<'a> {
    name: &'a str,
    filters: Vec<(Filter, Destination<'a>)>,
    fallback: Destination<'a>,
}

#[derive(Debug)]
struct Part {
    x: u64,
    m: u64,
    a: u64,
    s: u64,
}

impl Part {
    pub fn get(&self, category: Category) -> u64 {
        match category {
            Category::X => self.x,
            Category::M => self.m,
            Category::A => self.a,
            Category::S => self.s,
        }
    }
}

impl DimensionCollection<Category, u64> for Part {
    fn get_dimension(&self, dimension: &Category) -> u64 {
        self.get(*dimension)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Filter {
    category: Category,
    condition: Condition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    pub const ALL: [Self; 4] = [Self::X, Self::M, Self::A, Self::S];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Condition {
    GreaterThan(u64),
    LessThan(u64),
}

impl Condition {
    pub fn boundary(&self) -> u64 {
        match *self {
            Self::GreaterThan(target) => target + 1,
            Self::LessThan(target) => target,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Destination<'a> {
    Terminal(bool),
    Workflow(&'a str),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Ranges {
    x: MultiRange<u64>,
    m: MultiRange<u64>,
    a: MultiRange<u64>,
    s: MultiRange<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}
";

    #[test]
    fn test_part1() {
        let solution = solve_part1(TEST_INPUT);
        assert_eq!(solution, 19114);
    }

    #[test]
    fn test_part2() {
        let solution = solve_part2(TEST_INPUT);
        assert_eq!(solution, 167409079868000);
    }
}
