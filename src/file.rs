use color_eyre::{eyre::eyre, Result};
use nom::{
  branch::alt,
  bytes::complete::{tag, take_while1},
  character::complete::*,
  combinator::{map, map_res, opt, peek},
  multi::{count, many1},
  number::complete::float,
  sequence::{preceded, separated_pair, terminated, tuple},
  IResult,
};
use num_traits::float::Float;

fn not_whitespace1(i: &str) -> IResult<&str, &str> {
  take_while1(|c: char| !c.is_whitespace())(i)
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct MPSFile<'a, T: Float> {
  pub name: &'a str,
  pub rows: Rows<'a>,
  pub columns: Columns<'a, T>,
  pub rhs: Option<Rhs<'a, T>>,
  pub ranges: Option<Ranges<'a, T>>,
  pub bounds: Option<Bounds<'a, T>>,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct RowLine<'a> {
  pub row_type: RowType,
  pub row_name: &'a str,
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum RowType {
  #[default]
  Eq,
  Leq,
  Geq,
  Nr,
}

impl TryFrom<char> for RowType {
  type Error = color_eyre::Report;

  fn try_from(c: char) -> Result<Self> {
    match c {
      'E' => Ok(RowType::Eq),
      'L' => Ok(RowType::Leq),
      'G' => Ok(RowType::Geq),
      'N' => Ok(RowType::Nr),
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

pub type Rhs<'a, T> = Vec<WideLine<'a, T>>;

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
  Lo, // lower bound     :  l_j <= x_j <= inf
  Up, // upper bound     :    0 <= x_j <= u_j
  Fx, // fixed variable  :  l_j == x_j == u_j
  Fr, // free variable   : -inf <= x_j <= inf
  Mi, // Unbounded below : -inf <= x_j <= 0
  Pl, // Unbounded above :    0 <= x_j <= inf
}

impl TryFrom<&str> for BoundType {
  type Error = color_eyre::Report;

  fn try_from(s: &str) -> Result<Self> {
    match s {
      "LO" => Ok(BoundType::Lo),
      "UP" => Ok(BoundType::Up),
      "FX" => Ok(BoundType::Fx),
      "FR" => Ok(BoundType::Fr),
      "MI" => Ok(BoundType::Mi),
      "PL" => Ok(BoundType::Pl),
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
 * both L_i and U_i should be set to the respective Rhs value b_i.
 */

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum RangeType {
  #[default]
  _Le,
  _Ge,
  _Ep,
  _Em,
  _Ez,
}

impl<'a, T: Float> MPSFile<'a, T> {
  pub fn parse(input: &'a str) -> IResult<&'a str, MPSFile<'a, f32>> {
    map(
      tuple((
        Self::name,
        Self::rows,
        Self::columns,
        opt(Self::rhs),
        opt(Self::ranges),
        opt(Self::bounds),
        opt(Self::endata),
      )),
      |(name, rows, columns, rhs, ranges, bounds, _)| MPSFile {
        name,
        rows,
        columns,
        rhs,
        ranges,
        bounds,
      },
    )(input)
  }

  pub fn name(i: &str) -> IResult<&str, &str> {
    terminated(
      preceded(tag("NAME"), preceded(count(anychar, 10), not_line_ending)),
      newline,
    )(i)
  }

  pub fn row_line(i: &str) -> IResult<&str, RowLine> {
    map_res(
      preceded(
        tag(" "),
        terminated(
          separated_pair(one_of("ELGN"), multispace1, not_whitespace1),
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

  pub fn rows(i: &str) -> IResult<&str, Vec<RowLine>> {
    terminated(
      preceded(terminated(tag("ROWS"), newline), many1(Self::row_line)),
      peek(anychar),
    )(i)
  }

  pub fn line(i: &str) -> IResult<&str, WideLine<f32>> {
    map(
      preceded(
        tag("    "),
        terminated(
          tuple((
            terminated(not_whitespace1, multispace1),
            terminated(not_whitespace1, multispace1),
            float,
            opt(preceded(
              multispace1,
              tuple((terminated(not_whitespace1, multispace1), float)),
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

  pub fn columns_line(i: &str) -> IResult<&str, WideLine<f32>> {
    Self::line(i)
  }

  pub fn columns(i: &str) -> IResult<&str, Vec<WideLine<f32>>> {
    terminated(
      preceded(
        terminated(tag("COLUMNS"), newline),
        many1(Self::columns_line),
      ),
      peek(anychar),
    )(i)
  }

  pub fn rhs_line(i: &str) -> IResult<&str, WideLine<f32>> {
    Self::line(i)
  }

  pub fn rhs(i: &str) -> IResult<&str, Vec<WideLine<f32>>> {
    terminated(
      preceded(terminated(tag("RHS"), newline), many1(Self::rhs_line)),
      peek(anychar),
    )(i)
  }

  pub fn ranges_line(i: &str) -> IResult<&str, WideLine<f32>> {
    Self::line(i)
  }

  pub fn ranges(i: &str) -> IResult<&str, Vec<WideLine<f32>>> {
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

  pub fn bounds_line(i: &str) -> IResult<&str, BoundsLine<f32>> {
    map_res(
      preceded(
        tag(" "),
        terminated(
          tuple((
            terminated(
              Self::bound_type,
              multispace1,
            ),
            terminated(not_whitespace1, multispace1),
            terminated(not_whitespace1, multispace1),
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

  pub fn bounds(i: &str) -> IResult<&str, Vec<BoundsLine<f32>>> {
    terminated(
      preceded(terminated(tag("BOUNDS"), newline), many1(Self::bounds_line)),
      peek(anychar),
    )(i)
  }

  pub fn endata(i: &str) -> IResult<&str, &str> {
    tag("ENDATA")(i)
  }
}
