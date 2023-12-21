use color_eyre::{eyre::eyre, Result};
use nom::{
  branch::alt,
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
  // TODO: Check for ENDATA
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

pub type Columns<'a, T> = Vec<WideLine<'a, T>>;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct RowValuePair<'a, T> {
  pub row_name: &'a str,
  pub value: T,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct WideLine<'a, T> {
  pub name: &'a str,
  pub first_pair: RowValuePair<'a, T>,
  pub second_pair: Option<RowValuePair<'a, T>>,
}

pub type RHS<'a, T> = Vec<WideLine<'a, T>>;

pub type Ranges<'a, T> = Vec<WideLine<'a, T>>;

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

impl TryFrom<&str> for BoundType {
  type Error = color_eyre::Report;

  fn try_from(s: &str) -> Result<Self> {
    match s {
      "LO" => Ok(BoundType::LO),
      "UP" => Ok(BoundType::UP),
      "FX" => Ok(BoundType::FX),
      "FR" => Ok(BoundType::FR),
      "MI" => Ok(BoundType::MI),
      "PL" => Ok(BoundType::PL),
      "BV" => unimplemented!(),
      "LI" => unimplemented!(),
      "UI" => unimplemented!(),
      "SC" => unimplemented!(),
      _ => Err(eyre!("invalid bound type")),
    }
  }
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
    //    .map(|(n, _, c, r, v)| WideLine {
    //      column_name: n,
    //      row_name: r,
    //      value: c,
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

