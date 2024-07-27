use core::str;
use std::borrow::Cow;

use nom::character::complete::{crlf, line_ending, not_line_ending, one_of};
use nom::character::streaming::char;
use nom::combinator::map_res;
use nom::complete::tag;
use nom::sequence::delimited;
use nom::IResult;

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum RespType<'a> {
    SimpleString(Cow<'a, str>),
    Error(Cow<'a, str>),
    BulkError(Cow<'a, str>),
    Integer(i64),
    BulkString(Cow<'a, str>),
    Array(Vec<RespType<'a>>),
    Null,
    Boolean(bool),
    Double(Cow<'a, str>),
    BigNumber(i128),
    Pushes,
}

impl<'a> RespType<'a> {
    pub fn simple_string<T>(str: &'a T) -> Self
    where
        T: AsRef<str> + ?Sized,
    {
        Self::SimpleString(str.as_ref().into())
    }
}

pub fn parse(data: &str) -> IResult<&str, RespType> {
    let (data, resp_type) = one_of("+-:$*_#,(!=%~>")(data)?;
    match resp_type {
        '+' => parse_simple_string(data),
        '-' => parse_simple_error(data),
        ':' => todo!(),
        '$' => todo!(),
        '*' => todo!(),
        '_' => todo!(),
        '#' => todo!(),
        ',' => todo!(),
        '(' => todo!(),
        '!' => todo!(),
        '=' => todo!(),
        '%' => todo!(),
        '~' => todo!(),
        '>' => todo!(),
        _ => unreachable!("invalid resp_type"),
    }
}

fn parse_simple_string(data: &str) -> IResult<&str, RespType> {
    let (data, result) = map_res(
        delimited(char('+'), line_ending, crlf),
        |s: &str| -> Result<RespType, std::convert::Infallible> { Ok(RespType::simple_string(s)) },
    )(data)?;

    Ok((data, result))
}

fn parse_simple_error(data: &str) -> IResult<&str, RespType> {
    let (data, result) = map_res(
        delimited(char('-'), line_ending, crlf),
        |s: &str| -> Result<RespType, std::convert::Infallible> { Ok(RespType::simple_string(s)) },
    )(data)?;

    Ok((data, result))
}
