mod bounds_map;
mod ranges_map;
mod rhs_map;
mod row_column_value_map;
mod row_type_map;

use crate::model::bounds_map::BoundsMap;
use crate::model::ranges_map::RangesMap;
use crate::model::rhs_map::RhsMap;
use crate::model::row_column_value_map::RowColumnValueMap;
use crate::model::row_type_map::RowTypeMap;
use crate::types::Parser;
use color_eyre::Result;
use hashbrown::HashSet;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Model {
  pub name: String,
  pub row_types: RowTypeMap,
  pub values: RowColumnValueMap,
  pub rhs: RhsMap,
  pub bounds: BoundsMap,
  pub ranges: RangesMap,
}

impl TryFrom<Parser<'_, f32>> for Model {
  type Error = color_eyre::Report;

  fn try_from(parsed: Parser<f32>) -> Result<Self> {
    let row_types = RowTypeMap::try_from(&parsed.rows)?;
    let values = RowColumnValueMap::try_from((&parsed.columns, &row_types))?;
    let rhs = match parsed.rhs {
      Some(rhs) => RhsMap::try_from((&rhs, &row_types)),
      None => Ok(RhsMap::default()),
    }?;
    let mut column_names = HashSet::<&str>::new();
    for c in &parsed.columns {
      column_names.insert(c.name);
    }
    let bounds = match parsed.bounds {
      Some(bounds) => BoundsMap::try_from((&bounds, &column_names)),
      None => Ok(BoundsMap::default()),
    }?;
    let ranges = match parsed.ranges {
      Some(ranges) => RangesMap::try_from((&ranges, &row_types)),
      None => Ok(RangesMap::default()),
    }?;
    Ok(Model {
      name: parsed.name.to_string(),
      row_types,
      values,
      rhs,
      bounds,
      ranges,
    })
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use color_eyre::{eyre::eyre, Result};

  #[test]
  fn test_conflicting_ranges_line() -> Result<()> {
    let parsed = Parser::<f32>::parse(include_str!(
      "../../tests/data/should_fail/conflicting_ranges_line"
    ))?;
    let error = eyre!(
        "duplicate entry in RANGES \"RANGE1\" at row \"VILLKOR6\": found 2.5 and 2.5"
    );
    match Model::try_from(parsed) {
      Ok(_) => panic!(),
      Err(e) => assert_eq!(e.to_string(), error.to_string()),
    };
    Ok(())
  }

  #[test]
  fn test_conflicting_bounds_line() -> Result<()> {
    let parsed = Parser::<f32>::parse(include_str!(
      "../../tests/data/should_fail/conflicting_bounds_line"
    ))?;
    let error = eyre!(
      "duplicate entry in BOUNDS \"BOUND\" for column \"UGTD03\": found Some(0.2) and Some(20.2)"
    );
    match Model::try_from(parsed) {
      Ok(_) => panic!(),
      Err(e) => assert_eq!(e.to_string(), error.to_string()),
    };
    Ok(())
  }

  #[test]
  fn test_conflicting_rhs_line() -> Result<()> {
    let parsed = Parser::<f32>::parse(include_str!(
      "../../tests/data/should_fail/conflicting_rhs_line"
    ))?;
    let error = eyre!(
      "duplicate entry in RHS \"B\" at row \"X51\": found 120.0 and 300.0"
    );
    match Model::try_from(parsed) {
      Ok(_) => panic!(),
      Err(e) => assert_eq!(e.to_string(), error.to_string()),
    };
    Ok(())
  }

  #[test]
  fn test_conflicting_rows_line() -> Result<()> {
    let parsed = Parser::<f32>::parse(include_str!(
      "../../tests/data/should_fail/conflicting_rows_line"
    ))?;
    let error =
      eyre!("conflicting row type information for R09: found Leq and Eq");
    match Model::try_from(parsed) {
      Ok(_) => panic!(),
      Err(e) => assert_eq!(e.to_string(), error.to_string()),
    };
    Ok(())
  }

  #[test]
  fn test_unspecified_row_type() -> Result<()> {
    let parsed = Parser::<f32>::parse(include_str!(
      "../../tests/data/should_fail/unspecified_row_type"
    ))?;
    let error = eyre!("referenced row of unspecified type: X27");
    match Model::try_from(parsed) {
      Ok(_) => panic!(),
      Err(e) => assert_eq!(e.to_string(), error.to_string()),
    };
    Ok(())
  }

  #[test]
  fn test_try_from_afiro() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../../tests/data/netlib/afiro"))?;
    Model::try_from(parsed)?;
    Ok(())
  }

  #[test]
  fn test_try_from_bnl1() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../../tests/data/netlib/bnl1"))?;
    Model::try_from(parsed)?;
    Ok(())
  }

  #[test]
  fn test_try_from_bnl1_snapshot() -> Result<()> {
    let parsed =
      Parser::<f32>::parse(include_str!("../../tests/data/netlib/bnl1"))?;
    let model = format!("{:?}", Model::try_from(parsed)?);
    insta::assert_yaml_snapshot!(model);
    Ok(())
  }
}
