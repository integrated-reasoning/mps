use crate::types::{Parser, RowType, Rows};
use color_eyre::{eyre::eyre, Result};
use hashbrown::HashMap;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct RowTypeMap(HashMap<String, RowType>);

impl TryFrom<&Rows<'_>> for RowTypeMap {
  type Error = color_eyre::Report;

  fn try_from(rows: &Rows<'_>) -> Result<Self> {
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

impl RowTypeMap {
  pub fn exists(&self, name: &str) -> Result<()> {
    match self.get(name) {
      Some(_) => Ok(()),
      None => Err(eyre!(format!(
        "referenced row of unspecified type: {}",
        name
      ))),
    }?;
    Ok(())
  }
  pub fn get(&self, row_name: &str) -> Option<&RowType> {
    self.0.get(row_name)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_conflicting_rows_line() -> Result<()> {
    let parsed = Parser::<f32>::parse(include_str!(
      "../../tests/data/should_fail/conflicting_rows_line"
    ))?;
    let error =
      eyre!("conflicting row type information for R09: found Leq and Eq");
    match RowTypeMap::try_from(&parsed.rows) {
      Ok(_) => panic!(),
      Err(e) => assert_eq!(e.to_string(), error.to_string()),
    };
    Ok(())
  }
}
