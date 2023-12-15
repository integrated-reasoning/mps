use num_traits::float::Float;

pub struct MPSFile<T: Float> {
  name: NameSection,
  rows: RowsSection,
  columns: ColumnsSection<T>,
  rhs: RHSSection<T>,
  ranges: RangesSection<T>,
  bounds: BoundsSection<T>,
  // sos: SOSSection<T>, // TODO
}

pub enum Section {
  Name(NameSection),
  Rows,
  Columns,
  RHS,
  Ranges,
  Bounds,
  Endata,
}

pub type NameSection = String;

pub type RowsSection = Vec<RowLine>;
pub type RowLine = (RowType, RowName);
pub type RowName = String;

pub type ColumnsSection<T> = Vec<ColumnLine<T>>;
pub type ColumnLine<T> = (ColumnName, RowName, T, Option<(RowName, T)>);
pub type ColumnName = String;

pub type RHSSection<T> = Vec<RHSLine<T>>;
pub type RHSLine<T> = (RHSName, RowName, T, Option<(RowName, T)>);
pub type RHSName = String;

pub type RangesSection<T> = Vec<RangesLine<T>>;
pub type RangesLine<T> = (RangeName, RowName, T, Option<(RowName, T)>);
pub type RangeName = String;

pub type BoundsSection<T> = (BoundType, BoundName, ColumnName, T);
pub type BoundName = String;

pub enum RowType {
  EQ,
  LEQ,
  GEQ,
  NR,
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

pub enum RangeType {
  LE,
  GE,
  EP,
  EM,
  EZ,
}

pub enum BoundType {
  LO, // lower bound     :  l_j <= x_j <= inf
  UP, // upper bound     :    0 <= x_j <= u_j
  FX, // fixed variable  :  l_j == x_j == u_j
  FR, // free variable   : -inf <= x_j <= inf
  MI, // Unbounded below : -inf <= x_j <= 0
  PL, // Unbounded above :    0 <= x_j <= inf
}
