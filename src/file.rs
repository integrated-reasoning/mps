use color_eyre::{eyre::eyre, Result};
use nom::{
  bytes::complete::tag,
  character::complete::*,
  combinator::{map, map_res, opt, peek},
  multi::{count, many1},
  number::complete::float,
  sequence::{preceded, separated_pair, terminated, tuple},
  IResult,
};
use num_traits::float::Float;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct MPSFile<'a, T: Float> {
  pub name: &'a str,
  pub rows: Rows<'a>,
  pub columns: Columns<'a, T>,
  pub rhs: RHS<'a, T>,
  pub ranges: Ranges<'a, T>,
  pub bounds: Bounds<'a, T>,
}

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

impl TryFrom<char> for RowType {
  type Error = color_eyre::Report;

  fn try_from(c: char) -> Result<Self> {
    match c {
      'E' => Ok(RowType::EQ),
      'L' => Ok(RowType::LEQ),
      'G' => Ok(RowType::GEQ),
      'N' => Ok(RowType::NR),
      _ => Err(eyre!("invalid row type")),
    }
  }
}

pub type Rows<'a> = Vec<RowLine<'a>>;

pub type Columns<'a, T> = Vec<ColumnLine<'a, T>>;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct RowCoefficientPair<'a, T> {
  pub row_name: &'a str,
  pub coefficient: T,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct ColumnLine<'a, T> {
  pub column_name: &'a str,
  pub first_pair: RowCoefficientPair<'a, T>,
  pub second_pair: Option<RowCoefficientPair<'a, T>>,
}

pub type RHS<'a, T> = Vec<RHSLine<'a, T>>;

pub type RHSLine<'a, T> = ColumnLine<'a, T>;

pub type Ranges<'a, T> = Vec<RangesLine<'a, T>>;

pub type RangesLine<'a, T> = ColumnLine<'a, T>;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct BoundsLine<'a, T> {
  pub bound_type: BoundType,
  pub bound_name: &'a str,
  pub column_name: &'a str,
  pub value: T,
}

