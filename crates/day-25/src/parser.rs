use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space1},
    multi::{many1, separated_list1},
    sequence::{pair, separated_pair, terminated},
    IResult,
};
use nom_supreme::final_parser::final_parser;

use crate::Graph;

pub fn parse(input: &str) -> Result<Graph, nom::error::Error<&str>> {
    let adjacency = final_parser(parser)(input)?;

    Ok(Graph::from_iter(adjacency))
}

fn parser(input: &str) -> IResult<&str, Vec<(&str, Vec<&str>)>> {
    many1(terminated(line_parser, line_ending))(input)
}

fn line_parser(input: &str) -> IResult<&str, (&str, Vec<&str>)> {
    separated_pair(name_parser, pair(tag(":"), space1), names_parser)(input)
}

fn names_parser(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(space1, name_parser)(input)
}

fn name_parser(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}
