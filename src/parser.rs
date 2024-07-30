use core::str;

use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::{crlf, i64, not_line_ending};
use nom::character::streaming::char;
use nom::combinator::map;
use nom::error::ErrorKind;
use nom::multi::many_m_n;
use nom::number::complete::double;
use nom::sequence::{delimited, terminated};
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
    Double(String),
    BigNumber(i128),
    Pushes,
}
pub fn parse(data: &str) -> IResult<&str, RespType> {
    alt((
        parse_null,
        parse_simple_string,
        parse_simple_error,
        parse_integer,
        parse_bulk_string,
        parse_array,
        parse_boolean,
        parse_double,
    ))(data)
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

fn parse_bulk_string(data: &str) -> IResult<&str, RespType> {
    let (data, len) = delimited(char('$'), i64, crlf)(data)?;
    Ok(match len {
        -1 => (data, RespType::Null),
        0.. => map(terminated(take(len as usize), crlf), |s: &str| {
            RespType::BulkString(s)
        })(data)?,
        _ => {
            return Err(nom::Err::Failure(nom::error::Error::new(
                data,
                ErrorKind::Verify,
            )))
        }
    })
}

fn parse_array(data: &str) -> IResult<&str, RespType> {
    let (data, len) = delimited(char('*'), i64, crlf)(data)?;
    let (data, elements) = many_m_n(len.try_into().unwrap(), len.try_into().unwrap(), parse)(data)?;
    Ok((data, RespType::Array(elements)))
}

fn parse_null(data: &str) -> IResult<&str, RespType> {
    let (data, _) = delimited(char('_'), not_line_ending, crlf)(data)?;
    Ok((data, RespType::Null))
}

fn parse_boolean(data: &str) -> IResult<&str, RespType> {
    map(
        delimited(char('#'), alt((tag("t"), tag("f"))), crlf),
        |s: &str| RespType::Boolean(s == "t"),
    )(data)
}

fn parse_double(data: &str) -> IResult<&str, RespType> {
    let (data, value) = delimited(char(','), double, crlf)(data)?;
    Ok((data, RespType::Double(format!("{}", value))))
}
