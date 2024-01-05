use crate::types::{Parser, RowType, Rows};
use color_eyre::{eyre::eyre, Result};
use hashbrown::HashMap;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Model {
  pub name: String,
  pub row_types: RowTypeMap,
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RowTypeMap(HashMap<String, RowType>);

impl TryFrom<Parser<'_, f32>> for Model {
  type Error = color_eyre::Report;

  fn try_from(parsed: Parser<f32>) -> Result<Self> {
    let row_types = RowTypeMap::try_from(parsed.rows)?;
    Ok(Model {
      name: parsed.name.to_string(),
      row_types,
    })
  }
}

impl TryFrom<Rows<'_>> for RowTypeMap {
  type Error = color_eyre::Report;

  fn try_from(rows: Rows<'_>) -> Result<Self> {
    let mut row_types = HashMap::new();
    for r in rows {
      match row_types.insert(r.row_name.to_string(), r.row_type.clone()) {
        Some(row_type) => Err(eyre!(format!(
          "conflicting row type information for {}: found {:?} and {:?}",
          r.row_name,
          r.row_type.clone(),
          row_type
        ))),
        None => Ok(()),
      }?;
    }
    Ok(RowTypeMap(row_types))
  }
}

mod tests {
  use super::*;

  #[test]
  fn test_try_from_afiro() -> Result<()> {
    let (_, parsed) =
      Parser::<f32>::parse(include_str!("../tests/data/netlib/afiro"))?;
    Model::try_from(parsed)?;
    Ok(())
  }

  #[test]
  fn test_model_conflicting_rows_line() -> Result<()> {
    let (_, parsed) = Parser::<f32>::parse(include_str!(
      "../tests/data/should_fail/conflicting_rows_line"
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
  fn test_rowtypemap_conflicting_rows_line() -> Result<()> {
    let (_, parsed) = Parser::<f32>::parse(include_str!(
      "../tests/data/should_fail/conflicting_rows_line"
    ))?;
    let error =
      eyre!("conflicting row type information for R09: found Leq and Eq");
    match RowTypeMap::try_from(parsed.rows) {
      Ok(_) => panic!(),
      Err(e) => assert_eq!(e.to_string(), error.to_string()),
    };
    Ok(())
  }
}
