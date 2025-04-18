use nom::{
    branch::alt,
    bytes::{
        complete::{is_not, tag, take_until, take_while},
        take,
    },
    character::complete::i64,
    combinator::map,
    sequence::{preceded, terminated},
    IResult, Parser,
};
use std::fmt;

#[derive(Debug, Clone)]
pub enum Resp {
    SimpleString(String),
    SimpleError(String),
    Integer(i64),
    BulkString(Option<String>),
    Array(Vec<Resp>),
}

pub fn parse_simple_string(input: &str) -> IResult<&str, Resp> {
    map(preceded(tag("+"), is_not("\r\n")), |s: &str| {
        Resp::SimpleString(s.to_string())
    })
    .parse(input)
}

pub fn parse_simple_error(input: &str) -> IResult<&str, Resp> {
    map(preceded(tag("-"), is_not("\r\n")), |s: &str| {
        Resp::SimpleError(s.to_string())
    })
    .parse(input)
}

pub fn parse_integer(input: &str) -> IResult<&str, Resp> {
    map(preceded(tag(":"), terminated(i64, tag("\r\n"))), |s| {
        Resp::Integer(s)
    })
    .parse(input)
}

pub fn parse_array(input: &str) -> IResult<&str, Resp> {
    let (input, _) = tag("*").parse(input)?;
    let (input, length_str) = take_until("\r\n").parse(input)?;
    let length: i32 = length_str.parse().unwrap_or(0);
    let (input, _) = tag("\r\n").parse(input)?;
    let mut remaining_input = input;
    let mut v: Vec<Resp> = Vec::new();
    for _ in 0..length {
        let (input, current) = parse_resp(remaining_input)?;
        remaining_input = input;
        v.push(current);
    }
    Ok((remaining_input, Resp::Array(v)))
}

pub fn parse_bulk_string(input: &str) -> IResult<&str, Resp> {
    let (input, _) = tag("$")(input)?;
    let (input, length_str) = take_until("\r\n")(input)?;
    let length: i64 = length_str.parse().unwrap_or(0);
    let (input, _) = tag("\r\n")(input)?;

    if length == -1 {
        return Ok((input, Resp::BulkString(None)));
    }
    let (input, content) = take(length as usize).parse(input)?;

    let (input, _) = tag("\r\n")(input)?;

    Ok((input, Resp::BulkString(Some(content.to_string()))))
}

pub fn parse_resp(input: &str) -> IResult<&str, Resp> {
    let (input, _) = take_while(|c| c == ' ' || c == '\r' || c == '\n').parse(input)?;

    let (input, result) = alt((
        parse_simple_string,
        parse_simple_error,
        parse_integer,
        parse_bulk_string,
        parse_array,
    ))
    .parse(input)?;

    Ok((input, result))
}

impl fmt::Display for Resp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Resp::SimpleString(s) => write!(f, "+{}\r\n", s),
            Resp::SimpleError(s) => write!(f, "-{}\r\n", s),
            Resp::Integer(i) => write!(f, ":{}\r\n", i),
            Resp::BulkString(Some(s)) => write!(f, "${}\r\n{}\r\n", s.len(), s),
            Resp::BulkString(None) => write!(f, "$-1\r\n"),
            Resp::Array(arr) => {
                write!(f, "*{}\r\n", arr.len())?;
                for item in arr {
                    write!(f, "{}", item)?;
                }
                Ok(())
            }
        }
    }
}
