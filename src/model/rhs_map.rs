use crate::model::row_type_map::RowTypeMap;
use crate::types::Rhs;
use color_eyre::{eyre::eyre, Result};
use hashbrown::HashMap;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct RhsMap(HashMap<String, HashMap<String, f32>>);

impl TryFrom<(&Rhs<'_, f32>, &RowTypeMap)> for RhsMap {
  type Error = color_eyre::Report;

  fn try_from(t: (&Rhs<'_, f32>, &RowTypeMap)) -> Result<Self> {
    let mut rhs = RhsMap(HashMap::new());
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

impl RhsMap {
  fn insert(
    &mut self,
    rhs_name: &str,
    row_name: &str,
    value: f32,
  ) -> Result<()> {
    match self.0.get_mut(rhs_name) {
      None => {
        let mut rhs = HashMap::new();
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