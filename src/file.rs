use color_eyre::Result;
use nom::{
  bytes::complete::tag,
  character::complete::*,
  combinator::peek,
  multi::{count, many1},
  number::complete::float,
  sequence::{preceded, separated_pair, terminated, tuple},
  IResult,
};
use num_traits::float::Float;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct MPSFile<'a, T: Float> {
  pub name: Name<'a>,
  pub rows: Rows<'a>,
  pub columns: Columns<'a, T>,
  pub rhs: RHS<'a, T>,
  pub ranges: Ranges<'a, T>,
  pub bounds: Bounds<'a, T>,
}

pub type Name<'a> = &'a str;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct RowLine<'a> {
  pub row_type: RowType,
  pub row_name: &'a str,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum RowType {
  #[default]
  EQ,
  LEQ,
  GEQ,
  NR,
}

pub type Rows<'a> = Vec<RowLine<'a>>;

pub type Columns<'a, T> = Vec<ColumnLine<'a, T>>;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ColumnLine<'a, T> {
  pub column_name: &'a str,
  pub row_name: &'a str,
  pub coefficient: T,
  pub extended: Option<(&'a str, T)>,
}

pub type RHS<'a, T> = Vec<RHSLine<'a, T>>;

pub type RHSLine<'a, T> = (&'a str, &'a str, T, Option<(&'a str, T)>);

pub type Ranges<'a, T> = Vec<RangesLine<'a, T>>;

pub type RangesLine<'a, T> = (&'a str, &'a str, T, Option<(&'a str, T)>);

pub type Bounds<'a, T> = (BoundType, &'a str, &'a str, T);

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum BoundType {
  #[default]
  LO, // lower bound     :  l_j <= x_j <= inf
  UP, // upper bound     :    0 <= x_j <= u_j
  FX, // fixed variable  :  l_j == x_j == u_j
  FR, // free variable   : -inf <= x_j <= inf
  MI, // Unbounded below : -inf <= x_j <= 0
  PL, // Unbounded above :    0 <= x_j <= inf
}

/* U_i L_i Limit Table (RANGES)
 *
 * Row type | Sign of R_i | Lower limit L_i | Upper limit U_i
 * ------------------------------------------------------------
 *  LE (<=)  |   + or -    |  b_i - |R_i|    |  b_i
 *  GE (>=)  |   + or -    |  b_i            |  b_i + |R_i|
 *  EP (==)  |   +         |  b_i            |  b_i + |R_i|
 *  EM (==)  |        -    |  b_i - |R_i|    |  b_i
 *  EZ (==)  |             |  b_i            |  b_i
 *
 * Reference: Maros CTSM p.91
 * Note: CTSM doesn't mention the case where R_i == 0, but it follows that
 * both L_i and U_i should be set to the respective RHS value b_i.
 */

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum RangeType {
  #[default]
  LE,
  GE,
  EP,
  EM,
  EZ,
}

impl<'a, T: Float> MPSFile<'a, T> {
  fn parse<'mps>(mps_string: &'mps str) -> Result<MPSFile<'static, f32>> {
    //let (rest, name) = Self::name(mps_string)?;
    //let (rest, rows) = Self::rows(rest)?;
    //let (rest, columns) = Self::columns(rest)?;
    todo!()

    //Ok(MPSFile {
    //  name,
    //  //objective_sense: None,
    //  rows: rows
    //    .into_iter()
    //    .map(|(t, n)| RowLine {
    //      row_type: match t {
    //        'E' => RowType::EQ,
    //        'L' => RowType::LEQ,
    //        'G' => RowType::GEQ,
    //        'N' => RowType::NR,
    //        _ => panic!("Invalid row type"),
    //      },
    //      row_name: n,
    //    })
    //    .collect(),
    //  columns: columns
    //    .into_iter()
    //    .map(|(n, _, c, r, v)| ColumnLine {
    //      column_name: n,
    //      row_name: r,
    //      coefficient: c,
    //      optional_tuple: Some((r, v)),
    //    })
    //    .collect(),
    //  rhs: Vec::new(),
    //  ranges: Vec::new(),
    //  //bounds: Vec::new(),
    //})
  }

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
      preceded(terminated(tag("ROWS"), newline), many1(Self::row)),
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
      preceded(terminated(tag("COLUMNS"), newline), many1(Self::column)),
      peek(anychar),
    )(i)
  }
}

#[cfg(feature = "proptest")]
#[cfg(test)]
mod proptests {
  use super::*;
  use proptest::prelude::*;

  proptest! {
    #[test]
    fn test_name_doesnt_crash(s in "\\PC*") {
        let _ = MPSFile::<f32>::name(&s);
    }
  }

  proptest! {
    #[test]
    fn test_row_doesnt_crash(s in "\\PC*") {
        let _ = MPSFile::<f32>::row(&s);
    }
  }

  proptest! {
    #[test]
    fn test_rows_doesnt_crash(s in "\\PC*") {
        let _ = MPSFile::<f32>::rows(&s);
    }
  }

  proptest! {
    #[test]
    fn test_column_doesnt_crash(s in "\\PC*") {
        let _ = MPSFile::<f32>::column(&s);
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_name() {
    let a = "NAME          AFIRO\n";
    assert_eq!(MPSFile::<f32>::name(a), Ok(("", "AFIRO")));
  }

  #[test]
  fn test_row() {
    let a = " E  R09\n";
    let b = " E  R10\n";
    let c = " L  X05\n";
    let d = " L  X21\n";
    assert_eq!(MPSFile::<f32>::row(a), Ok(("", ('E', "R09"))));
    assert_eq!(MPSFile::<f32>::row(b), Ok(("", ('E', "R10"))));
    assert_eq!(MPSFile::<f32>::row(c), Ok(("", ('L', "X05"))));
    assert_eq!(MPSFile::<f32>::row(d), Ok(("", ('L', "X21"))));
  }

  #[test]
  fn test_rows() {
    let a = "ROWS\n E  R09\n E  R10\n L  X05\n L  X21\nCOLUMNS";
    assert_eq!(
      MPSFile::<f32>::rows(a),
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
    assert_eq!(
      MPSFile::<f32>::column(a),
      Ok(("", ("X01", "X48", 0.301, "R09", -1.0)))
    );
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
      MPSFile::<f32>::columns(a),
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
