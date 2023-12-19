#![allow(clippy::type_complexity)]

use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending, one_of},
    combinator::{map, rest},
    multi::{many1, separated_list1},
    sequence::{delimited, pair, preceded, separated_pair, terminated},
    IResult,
};
use nom_supreme::final_parser::final_parser;

use crate::{Category, Condition, Destination, Filter, Part, Workflow};

pub fn parse(input: &str) -> Result<(Vec<Workflow>, Vec<Part>), nom::error::Error<&str>> {
    final_parser(parser)(input)
}

pub fn parse_workflows(input: &str) -> Result<Vec<Workflow>, nom::error::Error<&str>> {
    final_parser(terminated(workflows_parser, rest))(input)
}

fn parser(input: &str) -> IResult<&str, (Vec<Workflow>, Vec<Part>)> {
    separated_pair(workflows_parser, line_ending, parts_parser)(input)
}

fn workflows_parser(input: &str) -> IResult<&str, Vec<Workflow>> {
    many1(terminated(workflow_parser, line_ending))(input)
}

fn workflow_parser(input: &str) -> IResult<&str, Workflow> {
    map(
        pair(
            alpha1,
            delimited(tag("{"), workflow_definition_parser, tag("}")),
        ),
        |(name, (filters, fallback))| Workflow {
            name,
            filters,
            fallback,
        },
    )(input)
}

fn workflow_definition_parser(
    input: &str,
) -> IResult<&str, (Vec<(Filter, Destination)>, Destination)> {
    separated_pair(workflow_filters_parser, tag(","), destination_parser)(input)
}

fn workflow_filters_parser(input: &str) -> IResult<&str, Vec<(Filter, Destination)>> {
    separated_list1(tag(","), workflow_filter_parser)(input)
}

fn workflow_filter_parser(input: &str) -> IResult<&str, (Filter, Destination)> {
    separated_pair(filter_parser, tag(":"), destination_parser)(input)
}

fn filter_parser(input: &str) -> IResult<&str, Filter> {
    map(
        pair(category_parser, condition_parser),
        |(category, condition)| Filter {
            category,
            condition,
        },
    )(input)
}

fn category_parser(input: &str) -> IResult<&str, Category> {
    map(one_of("xmas"), |c| match c {
        'x' => Category::X,
        'm' => Category::M,
        'a' => Category::A,
        's' => Category::S,
        _ => unreachable!(),
    })(input)
}

fn condition_parser(input: &str) -> IResult<&str, Condition> {
    map(pair(one_of("><"), complete::u64), |(op, target)| match op {
        '>' => Condition::GreaterThan(target),
        '<' => Condition::LessThan(target),
        _ => unreachable!(),
    })(input)
}

fn destination_parser(input: &str) -> IResult<&str, Destination> {
    map(alpha1, |s| match s {
        "A" => Destination::Terminal(true),
        "R" => Destination::Terminal(false),
        s => Destination::Workflow(s),
    })(input)
}

fn parts_parser(input: &str) -> IResult<&str, Vec<Part>> {
    many1(terminated(part_parser, line_ending))(input)
}

fn part_parser(input: &str) -> IResult<&str, Part> {
    delimited(tag("{"), part_fields_parser, tag("}"))(input)
}

fn part_fields_parser(input: &str) -> IResult<&str, Part> {
    let (input, x) = part_field_parser("x")(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, m) = part_field_parser("m")(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, a) = part_field_parser("a")(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, s) = part_field_parser("s")(input)?;

    let part = Part { x, m, a, s };
    Ok((input, part))
}

fn part_field_parser(field: &'static str) -> impl FnMut(&str) -> IResult<&str, u64> {
    move |input: &str| preceded(pair(tag(field), tag("=")), complete::u64)(input)
}
