use crate::types::{BoundType, Bounds};
use color_eyre::{eyre::eyre, Result};
use fast_float2::FastFloat;
use hashbrown::HashSet;
use indexmap::IndexMap;
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct BoundsMap<T: FastFloat>(
  pub IndexMap<String, IndexMap<(String, BoundType), Option<T>>>,
);

impl<T: FastFloat> TryFrom<(&Bounds<'_, T>, &HashSet<&str>)> for BoundsMap<T> {
  type Error = color_eyre::Report;

  fn try_from(t: (&Bounds<'_, T>, &HashSet<&str>)) -> Result<Self> {
    let mut bounds = BoundsMap(IndexMap::new());
    let (bounds_lines, column_names) = t;
    for b in bounds_lines {
      match column_names.get(b.column_name.trim()) {
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

impl<T: FastFloat> BoundsMap<T> {
  fn insert(
    &mut self,
    bound_name: &str,
    column_name: &str,
    bound_type: BoundType,
    value: Option<T>,
  ) -> Result<()> {
    match self.0.get_mut(bound_name.trim()) {
      None => {
        let mut bounds = IndexMap::new();
        bounds.insert((column_name.trim().to_string(), bound_type), value);
        self.0.insert(bound_name.trim().to_string(), bounds);
        Ok(())
      }
      Some(bounds) => {
        match bounds.insert((column_name.trim().to_string(), bound_type), value) {
          Some(conflicting_value) => Err(eyre!(format!(
            "duplicate entry in BOUNDS {:?} for column {:?}: found {:?} and {:?}",
            bound_name, column_name, value, conflicting_value
          ))),
          None => Ok(()),
        }
      }
    }?;
    Ok(())
  }
}
