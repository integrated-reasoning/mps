use crate::model::row_type_map::RowTypeMap;
use crate::types::Columns;
use color_eyre::{eyre::eyre, Result};
use fast_float::FastFloat;
use hashbrown::HashMap;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct RowColumnValueMap<T: FastFloat>(HashMap<(String, String), T>);

impl<T: FastFloat> TryFrom<(&Columns<'_, T>, &RowTypeMap)>
  for RowColumnValueMap<T>
{
  type Error = color_eyre::Report;

  fn try_from(t: (&Columns<'_, T>, &RowTypeMap)) -> Result<Self> {
    let mut row_column_values = RowColumnValueMap(HashMap::new());
    let (columns_lines, row_types) = t;
    for c in columns_lines {
      row_types.exists(c.first_pair.row_name)?;
      row_column_values.insert(
        c.first_pair.row_name,
        c.name,
        c.first_pair.value,
      )?;
      if let Some(second_pair) = c.second_pair.as_ref() {
        row_types.exists(second_pair.row_name)?;
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

impl<T: FastFloat> RowColumnValueMap<T> {
  fn insert(
    &mut self,
    row_name: &str,
    column_name: &str,
    value: T,
  ) -> Result<()> {
    match self.0.insert((row_name.to_string(), column_name.to_string()), value)
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
