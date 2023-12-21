#[cfg(feature = "proptest")]
#[cfg(test)]
mod tests {
  use mps::file::*;
  use proptest::prelude::*;

  proptest! {
    #[test]
    fn test_name_doesnt_crash(s in "\\PC*") {
        let _ = MPSFile::<f32>::name(&s);
    }
  }

  proptest! {
    #[test]
    fn test_row_doesnt_crash(s in "\\PC*") {
        let _ = MPSFile::<f32>::row_line(&s);
    }
  }

  proptest! {
    #[test]
    fn test_rows_doesnt_crash(s in "\\PC*") {
        let _ = MPSFile::<f32>::rows(&s);
    }
  }

  proptest! {
    #[test]
    fn test_columns_line_doesnt_crash(s in "\\PC*") {
        let _ = MPSFile::<f32>::columns_line(&s);
    }
  }

  proptest! {
    #[test]
    fn test_columns_doesnt_crash(s in "\\PC*") {
        let _ = MPSFile::<f32>::columns(&s);
    }
  }

  proptest! {
    #[test]
    fn test_rhs_line_doesnt_crash(s in "\\PC*") {
        let _ = MPSFile::<f32>::rhs_line(&s);
    }
  }

  proptest! {
    #[test]
    fn test_rhs_doesnt_crash(s in "\\PC*") {
        let _ = MPSFile::<f32>::rhs(&s);
    }
  }

  proptest! {
    #[test]
    fn test_ranges_line_doesnt_crash(s in "\\PC*") {
        let _ = MPSFile::<f32>::ranges_line(&s);
    }
  }

  proptest! {
    #[test]
    fn test_ranges_doesnt_crash(s in "\\PC*") {
        let _ = MPSFile::<f32>::ranges(&s);
    }
  }

  proptest! {
    #[test]
    fn test_bound_type_doesnt_crash(s in "\\PC*") {
        let _ = MPSFile::<f32>::bound_type(&s);
    }
  }

  proptest! {
    #[test]
    fn test_bounds_line_doesnt_crash(s in "\\PC*") {
        let _ = MPSFile::<f32>::bounds_line(&s);
    }
  }

  proptest! {
    #[test]
    fn test_bounds_doesnt_crash(s in "\\PC*") {
        let _ = MPSFile::<f32>::bounds(&s);
    }
  }

  proptest! {
    #[test]
    fn test_parse_doesnt_crash(s in "\\PC*") {
        let _ = MPSFile::<f32>::parse(&s);
    }
  }
}
