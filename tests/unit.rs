mod tests {
  use color_eyre::Result;
  use mps::file::*;
  use num_traits::float::Float;
  cfg_if::cfg_if! {
    if #[cfg(feature = "located")] {
      use nom_locate::LocatedSpan;
      use nom_tracable::TracableInfo;
    }
  }

  #[derive(Debug)]
  struct TestData<T> {
    input: &'static str,
    expected: (&'static str, T),
  }

  #[test]
  fn test_name() -> Result<()> {
    let test_cases = vec![TestData {
      input: "NAME          AFIRO\n",
      expected: ("", "AFIRO"),
    }];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = MPSFile::<f32>::name(LocatedSpan::new_extra(&case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = MPSFile::<f32>::name(&case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  #[test]
  fn test_row_line() -> Result<()> {
    let test_cases = vec![
      TestData {
        input: " E  R09\n",
        expected: (
          "",
          RowLine {
            row_type: RowType::try_from('E')?,
            row_name: "R09",
          },
        ),
      },
      TestData {
        input: " E  R10\n",
        expected: (
          "",
          RowLine {
            row_type: RowType::try_from('E')?,
            row_name: "R10",
          },
        ),
      },
      TestData {
        input: " L  X05\n",
        expected: (
          "",
          RowLine {
            row_type: RowType::try_from('L')?,
            row_name: "X05",
          },
        ),
      },
      TestData {
        input: " L  X21\n",
        expected: (
          "",
          RowLine {
            row_type: RowType::try_from('L')?,
            row_name: "X21",
          },
        ),
      },
    ];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = MPSFile::<f32>::row_line(LocatedSpan::new_extra(&case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = MPSFile::<f32>::row_line(&case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  #[test]
  fn test_rows() -> Result<()> {
    let test_cases = vec![TestData {
      input: "ROWS\n E  R09\n E  R10\n L  X05\n L  X21\nCOLUMNS",
      expected: (
        "COLUMNS",
        vec![
          RowLine {
            row_type: RowType::try_from('E')?,
            row_name: "R09",
          },
          RowLine {
            row_type: RowType::try_from('E')?,
            row_name: "R10",
          },
          RowLine {
            row_type: RowType::try_from('L')?,
            row_name: "X05",
          },
          RowLine {
            row_type: RowType::try_from('L')?,
            row_name: "X21",
          },
        ],
      ),
    }];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = MPSFile::<f32>::rows(LocatedSpan::new_extra(&case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = MPSFile::<f32>::rows(&case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  #[test]
  fn test_columns_line() -> Result<()> {
    let test_cases = vec![
      TestData {
        input:
          "    X01       X48               .301   R09                -1.\n",
        expected: (
          "",
          WideLine::<f32> {
            name: "X01",
            first_pair: RowValuePair {
              row_name: "X48",
              value: 0.301,
            },
            second_pair: Some(RowValuePair {
              row_name: "R09",
              value: -1.0,
            }),
          },
        ),
      },
      TestData {
        input: "    X02       COST               -.4\n",
        expected: (
          "",
          WideLine::<f32> {
            name: "X02",
            first_pair: RowValuePair {
              row_name: "COST",
              value: -0.4,
            },
            second_pair: None,
          },
        ),
      },
    ];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = MPSFile::<f32>::columns_line(LocatedSpan::new_extra(&case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = MPSFile::<f32>::columns_line(&case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  #[test]
  fn test_columns() -> Result<()> {
    let test_cases = vec![TestData {
      input: "COLUMNS
    X01       X48               .301   R09                -1.
    X01       R10              -1.06   X05                 1.
    X02       X21                -1.   R09                 1.
    X02       COST               -.4
    X03       X46                -1.   R09                 1.\nRHS",
      expected: (
        "RHS",
        vec![
          WideLine::<f32> {
            name: "X01",
            first_pair: RowValuePair {
              row_name: "X48",
              value: 0.301,
            },
            second_pair: Some(RowValuePair {
              row_name: "R09",
              value: -1.0,
            }),
          },
          WideLine::<f32> {
            name: "X01",
            first_pair: RowValuePair {
              row_name: "R10",
              value: -1.06,
            },
            second_pair: Some(RowValuePair {
              row_name: "X05",
              value: 1.0,
            }),
          },
          WideLine::<f32> {
            name: "X02",
            first_pair: RowValuePair {
              row_name: "X21",
              value: -1.0,
            },
            second_pair: Some(RowValuePair {
              row_name: "R09",
              value: 1.0,
            }),
          },
          WideLine::<f32> {
            name: "X02",
            first_pair: RowValuePair {
              row_name: "COST",
              value: -0.4,
            },
            second_pair: None,
          },
          WideLine::<f32> {
            name: "X03",
            first_pair: RowValuePair {
              row_name: "X46",
              value: -1.0,
            },
            second_pair: Some(RowValuePair {
              row_name: "R09",
              value: 1.0,
            }),
          },
        ],
      ),
    }];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = MPSFile::<f32>::columns(LocatedSpan::new_extra(&case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = MPSFile::<f32>::columns(&case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  #[test]
  fn test_rhs_line() -> Result<()> {
    let test_cases = vec![
      TestData {
        input:
          "    RHS1      LIM1                 5   LIM2                10\n",
        expected: (
          "",
          WideLine::<f32> {
            name: "RHS1",
            first_pair: RowValuePair {
              row_name: "LIM1",
              value: 5.0,
            },
            second_pair: Some(RowValuePair {
              row_name: "LIM2",
              value: 10.0,
            }),
          },
        ),
      },
      TestData {
        input: "    RHS1      MYEQN                7\n",
        expected: (
          "",
          WideLine::<f32> {
            name: "RHS1",
            first_pair: RowValuePair {
              row_name: "MYEQN",
              value: 7.0,
            },
            second_pair: None,
          },
        ),
      },
    ];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = MPSFile::<f32>::rhs_line(LocatedSpan::new_extra(&case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = MPSFile::<f32>::rhs_line(&case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  #[test]
  fn test_rhs() -> Result<()> {
    let test_cases = vec![TestData {
      input: "RHS
    RHS1      LIM1                 5   LIM2                10
    RHS1      MYEQN                7\nBOUNDS",
      expected: (
        "BOUNDS",
        vec![
          WideLine::<f32> {
            name: "RHS1",
            first_pair: RowValuePair {
              row_name: "LIM1",
              value: 5.0,
            },
            second_pair: Some(RowValuePair {
              row_name: "LIM2",
              value: 10.0,
            }),
          },
          WideLine::<f32> {
            name: "RHS1",
            first_pair: RowValuePair {
              row_name: "MYEQN",
              value: 7.0,
            },
            second_pair: None,
          },
        ],
      ),
    }];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = MPSFile::<f32>::rhs(LocatedSpan::new_extra(&case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = MPSFile::<f32>::rhs(&case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  #[test]
  fn test_ranges_line() -> Result<()> {
    let test_cases = vec![
      TestData {
        input:
          "    RANGE1    VILLKOR6           2.5   VILLKOR7           30.\n",
        expected: (
          "",
          WideLine::<f32> {
            name: "RANGE1",
            first_pair: RowValuePair {
              row_name: "VILLKOR6",
              value: 2.5,
            },
            second_pair: Some(RowValuePair {
              row_name: "VILLKOR7",
              value: 30.0,
            }),
          },
        ),
      },
      TestData {
        input: "    RANGE1    VILLKOR8           7.5\n",
        expected: (
          "",
          WideLine::<f32> {
            name: "RANGE1",
            first_pair: RowValuePair {
              row_name: "VILLKOR8",
              value: 7.5,
            },
            second_pair: None,
          },
        ),
      },
    ];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = MPSFile::<f32>::ranges_line(LocatedSpan::new_extra(&case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = MPSFile::<f32>::ranges_line(&case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  #[test]
  fn test_ranges() -> Result<()> {
    let test_cases = vec![TestData {
      input: "RANGES
    RANGE1    VILLKOR2            7.   VILLKOR3            7.
    RANGE1    VILLKOR4           3.5   VILLKOR5           10.
    RANGE1    VILLKOR6           2.5   VILLKOR7           30.
    RANGE1    VILLKOR8           7.5\nBOUNDS",
      expected: (
        "BOUNDS",
        vec![
          WideLine::<f32> {
            name: "RANGE1",
            first_pair: RowValuePair {
              row_name: "VILLKOR2",
              value: 7.0,
            },
            second_pair: Some(RowValuePair {
              row_name: "VILLKOR3",
              value: 7.0,
            }),
          },
          WideLine::<f32> {
            name: "RANGE1",
            first_pair: RowValuePair {
              row_name: "VILLKOR4",
              value: 3.5,
            },
            second_pair: Some(RowValuePair {
              row_name: "VILLKOR5",
              value: 10.0,
            }),
          },
          WideLine::<f32> {
            name: "RANGE1",
            first_pair: RowValuePair {
              row_name: "VILLKOR6",
              value: 2.5,
            },
            second_pair: Some(RowValuePair {
              row_name: "VILLKOR7",
              value: 30.0,
            }),
          },
          WideLine::<f32> {
            name: "RANGE1",
            first_pair: RowValuePair {
              row_name: "VILLKOR8",
              value: 7.5,
            },
            second_pair: None,
          },
        ],
      ),
    }];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = MPSFile::<f32>::ranges(LocatedSpan::new_extra(&case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = MPSFile::<f32>::ranges(&case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  #[test]
  fn test_bound_type() -> Result<()> {
    let test_cases = vec![
      TestData {
        input: "LO",
        expected: ("", BoundType::Lo),
      },
      TestData {
        input: "UP",
        expected: ("", BoundType::Up),
      },
      TestData {
        input: "FX",
        expected: ("", BoundType::Fx),
      },
      TestData {
        input: "FR",
        expected: ("", BoundType::Fr),
      },
      TestData {
        input: "MI",
        expected: ("", BoundType::Mi),
      },
      TestData {
        input: "PL",
        expected: ("", BoundType::Pl),
      },
    ];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = MPSFile::<f32>::bound_type(LocatedSpan::new_extra(&case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = MPSFile::<f32>::bound_type(&case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  #[test]
  fn test_bounds_line() -> Result<()> {
    let test_cases = vec![
      TestData {
        input: " UP BND1      XONE                 4\n",
        expected: (
          "",
          BoundsLine::<f32> {
            bound_type: BoundType::Up,
            bound_name: "BND1",
            column_name: "XONE",
            value: 4.0,
          },
        ),
      },
      TestData {
        input: " LO BND1      YTWO                -1\n",
        expected: (
          "",
          BoundsLine::<f32> {
            bound_type: BoundType::Lo,
            bound_name: "BND1",
            column_name: "YTWO",
            value: -1.0,
          },
        ),
      },
      TestData {
        input: " UP BND1      YTWO                 1\n",
        expected: (
          "",
          BoundsLine::<f32> {
            bound_type: BoundType::Up,
            bound_name: "BND1",
            column_name: "YTWO",
            value: 1.0,
          },
        ),
      },
    ];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = MPSFile::<f32>::bounds_line(LocatedSpan::new_extra(&case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = MPSFile::<f32>::bounds_line(&case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  #[test]
  fn test_bounds() -> Result<()> {
    let test_cases = vec![TestData {
      input: "BOUNDS
 UP BND1      XONE                 4
 LO BND1      YTWO                -1
 UP BND1      YTWO                 1\nENDATA",
      expected: (
        "ENDATA",
        vec![
          BoundsLine::<f32> {
            bound_type: BoundType::Up,
            bound_name: "BND1",
            column_name: "XONE",
            value: 4.0,
          },
          BoundsLine::<f32> {
            bound_type: BoundType::Lo,
            bound_name: "BND1",
            column_name: "YTWO",
            value: -1.0,
          },
          BoundsLine::<f32> {
            bound_type: BoundType::Up,
            bound_name: "BND1",
            column_name: "YTWO",
            value: 1.0,
          },
        ],
      ),
    }];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = MPSFile::<f32>::bounds(LocatedSpan::new_extra(&case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = MPSFile::<f32>::bounds(&case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  #[test]
  fn test_parse() -> Result<()> {
    let test_cases = vec![TestData {
      input: include_str!("../data/netlib/afiro"),
      expected: (
        "\n",
        MPSFile {
          name: "AFIRO",
          rows: vec![
            RowLine {
              row_type: RowType::Eq,
              row_name: "R09",
            },
            RowLine {
              row_type: RowType::Eq,
              row_name: "R10",
            },
            RowLine {
              row_type: RowType::Leq,
              row_name: "X05",
            },
            RowLine {
              row_type: RowType::Leq,
              row_name: "X21",
            },
            RowLine {
              row_type: RowType::Eq,
              row_name: "R12",
            },
            RowLine {
              row_type: RowType::Eq,
              row_name: "R13",
            },
            RowLine {
              row_type: RowType::Leq,
              row_name: "X17",
            },
            RowLine {
              row_type: RowType::Leq,
              row_name: "X18",
            },
            RowLine {
              row_type: RowType::Leq,
              row_name: "X19",
            },
            RowLine {
              row_type: RowType::Leq,
              row_name: "X20",
            },
            RowLine {
              row_type: RowType::Eq,
              row_name: "R19",
            },
            RowLine {
              row_type: RowType::Eq,
              row_name: "R20",
            },
            RowLine {
              row_type: RowType::Leq,
              row_name: "X27",
            },
            RowLine {
              row_type: RowType::Leq,
              row_name: "X44",
            },
            RowLine {
              row_type: RowType::Eq,
              row_name: "R22",
            },
            RowLine {
              row_type: RowType::Eq,
              row_name: "R23",
            },
            RowLine {
              row_type: RowType::Leq,
              row_name: "X40",
            },
            RowLine {
              row_type: RowType::Leq,
              row_name: "X41",
            },
            RowLine {
              row_type: RowType::Leq,
              row_name: "X42",
            },
            RowLine {
              row_type: RowType::Leq,
              row_name: "X43",
            },
            RowLine {
              row_type: RowType::Leq,
              row_name: "X45",
            },
            RowLine {
              row_type: RowType::Leq,
              row_name: "X46",
            },
            RowLine {
              row_type: RowType::Leq,
              row_name: "X47",
            },
            RowLine {
              row_type: RowType::Leq,
              row_name: "X48",
            },
            RowLine {
              row_type: RowType::Leq,
              row_name: "X49",
            },
            RowLine {
              row_type: RowType::Leq,
              row_name: "X50",
            },
            RowLine {
              row_type: RowType::Leq,
              row_name: "X51",
            },
            RowLine {
              row_type: RowType::Nr,
              row_name: "COST",
            },
          ],
          columns: vec![
            WideLine {
              name: "X01",
              first_pair: RowValuePair {
                row_name: "X48",
                value: 0.301,
              },
              second_pair: Some(RowValuePair {
                row_name: "R09",
                value: -1.0,
              }),
            },
            WideLine {
              name: "X01",
              first_pair: RowValuePair {
                row_name: "R10",
                value: -1.06,
              },
              second_pair: Some(RowValuePair {
                row_name: "X05",
                value: 1.0,
              }),
            },
            WideLine {
              name: "X02",
              first_pair: RowValuePair {
                row_name: "X21",
                value: -1.0,
              },
              second_pair: Some(RowValuePair {
                row_name: "R09",
                value: 1.0,
              }),
            },
            WideLine {
              name: "X02",
              first_pair: RowValuePair {
                row_name: "COST",
                value: -0.4,
              },
              second_pair: None,
            },
            WideLine {
              name: "X03",
              first_pair: RowValuePair {
                row_name: "X46",
                value: -1.0,
              },
              second_pair: Some(RowValuePair {
                row_name: "R09",
                value: 1.0,
              }),
            },
            WideLine {
              name: "X04",
              first_pair: RowValuePair {
                row_name: "X50",
                value: 1.0,
              },
              second_pair: Some(RowValuePair {
                row_name: "R10",
                value: 1.0,
              }),
            },
            WideLine {
              name: "X06",
              first_pair: RowValuePair {
                row_name: "X49",
                value: 0.301,
              },
              second_pair: Some(RowValuePair {
                row_name: "R12",
                value: -1.0,
              }),
            },
            WideLine {
              name: "X06",
              first_pair: RowValuePair {
                row_name: "R13",
                value: -1.06,
              },
              second_pair: Some(RowValuePair {
                row_name: "X17",
                value: 1.0,
              }),
            },
            WideLine {
              name: "X07",
              first_pair: RowValuePair {
                row_name: "X49",
                value: 0.313,
              },
              second_pair: Some(RowValuePair {
                row_name: "R12",
                value: -1.0,
              }),
            },
            WideLine {
              name: "X07",
              first_pair: RowValuePair {
                row_name: "R13",
                value: -1.06,
              },
              second_pair: Some(RowValuePair {
                row_name: "X18",
                value: 1.0,
              }),
            },
            WideLine {
              name: "X08",
              first_pair: RowValuePair {
                row_name: "X49",
                value: 0.313,
              },
              second_pair: Some(RowValuePair {
                row_name: "R12",
                value: -1.0,
              }),
            },
            WideLine {
              name: "X08",
              first_pair: RowValuePair {
                row_name: "R13",
                value: -0.96,
              },
              second_pair: Some(RowValuePair {
                row_name: "X19",
                value: 1.0,
              }),
            },
            WideLine {
              name: "X09",
              first_pair: RowValuePair {
                row_name: "X49",
                value: 0.326,
              },
              second_pair: Some(RowValuePair {
                row_name: "R12",
                value: -1.0,
              }),
            },
            WideLine {
              name: "X09",
              first_pair: RowValuePair {
                row_name: "R13",
                value: -0.86,
              },
              second_pair: Some(RowValuePair {
                row_name: "X20",
                value: 1.0,
              }),
            },
            WideLine {
              name: "X10",
              first_pair: RowValuePair {
                row_name: "X45",
                value: 2.364,
              },
              second_pair: Some(RowValuePair {
                row_name: "X17",
                value: -1.0,
              }),
            },
            WideLine {
              name: "X11",
              first_pair: RowValuePair {
                row_name: "X45",
                value: 2.386,
              },
              second_pair: Some(RowValuePair {
                row_name: "X18",
                value: -1.0,
              }),
            },
            WideLine {
              name: "X12",
              first_pair: RowValuePair {
                row_name: "X45",
                value: 2.408,
              },
              second_pair: Some(RowValuePair {
                row_name: "X19",
                value: -1.0,
              }),
            },
            WideLine {
              name: "X13",
              first_pair: RowValuePair {
                row_name: "X45",
                value: 2.429,
              },
              second_pair: Some(RowValuePair {
                row_name: "X20",
                value: -1.0,
              }),
            },
            WideLine {
              name: "X14",
              first_pair: RowValuePair {
                row_name: "X21",
                value: 1.4,
              },
              second_pair: Some(RowValuePair {
                row_name: "R12",
                value: 1.0,
              }),
            },
            WideLine {
              name: "X14",
              first_pair: RowValuePair {
                row_name: "COST",
                value: -0.32,
              },
              second_pair: None,
            },
            WideLine {
              name: "X15",
              first_pair: RowValuePair {
                row_name: "X47",
                value: -1.0,
              },
              second_pair: Some(RowValuePair {
                row_name: "R12",
                value: 1.0,
              }),
            },
            WideLine {
              name: "X16",
              first_pair: RowValuePair {
                row_name: "X51",
                value: 1.0,
              },
              second_pair: Some(RowValuePair {
                row_name: "R13",
                value: 1.0,
              }),
            },
            WideLine {
              name: "X22",
              first_pair: RowValuePair {
                row_name: "X46",
                value: 0.109,
              },
              second_pair: Some(RowValuePair {
                row_name: "R19",
                value: -1.0,
              }),
            },
            WideLine {
              name: "X22",
              first_pair: RowValuePair {
                row_name: "R20",
                value: -0.43,
              },
              second_pair: Some(RowValuePair {
                row_name: "X27",
                value: 1.0,
              }),
            },
            WideLine {
              name: "X23",
              first_pair: RowValuePair {
                row_name: "X44",
                value: -1.0,
              },
              second_pair: Some(RowValuePair {
                row_name: "R19",
                value: 1.0,
              }),
            },
            WideLine {
              name: "X23",
              first_pair: RowValuePair {
                row_name: "COST",
                value: -0.6,
              },
              second_pair: None,
            },
            WideLine {
              name: "X24",
              first_pair: RowValuePair {
                row_name: "X48",
                value: -1.0,
              },
              second_pair: Some(RowValuePair {
                row_name: "R19",
                value: 1.0,
              }),
            },
            WideLine {
              name: "X25",
              first_pair: RowValuePair {
                row_name: "X45",
                value: -1.0,
              },
              second_pair: Some(RowValuePair {
                row_name: "R19",
                value: 1.0,
              }),
            },
            WideLine {
              name: "X26",
              first_pair: RowValuePair {
                row_name: "X50",
                value: 1.0,
              },
              second_pair: Some(RowValuePair {
                row_name: "R20",
                value: 1.0,
              }),
            },
            WideLine {
              name: "X28",
              first_pair: RowValuePair {
                row_name: "X47",
                value: 0.109,
              },
              second_pair: Some(RowValuePair {
                row_name: "R22",
                value: -0.43,
              }),
            },
            WideLine {
              name: "X28",
              first_pair: RowValuePair {
                row_name: "R23",
                value: 1.0,
              },
              second_pair: Some(RowValuePair {
                row_name: "X40",
                value: 1.0,
              }),
            },
            WideLine {
              name: "X29",
              first_pair: RowValuePair {
                row_name: "X47",
                value: 0.108,
              },
              second_pair: Some(RowValuePair {
                row_name: "R22",
                value: -0.43,
              }),
            },
            WideLine {
              name: "X29",
              first_pair: RowValuePair {
                row_name: "R23",
                value: 1.0,
              },
              second_pair: Some(RowValuePair {
                row_name: "X41",
                value: 1.0,
              }),
            },
            WideLine {
              name: "X30",
              first_pair: RowValuePair {
                row_name: "X47",
                value: 0.108,
              },
              second_pair: Some(RowValuePair {
                row_name: "R22",
                value: -0.39,
              }),
            },
            WideLine {
              name: "X30",
              first_pair: RowValuePair {
                row_name: "R23",
                value: 1.0,
              },
              second_pair: Some(RowValuePair {
                row_name: "X42",
                value: 1.0,
              }),
            },
            WideLine {
              name: "X31",
              first_pair: RowValuePair {
                row_name: "X47",
                value: 0.107,
              },
              second_pair: Some(RowValuePair {
                row_name: "R22",
                value: -0.37,
              }),
            },
            WideLine {
              name: "X31",
              first_pair: RowValuePair {
                row_name: "R23",
                value: 1.0,
              },
              second_pair: Some(RowValuePair {
                row_name: "X43",
                value: 1.0,
              }),
            },
            WideLine {
              name: "X32",
              first_pair: RowValuePair {
                row_name: "X45",
                value: 2.191,
              },
              second_pair: Some(RowValuePair {
                row_name: "X40",
                value: -1.0,
              }),
            },
            WideLine {
              name: "X33",
              first_pair: RowValuePair {
                row_name: "X45",
                value: 2.219,
              },
              second_pair: Some(RowValuePair {
                row_name: "X41",
                value: -1.0,
              }),
            },
            WideLine {
              name: "X34",
              first_pair: RowValuePair {
                row_name: "X45",
                value: 2.249,
              },
              second_pair: Some(RowValuePair {
                row_name: "X42",
                value: -1.0,
              }),
            },
            WideLine {
              name: "X35",
              first_pair: RowValuePair {
                row_name: "X45",
                value: 2.279,
              },
              second_pair: Some(RowValuePair {
                row_name: "X43",
                value: -1.0,
              }),
            },
            WideLine {
              name: "X36",
              first_pair: RowValuePair {
                row_name: "X44",
                value: 1.4,
              },
              second_pair: Some(RowValuePair {
                row_name: "R23",
                value: -1.0,
              }),
            },
            WideLine {
              name: "X36",
              first_pair: RowValuePair {
                row_name: "COST",
                value: -0.48,
              },
              second_pair: None,
            },
            WideLine {
              name: "X37",
              first_pair: RowValuePair {
                row_name: "X49",
                value: -1.0,
              },
              second_pair: Some(RowValuePair {
                row_name: "R23",
                value: 1.0,
              }),
            },
            WideLine {
              name: "X38",
              first_pair: RowValuePair {
                row_name: "X51",
                value: 1.0,
              },
              second_pair: Some(RowValuePair {
                row_name: "R22",
                value: 1.0,
              }),
            },
            WideLine {
              name: "X39",
              first_pair: RowValuePair {
                row_name: "R23",
                value: 1.0,
              },
              second_pair: Some(RowValuePair {
                row_name: "COST",
                value: 10.0,
              }),
            },
          ],
          rhs: Some(vec![
            WideLine {
              name: "B",
              first_pair: RowValuePair {
                row_name: "X50",
                value: 310.0,
              },
              second_pair: Some(RowValuePair {
                row_name: "X51",
                value: 300.0,
              }),
            },
            WideLine {
              name: "B",
              first_pair: RowValuePair {
                row_name: "X05",
                value: 80.0,
              },
              second_pair: Some(RowValuePair {
                row_name: "X17",
                value: 80.0,
              }),
            },
            WideLine {
              name: "B",
              first_pair: RowValuePair {
                row_name: "X27",
                value: 500.0,
              },
              second_pair: Some(RowValuePair {
                row_name: "R23",
                value: 44.0,
              }),
            },
            WideLine {
              name: "B",
              first_pair: RowValuePair {
                row_name: "X40",
                value: 500.0,
              },
              second_pair: None,
            },
          ]),
          ranges: None,
          bounds: None,
        },
      ),
    }];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = MPSFile::<f32>::parse(LocatedSpan::new_extra(&case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = MPSFile::<f32>::parse(&case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  #[test]
  fn test_endata() -> Result<()> {
    let test_cases = vec![
      TestData {
        input: "ENDATA\n",
        expected: ("\n", "ENDATA"),
      },
      TestData {
        input: "ENDATA",
        expected: ("", "ENDATA"),
      },
    ];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "located")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = MPSFile::<f32>::endata(LocatedSpan::new_extra(&case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = MPSFile::<f32>::endata(&case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  fn parse<T: Float>(input: &'static str) -> Result<()> {
    cfg_if::cfg_if! {
      if #[cfg(feature = "located")] {
        let info = TracableInfo::new().forward(false).backward(false);
        MPSFile::<T>::parse(LocatedSpan::new_extra(&input, info))?;
      } else {
        MPSFile::<T>::parse(&input)?;
      }
    }
    Ok(())
  }

  #[test]
  fn test_parse_agg() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/agg"))
  }

  #[test]
  fn test_parse_ship04l() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/ship04l"))
  }

  #[test]
  fn test_parse_d2q06c() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/d2q06c"))
  }

  #[test]
  fn test_parse_e226() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/e226"))
  }

  #[test]
  fn test_parse_nl25fv47() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/25fv47"))
  }

  #[test]
  fn test_parse_bore3d() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/bore3d"))
  }

  #[test]
  fn test_parse_ganges() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/ganges"))
  }

  #[test]
  fn test_parse_adlittle() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/adlittle"))
  }

  #[ignore] // TODO: Fix
  fn test_parse_forplan() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/forplan"))
  }

  #[test]
  fn test_parse_sc205() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/sc205"))
  }

  #[test]
  fn test_parse_nl80bau3b() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/80bau3b"))
  }

  #[test]
  fn test_parse_scrs8() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/scrs8"))
  }

  #[test]
  fn test_parse_wood1p() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/wood1p"))
  }

  #[test]
  fn test_parse_boeing1() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/boeing1"))
  }

  #[test]
  fn test_parse_kb2() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/kb2"))
  }

  #[test]
  fn test_parse_ship08s() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/ship08s"))
  }

  #[test]
  fn test_parse_scfxm1() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/scfxm1"))
  }

  #[test]
  fn test_parse_agg2() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/agg2"))
  }

  #[test]
  fn test_parse_finnis() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/finnis"))
  }

  #[test]
  fn test_parse_dfl001() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/dfl001"))
  }

  #[test]
  fn test_parse_pilot87() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/pilot87"))
  }

  #[test]
  fn test_parse_sctap1() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/sctap1"))
  }

  #[test]
  fn test_parse_agg3() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/agg3"))
  }

  #[test]
  fn test_parse_grow7() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/grow7"))
  }

  #[test]
  fn test_parse_scorpion() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/scorpion"))
  }

  #[test]
  fn test_parse_maros() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/maros"))
  }

  #[test]
  fn test_parse_shell() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/shell"))
  }

  #[test]
  fn test_parse_greenbeb() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/greenbeb"))
  }

  #[test]
  fn test_parse_sc50b() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/sc50b"))
  }

  #[test]
  fn test_parse_recipe() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/recipe"))
  }

  #[test]
  fn test_parse_sierra() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/sierra"))
  }

  #[test]
  fn test_parse_scagr25() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/scagr25"))
  }

  #[test]
  fn test_parse_modszk1() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/modszk1"))
  }

  #[test]
  fn test_parse_ship12l() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/ship12l"))
  }

  #[test]
  fn test_parse_stair() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/stair"))
  }

  #[test]
  fn test_parse_cycle() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/cycle"))
  }

  #[test]
  fn test_parse_sc105() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/sc105"))
  }

  #[test]
  fn test_parse_pilot_ja() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/pilot.ja"))
  }

  #[test]
  fn test_parse_beaconfd() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/beaconfd"))
  }

  #[test]
  fn test_parse_czprob() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/czprob"))
  }

  #[test]
  fn test_parse_pilot_we() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/pilot.we"))
  }

  #[test]
  fn test_parse_standgub() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/standgub"))
  }

  #[test]
  fn test_parse_standmps() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/standmps"))
  }

  #[test]
  fn test_parse_scsd8() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/scsd8"))
  }

  #[test]
  fn test_parse_woodw() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/woodw"))
  }

  #[test]
  fn test_parse_scsd6() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/scsd6"))
  }

  #[test]
  fn test_parse_scsd1() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/scsd1"))
  }

  #[test]
  fn test_parse_share2b() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/share2b"))
  }

  #[test]
  fn test_parse_gfrd_pnc() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/gfrd-pnc"))
  }

  #[test]
  fn test_parse_bnl2() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/bnl2"))
  }

  #[test]
  fn test_parse_stocfor2() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/stocfor2"))
  }

  #[test]
  fn test_parse_nesm() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/nesm"))
  }

  #[test]
  fn test_parse_share1b() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/share1b"))
  }

  #[test]
  fn test_parse_ship04s() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/ship04s"))
  }

  #[test]
  fn test_parse_grow15() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/grow15"))
  }

  #[test]
  fn test_parse_maros_r7() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/maros-r7"))
  }

  #[test]
  fn test_parse_blend() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/blend"))
  }

  #[test]
  fn test_parse_lotfi() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/lotfi"))
  }

  #[test]
  fn test_parse_standata() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/standata"))
  }

  //#[test]
  //fn test_parse_d6cube() -> Result<()> {
  //  parse::<f32>(include_str!("../data/netlib/d6cube"))
  //}

  #[test]
  fn test_parse_degen3() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/degen3"))
  }

  #[test]
  fn test_parse_capri() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/capri"))
  }

  #[test]
  fn test_parse_grow22() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/grow22"))
  }

  #[test]
  fn test_parse_etamacro() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/etamacro"))
  }

  #[test]
  fn test_parse_ship08l() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/ship08l"))
  }

  #[test]
  fn test_parse_afiro() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/afiro"))
  }

  #[test]
  fn test_parse_degen2() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/degen2"))
  }

  #[test]
  fn test_parse_boeing2() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/boeing2"))
  }

  #[test]
  fn test_parse_fit1d() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/fit1d"))
  }

  #[test]
  fn test_parse_scfxm2() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/scfxm2"))
  }

  #[test]
  fn test_parse_sctap3() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/sctap3"))
  }

  #[test]
  fn test_parse_fit1p() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/fit1p"))
  }

  #[test]
  fn test_parse_pilot() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/pilot"))
  }

  #[test]
  fn test_parse_fit2d() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/fit2d"))
  }

  #[test]
  fn test_parse_sctap2() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/sctap2"))
  }

  #[test]
  fn test_parse_scfxm3() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/scfxm3"))
  }

  #[test]
  fn test_parse_brandy() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/brandy"))
  }

  #[test]
  fn test_parse_greenbea() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/greenbea"))
  }

  #[test]
  fn test_parse_tuff() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/tuff"))
  }

  #[test]
  fn test_parse_sc50a() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/sc50a"))
  }

  #[test]
  fn test_parse_vtp_base() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/vtp.base"))
  }

  #[test]
  fn test_parse_pilotnov() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/pilotnov"))
  }

  #[test]
  fn test_parse_ship12s() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/ship12s"))
  }

  #[test]
  fn test_parse_seba() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/seba"))
  }

  #[test]
  fn test_parse_fffff800() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/fffff800"))
  }

  #[test]
  fn test_parse_israel() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/israel"))
  }

  #[test]
  fn test_parse_perold() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/perold"))
  }

  #[test]
  fn test_parse_pilot4() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/pilot4"))
  }

  #[test]
  fn test_parse_scagr7() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/scagr7"))
  }

  #[test]
  fn test_parse_bandm() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/bandm"))
  }

  #[test]
  fn test_parse_bnl1() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/bnl1"))
  }

  #[test]
  fn test_parse_stocfor1() -> Result<()> {
    parse::<f32>(include_str!("../data/netlib/stocfor1"))
  }
}
