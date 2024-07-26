
use std::borrow::Cow;

use nom::character::complete::{one_of};
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
        '-' => todo!(),
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
    let (data, val) = nom::bytes::complete::take_until("\r\n")(data)?;
    let (data, _) = nom::bytes::complete::tag("\r\n")(data)?;
    Ok((data, RespType::simple_string(val)))
}