pub type Bounds<'a, T> = Vec<BoundsLine<'a, T>>;

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

  pub fn row(i: &str) -> IResult<&str, RowLine<'_>> {
    map_res(
      preceded(
        tag(" "),
        terminated(
          separated_pair(one_of("ELGN"), multispace1, alphanumeric1),
          newline,
        ),
      ),
      |(t, n)| -> Result<RowLine> {
        Ok(RowLine {
          row_type: RowType::try_from(t)?,
          row_name: n,
        })
      },
    )(i)
  }

  pub fn rows(i: &str) -> IResult<&str, Vec<RowLine<'_>>> {
    terminated(
      preceded(terminated(tag("ROWS"), newline), many1(Self::row)),
      peek(anychar),
    )(i)
  }

  pub fn column(i: &str) -> IResult<&str, ColumnLine<'_, f32>> {
    map(
      preceded(
        tag("    "),
        terminated(
          tuple((
            terminated(alphanumeric1, multispace1),
            terminated(alphanumeric1, multispace1),
            float,
            opt(preceded(multispace1, tuple((terminated(alphanumeric1, multispace1), float)))),
          )),
          newline,
        ),
      ),
      |(column_name, first_row_name, first_coefficient, opt)| -> ColumnLine<f32> {
         let first_pair = RowCoefficientPair { row_name: first_row_name, coefficient: first_coefficient };
         let second_pair = opt.map(|(row_name, coefficient)| RowCoefficientPair { row_name, coefficient });
         ColumnLine::<f32> { column_name, first_pair, second_pair }
      })(i)
  }

  pub fn columns(i: &str) -> IResult<&str, Vec<ColumnLine<f32>>> {
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
  fn test_row() -> Result<()> {
    let a = " E  R09\n";
    let b = " E  R10\n";
    let c = " L  X05\n";
    let d = " L  X21\n";
    assert_eq!(
      MPSFile::<f32>::row(a),
      Ok((
        "",
        RowLine {
          row_type: RowType::try_from('E')?,
          row_name: "R09"
        }
      ))
    );
    assert_eq!(
      MPSFile::<f32>::row(b),
      Ok((
        "",
        RowLine {
          row_type: RowType::try_from('E')?,
          row_name: "R10"
        }
      ))
    );
    assert_eq!(
      MPSFile::<f32>::row(c),
      Ok((
        "",
        RowLine {
          row_type: RowType::try_from('L')?,
          row_name: "X05"
        }
      ))
    );
    assert_eq!(
      MPSFile::<f32>::row(d),
      Ok((
        "",
        RowLine {
          row_type: RowType::try_from('L')?,
          row_name: "X21"
        }
      ))
    );
    Ok(())
  }

  #[test]
  fn test_rows() -> Result<()> {
    let a = "ROWS\n E  R09\n E  R10\n L  X05\n L  X21\nCOLUMNS";
    assert_eq!(
      MPSFile::<f32>::rows(a),
      Ok((
        "COLUMNS",
        vec![
          RowLine {
            row_type: RowType::try_from('E')?,
            row_name: "R09"
          },
          RowLine {
            row_type: RowType::try_from('E')?,
            row_name: "R10"
          },
          RowLine {
            row_type: RowType::try_from('L')?,
            row_name: "X05"
          },
          RowLine {
            row_type: RowType::try_from('L')?,
            row_name: "X21"
          },
        ]
      ))
    );
    Ok(())
  }

  #[test]
  fn test_column() {
    let a = "    X01       X48               .301   R09                -1.\n";
    let b = "    X02       COST               -.4\n";
    assert_eq!(
      MPSFile::<f32>::column(a),
      Ok((
        "",
        ColumnLine::<f32> {
          column_name: "X01",
          first_pair: RowCoefficientPair {
            row_name: "X48",
            coefficient: 0.301
          },
          second_pair: Some(RowCoefficientPair {
            row_name: "R09",
            coefficient: -1.0
          })
        }
      ))
    );
    assert_eq!(
      MPSFile::<f32>::column(b),
      Ok((
        "",
        ColumnLine::<f32> {
          column_name: "X02",
          first_pair: RowCoefficientPair {
            row_name: "COST",
            coefficient: -0.4
          },
          second_pair: None
        }
      ))
    );
  }

  #[test]
  fn test_columns() {
    let a = "COLUMNS
    X01       X48               .301   R09                -1.
    X01       R10              -1.06   X05                 1.
    X02       X21                -1.   R09                 1.
    X02       COST               -.4
    X03       X46                -1.   R09                 1.\nRHS";
    assert_eq!(
      MPSFile::<f32>::columns(a),
      Ok((
        "RHS",
        vec![
          ColumnLine::<f32> {
            column_name: "X01",
            first_pair: RowCoefficientPair {
              row_name: "X48",
              coefficient: 0.301
            },
            second_pair: Some(RowCoefficientPair {
              row_name: "R09",
              coefficient: -1.0
            })
          },
          ColumnLine::<f32> {
            column_name: "X01",
            first_pair: RowCoefficientPair {
              row_name: "R10",
              coefficient: -1.06
            },
            second_pair: Some(RowCoefficientPair {
              row_name: "X05",
              coefficient: 1.0
            })
          },
          ColumnLine::<f32> {
            column_name: "X02",
            first_pair: RowCoefficientPair {
              row_name: "X21",
              coefficient: -1.0
            },
            second_pair: Some(RowCoefficientPair {
              row_name: "R09",
              coefficient: 1.0
            })
          },
          ColumnLine::<f32> {
            column_name: "X02",
            first_pair: RowCoefficientPair {
              row_name: "COST",
              coefficient: -0.4
            },
            second_pair: None
          },
          ColumnLine::<f32> {
            column_name: "X03",
            first_pair: RowCoefficientPair {
              row_name: "X46",
              coefficient: -1.0
            },
            second_pair: Some(RowCoefficientPair {
              row_name: "R09",
              coefficient: 1.0
            })
          },
        ]
      ))
    );
  }
}