  pub fn row_line(i: &str) -> IResult<&str, RowLine<'_>> {
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
      preceded(terminated(tag("ROWS"), newline), many1(Self::row_line)),
      peek(anychar),
    )(i)
  }

  pub fn line(i: &str) -> IResult<&str, WideLine<'_, f32>> {
    map(
      preceded(
        tag("    "),
        terminated(
          tuple((
            terminated(alphanumeric1, multispace1),
            terminated(alphanumeric1, multispace1),
            float,
            opt(preceded(
              multispace1,
              tuple((terminated(alphanumeric1, multispace1), float)),
            )),
          )),
          newline,
        ),
      ),
      |(column_name, row_name, value, opt)| WideLine::<f32> {
        name: column_name,
        first_pair: RowValuePair { row_name, value },
        second_pair: opt.map(|(opt_row_name, opt_value)| RowValuePair {
          row_name: opt_row_name,
          value: opt_value,
        }),
      },
    )(i)
  }

  pub fn columns_line(i: &str) -> IResult<&str, WideLine<'_, f32>> {
    Self::line(i)
  }

  pub fn columns(i: &str) -> IResult<&str, Vec<WideLine<'_, f32>>> {
    terminated(
      preceded(
        terminated(tag("COLUMNS"), newline),
        many1(Self::columns_line),
      ),
      peek(anychar),
    )(i)
  }

  pub fn rhs_line(i: &str) -> IResult<&str, WideLine<'_, f32>> {
    Self::line(i)
  }

  pub fn rhs(i: &str) -> IResult<&str, Vec<WideLine<'_, f32>>> {
    terminated(
      preceded(terminated(tag("RHS"), newline), many1(Self::rhs_line)),
      peek(anychar),
    )(i)
  }

  pub fn ranges_line(i: &str) -> IResult<&str, WideLine<'_, f32>> {
    Self::line(i)
  }

  pub fn ranges(i: &str) -> IResult<&str, Vec<WideLine<'_, f32>>> {
    terminated(
      preceded(terminated(tag("RANGES"), newline), many1(Self::ranges_line)),
      peek(anychar),
    )(i)
  }

  pub fn bound_type(i: &str) -> IResult<&str, BoundType> {
    map_res(
      alt((
        tag("LO"),
        tag("UP"),
        tag("FX"),
        tag("FR"),
        tag("MI"),
        tag("PL"),
        tag("BV"),
        tag("LI"),
        tag("UI"),
        tag("SC"),
      )),
      BoundType::try_from,
    )(i)
  }

  pub fn bounds_line(i: &str) -> IResult<&str, BoundsLine<'_, f32>> {
    map_res(
      preceded(
        tag(" "),
        terminated(
          tuple((
            terminated(
                Self::bound_type,
              multispace1,
            ),
            terminated(alphanumeric1, multispace1),
            terminated(alphanumeric1, multispace1),
            float,
          )),
          newline,
        ),
      ),
      |(bound_type, bound_name, column_name, value)| -> Result<BoundsLine<f32>> {
        Ok(BoundsLine {
          bound_type,
          bound_name,
          column_name,
          value,
        })
      },
    )(i)
  }

  pub fn bounds(i: &str) -> IResult<&str, Vec<BoundsLine<'_, f32>>> {
    terminated(
      preceded(terminated(tag("BOUNDS"), newline), many1(Self::bounds_line)),
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
        let _ = MPSFile::<f32>::row_line(&s);
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
    fn test_columns_line_doesnt_crash(s in "\\PC*") {
        let _ = MPSFile::<f32>::columns_line(&s);
    }
  }

  proptest! {
    #[test]
    fn test_columns_doesnt_crash(s in "\\PC*") {
        let _ = MPSFile::<f32>::columns(&s);
    }
  }

  proptest! {
    #[test]
    fn test_rhs_line_doesnt_crash(s in "\\PC*") {
        let _ = MPSFile::<f32>::rhs_line(&s);
    }
  }

  proptest! {
    #[test]
    fn test_rhs_doesnt_crash(s in "\\PC*") {
        let _ = MPSFile::<f32>::rhs(&s);
    }
  }

  proptest! {
    #[test]
    fn test_ranges_line_doesnt_crash(s in "\\PC*") {
        let _ = MPSFile::<f32>::ranges_line(&s);
    }
  }

  proptest! {
  #[test]
  fn test_ranges_doesnt_crash(s in "\\PC*") {
      let _ = MPSFile::<f32>::ranges(&s);
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
  fn test_row_line() -> Result<()> {
    let a = " E  R09\n";
    let b = " E  R10\n";
    let c = " L  X05\n";
    let d = " L  X21\n";
    assert_eq!(
      MPSFile::<f32>::row_line(a),
      Ok((
        "",
        RowLine {
          row_type: RowType::try_from('E')?,
          row_name: "R09"
        }
      ))
    );
    assert_eq!(
      MPSFile::<f32>::row_line(b),
      Ok((
        "",
        RowLine {
          row_type: RowType::try_from('E')?,
          row_name: "R10"
        }
      ))
    );
    assert_eq!(
      MPSFile::<f32>::row_line(c),
      Ok((
        "",
        RowLine {
          row_type: RowType::try_from('L')?,
          row_name: "X05"
        }
      ))
    );
    assert_eq!(
      MPSFile::<f32>::row_line(d),
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
  fn test_columns_line() {
    let a = "    X01       X48               .301   R09                -1.\n";
    let b = "    X02       COST               -.4\n";
    assert_eq!(
      MPSFile::<f32>::columns_line(a),
      Ok((
        "",
        WideLine::<f32> {
          name: "X01",
          first_pair: RowValuePair {
            row_name: "X48",
            value: 0.301
          },
          second_pair: Some(RowValuePair {
            row_name: "R09",
            value: -1.0
          })
        }
      ))
    );
    assert_eq!(
      MPSFile::<f32>::columns_line(b),
      Ok((
        "",
        WideLine::<f32> {
          name: "X02",
          first_pair: RowValuePair {
            row_name: "COST",
            value: -0.4
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
          WideLine::<f32> {
            name: "X01",
            first_pair: RowValuePair {
              row_name: "X48",
              value: 0.301
            },
            second_pair: Some(RowValuePair {
              row_name: "R09",
              value: -1.0
            })
          },
          WideLine::<f32> {
            name: "X01",
            first_pair: RowValuePair {
              row_name: "R10",
              value: -1.06
            },
            second_pair: Some(RowValuePair {
              row_name: "X05",
              value: 1.0
            })
          },
          WideLine::<f32> {
            name: "X02",
            first_pair: RowValuePair {
              row_name: "X21",
              value: -1.0
            },
            second_pair: Some(RowValuePair {
              row_name: "R09",
              value: 1.0
            })
          },
          WideLine::<f32> {
            name: "X02",
            first_pair: RowValuePair {
              row_name: "COST",
              value: -0.4
            },
            second_pair: None
          },
          WideLine::<f32> {
            name: "X03",
            first_pair: RowValuePair {
              row_name: "X46",
              value: -1.0
            },
            second_pair: Some(RowValuePair {
              row_name: "R09",
              value: 1.0
            })
          },
        ]
      ))
    );
  }

  #[test]
  fn test_rhs_line() {
    let a = "    RHS1      LIM1                 5   LIM2                10\n";
    let b = "    RHS1      MYEQN                7\n";
    assert_eq!(
      MPSFile::<f32>::rhs_line(a),
      Ok((
        "",
        WideLine::<f32> {
          name: "RHS1",
          first_pair: RowValuePair {
            row_name: "LIM1",
            value: 5.0
          },
          second_pair: Some(RowValuePair {
            row_name: "LIM2",
            value: 10.0
          })
        }
      ))
    );
    assert_eq!(
      MPSFile::<f32>::rhs_line(b),
      Ok((
        "",
        WideLine::<f32> {
          name: "RHS1",
          first_pair: RowValuePair {
            row_name: "MYEQN",
            value: 7.0
          },
          second_pair: None
        }
      ))
    );
  }

  #[test]
  fn test_rhs() {
    let a = "RHS
    RHS1      LIM1                 5   LIM2                10
    RHS1      MYEQN                7\nBOUNDS";
    assert_eq!(
      MPSFile::<f32>::rhs(a),
      Ok((
        "BOUNDS",
        vec![
          WideLine::<f32> {
            name: "RHS1",
            first_pair: RowValuePair {
              row_name: "LIM1",
              value: 5.0
            },
            second_pair: Some(RowValuePair {
              row_name: "LIM2",
              value: 10.0
            })
          },
          WideLine::<f32> {
            name: "RHS1",
            first_pair: RowValuePair {
              row_name: "MYEQN",
              value: 7.0
            },
            second_pair: None
          }
        ]
      ))
    );
  }

  #[test]
  fn test_ranges_line() {
    let a = "    RANGE1    VILLKOR6           2.5   VILLKOR7           30.\n";
    let b = "    RANGE1    VILLKOR8           7.5\n";
    assert_eq!(
      MPSFile::<f32>::ranges_line(a),
      Ok((
        "",
        WideLine::<f32> {
          name: "RANGE1",
          first_pair: RowValuePair {
            row_name: "VILLKOR6",
            value: 2.5
          },
          second_pair: Some(RowValuePair {
            row_name: "VILLKOR7",
            value: 30.0
          })
        }
      ))
    );
    assert_eq!(
      MPSFile::<f32>::ranges_line(b),
      Ok((
        "",
        WideLine::<f32> {
          name: "RANGE1",
          first_pair: RowValuePair {
            row_name: "VILLKOR8",
            value: 7.5
          },
          second_pair: None
        }
      ))
    );
  }

  #[test]
  fn test_ranges() {
    let a = "RANGES
    RANGE1    VILLKOR2            7.   VILLKOR3            7.
    RANGE1    VILLKOR4           3.5   VILLKOR5           10.
    RANGE1    VILLKOR6           2.5   VILLKOR7           30.
    RANGE1    VILLKOR8           7.5\nBOUNDS";
    assert_eq!(
      MPSFile::<f32>::ranges(a),
      Ok((
        "BOUNDS",
        vec![
          WideLine::<f32> {
            name: "RANGE1",
            first_pair: RowValuePair {
              row_name: "VILLKOR2",
              value: 7.0
            },
            second_pair: Some(RowValuePair {
              row_name: "VILLKOR3",
              value: 7.0
            })
          },
          WideLine::<f32> {
            name: "RANGE1",
            first_pair: RowValuePair {
              row_name: "VILLKOR4",
              value: 3.5
            },
            second_pair: Some(RowValuePair {
              row_name: "VILLKOR5",
              value: 10.0
            })
          },
          WideLine::<f32> {
            name: "RANGE1",
            first_pair: RowValuePair {
              row_name: "VILLKOR6",
              value: 2.5
            },
            second_pair: Some(RowValuePair {
              row_name: "VILLKOR7",
              value: 30.0
            })
          },
          WideLine::<f32> {
            name: "RANGE1",
            first_pair: RowValuePair {
              row_name: "VILLKOR8",
              value: 7.5
            },
            second_pair: None
          },
        ]
      ))
    );
  }

  #[test]
  fn test_bound_type() {
    assert_eq!(MPSFile::<f32>::bound_type("LO"), Ok(("", BoundType::LO)));
    assert_eq!(MPSFile::<f32>::bound_type("UP"), Ok(("", BoundType::UP)));
    assert_eq!(MPSFile::<f32>::bound_type("FX"), Ok(("", BoundType::FX)));
    assert_eq!(MPSFile::<f32>::bound_type("FR"), Ok(("", BoundType::FR)));
    assert_eq!(MPSFile::<f32>::bound_type("MI"), Ok(("", BoundType::MI)));
    assert_eq!(MPSFile::<f32>::bound_type("PL"), Ok(("", BoundType::PL)));
  }

  #[test]
  fn test_bounds_line() {
    let a = " UP BND1      XONE                 4\n";
    let b = " LO BND1      YTWO                -1\n";
    let c = " UP BND1      YTWO                 1\n";
    assert_eq!(
      MPSFile::<f32>::bounds_line(a),
      Ok((
        "",
        BoundsLine::<f32> {
          bound_type: BoundType::UP,
          bound_name: "BND1",
          column_name: "XONE",
          value: 4.0
        }
      ))
    );
    assert_eq!(
      MPSFile::<f32>::bounds_line(b),
      Ok((
        "",
        BoundsLine::<f32> {
          bound_type: BoundType::LO,
          bound_name: "BND1",
          column_name: "YTWO",
          value: -1.0
        }
      ))
    );
    assert_eq!(
      MPSFile::<f32>::bounds_line(c),
      Ok((
        "",
        BoundsLine::<f32> {
          bound_type: BoundType::UP,
          bound_name: "BND1",
          column_name: "YTWO",
          value: 1.0
        }
      ))
    );
  }

  #[test]
  fn test_bounds() {
    let a = "BOUNDS
 UP BND1      XONE                 4
 LO BND1      YTWO                -1
 UP BND1      YTWO                 1\nENDATA";
    assert_eq!(
      MPSFile::<f32>::bounds(a),
      Ok((
        "ENDATA",
        vec![
          BoundsLine::<f32> {
            bound_type: BoundType::UP,
            bound_name: "BND1",
            column_name: "XONE",
            value: 4.0
          },
          BoundsLine::<f32> {
            bound_type: BoundType::LO,
            bound_name: "BND1",
            column_name: "YTWO",
            value: -1.0
          },
          BoundsLine::<f32> {
            bound_type: BoundType::UP,
            bound_name: "BND1",
            column_name: "YTWO",
            value: 1.0
          }
        ],
      ))
    );
  }
}
