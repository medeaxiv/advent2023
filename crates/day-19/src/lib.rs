#![allow(dead_code)]

use ahash::AHashMap as HashMap;
use aoc_util::range::MultiRange;
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
    let _workflows = Workflows::from_iter(workflows);

    let mut _accepted_ranges = Ranges::empty();

    let x = 4000;
    let m = 4000;
    let a = 4000;
    let s = 4000;

    x * m * a * s
}

struct Workflows<'a> {
    graph: HashMap<&'a str, Workflow<'a>>,
}

impl<'a> Workflows<'a> {
    pub fn get(&self, name: &str) -> Option<&Workflow> {
        self.graph.get(name)
    }

    pub fn accepts(&self, part: &Part) -> bool {
        let mut position = Destination::Workflow("in");

        loop {
            match position {
                Destination::Accepted => {
                    return true;
                }
                Destination::Rejected => {
                    return false;
                }
                Destination::Workflow(workflow) => {
                    position = self
                        .graph
                        .get(workflow)
                        .map(|workflow| workflow.process(part))
                        .unwrap_or(Destination::Rejected)
                }
            }
        }
    }
}

impl<'a> FromIterator<Workflow<'a>> for Workflows<'a> {
    fn from_iter<T: IntoIterator<Item = Workflow<'a>>>(iter: T) -> Self {
        let graph = HashMap::from_iter(iter.into_iter().map(|workflow| (workflow.name, workflow)));

        Self { graph }
    }
}

#[derive(Debug)]
struct Workflow<'a> {
    name: &'a str,
    filters: Vec<(Filter, Destination<'a>)>,
    fallback: Destination<'a>,
}

impl<'a> Workflow<'a> {
    pub fn process(&self, part: &Part) -> Destination {
        for (filter, destination) in self.filters.iter() {
            if filter.process(part) {
                return *destination;
            }
        }

        self.fallback
    }
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

#[derive(Debug)]
struct Filter {
    category: Category,
    condition: Condition,
}

impl Filter {
    pub fn process(&self, part: &Part) -> bool {
        self.condition.check(part.get(self.category))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Category {
    X,
    M,
    A,
    S,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Condition {
    GreaterThan(u64),
    LessThan(u64),
}

impl Condition {
    pub fn check(&self, value: u64) -> bool {
        match *self {
            Self::GreaterThan(target) => value > target,
            Self::LessThan(target) => value < target,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Destination<'a> {
    Accepted,
    Rejected,
    Workflow(&'a str),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Ranges {
    x: MultiRange<u64>,
    m: MultiRange<u64>,
    a: MultiRange<u64>,
    s: MultiRange<u64>,
}

impl Ranges {
    pub fn new(min: u64, max: u64) -> Self {
        let max = max + 1;
        Self {
            x: (min..max).into(),
            m: (min..max).into(),
            a: (min..max).into(),
            s: (min..max).into(),
        }
    }

    pub fn empty() -> Self {
        Self {
            x: MultiRange::empty(),
            m: MultiRange::empty(),
            a: MultiRange::empty(),
            s: MultiRange::empty(),
        }
    }

    pub fn split(&self, filter: &Filter) -> (Self, Self) {
        let at = match filter.condition {
            Condition::GreaterThan(target) => target + 1,
            Condition::LessThan(target) => target,
        };

        let (lt, gt) = self.get(filter.category).split(at);

        let mut a = self.clone();
        a.set(filter.category, lt);

        let mut b = self.clone();
        b.set(filter.category, gt);

        match filter.condition {
            Condition::GreaterThan(..) => (b, a),
            Condition::LessThan(..) => (a, b),
        }
    }

    pub fn merge(&self, other: &Self) -> Self {
        Self {
            x: self.x.merge(&other.x),
            m: self.m.merge(&other.m),
            a: self.a.merge(&other.a),
            s: self.s.merge(&other.s),
        }
    }

    fn get(&self, category: Category) -> &MultiRange<u64> {
        match category {
            Category::X => &self.x,
            Category::M => &self.m,
            Category::A => &self.a,
            Category::S => &self.s,
        }
    }

    fn set(&mut self, category: Category, value: MultiRange<u64>) {
        match category {
            Category::X => {
                self.x = value;
            }
            Category::M => {
                self.m = value;
            }
            Category::A => {
                self.a = value;
            }
            Category::S => {
                self.s = value;
            }
        }
    }
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
