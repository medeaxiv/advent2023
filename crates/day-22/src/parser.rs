use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    combinator::map,
    multi::many1,
    sequence::{separated_pair, terminated},
    IResult,
};
use nom_supreme::final_parser::final_parser;

use crate::{Brick, Position};

pub fn parse(input: &str) -> Result<Vec<Brick>, nom::error::Error<&str>> {
    final_parser(parser)(input)
}

fn parser(input: &str) -> IResult<&str, Vec<Brick>> {
    many1(terminated(brick_parser, line_ending))(input)
}

fn brick_parser(input: &str) -> IResult<&str, Brick> {
    map(
        separated_pair(position_parser, tag("~"), position_parser),
        |(a, b)| Brick::new(a, b),
    )(input)
}

fn position_parser(input: &str) -> IResult<&str, Position> {
    let (input, x) = complete::i64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, y) = complete::i64(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, z) = complete::i64(input)?;

    Ok((input, Position::new(x, y, z)))
}
