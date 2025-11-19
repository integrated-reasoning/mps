mod tests {
  use color_eyre::Result;
  use mps::types::*;
  cfg_if::cfg_if! {
    if #[cfg(feature = "trace")] {
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
        if #[cfg(feature = "trace")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = Parser::<f32>::name(LocatedSpan::new_extra(case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = Parser::<f32>::name(case.input)?;
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
          Some(RowLine {
            row_type: RowType::try_from('E')?,
            row_name: "R09",
          }),
        ),
      },
      TestData {
        input: " N   OBJ\n",
        expected: (
          "",
          Some(RowLine {
            row_type: RowType::try_from('N')?,
            row_name: "OBJ",
          }),
        ),
      },
      TestData {
        input: " E  R10\n",
        expected: (
          "",
          Some(RowLine {
            row_type: RowType::try_from('E')?,
            row_name: "R10",
          }),
        ),
      },
      TestData {
        input: " L  X05\n",
        expected: (
          "",
          Some(RowLine {
            row_type: RowType::try_from('L')?,
            row_name: "X05",
          }),
        ),
      },
      TestData {
        input: " L  X21\n",
        expected: (
          "",
          Some(RowLine {
            row_type: RowType::try_from('L')?,
            row_name: "X21",
          }),
        ),
      },
    ];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "trace")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = Parser::<f32>::row_line_or_end(LocatedSpan::new_extra(case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = Parser::<f32>::row_line_or_end(case.input)?;
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
        if #[cfg(feature = "trace")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = Parser::<f32>::rows(LocatedSpan::new_extra(case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = Parser::<f32>::rows(case.input)?;
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
          Some(WideLine {
            name: "X01",
            first_pair: RowValuePair {
              row_name: "X48",
              value: 0.301,
            },
            second_pair: Some(RowValuePair {
              row_name: "R09",
              value: -1.0,
            }),
          }),
        ),
      },
      TestData {
        input: "    X02       COST               -.4\n",
        expected: (
          "",
          Some(WideLine {
            name: "X02",
            first_pair: RowValuePair {
              row_name: "COST",
              value: -0.4,
            },
            second_pair: None,
          }),
        ),
      },
      // Regression test: line without leading spaces and large negative value
      TestData {
        input: "    C0000823  OBJECTRW -401552000.00\n",
        expected: (
          "",
          Some(WideLine {
            name: "C0000823",
            first_pair: RowValuePair {
              row_name: "OBJECTRW",
              value: -401552000.0,
            },
            second_pair: None,
          }),
        ),
      },
      // Test case with properly formatted strict field positioning
      TestData {
        input: "    X99       ROW1             -123.0\n",
        expected: (
          "",
          Some(WideLine {
            name: "X99",
            first_pair: RowValuePair {
              row_name: "ROW1",
              value: -123.0,
            },
            second_pair: None,
          }),
        ),
      },
      // Test case with two values where sign character falls just before second value field
      // This should trigger the sign validation check for second value (line 538-546 in parse.rs)
      TestData {
        input: "    X98       ROW1              100.0   ROW2                -20\n",
        expected: (
          "",
          Some(WideLine {
            name: "X98",
            first_pair: RowValuePair {
              row_name: "ROW1",
              value: 100.0,
            },
            second_pair: Some(RowValuePair {
              row_name: "ROW2",
              value: -20.0,
            }),
          }),
        ),
      },
    ];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "trace")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = Parser::<f32>::columns_line(LocatedSpan::new_extra(case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = Parser::<f32>::columns_line(case.input)?;
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
        ],
      ),
    }];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "trace")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = Parser::<f32>::columns(LocatedSpan::new_extra(case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = Parser::<f32>::columns(case.input)?;
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
          Some(WideLine {
            name: "RHS1",
            first_pair: RowValuePair {
              row_name: "LIM1",
              value: 5.0,
            },
            second_pair: Some(RowValuePair {
              row_name: "LIM2",
              value: 10.0,
            }),
          }),
        ),
      },
      TestData {
        input: "    RHS1      MYEQN                7\n",
        expected: (
          "",
          Some(WideLine {
            name: "RHS1",
            first_pair: RowValuePair {
              row_name: "MYEQN",
              value: 7.0,
            },
            second_pair: None,
          }),
        ),
      },
    ];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "trace")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = Parser::<f32>::rhs_line(LocatedSpan::new_extra(case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = Parser::<f32>::rhs_line(case.input)?;
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
          WideLine {
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
          WideLine {
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
        if #[cfg(feature = "trace")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = Parser::<f32>::rhs(LocatedSpan::new_extra(case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = Parser::<f32>::rhs(case.input)?;
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
          Some(WideLine {
            name: "RANGE1",
            first_pair: RowValuePair {
              row_name: "VILLKOR6",
              value: 2.5,
            },
            second_pair: Some(RowValuePair {
              row_name: "VILLKOR7",
              value: 30.0,
            }),
          }),
        ),
      },
      TestData {
        input: "    RANGE1    VILLKOR8           7.5\n",
        expected: (
          "",
          Some(WideLine {
            name: "RANGE1",
            first_pair: RowValuePair {
              row_name: "VILLKOR8",
              value: 7.5,
            },
            second_pair: None,
          }),
        ),
      },
    ];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "trace")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = Parser::<f32>::ranges_line(LocatedSpan::new_extra(case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = Parser::<f32>::ranges_line(case.input)?;
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
          WideLine {
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
          WideLine {
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
          WideLine {
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
          WideLine {
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
        if #[cfg(feature = "trace")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = Parser::<f32>::ranges(LocatedSpan::new_extra(case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = Parser::<f32>::ranges(case.input)?;
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
        if #[cfg(feature = "trace")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = Parser::<f32>::bound_type(LocatedSpan::new_extra(case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = Parser::<f32>::bound_type(case.input)?;
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
          Some(BoundsLine {
            bound_type: BoundType::Up,
            bound_name: "BND1",
            column_name: "XONE",
            value: Some(4.0),
          }),
        ),
      },
      TestData {
        input: " LO BND1      YTWO                -1\n",
        expected: (
          "",
          Some(BoundsLine {
            bound_type: BoundType::Lo,
            bound_name: "BND1",
            column_name: "YTWO",
            value: Some(-1.0),
          }),
        ),
      },
      TestData {
        input: " UP BND1      YTWO                 1\n",
        expected: (
          "",
          Some(BoundsLine {
            bound_type: BoundType::Up,
            bound_name: "BND1",
            column_name: "YTWO",
            value: Some(1.0),
          }),
        ),
      },
      // Test FR (Free) bound type - no value field (line 924-932 in parse.rs)
      TestData {
        input: " FR BND1      XFREE\n",
        expected: (
          "",
          Some(BoundsLine {
            bound_type: BoundType::Fr,
            bound_name: "BND1",
            column_name: "XFREE",
            value: None,
          }),
        ),
      },
      // Test PL (Plus infinity) bound type - no value field (line 924-932 in parse.rs)
      TestData {
        input: " PL BND1      XPLUS\n",
        expected: (
          "",
          Some(BoundsLine {
            bound_type: BoundType::Pl,
            bound_name: "BND1",
            column_name: "XPLUS",
            value: None,
          }),
        ),
      },
      // Test FX (Fixed) bound type with value - should trigger sign check (line 934-956 in parse.rs)
      TestData {
        input: " FX BND1      XFIXED               -5\n",
        expected: (
          "",
          Some(BoundsLine {
            bound_type: BoundType::Fx,
            bound_name: "BND1",
            column_name: "XFIXED",
            value: Some(-5.0),
          }),
        ),
      },
      // Test MI (Minus infinity) bound type with value - should trigger sign check (line 934-956 in parse.rs)
      TestData {
        input: " MI BND1      XMINUS\n",
        expected: (
          "",
          Some(BoundsLine {
            bound_type: BoundType::Mi,
            bound_name: "BND1",
            column_name: "XMINUS",
            value: None,
          }),
        ),
      },
    ];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "trace")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = Parser::<f32>::bounds_line(LocatedSpan::new_extra(case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = Parser::<f32>::bounds_line(case.input)?;
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
          BoundsLine {
            bound_type: BoundType::Up,
            bound_name: "BND1",
            column_name: "XONE",
            value: Some(4.0),
          },
          BoundsLine {
            bound_type: BoundType::Lo,
            bound_name: "BND1",
            column_name: "YTWO",
            value: Some(-1.0),
          },
          BoundsLine {
            bound_type: BoundType::Up,
            bound_name: "BND1",
            column_name: "YTWO",
            value: Some(1.0),
          },
        ],
      ),
    }];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "trace")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = Parser::<f32>::bounds(LocatedSpan::new_extra(case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = Parser::<f32>::bounds(case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  #[test]
  fn test_parse() -> Result<()> {
    let test_cases = vec![TestData {
      input: include_str!("../tests/data/netlib/afiro"),
      expected: (
        "\n",
        Parser {
          name: "AFIRO",
          objective_sense: None,
          objective_name: None,
          reference_row: None,
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
          user_cuts: None,
          indicators: None,
          lazy_constraints: None,
          quadratic_objective: None,
          special_ordered_sets: None,
          quadratic_constraints: None,
          cone_constraints: None,
          branch_priorities: None,
        },
      ),
    }];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "trace")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = Parser::<f32>::mps_file(LocatedSpan::new_extra(case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = Parser::<f32>::mps_file(case.input)?;
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
        if #[cfg(feature = "trace")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = Parser::<f32>::endata(LocatedSpan::new_extra(case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = Parser::<f32>::endata(case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  #[test]
  fn test_objsen() -> Result<()> {
    let test_cases = vec![
      TestData {
        input: "OBJSENSE\nMAX\n",
        expected: ("", ObjectiveSense::Max),
      },
      TestData {
        input: "OBJSENSE\nMIN\n",
        expected: ("", ObjectiveSense::Min),
      },
    ];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "trace")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = Parser::<f32>::objsen(LocatedSpan::new_extra(case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = Parser::<f32>::objsen(case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  #[test]
  fn test_objname() -> Result<()> {
    let test_cases = vec![
      TestData {
        input: "OBJNAME\nmy_objective\n",
        expected: ("", "my_objective"),
      },
      TestData {
        input: "OBJNAME\nobjective_row\n",
        expected: ("", "objective_row"),
      },
    ];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "trace")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = Parser::<f32>::objname(LocatedSpan::new_extra(case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = Parser::<f32>::objname(case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  #[test]
  fn test_refrow() -> Result<()> {
    let test_cases = vec![
      TestData {
        input: "REFROW\nweights\n",
        expected: ("", "weights"),
      },
      TestData {
        input: "REFROW\nref_constraint\n",
        expected: ("", "ref_constraint"),
      },
    ];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "trace")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = Parser::<f32>::refrow(LocatedSpan::new_extra(case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = Parser::<f32>::refrow(case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  #[test]
  fn test_usercuts() -> Result<()> {
    let test_cases = vec![TestData {
      input: "USERCUTS\n E  cut1\n E  cut2\n E  cut3\nROWS",
      expected: (
        "ROWS",
        vec![
          RowLine {
            row_type: RowType::Eq,
            row_name: "cut1",
          },
          RowLine {
            row_type: RowType::Eq,
            row_name: "cut2",
          },
          RowLine {
            row_type: RowType::Eq,
            row_name: "cut3",
          },
        ],
      ),
    }];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "trace")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = Parser::<f32>::usercuts(LocatedSpan::new_extra(case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = Parser::<f32>::usercuts(case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  #[test]
  fn test_qmatrix_line() -> Result<()> {
    let test_cases = vec![
      TestData {
        input: " x y 2.5\n",
        expected: (
          "",
          Some(QuadraticTerm {
            var1: "x",
            var2: "y",
            coefficient: 2.5,
          }),
        ),
      },
      TestData {
        input: " a b -1.0\n",
        expected: (
          "",
          Some(QuadraticTerm {
            var1: "a",
            var2: "b",
            coefficient: -1.0,
          }),
        ),
      },
    ];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "trace")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = Parser::<f32>::qmatrix_line(LocatedSpan::new_extra(case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = Parser::<f32>::qmatrix_line(case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  #[test]
  fn test_qmatrix() -> Result<()> {
    let test_cases = vec![TestData {
      input: "QMATRIX\n x y 2.0\n x x 1.0\n y y 7.0\nENDATA",
      expected: (
        "ENDATA",
        vec![QuadraticConstraint {
          row_name: "OBJ",
          terms: vec![
            QuadraticTerm {
              var1: "x",
              var2: "y",
              coefficient: 2.0,
            },
            QuadraticTerm {
              var1: "x",
              var2: "x",
              coefficient: 1.0,
            },
            QuadraticTerm {
              var1: "y",
              var2: "y",
              coefficient: 7.0,
            },
          ],
        }],
      ),
    }];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "trace")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = Parser::<f32>::qmatrix(LocatedSpan::new_extra(case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = Parser::<f32>::qmatrix(case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  #[test]
  fn test_csection() -> Result<()> {
    let test_cases = vec![TestData {
      input: "CSECTION\n QUAD\n x\n y\n z\nENDATA",
      expected: (
        "ENDATA",
        vec![ConeConstraint {
          cone_name: "CONE",
          cone_type: ConeType::Quad,
          members: vec![
            ConeMember {
              var_name: "x",
              coefficient: None,
            },
            ConeMember {
              var_name: "y",
              coefficient: None,
            },
            ConeMember {
              var_name: "z",
              coefficient: None,
            },
          ],
        }],
      ),
    }];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "trace")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = Parser::<f32>::csection(LocatedSpan::new_extra(case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = Parser::<f32>::csection(case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  #[test]
  fn test_qsection() -> Result<()> {
    let test_cases = vec![TestData {
      input: "QSECTION\n x y 2.0\n x x 1.0\n y y 7.0\nENDATA",
      expected: (
        "ENDATA",
        vec![
          QuadraticObjectiveTerm {
            var1: "x",
            var2: "y",
            coefficient: 2.0,
          },
          QuadraticObjectiveTerm {
            var1: "x",
            var2: "x",
            coefficient: 1.0,
          },
          QuadraticObjectiveTerm {
            var1: "y",
            var2: "y",
            coefficient: 7.0,
          },
        ],
      ),
    }];
    for case in test_cases {
      cfg_if::cfg_if! {
        if #[cfg(feature = "trace")] {
          let info = TracableInfo::new().forward(false).backward(false);
          let (s, x) = Parser::<f32>::qsection(LocatedSpan::new_extra(case.input, info))?;
          assert_eq!((*s.fragment(), x), case.expected);
        } else {
          let (s, x) = Parser::<f32>::qsection(case.input)?;
          assert_eq!((s, x), case.expected);
        }
      }
    }
    Ok(())
  }

  /// Comprehensive integration test with all section types in correct CPLEX order
  #[test]
  fn test_full_mps_with_all_sections() -> Result<()> {
    let input = r#"NAME          FULLTEST
OBJSENSE
MAX
OBJNAME
cost
REFROW
weights
ROWS
 N  cost
 L  c1
 E  c2
 G  c3
USERCUTS
 L  cut1
COLUMNS
    x1        cost                 1.0   c1                    -1.0
    x1        c2                   1.0
    x2        cost                 2.0   c1                    1.0
    x2        c2                   -3.0  c3                    1.0
    x3        cost                 3.0   c1                    1.0
    x3        c2                   1.0
RHS
    rhs1      c1                  20.0   c2                   30.0
RANGES
    rng1      c1                  15.0
BOUNDS
 UP bnd1      x1                  40.0
 LO bnd1      x2                   0.0
 FX bnd1      x3                   5.0
SOS
 S1 set1
    x1 1.0
    x2 2.0
QSECTION
    x1        x1                  2.0
    x1        x2                  1.0
    x2        x2                  3.0
QCMATRIX      qc1
    x1        x1                  1.0
    x1        x2                  0.5
    x2        x2                  1.5
INDICATORS
 IF c1 x2 1
LAZYCONS
    L  lazy1
ENDATA
"#;

    let parser = Parser::<f64>::parse(input)?;

    // Verify core sections
    assert_eq!(parser.name, "FULLTEST");
    assert_eq!(parser.objective_sense, Some(ObjectiveSense::Max));
    assert_eq!(parser.objective_name, Some("cost"));
    assert_eq!(parser.reference_row, Some("weights"));

    // Verify required sections
    // 4 rows: cost (N), c1 (L), c2 (E), c3 (G), cut1 (L in USERCUTS)
    assert_eq!(parser.rows.len(), 4);
    // 6 column lines: x1 appears twice, x2 appears twice, x3 appears twice
    // (each variable can appear on multiple lines if it has coefficients in multiple rows)
    assert_eq!(parser.columns.len(), 6);

    // Verify optional core sections
    assert!(parser.rhs.is_some());
    assert!(parser.ranges.is_some());
    assert!(parser.bounds.is_some());

    // Verify user cuts
    assert!(parser.user_cuts.is_some());
    assert_eq!(parser.user_cuts.as_ref().unwrap().len(), 1);

    // Verify MIP/QP extensions
    assert!(parser.special_ordered_sets.is_some());
    assert_eq!(parser.special_ordered_sets.as_ref().unwrap().len(), 1);

    // Verify quadratic objective
    assert!(parser.quadratic_objective.is_some());
    assert_eq!(parser.quadratic_objective.as_ref().unwrap().len(), 3);

    // Verify quadratic constraints
    assert!(parser.quadratic_constraints.is_some());
    assert_eq!(parser.quadratic_constraints.as_ref().unwrap().len(), 1);

    // Verify indicators
    assert!(parser.indicators.is_some());
    assert_eq!(parser.indicators.as_ref().unwrap().len(), 1);

    // Verify lazy constraints
    assert!(parser.lazy_constraints.is_some());
    assert_eq!(parser.lazy_constraints.as_ref().unwrap().len(), 1);

    Ok(())
  }

  /// Test that SOS section correctly follows BOUNDS (per CPLEX spec)
  #[test]
  fn test_sos_section_ordering() -> Result<()> {
    let input = r#"NAME          SOSTEST
ROWS
 N  obj
 L  c1
COLUMNS
    x1        obj                  1.0   c1                    1.0
    x2        obj                  2.0   c1                    1.0
    x3        obj                  3.0   c1                    1.0
RHS
    rhs1      c1                  10.0
BOUNDS
 UP bnd1      x1                  10.0
 UP bnd1      x2                  10.0
 UP bnd1      x3                  10.0
SOS
 S1 sos_set
    x1 1.0
    x2 2.0
    x3 3.0
ENDATA
"#;

    let parser = Parser::<f64>::parse(input)?;
    assert!(parser.special_ordered_sets.is_some());
    assert_eq!(parser.special_ordered_sets.as_ref().unwrap().len(), 1);

    let sos = &parser.special_ordered_sets.as_ref().unwrap()[0];
    assert_eq!(sos.sos_type, SOSType::S1);
    assert_eq!(sos.set_name, "sos_set");
    assert_eq!(sos.members.len(), 3);

    Ok(())
  }

  /// Test QMATRIX vs QSECTION equivalence
  #[test]
  fn test_qmatrix_vs_qsection() -> Result<()> {
    let qmatrix_input = r#"NAME          QTEST
ROWS
 N  obj
 L  c1
COLUMNS
    x         obj                  1.0   c1                    1.0
    y         obj                  1.0   c1                    1.0
RHS
    rhs1      c1                  10.0
QMATRIX
    x         x                    1.0
    x         y                    2.0
    y         x                    2.0
    y         y                    7.0
ENDATA
"#;

    let qsection_input = r#"NAME          QTEST
ROWS
 N  obj
 L  c1
COLUMNS
    x         obj                  1.0   c1                    1.0
    y         obj                  1.0   c1                    1.0
RHS
    rhs1      c1                  10.0
QSECTION
    x         x                    1.0
    x         y                    2.0
    y         y                    7.0
ENDATA
"#;

    let qmatrix_parser = Parser::<f64>::parse(qmatrix_input)?;
    let qsection_parser = Parser::<f64>::parse(qsection_input)?;

    // Both should have quadratic objective terms
    assert!(qmatrix_parser.quadratic_objective.is_some());
    assert!(qsection_parser.quadratic_objective.is_some());

    // QMATRIX includes all terms (including symmetric: x-y and y-x)
    // QSECTION only includes upper diagonal (x-y but not y-x)
    let qmatrix_terms = qmatrix_parser.quadratic_objective.as_ref().unwrap();
    let qsection_terms = qsection_parser.quadratic_objective.as_ref().unwrap();

    // QMATRIX should have 4 terms, QSECTION should have 3
    assert_eq!(qmatrix_terms.len(), 4);
    assert_eq!(qsection_terms.len(), 3);

    Ok(())
  }

  /// Test sign character validation for columns lines
  /// When a sign character appears just before the expected field position,
  /// the strict parser should detect it and fall back to flexible parsing
  #[test]
  fn test_columns_sign_validation() -> Result<()> {
    // Test case: sign character at position L4-1 (position 22, just before value field at 23)
    // L4 = 23, so position 22 should have the sign character
    // Format: "    COL1      ROW1     -123\n"
    // Positions: 0-3 (spaces), 4-11 (COL1), 12-13 (spaces), 14-21 (ROW1), 22 (-), 23-35 (value field)
    let input = "    COL1      ROW1     -123\n";
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let info = TracableInfo::new().forward(false).backward(false);
        let result = Parser::<f32>::columns_line(LocatedSpan::new_extra(input, info));
      } else {
        let result = Parser::<f32>::columns_line(input);
      }
    }
    // This should succeed with flexible parsing fallback
    assert!(result.is_ok());

    // Also test the second value field sign validation
    // L6 = 48, so position 47 should have the sign character for second value
    let input2 = "    COL2      ROW1      100   ROW2       -200\n";
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let info = TracableInfo::new().forward(false).backward(false);
        let result2 = Parser::<f32>::columns_line(LocatedSpan::new_extra(input2, info));
      } else {
        let result2 = Parser::<f32>::columns_line(input2);
      }
    }
    assert!(result2.is_ok());

    Ok(())
  }

  /// Test sign character validation for bounds lines
  /// When a sign character appears just before the value field position,
  /// the strict parser should detect it and fall back to flexible parsing
  #[test]
  fn test_bounds_sign_validation() -> Result<()> {
    // Test case: sign character at position L4-1 (just before value field)
    // For bound types that require a value (not FR or PL)
    let input = " UP BND1      VAR1-999.99\n";
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let info = TracableInfo::new().forward(false).backward(false);
        let result = Parser::<f32>::bounds_line(LocatedSpan::new_extra(input, info));
      } else {
        let result = Parser::<f32>::bounds_line(input);
      }
    }
    // This should succeed with flexible parsing fallback
    assert!(result.is_ok());

    // Additional test: ensure FR and PL bounds (no value) work correctly
    let fr_input = " FR BND1      VAR2\n";
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let info = TracableInfo::new().forward(false).backward(false);
        let fr_result = Parser::<f32>::bounds_line(LocatedSpan::new_extra(fr_input, info));
      } else {
        let fr_result = Parser::<f32>::bounds_line(fr_input);
      }
    }
    assert!(fr_result.is_ok());
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        if let Ok((_, Some(bound))) = fr_result {
          assert_eq!(bound.bound_type, BoundType::Fr);
          assert_eq!(bound.value, None);
        }
      } else {
        if let Ok((_, Some(bound))) = fr_result {
          assert_eq!(bound.bound_type, BoundType::Fr);
          assert_eq!(bound.value, None);
        }
      }
    }

    let pl_input = " PL BND1      VAR3\n";
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let info = TracableInfo::new().forward(false).backward(false);
        let pl_result = Parser::<f32>::bounds_line(LocatedSpan::new_extra(pl_input, info));
      } else {
        let pl_result = Parser::<f32>::bounds_line(pl_input);
      }
    }
    assert!(pl_result.is_ok());
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        if let Ok((_, Some(bound))) = pl_result {
          assert_eq!(bound.bound_type, BoundType::Pl);
          assert_eq!(bound.value, None);
        }
      } else {
        if let Ok((_, Some(bound))) = pl_result {
          assert_eq!(bound.bound_type, BoundType::Pl);
          assert_eq!(bound.value, None);
        }
      }
    }

    Ok(())
  }

  /// Test indicator constraints with correct format
  #[test]
  fn test_indicators_format() -> Result<()> {
    let input = r#"NAME          INDTEST
ROWS
 N  obj
 L  c1
 E  c2
COLUMNS
    x         obj                  1.0
    y         c1                   1.0
    z         c2                   1.0
BOUNDS
 UI bnd1      y                    1.0
INDICATORS
 IF c1 y 1
 IF c2 y 0
ENDATA
"#;

    let parser = Parser::<f64>::parse(input)?;
    assert!(parser.indicators.is_some());

    let indicators = parser.indicators.as_ref().unwrap();
    assert_eq!(indicators.len(), 2);

    assert_eq!(indicators[0].constraint_name, "c1");
    assert_eq!(indicators[0].binary_var, "y");
    assert_eq!(indicators[0].trigger_value, 1);

    assert_eq!(indicators[1].constraint_name, "c2");
    assert_eq!(indicators[1].binary_var, "y");
    assert_eq!(indicators[1].trigger_value, 0);

    Ok(())
  }
}
