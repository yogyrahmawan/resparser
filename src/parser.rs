use core::str;

use nom::branch::alt;
use nom::character::complete::{crlf, i64, not_line_ending};
use nom::character::streaming::char;
use nom::combinator::map;
use nom::sequence::delimited;
use nom::IResult;

#[derive(PartialEq, Clone, Debug, Hash)]
pub enum RespType<'a> {
    SimpleString(&'a str),
    Error(&'a str),
    BulkError(&'a str),
    Integer(i64),
    BulkString(&'a str),
    Array(Vec<RespType<'a>>),
    Null,
    Boolean(bool),
    Double(&'a str),
    BigNumber(i128),
    Pushes,
}
pub fn parse(data: &str) -> IResult<&str, RespType> {
    alt((parse_simple_string, parse_simple_error, parse_integer))(data)
}

fn parse_simple_string(data: &str) -> IResult<&str, RespType> {
    map(delimited(char('+'), not_line_ending, crlf), |s: &str| {
        RespType::SimpleString(s)
    })(data)
}

fn parse_simple_error(data: &str) -> IResult<&str, RespType> {
    map(delimited(char('-'), not_line_ending, crlf), |s: &str| {
        RespType::Error(s)
    })(data)
}

fn parse_integer(data: &str) -> IResult<&str, RespType> {
    map(delimited(char(':'), i64, crlf), |s: i64| {
        RespType::Integer(s)
    })(data)
}
