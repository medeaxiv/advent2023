use crate::{Module, ModuleType};

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, one_of},
    combinator::{map, opt},
    multi::{many1, separated_list1},
    sequence::{pair, separated_pair, terminated},
    IResult,
};
use nom_supreme::final_parser::final_parser;

pub fn parse(input: &str) -> Result<Vec<Module>, nom::error::Error<&str>> {
    final_parser(parser)(input)
}

fn parser(input: &str) -> IResult<&str, Vec<Module>> {
    many1(terminated(node_parser, line_ending))(input)
}

fn node_parser(input: &str) -> IResult<&str, Module> {
    map(
        separated_pair(module_parser, tag(" -> "), destinations_parser),
        |((module_type, name), destinations)| Module {
            name,
            module_type,
            destinations,
        },
    )(input)
}

fn module_parser(input: &str) -> IResult<&str, (ModuleType, &str)> {
    pair(module_type_parser, alpha1)(input)
}

fn module_type_parser(input: &str) -> IResult<&str, ModuleType> {
    map(opt(one_of("%&")), |c| match c {
        Some('%') => ModuleType::FlipFlop,
        Some('&') => ModuleType::Conjunction(0),
        None => ModuleType::None,
        _ => unreachable!(),
    })(input)
}

fn destinations_parser(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag(", "), alpha1)(input)
}
