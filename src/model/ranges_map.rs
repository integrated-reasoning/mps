use crate::model::row_type_map::RowTypeMap;
use crate::types::Ranges;
use color_eyre::{eyre::eyre, Result};
use hashbrown::HashMap;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct RangesMap(HashMap<String, HashMap<String, f32>>);

impl TryFrom<(&Ranges<'_, f32>, &RowTypeMap)> for RangesMap {
  type Error = color_eyre::Report;

  fn try_from(t: (&Ranges<'_, f32>, &RowTypeMap)) -> Result<Self> {
    let mut ranges = RangesMap(HashMap::new());
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

impl RangesMap {
  fn insert(
    &mut self,
    ranges_name: &str,
    row_name: &str,
    value: f32,
  ) -> Result<()> {
    match self.0.get_mut(ranges_name) {
      None => {
        let mut ranges = HashMap::new();
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
