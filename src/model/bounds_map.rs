use crate::types::{BoundType, Bounds};
use color_eyre::{eyre::eyre, Result};
use hashbrown::{HashMap, HashSet};

#[derive(Debug, Default, Clone, PartialEq)]
pub struct BoundsMap(HashMap<String, HashMap<String, (f32, BoundType)>>);

impl TryFrom<(&Bounds<'_, f32>, &HashSet<&str>)> for BoundsMap {
  type Error = color_eyre::Report;

  fn try_from(t: (&Bounds<'_, f32>, &HashSet<&str>)) -> Result<Self> {
    let mut bounds = BoundsMap(HashMap::new());
    let (bounds_lines, column_names) = t;
    for b in bounds_lines {
      match column_names.get(b.column_name) {
        Some(_) => bounds.insert(
          b.bound_name,
          b.column_name,
          b.bound_type.clone(),
          b.value,
        ),
        None => Err(eyre!(format!(
          "specified bound {:?} of type {:?} for unspecified column {:?}",
          b.bound_name, b.bound_type, b.column_name
        ))),
      }?;
    }
    Ok(bounds)
  }
}

impl BoundsMap {
  fn insert(
    &mut self,
    bound_name: &str,
    column_name: &str,
    bound_type: BoundType,
    value: f32,
  ) -> Result<()> {
    match self.0.get_mut(bound_name) {
      None => {
        let mut bounds = HashMap::new();
        bounds.insert(column_name.to_string(), (value, bound_type));
        self.0.insert(bound_name.to_string(), bounds);
        Ok(())
      }
      Some(bounds) => {
        match bounds.insert(column_name.to_string(), (value, bound_type)) {
          Some(conflicting_value) => Err(eyre!(format!(
            "duplicate entry in bound {:?} at column {:?}: found {:?} and {:?}",
            bound_name, column_name, value, conflicting_value
          ))),
          None => Ok(()),
        }
      }
    }?;
    Ok(())
  }
}
