use crate::model::row_type_map::RowTypeMap;
use crate::types::Ranges;
use color_eyre::{eyre::eyre, Result};
use fast_float2::FastFloat;
use indexmap::IndexMap;
#[cfg(feature = "serde")]
use serde::Serialize;

#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct RangesMap<T: FastFloat>(pub IndexMap<String, IndexMap<String, T>>);

impl<T: FastFloat> TryFrom<(&Ranges<'_, T>, &RowTypeMap)> for RangesMap<T> {
  type Error = color_eyre::Report;

  fn try_from(t: (&Ranges<'_, T>, &RowTypeMap)) -> Result<Self> {
    let mut ranges = RangesMap(IndexMap::new());
    let (ranges_lines, row_types) = t;
    for r in ranges_lines {
      row_types.exists(r.first_pair.row_name)?;
      ranges.insert(r.name, r.first_pair.row_name, r.first_pair.value)?;
      if let Some(second_pair) = r.second_pair.as_ref() {
        row_types.exists(second_pair.row_name)?;
        ranges.insert(r.name, second_pair.row_name, second_pair.value)?;
      }
    }
    Ok(ranges)
  }
}

impl<T: FastFloat> RangesMap<T> {
  fn insert(
    &mut self,
    ranges_name: &str,
    row_name: &str,
    value: T,
  ) -> Result<()> {
    match self.0.get_mut(ranges_name) {
      None => {
        let mut ranges = IndexMap::new();
        ranges.insert(row_name.to_string(), value);
        self.0.insert(ranges_name.to_string(), ranges);
        Ok(())
      }
      Some(ranges) => match ranges.insert(row_name.to_string(), value) {
        Some(conflicting_value) => Err(eyre!(format!(
          "duplicate entry in RANGES {:?} at row {:?}: found {:?} and {:?}",
          ranges_name, row_name, value, conflicting_value
        ))),
        None => Ok(()),
      },
    }?;
    Ok(())
  }
}
