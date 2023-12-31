#[cfg(feature = "proptest")]
#[cfg(test)]
mod tests {
  use mps::types::*;
  use proptest::prelude::*;
  cfg_if::cfg_if! {
    if #[cfg(feature = "located")] {
      use nom_locate::LocatedSpan;
      use nom_tracable::TracableInfo;
    }
  }

  proptest! {
    #[test]
    fn test_name_doesnt_crash(s in "\\PC*") {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let _ = Parser::<f32>::name(LocatedSpan::new_extra(&s, info));
        } else {
          let _ = Parser::<f32>::name(&s);
        }
      }
    }
  }

  proptest! {
    #[test]
    fn test_row_line_doesnt_crash(s in "\\PC*") {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let _ = Parser::<f32>::row_line(LocatedSpan::new_extra(&s, info));
        } else {
          let _ = Parser::<f32>::row_line(&s);
        }
      }
    }
  }

  proptest! {
    #[test]
    fn test_rows_doesnt_crash(s in "\\PC*") {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let _ = Parser::<f32>::rows(LocatedSpan::new_extra(&s, info));
        } else {
          let _ = Parser::<f32>::rows(&s);
        }
      }
    }
  }

  proptest! {
    #[test]
    fn test_columns_line_doesnt_crash(s in "\\PC*") {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let _ = Parser::<f32>::columns_line(LocatedSpan::new_extra(&s, info));
        } else {
          let _ = Parser::<f32>::columns_line(&s);
        }
      }
    }
  }

  proptest! {
    #[test]
    fn test_columns_doesnt_crash(s in "\\PC*") {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let _ = Parser::<f32>::columns(LocatedSpan::new_extra(&s, info));
        } else {
          let _ = Parser::<f32>::columns(&s);
        }
      }
    }
  }

  proptest! {
    #[test]
    fn test_rhs_line_doesnt_crash(s in "\\PC*") {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let _ = Parser::<f32>::rhs_line(LocatedSpan::new_extra(&s, info));
        } else {
          let _ = Parser::<f32>::rhs_line(&s);
        }
      }
    }
  }

  proptest! {
    #[test]
    fn test_rhs_doesnt_crash(s in "\\PC*") {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let _ = Parser::<f32>::rhs(LocatedSpan::new_extra(&s, info));
        } else {
          let _ = Parser::<f32>::rhs(&s);
        }
      }
    }
  }

  proptest! {
    #[test]
    fn test_ranges_line_doesnt_crash(s in "\\PC*") {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let _ = Parser::<f32>::ranges_line(LocatedSpan::new_extra(&s, info));
        } else {
          let _ = Parser::<f32>::ranges_line(&s);
        }
      }
    }
  }

  proptest! {
    #[test]
    fn test_ranges_doesnt_crash(s in "\\PC*") {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let _ = Parser::<f32>::ranges(LocatedSpan::new_extra(&s, info));
        } else {
          let _ = Parser::<f32>::ranges(&s);
        }
      }
    }
  }

  proptest! {
    #[test]
    fn test_bound_type_doesnt_crash(s in "\\PC*") {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let _ = Parser::<f32>::bound_type(LocatedSpan::new_extra(&s, info));
        } else {
          let _ = Parser::<f32>::bound_type(&s);
        }
      }
    }
  }

  proptest! {
    #[test]
    fn test_bounds_line_doesnt_crash(s in "\\PC*") {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let _ = Parser::<f32>::bounds_line(LocatedSpan::new_extra(&s, info));
        } else {
          let _ = Parser::<f32>::bounds_line(&s);
        }
      }
    }
  }

  proptest! {
    #[test]
    fn test_bounds_doesnt_crash(s in "\\PC*") {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let _ = Parser::<f32>::bounds(LocatedSpan::new_extra(&s, info));
        } else {
          let _ = Parser::<f32>::bounds(&s);
        }
      }
    }
  }

  proptest! {
    #[test]
    fn test_parse_doesnt_crash(s in "\\PC*") {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let _ = Parser::<f32>::parse(LocatedSpan::new_extra(&s, info));
        } else {
          let _ = Parser::<f32>::parse(&s);
        }
      }
    }
  }
}
