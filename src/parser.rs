use crate::file;

use nom::branch::alt;
use nom::bytes::streaming::{tag, take_till1};
use nom::character::streaming::*;
use nom::combinator::all_consuming;
use nom::error::ParseError;
use nom::multi::{count, many1};
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use nom::{AsChar, Compare, InputIter, InputLength, Slice};
use std::ops::{Range, RangeFrom, RangeTo};

use nom::IResult;

fn name(i: &str) -> IResult<&str, &str> {
  terminated(
    preceded(tag("NAME"), preceded(count(anychar, 10), not_line_ending)),
    newline,
  )(i)
}

fn row(i: &str) -> IResult<&str, (char, &str)> {
  preceded(
    tag(" "),
    terminated(
      separated_pair(one_of("ELGN"), multispace1, alphanumeric1),
      newline,
    ),
  )(i)
}

fn rows(i: &str) -> IResult<&str, Vec<(char, &str)>> {
  terminated(
    preceded(terminated(tag("ROWS"), newline), many1(row)),
    tag("COLUMNS"),
  )(i)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_name() {
    let a = "NAME          AFIRO\n";
    assert_eq!(name(a), Ok(("", "AFIRO")));
  }

  #[test]
  fn test_row() {
    let a = " E  R09\n";
    let b = " E  R10\n";
    let c = " L  X05\n";
    let d = " L  X21\n";
    assert_eq!(row(a), Ok(("", ('E', "R09"))));
    assert_eq!(row(b), Ok(("", ('E', "R10"))));
    assert_eq!(row(c), Ok(("", ('L', "X05"))));
    assert_eq!(row(d), Ok(("", ('L', "X21"))));
  }

  #[test]
  fn test_rows() {
    let a = "ROWS\n E  R09\n E  R10\n L  X05\n L  X21\nCOLUMNS";
    assert_eq!(
      rows(a),
      Ok((
        "",
        vec![('E', "R09"), ('E', "R10"), ('L', "X05"), ('L', "X21")]
      ))
    );
  }
}
