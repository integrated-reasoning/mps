use crate::model::row_type_map::RowTypeMap;
use crate::types::Rhs;
use color_eyre::{eyre::eyre, Result};
use fast_float::FastFloat;
use indexmap::IndexMap;
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct RhsMap<T: FastFloat>(IndexMap<String, IndexMap<String, T>>);

impl<T: FastFloat> TryFrom<(&Rhs<'_, T>, &RowTypeMap)> for RhsMap<T> {
  type Error = color_eyre::Report;

  fn try_from(t: (&Rhs<'_, T>, &RowTypeMap)) -> Result<Self> {
    let mut rhs = RhsMap(IndexMap::new());
    let (rhs_lines, row_types) = t;
    for r in rhs_lines {
      row_types.exists(r.first_pair.row_name)?;
      rhs.insert(r.name, r.first_pair.row_name, r.first_pair.value)?;
      if let Some(second_pair) = r.second_pair.as_ref() {
        row_types.exists(second_pair.row_name)?;
        rhs.insert(r.name, second_pair.row_name, second_pair.value)?;
      }
    }
    Ok(rhs)
  }
}

impl<T: FastFloat> RhsMap<T> {
  fn insert(&mut self, rhs_name: &str, row_name: &str, value: T) -> Result<()> {
    match self.0.get_mut(rhs_name) {
      None => {
        let mut rhs = IndexMap::new();
        rhs.insert(row_name.to_string(), value);
        self.0.insert(rhs_name.to_string(), rhs);
        Ok(())
      }
      Some(rhs) => match rhs.insert(row_name.to_string(), value) {
        Some(conflicting_value) => Err(eyre!(format!(
          "duplicate entry in RHS {:?} at row {:?}: found {:?} and {:?}",
          rhs_name, row_name, value, conflicting_value
        ))),
        None => Ok(()),
      },
    }?;
    Ok(())
  }
}
