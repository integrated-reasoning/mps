use crate::model::row_type_map::RowTypeMap;
use crate::types::Columns;
use color_eyre::{eyre::eyre, Result};
use hashbrown::HashMap;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct RowColumnValueMap(HashMap<(String, String), f32>);

impl TryFrom<(&Columns<'_, f32>, &RowTypeMap)> for RowColumnValueMap {
  type Error = color_eyre::Report;

  fn try_from(t: (&Columns<'_, f32>, &RowTypeMap)) -> Result<Self> {
    let mut row_column_values = RowColumnValueMap(HashMap::new());
    let (columns_lines, row_types) = t;
    for c in columns_lines {
      exists(c.first_pair.row_name, row_types)?;
      row_column_values.insert(
        c.first_pair.row_name,
        c.name,
        c.first_pair.value,
      )?;
      if let Some(second_pair) = c.second_pair.as_ref() {
        exists(second_pair.row_name, row_types)?;
        row_column_values.insert(
          second_pair.row_name,
          c.name,
          second_pair.value,
        )?;
      }
    }
    Ok(row_column_values)
  }
}

fn exists(name: &str, map: &RowTypeMap) -> Result<()> {
  match map.get(name) {
    Some(_) => Ok(()),
    None => Err(eyre!(format!(
      "referenced row of unspecified type: {}",
      name
    ))),
  }?;
  Ok(())
}

impl RowColumnValueMap {
  fn insert(
    &mut self,
    row_name: &str,
    column_name: &str,
    value: f32,
  ) -> Result<()> {
    let v = 9.2; // todo
    match self.0.insert((row_name.to_string(), column_name.to_string()), v)
      {
        Some(conflicting_value) => Err(eyre!(format!(
          "conflicting (row, column, value) information for {:?}: found {:?} and {:?}",
          (row_name, column_name), value, conflicting_value
        ))),
        None => Ok(()),
      }?;
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
}
