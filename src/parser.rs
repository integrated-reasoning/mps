use crate::file;
use nom::branch::alt;
use nom::bytes::streaming::{tag, take_till1};
use nom::character::streaming::*;
use nom::combinator::{opt, peek};
use nom::error::ParseError;
use nom::multi::{count, many1};
use nom::number::streaming::{f32, float};
use nom::sequence::{preceded, separated_pair, terminated, tuple};
use nom::IResult;
use nom::{AsChar, Compare, InputIter, InputLength, Slice};
use std::ops::{Range, RangeFrom, RangeTo};

pub fn name(i: &str) -> IResult<&str, &str> {
  terminated(
    preceded(tag("NAME"), preceded(count(anychar, 10), not_line_ending)),
    newline,
  )(i)
}

pub fn row(i: &str) -> IResult<&str, (char, &str)> {
  preceded(
    tag(" "),
    terminated(
      separated_pair(one_of("ELGN"), multispace1, alphanumeric1),
      newline,
    ),
  )(i)
}

pub fn rows(i: &str) -> IResult<&str, Vec<(char, &str)>> {
  terminated(
    preceded(terminated(tag("ROWS"), newline), many1(row)),
    peek(anychar),
  )(i)
}

pub fn column(i: &str) -> IResult<&str, (&str, &str, f32, &str, f32)> {
  preceded(
    tag("    "),
    terminated(
      tuple((
        terminated(alphanumeric1, multispace1),
        terminated(alphanumeric1, multispace1),
        terminated(float, multispace1),
        terminated(alphanumeric1, multispace1),
        float,
      )),
      newline,
    ),
  )(i)
}

pub fn columns(i: &str) -> IResult<&str, Vec<(&str, &str, f32, &str, f32)>> {
  terminated(
    preceded(terminated(tag("COLUMNS"), newline), many1(column)),
    peek(anychar),
  )(i)
}

#[cfg(feature = "proptest")]
#[cfg(test)]
mod proptests {
  use super::*;
  use proptest::prelude::*;

  proptest! {
    #[test]
    fn test_name_doesnt_crash(s in "\\PC*") {
        let _ = name(&s);
    }
  }

  proptest! {
    #[test]
    fn test_row_doesnt_crash(s in "\\PC*") {
        let _ = row(&s);
    }
  }

  proptest! {
    #[test]
    fn test_rows_doesnt_crash(s in "\\PC*") {
        let _ = rows(&s);
    }
  }

  proptest! {
    #[test]
    fn test_column_doesnt_crash(s in "\\PC*") {
        let _ = column(&s);
    }
  }
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
        "COLUMNS",
        vec![('E', "R09"), ('E', "R10"), ('L', "X05"), ('L', "X21")]
      ))
    );
  }

  #[test]
  fn test_column() {
    let a = "    X01       X48               .301   R09                -1.\n";
    //let b = "    X02       COST               -.4\n";
    assert_eq!(column(a), Ok(("", ("X01", "X48", 0.301, "R09", -1.0))));
    //assert_eq!(column(b), Ok(("", ("X01", "COST", -4))));
  }

  #[test]
  fn test_columns() {
    let a = "COLUMNS
    X01       X48               .301   R09                -1.
    X01       R10              -1.06   X05                 1.
    X02       X21                -1.   R09                 1.
    X03       X46                -1.   R09                 1.\nRHS";
    let b = "COLUMNS
    X01       X48               .301   R09                -1.
    X01       R10              -1.06   X05                 1.
    X02       X21                -1.   R09                 1.
    X02       COST               -.4
    X03       X46                -1.   R09                 1.\nRHS"; // TODO
    assert_eq!(
      columns(a),
      Ok((
        "RHS",
        vec![
          ("X01", "X48", 0.301, "R09", -1.0),
          ("X01", "R10", -1.06, "X05", 1.0),
          ("X02", "X21", -1.0, "R09", 1.0),
          ("X03", "X46", -1.0, "R09", 1.0),
        ]
      ))
    );
  }
}
