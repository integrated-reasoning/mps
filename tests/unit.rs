mod tests {
  use color_eyre::Result;
  use mps::file::*;

  #[test]
  fn test_name() {
    let a = "NAME          AFIRO\n";
    assert_eq!(MPSFile::<f32>::name(a), Ok(("", "AFIRO")));
  }

  #[test]
  fn test_row_line() -> Result<()> {
    let a = " E  R09\n";
    let b = " E  R10\n";
    let c = " L  X05\n";
    let d = " L  X21\n";
    assert_eq!(
      MPSFile::<f32>::row_line(a),
      Ok((
        "",
        RowLine {
          row_type: RowType::try_from('E')?,
          row_name: "R09"
        }
      ))
    );
    assert_eq!(
      MPSFile::<f32>::row_line(b),
      Ok((
        "",
        RowLine {
          row_type: RowType::try_from('E')?,
          row_name: "R10"
        }
      ))
    );
    assert_eq!(
      MPSFile::<f32>::row_line(c),
      Ok((
        "",
        RowLine {
          row_type: RowType::try_from('L')?,
          row_name: "X05"
        }
      ))
    );
    assert_eq!(
      MPSFile::<f32>::row_line(d),
      Ok((
        "",
        RowLine {
          row_type: RowType::try_from('L')?,
          row_name: "X21"
        }
      ))
    );
    Ok(())
  }

  #[test]
  fn test_rows() -> Result<()> {
    let a = "ROWS\n E  R09\n E  R10\n L  X05\n L  X21\nCOLUMNS";
    assert_eq!(
      MPSFile::<f32>::rows(a),
      Ok((
        "COLUMNS",
        vec![
          RowLine {
            row_type: RowType::try_from('E')?,
            row_name: "R09"
          },
          RowLine {
            row_type: RowType::try_from('E')?,
            row_name: "R10"
          },
          RowLine {
            row_type: RowType::try_from('L')?,
            row_name: "X05"
          },
          RowLine {
            row_type: RowType::try_from('L')?,
            row_name: "X21"
          },
        ]
      ))
    );
    Ok(())
  }

  #[test]
  fn test_columns_line() {
    let a = "    X01       X48               .301   R09                -1.\n";
    let b = "    X02       COST               -.4\n";
    assert_eq!(
      MPSFile::<f32>::columns_line(a),
      Ok((
        "",
        WideLine::<f32> {
          name: "X01",
          first_pair: RowValuePair {
            row_name: "X48",
            value: 0.301
          },
          second_pair: Some(RowValuePair {
            row_name: "R09",
            value: -1.0
          })
        }
      ))
    );
    assert_eq!(
      MPSFile::<f32>::columns_line(b),
      Ok((
        "",
        WideLine::<f32> {
          name: "X02",
          first_pair: RowValuePair {
            row_name: "COST",
            value: -0.4
          },
          second_pair: None
        }
      ))
    );
  }

  #[test]
  fn test_columns() {
    let a = "COLUMNS
    X01       X48               .301   R09                -1.
    X01       R10              -1.06   X05                 1.
    X02       X21                -1.   R09                 1.
    X02       COST               -.4
    X03       X46                -1.   R09                 1.\nRHS";
    assert_eq!(
      MPSFile::<f32>::columns(a),
      Ok((
        "RHS",
        vec![
          WideLine::<f32> {
            name: "X01",
            first_pair: RowValuePair {
              row_name: "X48",
              value: 0.301
            },
            second_pair: Some(RowValuePair {
              row_name: "R09",
              value: -1.0
            })
          },
          WideLine::<f32> {
            name: "X01",
            first_pair: RowValuePair {
              row_name: "R10",
              value: -1.06
            },
            second_pair: Some(RowValuePair {
              row_name: "X05",
              value: 1.0
            })
          },
          WideLine::<f32> {
            name: "X02",
            first_pair: RowValuePair {
              row_name: "X21",
              value: -1.0
            },
            second_pair: Some(RowValuePair {
              row_name: "R09",
              value: 1.0
            })
          },
          WideLine::<f32> {
            name: "X02",
            first_pair: RowValuePair {
              row_name: "COST",
              value: -0.4
            },
            second_pair: None
          },
          WideLine::<f32> {
            name: "X03",
            first_pair: RowValuePair {
              row_name: "X46",
              value: -1.0
            },
            second_pair: Some(RowValuePair {
              row_name: "R09",
              value: 1.0
            })
          },
        ]
      ))
    );
  }

  #[test]
  fn test_rhs_line() {
    let a = "    RHS1      LIM1                 5   LIM2                10\n";
    let b = "    RHS1      MYEQN                7\n";
    assert_eq!(
      MPSFile::<f32>::rhs_line(a),
      Ok((
        "",
        WideLine::<f32> {
          name: "RHS1",
          first_pair: RowValuePair {
            row_name: "LIM1",
            value: 5.0
          },
          second_pair: Some(RowValuePair {
            row_name: "LIM2",
            value: 10.0
          })
        }
      ))
    );
    assert_eq!(
      MPSFile::<f32>::rhs_line(b),
      Ok((
        "",
        WideLine::<f32> {
          name: "RHS1",
          first_pair: RowValuePair {
            row_name: "MYEQN",
            value: 7.0
          },
          second_pair: None
        }
      ))
    );
  }

  #[test]
  fn test_rhs() {
    let a = "RHS
    RHS1      LIM1                 5   LIM2                10
    RHS1      MYEQN                7\nBOUNDS";
    assert_eq!(
      MPSFile::<f32>::rhs(a),
      Ok((
        "BOUNDS",
        vec![
          WideLine::<f32> {
            name: "RHS1",
            first_pair: RowValuePair {
              row_name: "LIM1",
              value: 5.0
            },
            second_pair: Some(RowValuePair {
              row_name: "LIM2",
              value: 10.0
            })
          },
          WideLine::<f32> {
            name: "RHS1",
            first_pair: RowValuePair {
              row_name: "MYEQN",
              value: 7.0
            },
            second_pair: None
          }
        ]
      ))
    );
  }

  #[test]
  fn test_ranges_line() {
    let a = "    RANGE1    VILLKOR6           2.5   VILLKOR7           30.\n";
    let b = "    RANGE1    VILLKOR8           7.5\n";
    assert_eq!(
      MPSFile::<f32>::ranges_line(a),
      Ok((
        "",
        WideLine::<f32> {
          name: "RANGE1",
          first_pair: RowValuePair {
            row_name: "VILLKOR6",
            value: 2.5
          },
          second_pair: Some(RowValuePair {
            row_name: "VILLKOR7",
            value: 30.0
          })
        }
      ))
    );
    assert_eq!(
      MPSFile::<f32>::ranges_line(b),
      Ok((
        "",
        WideLine::<f32> {
          name: "RANGE1",
          first_pair: RowValuePair {
            row_name: "VILLKOR8",
            value: 7.5
          },
          second_pair: None
        }
      ))
    );
  }

  #[test]
  fn test_ranges() {
    let a = "RANGES
    RANGE1    VILLKOR2            7.   VILLKOR3            7.
    RANGE1    VILLKOR4           3.5   VILLKOR5           10.
    RANGE1    VILLKOR6           2.5   VILLKOR7           30.
    RANGE1    VILLKOR8           7.5\nBOUNDS";
    assert_eq!(
      MPSFile::<f32>::ranges(a),
      Ok((
        "BOUNDS",
        vec![
          WideLine::<f32> {
            name: "RANGE1",
            first_pair: RowValuePair {
              row_name: "VILLKOR2",
              value: 7.0
            },
            second_pair: Some(RowValuePair {
              row_name: "VILLKOR3",
              value: 7.0
            })
          },
          WideLine::<f32> {
            name: "RANGE1",
            first_pair: RowValuePair {
              row_name: "VILLKOR4",
              value: 3.5
            },
            second_pair: Some(RowValuePair {
              row_name: "VILLKOR5",
              value: 10.0
            })
          },
          WideLine::<f32> {
            name: "RANGE1",
            first_pair: RowValuePair {
              row_name: "VILLKOR6",
              value: 2.5
            },
            second_pair: Some(RowValuePair {
              row_name: "VILLKOR7",
              value: 30.0
            })
          },
          WideLine::<f32> {
            name: "RANGE1",
            first_pair: RowValuePair {
              row_name: "VILLKOR8",
              value: 7.5
            },
            second_pair: None
          },
        ]
      ))
    );
  }

  #[test]
  fn test_bound_type() {
    assert_eq!(MPSFile::<f32>::bound_type("LO"), Ok(("", BoundType::LO)));
    assert_eq!(MPSFile::<f32>::bound_type("UP"), Ok(("", BoundType::UP)));
    assert_eq!(MPSFile::<f32>::bound_type("FX"), Ok(("", BoundType::FX)));
    assert_eq!(MPSFile::<f32>::bound_type("FR"), Ok(("", BoundType::FR)));
    assert_eq!(MPSFile::<f32>::bound_type("MI"), Ok(("", BoundType::MI)));
    assert_eq!(MPSFile::<f32>::bound_type("PL"), Ok(("", BoundType::PL)));
  }

  #[test]
  fn test_bounds_line() {
    let a = " UP BND1      XONE                 4\n";
    let b = " LO BND1      YTWO                -1\n";
    let c = " UP BND1      YTWO                 1\n";
    assert_eq!(
      MPSFile::<f32>::bounds_line(a),
      Ok((
        "",
        BoundsLine::<f32> {
          bound_type: BoundType::UP,
          bound_name: "BND1",
          column_name: "XONE",
          value: 4.0
        }
      ))
    );
    assert_eq!(
      MPSFile::<f32>::bounds_line(b),
      Ok((
        "",
        BoundsLine::<f32> {
          bound_type: BoundType::LO,
          bound_name: "BND1",
          column_name: "YTWO",
          value: -1.0
        }
      ))
    );
    assert_eq!(
      MPSFile::<f32>::bounds_line(c),
      Ok((
        "",
        BoundsLine::<f32> {
          bound_type: BoundType::UP,
          bound_name: "BND1",
          column_name: "YTWO",
          value: 1.0
        }
      ))
    );
  }

  #[test]
  fn test_bounds() {
    let a = "BOUNDS
 UP BND1      XONE                 4
 LO BND1      YTWO                -1
 UP BND1      YTWO                 1\nENDATA";
    assert_eq!(
      MPSFile::<f32>::bounds(a),
      Ok((
        "ENDATA",
        vec![
          BoundsLine::<f32> {
            bound_type: BoundType::UP,
            bound_name: "BND1",
            column_name: "XONE",
            value: 4.0
          },
          BoundsLine::<f32> {
            bound_type: BoundType::LO,
            bound_name: "BND1",
            column_name: "YTWO",
            value: -1.0
          },
          BoundsLine::<f32> {
            bound_type: BoundType::UP,
            bound_name: "BND1",
            column_name: "YTWO",
            value: 1.0
          }
        ],
      ))
    );
  }
}
