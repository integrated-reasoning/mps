use crate::types::*;
use color_eyre::{eyre::eyre, eyre::OptionExt, Result};
use fast_float2::FastFloat;
use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::*,
  combinator::{map, map_res, opt, peek},
  multi::many0,
  sequence::{preceded, terminated},
  IResult,
};
use nom_tracable::tracable_parser;
use std::cmp;
cfg_if::cfg_if! {
  if #[cfg(feature = "trace")] {
    use nom_locate::LocatedSpan;
    use nom_tracable::TracableInfo;
  }
}

static L1: usize = 0;
static R1: usize = 2;
static L2: usize = 3;
static R2: usize = 11;
static L3: usize = 13;
static R3: usize = 21;
static L4: usize = 23;
static R4: usize = 35;
static L5: usize = 38;
static R5: usize = 46;
static L6: usize = 48;
static R6: usize = 60;

/// Custom line ending parser that handles both Unix (\n) and Windows (\r\n) line endings
/// Tries Unix first for better performance since it's more common
fn line_ending_flexible(s: Span) -> IResult<Span, Span> {
  alt((tag("\n"), tag("\r\n")))(s)
}

impl<'a, T: FastFloat> Parser<'a, T> {
  /// Parses an MPS formatted string into a `Parser` instance.
  ///
  /// This acts as the primary public interface for converting MPS
  /// formatted data into a structured `Parser` format. It is designed
  /// to be the main entry point for most use cases.
  ///
  /// The `parse` method handles:
  ///
  /// - Wrapping the input with tracing infrastructure if enabled
  /// - Calling the lower-level `mps` parsing method
  /// - Mapping any parsing errors to a custom `nom` error
  /// - Returning a simplified `Result<Parser, Error>`
  ///
  /// By handling these internals, it provides a simplified interface
  /// focused on the end goal of parsing MPS data. This frees calling
  /// code from interacting directly with nom parser details.
  ///
  /// # Arguments
  ///
  /// * `input`: &str - A string slice containing the MPS formatted data
  ///
  /// # Returns
  ///
  /// Result<Parser, Error>
  ///
  /// - Ok(Parser): The parsed MPS data as a `Parser` struct
  /// - Err(Error): A nom error if parsing failed
  ///
  /// # Examples
  ///
  /// ```
  /// use mps::Parser;
  /// let input = "MPS formatted data...";
  /// match Parser::<f32>::parse(input) {
  ///     Ok(parsed) => { /* use parsed */ },
  ///     Err(err) => { /* handle error */ }
  /// }
  /// ```
  pub fn parse(
    input: &'a str,
  ) -> Result<Parser<'a, T>, nom::error::Error<String>> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "trace")] {
            let info = TracableInfo::new().forward(false).backward(false);
            let input = LocatedSpan::new_extra(input, info);
        }
    }
    let (_, parsed) = Parser::<T>::mps_file(input).map_err(|e| {
      // Extract context around the error location instead of showing entire file
      let error_msg = match e {
        nom::Err::Error(err) | nom::Err::Failure(err) => {
          cfg_if::cfg_if! {
            if #[cfg(feature = "trace")] {
              let remaining = err.input.fragment();
            } else {
              let remaining = err.input;
            }
          }
          // Show only first 200 characters of the remaining input where parsing failed
          let preview_len = std::cmp::min(200, remaining.len());
          let preview = &remaining[..preview_len];
          let error_context = if remaining.len() > 200 {
            format!("{}...", preview)
          } else {
            preview.to_string()
          };
          format!("Parse error near: {}", error_context)
        }
        nom::Err::Incomplete(_) => "Incomplete input".to_string(),
      };
      nom::error::Error::new(error_msg, nom::error::ErrorKind::Fail)
    })?;
    Ok(parsed)
  }

  /// Low-level parser directly exposing the MPS format.
  ///
  /// This method performs the direct parsing of MPS formatted sections
  /// (name, rows, columns, etc.) into a `Parser` instance.
  ///
  /// It uses parser combinators from the nom library and returns
  /// an IResult<Span, Parser> representing either success or failure.
  ///
  /// The `mps_file` method is called internally by `parse` but exposed
  /// publicly for advanced use cases needing direct access to the
  /// underlying nom-based parser.
  ///
  /// For most use cases, the simplified `parse` interface should
  /// be preferred over directly calling this method.
  ///
  /// # Section Ordering (per CPLEX MPS Format Specification)
  ///
  /// The MPS file format specifies strict section ordering:
  /// 1. NAME - Problem name (required)
  /// 2. OBJSENSE - Objective sense: MIN or MAX (optional, CPLEX extension)
  /// 3. OBJNAME - Objective function row name (optional, CPLEX extension)
  /// 4. REFROW - Reference row for SOS weights (optional, CPLEX extension)
  /// 5. ROWS - Row definitions (required)
  /// 6. USERCUTS - User-defined cuts (optional, CPLEX extension)
  /// 7. COLUMNS - Column definitions (required)
  /// 8. RHS - Right-hand side values (optional)
  /// 9. RANGES - Range constraints (optional)
  /// 10. BOUNDS - Variable bounds (optional)
  /// 11. SOS - Special ordered sets (optional, CPLEX extension)
  /// 12. QSECTION or QUADOBJ - Quadratic objective (optional, CPLEX extension)
  /// 13. QMATRIX - Quadratic objective (alternative format, optional)
  /// 14. QCMATRIX - Quadratic constraints (optional, CPLEX extension, multiple allowed)
  /// 15. CSECTION - Second-order cone constraints (optional, CPLEX extension)
  /// 16. INDICATORS - Indicator constraints (optional, CPLEX extension)
  /// 17. LAZYCONS - Lazy constraints (optional, CPLEX extension)
  /// 18. BRANCH - Branching priorities (optional, CPLEX extension)
  /// 19. ENDATA - End of data (required)
  #[tracable_parser]
  pub fn mps_file(s: Span<'a>) -> IResult<Span<'a>, Parser<'a, T>> {
    // 1. NAME section
    let (s, _) = many0(Self::skip_line)(s)?;
    let (s, name) = Self::name(s)?;
    let (s, _) = many0(Self::skip_line)(s)?;

    // 2. OBJSENSE section (optional)
    let (s, objective_sense) = opt(Self::objsen)(s)?;
    let (s, _) = many0(Self::skip_line)(s)?;

    // 3. OBJNAME section (optional)
    let (s, objective_name) = opt(Self::objname)(s)?;
    let (s, _) = many0(Self::skip_line)(s)?;

    // 4. REFROW section (optional)
    let (s, reference_row) = opt(Self::refrow)(s)?;
    let (s, _) = many0(Self::skip_line)(s)?;

    // 5. ROWS section
    let (s, rows) = Self::rows(s)?;
    let (s, _) = many0(Self::skip_line)(s)?;

    // 6. USERCUTS section (optional)
    let (s, user_cuts) = opt(Self::usercuts)(s)?;
    let (s, _) = many0(Self::skip_line)(s)?;

    // 7. COLUMNS section
    let (s, columns) = Self::columns(s)?;
    let (s, _) = many0(Self::skip_line)(s)?;

    // 8. RHS section (optional)
    let (s, rhs) = opt(Self::rhs)(s)?;
    let (s, _) = many0(Self::skip_line)(s)?;

    // 9. RANGES section (optional)
    let (s, ranges) = opt(Self::ranges)(s)?;
    let (s, _) = many0(Self::skip_line)(s)?;

    // 10. BOUNDS section (optional)
    let (s, bounds) = opt(Self::bounds)(s)?;
    let (s, _) = many0(Self::skip_line)(s)?;

    // 11. SOS section (optional) - MUST come after BOUNDS per CPLEX spec
    let (s, special_ordered_sets) = opt(Self::sos)(s)?;
    let (s, _) = many0(Self::skip_line)(s)?;

    // 12. QSECTION/QUADOBJ section (optional)
    let (s, qsection) = opt(alt((Self::qsection, Self::quadobj)))(s)?;
    let (s, _) = many0(Self::skip_line)(s)?;

    // 13. QMATRIX section (optional)
    let (s, qmatrix) = opt(Self::qmatrix)(s)?;
    let (s, _) = many0(Self::skip_line)(s)?;

    // 14. QCMATRIX sections (optional, multiple allowed)
    let (s, qcmatrices) = many0(Self::qcmatrix)(s)?;
    let (s, _) = many0(Self::skip_line)(s)?;

    // 15. CSECTION (optional)
    let (s, csection) = opt(Self::csection)(s)?;
    let (s, _) = many0(Self::skip_line)(s)?;

    // 16. INDICATORS section (optional)
    let (s, indicators) = opt(Self::indicators)(s)?;
    let (s, _) = many0(Self::skip_line)(s)?;

    // 17. LAZYCONS section (optional)
    let (s, lazy_constraints) = opt(Self::lazycons)(s)?;
    let (s, _) = many0(Self::skip_line)(s)?;

    // 18. BRANCH section (optional)
    let (s, branch_priorities) = opt(Self::branch)(s)?;
    let (s, _) = many0(Self::skip_line)(s)?;

    // 19. ENDATA section
    let (s, _) = Self::endata(s)?;

    // Combine QSECTION/QUADOBJ with QMATRIX and QCMATRIX sections
    let mut quad_obj = qsection;
    if quad_obj.is_none() {
      quad_obj = qmatrix.map(|qm| {
        // Convert QMATRIX (which is in constraint format) to objective format
        // QMATRIX represents the full Q matrix for the objective: 0.5 * x'Qx
        qm.into_iter()
          .flat_map(|qc| {
            qc.terms.into_iter().map(|qt| QuadraticObjectiveTerm {
              var1: qt.var1,
              var2: qt.var2,
              coefficient: qt.coefficient,
            })
          })
          .collect()
      });
    }

    // Combine QCMATRIX sections (quadratic constraints)
    let quad_constr: Vec<QuadraticConstraint<T>> =
      qcmatrices.into_iter().flatten().collect();

    let parser = Parser {
      name: name.trim(),
      objective_sense,
      objective_name,
      reference_row,
      rows,
      columns,
      rhs,
      ranges,
      bounds,
      user_cuts,
      special_ordered_sets,
      quadratic_objective: quad_obj,
      quadratic_constraints: if quad_constr.is_empty() {
        None
      } else {
        Some(quad_constr)
      },
      indicators,
      lazy_constraints,
      cone_constraints: csection,
      branch_priorities,
    };
    Ok((s, parser))
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn skip_line(s: Span) -> IResult<Span, ()> {
    alt((Self::comment_line, Self::empty_line))(s)
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn comment_line(s: Span) -> IResult<Span, ()> {
    let mut p = map(
      terminated(preceded(tag("*"), not_line_ending), line_ending_flexible),
      |_| (),
    );
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn empty_line(s: Span) -> IResult<Span, ()> {
    let mut p = map(terminated(space0, line_ending_flexible), |_| ());
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  #[doc(hidden)]
  pub fn name(s: Span<'_>) -> IResult<Span<'_>, &str> {
    // Parse NAME header - some files have fixed format, others are flexible
    let (s, _) = tag("NAME")(s)?;

    // Get the rest of the line after NAME
    let (s, rest) = not_line_ending(s)?;
    let (s, _) = line_ending_flexible(s)?;

    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let name = rest.fragment().trim();
      } else {
        let name = rest.trim();
      }
    }

    Ok((s, name))
  }

  #[doc(hidden)]
  pub fn objsen(s: Span) -> IResult<Span, ObjectiveSense> {
    let (s, _) = tag("OBJSENSE")(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = line_ending_flexible(s)?;
    let (s, _) = space0(s)?;

    let (s, sense_str) = alt((tag("MAX"), tag("MIN")))(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = line_ending_flexible(s)?;

    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let sense_str_val = *sense_str.fragment();
      } else {
        let sense_str_val = sense_str;
      }
    }

    let sense = match sense_str_val {
      "MAX" => ObjectiveSense::Max,
      "MIN" => ObjectiveSense::Min,
      _ => ObjectiveSense::Min,
    };

    Ok((s, sense))
  }

  #[doc(hidden)]
  pub fn objname(s: Span<'_>) -> IResult<Span<'_>, &str> {
    let (s, _) = tag("OBJNAME")(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = line_ending_flexible(s)?;
    let (s, _) = space0(s)?;

    let (s, name) = not_line_ending(s)?;
    let (s, _) = line_ending_flexible(s)?;

    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let obj_name = name.fragment().trim();
      } else {
        let obj_name = name.trim();
      }
    }

    Ok((s, obj_name))
  }

  #[doc(hidden)]
  pub fn refrow(s: Span<'_>) -> IResult<Span<'_>, &str> {
    let (s, _) = tag("REFROW")(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = line_ending_flexible(s)?;
    let (s, _) = space0(s)?;

    let (s, row_name) = not_line_ending(s)?;
    let (s, _) = line_ending_flexible(s)?;

    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let ref_row = row_name.fragment().trim();
      } else {
        let ref_row = row_name.trim();
      }
    }

    Ok((s, ref_row))
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn row_line_or_end(s: Span) -> IResult<Span, Option<RowLine>> {
    // Try to skip comment or empty lines first
    if let Ok((s, _)) = alt((Self::comment_line, Self::empty_line))(s) {
      return Ok((s, None));
    }

    // Check if this is LAZYCONS marker (appears within ROWS section in some MIP files)
    if let Ok((s, _)) = tag::<_, _, nom::error::Error<_>>("LAZYCONS")(s) {
      let (s, _) = opt(not_line_ending)(s)?;
      let (s, _) = line_ending_flexible(s)?;
      // Return None to skip this line but continue parsing rows
      return Ok((s, None));
    }

    // Check if this line starts with a space (row line) or not (section header)
    // Peek to avoid consuming input if it's not a row line
    let peeked = peek(not_line_ending)(s)?;
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let line_str = peeked.1.fragment();
      } else {
        let line_str = peeked.1;
      }
    }

    // If line doesn't start with space, it's likely a section header - stop parsing rows
    if !line_str.starts_with(' ') {
      return Err(nom::Err::Error(nom::error::Error::new(
        s,
        nom::error::ErrorKind::Eof,
      )));
    }

    // Parse the actual row line
    let mut p = map_res(
      terminated(not_line_ending, line_ending_flexible),
      |line: Span| -> Result<RowLine> {
        cfg_if::cfg_if! {
          if #[cfg(feature = "trace")] {
            let line_str = line.fragment();
          } else {
            let line_str = line;
          }
        }

        // Use byte-level operations for speed
        let bytes = line_str.as_bytes();
        let len = bytes.len();
        
        // Skip leading whitespace (should start with space per the check above)
        let mut pos = 0;
        while pos < len && bytes[pos] == b' ' {
          pos += 1;
        }
        
        if pos >= len {
          return Err(eyre!("empty row line"));
        }

        // First non-space character is the row type (E, L, G, N)
        let row_type_char = bytes[pos];
        pos += 1;
        let row_type = match row_type_char {
          b'E' => RowType::Eq,
          b'L' => RowType::Leq,
          b'G' => RowType::Geq,
          b'N' => RowType::Nr,
          _ => return Err(eyre!("invalid row type")),
        };

        // Skip whitespace to row name
        while pos < len && bytes[pos] == b' ' {
          pos += 1;
        }
        let name_start = pos;
        
        // Row name extends to end of line, but we trim trailing whitespace
        // Find the end (past any trailing whitespace)
        let mut end = len;
        while end > name_start && (bytes[end - 1] == b' ' || bytes[end - 1] == b'\t') {
          end -= 1;
        }
        let row_name = &line_str[name_start..end];

        Ok(RowLine { row_type, row_name })
      },
    );

    let (s, row_line) = p(s)?;
    Ok((s, Some(row_line)))
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn rows(s: Span) -> IResult<Span, Vec<RowLine>> {
    // Parse ROWS header with optional trailing spaces
    let (s, _) = tag("ROWS")(s)?;
    let (s, _) = space0(s)?; // Skip optional trailing spaces
    let (s, _) = line_ending_flexible(s)?;

    // Now parse the row lines
    let mut p = map(
      many0(Self::row_line_or_end),
      |rows: Vec<Option<RowLine>>| {
        // Filter out None values (comment/empty lines)
        rows.into_iter().flatten().collect()
      },
    );
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn line(s: Span) -> IResult<Span, WideLine<T>> {
    let mut p = map_res(
      terminated(preceded(tag(" "), not_line_ending), line_ending_flexible),
      |line: Span| -> Result<WideLine<T>> {
        cfg_if::cfg_if! {
          if #[cfg(feature = "trace")] {
            let line_str = line.fragment();
          } else {
            let line_str = line;
          }
        }

        // Try strict field positioning first (no comment stripping for strict parsing)
        // Optimized: use byte-level operations to avoid trim() allocations
        let strict_result = (|| -> Result<WideLine<T>> {
          let bytes = line_str.as_bytes();
          let len = bytes.len();
          
          // Helper: get trimmed slice from fixed positions (returns empty string if all whitespace)
          let get_trimmed = |start: usize, end: usize| -> &str {
            let end = end.min(len);
            if end <= start { return ""; }
            let s = &bytes[start..end];
            let mut first = 0;
            while first < s.len() && s[first].is_ascii_whitespace() { first += 1; }
            let mut last = s.len();
            while last > first && s[last-1].is_ascii_whitespace() { last -= 1; }
            if first >= last { return ""; }
            unsafe { std::str::from_utf8_unchecked(&s[first..last]) }
          };
          
          // Check for sign before L4 field
          if L4 > 0 && len > L4 {
            let prev = bytes[L4 - 1];
            if prev == b'-' || prev == b'+' {
              return Err(eyre!("potential sign character excluded from value field"));
            }
          }

          let val_str = get_trimmed(L4, R4);
          if val_str.is_empty() {
            return Err(eyre!("missing value field"));
          }
          let first_row_name = get_trimmed(L3, R3);
          if first_row_name.is_empty() {
            return Err(eyre!("missing row name field"));
          }
          let name = get_trimmed(L2, R2);

          let first_pair = RowValuePair {
            row_name: first_row_name,
            value: fast_float2::parse(val_str)?,
          };

          // Check for second pair
          let second_pair = if R5 <= len {
            let row_name = get_trimmed(L5, R5);
            if !row_name.is_empty() {
              // Check for sign before L6 field
              if L6 > 0 && len > L6 {
                let prev = bytes[L6 - 1];
                if prev == b'-' || prev == b'+' {
                  return Err(eyre!("potential sign character excluded from second value field"));
                }
              }
              let val2 = get_trimmed(L6, R6);
              if !val2.is_empty() {
                Some(RowValuePair {
                  row_name,
                  value: fast_float2::parse(val2)?,
                })
              } else {
                None
              }
            } else {
              None
            }
          } else {
            None
          };

          Ok(WideLine::<T> {
            name,
            first_pair,
            second_pair,
          })
        })();

        // If strict parsing fails, try flexible whitespace-separated parsing
        if strict_result.is_err() {
          Self::parse_flexible_line(line)
        } else {
          strict_result
        }
      },
    );
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  /// Parse a line using flexible whitespace-separated format
  /// Optimized: zero-allocation parser using span slicing
  fn parse_flexible_line(line: Span) -> Result<WideLine<T>> {
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let line_str = line.fragment();
      } else {
        let line_str = line;
      }
    }

    // For flexible parsing, only strip comments if $ appears after significant whitespace
    // This helps distinguish inline comments from $ in variable names
    // Look for "  $" pattern (two or more spaces followed by $)
    let line_str = if let Some(pos) = line_str.find("  $") {
      &line_str[..pos]
    } else if let Some(pos) = line_str.find("\t$") {
      &line_str[..pos]
    } else {
      line_str
    };

    // Zero-allocation flexible parser: find whitespace boundaries directly
    let bytes = line_str.as_bytes();
    let len = bytes.len();
    
    // Skip leading whitespace
    let mut pos = 0;
    while pos < len && bytes[pos].is_ascii_whitespace() {
      pos += 1;
    }
    
    // Field 1: column name
    let name_start = pos;
    while pos < len && !bytes[pos].is_ascii_whitespace() {
      pos += 1;
    }
    if pos == name_start {
      return Err(eyre!("insufficient fields in line"));
    }
    let name = &line_str[name_start..pos];
    
    // Skip whitespace to field 2
    while pos < len && bytes[pos].is_ascii_whitespace() {
      pos += 1;
    }
    let row1_start = pos;
    while pos < len && !bytes[pos].is_ascii_whitespace() {
      pos += 1;
    }
    let row1 = &line_str[row1_start..pos];
    
    // Skip whitespace to field 3 (value)
    while pos < len && bytes[pos].is_ascii_whitespace() {
      pos += 1;
    }
    let val1_start = pos;
    while pos < len && !bytes[pos].is_ascii_whitespace() {
      pos += 1;
    }
    let val1 = &line_str[val1_start..pos];
    
    // Skip whitespace to check for field 4
    while pos < len && bytes[pos].is_ascii_whitespace() {
      pos += 1;
    }
    
    let first_pair = RowValuePair {
      row_name: row1,
      value: fast_float2::parse(val1)?,
    };

    // Check if there's a second pair (row_name value)
    let second_pair = if pos < len {
      let row2_start = pos;
      while pos < len && !bytes[pos].is_ascii_whitespace() {
        pos += 1;
      }
      let row2 = &line_str[row2_start..pos];
      
      // Skip whitespace to value
      while pos < len && bytes[pos].is_ascii_whitespace() {
        pos += 1;
      }
      let val2_start = pos;
      while pos < len && !bytes[pos].is_ascii_whitespace() {
        pos += 1;
      }
      let val2 = &line_str[val2_start..pos];
      
      Some(RowValuePair {
        row_name: row2,
        value: fast_float2::parse(val2)?,
      })
    } else {
      None
    };

    Ok(WideLine::<T> {
      name,
      first_pair,
      second_pair,
    })
  }

  /// Skip a marker line
  #[tracable_parser]
  pub fn marker_line(s: Span) -> IResult<Span, ()> {
    let mut p = map(
      terminated(preceded(tag(" "), not_line_ending), line_ending_flexible),
      |_| (),
    );
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn columns_line(s: Span) -> IResult<Span, Option<WideLine<T>>> {
    // Try to skip comment or empty lines first
    if let Ok((s, _)) = alt((Self::comment_line, Self::empty_line))(s) {
      return Ok((s, None));
    }

    // Check if we've hit another section header
    let peeked = peek(not_line_ending)(s)?;
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let line_str = peeked.1.fragment();
      } else {
        let line_str = peeked.1;
      }
    }

    // If line doesn't start with space, it's likely a section header - stop parsing columns
    if !line_str.starts_with(' ') {
      return Err(nom::Err::Error(nom::error::Error::new(
        s,
        nom::error::ErrorKind::Eof,
      )));
    }

    // Try to parse as a marker line
    if let Ok((s, _)) = Self::try_marker_line(s) {
      return Ok((s, None));
    }

    // Otherwise, parse as normal data line
    let (s, wide_line) = Self::line(s)?;
    Ok((s, Some(wide_line)))
  }

  /// Try to parse a marker line, returning success if it is a marker
  #[tracable_parser]
  pub fn try_marker_line(s: Span) -> IResult<Span, ()> {
    // Peek at the line content to check if it's a marker
    let (_, line_content) = peek(preceded(tag(" "), not_line_ending))(s)?;

    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let line_str = line_content.fragment();
      } else {
        let line_str = line_content;
      }
    }

    // Check if this line contains MARKER syntax
    if line_str.contains("'MARKER'") {
      // Consume the marker line
      Self::marker_line(s)
    } else {
      // Not a marker line, fail this parser
      Err(nom::Err::Error(nom::error::Error::new(
        s,
        nom::error::ErrorKind::Tag,
      )))
    }
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn columns(s: Span) -> IResult<Span, Vec<WideLine<T>>> {
    // Parse COLUMNS header with optional trailing spaces
    let (s, _) = tag("COLUMNS")(s)?;
    let (s, _) = space0(s)?; // Skip optional trailing spaces
    let (s, _) = line_ending_flexible(s)?;

    let mut p = map(
      many0(Self::columns_line),
      |lines: Vec<Option<WideLine<T>>>| {
        // Filter out None values (marker lines)
        lines.into_iter().flatten().collect()
      },
    );
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn rhs_line(s: Span) -> IResult<Span, Option<WideLine<T>>> {
    // Try to skip comment or empty lines first
    if let Ok((s, _)) = alt((Self::comment_line, Self::empty_line))(s) {
      return Ok((s, None));
    }

    // Check if we've hit another section header
    let peeked = peek(not_line_ending)(s)?;
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let line_str = peeked.1.fragment();
      } else {
        let line_str = peeked.1;
      }
    }

    // If line doesn't start with space, it's likely a section header - stop parsing RHS
    if !line_str.starts_with(' ') {
      return Err(nom::Err::Error(nom::error::Error::new(
        s,
        nom::error::ErrorKind::Eof,
      )));
    }

    let (s, wide_line) = Self::line(s)?;
    Ok((s, Some(wide_line)))
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn rhs(s: Span) -> IResult<Span, Vec<WideLine<T>>> {
    // Parse RHS header with optional trailing spaces
    let (s, _) = tag("RHS")(s)?;
    let (s, _) = space0(s)?; // Skip optional trailing spaces
    let (s, _) = line_ending_flexible(s)?;

    let mut p =
      map(many0(Self::rhs_line), |lines: Vec<Option<WideLine<T>>>| {
        // Filter out None values (comment/empty lines)
        lines.into_iter().flatten().collect()
      });
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn ranges_line(s: Span) -> IResult<Span, Option<WideLine<T>>> {
    // Try to skip comment or empty lines first
    if let Ok((s, _)) = alt((Self::comment_line, Self::empty_line))(s) {
      return Ok((s, None));
    }

    // Check if we've hit another section header
    let peeked = peek(not_line_ending)(s)?;
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let line_str = peeked.1.fragment();
      } else {
        let line_str = peeked.1;
      }
    }

    // If line doesn't start with space, it's likely a section header - stop parsing ranges
    if !line_str.starts_with(' ') {
      return Err(nom::Err::Error(nom::error::Error::new(
        s,
        nom::error::ErrorKind::Eof,
      )));
    }

    let (s, wide_line) = Self::line(s)?;
    Ok((s, Some(wide_line)))
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn ranges(s: Span) -> IResult<Span, Vec<WideLine<T>>> {
    // Parse RANGES header with optional trailing spaces
    let (s, _) = tag("RANGES")(s)?;
    let (s, _) = space0(s)?; // Skip optional trailing spaces
    let (s, _) = line_ending_flexible(s)?;

    let mut p = map(
      many0(Self::ranges_line),
      |lines: Vec<Option<WideLine<T>>>| {
        // Filter out None values (comment/empty lines)
        lines.into_iter().flatten().collect()
      },
    );
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn bound_type(s: Span) -> IResult<Span, BoundType> {
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let f = |z: LocatedSpan<&str, TracableInfo>| BoundType::try_from(*z.fragment());
      } else {
        let f = BoundType::try_from;
      }
    }
    let mut p = map_res(
      alt((
        tag("LO"),
        tag("UP"),
        tag("FX"),
        tag("FR"),
        tag("MI"),
        tag("PL"),
        tag("BV"),
        tag("LI"),
        tag("UI"),
        tag("SC"),
      )),
      f,
    );
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn bounds_line(s: Span) -> IResult<Span, Option<BoundsLine<T>>> {
    // Try to skip comment or empty lines first
    if let Ok((s, _)) = alt((Self::comment_line, Self::empty_line))(s) {
      return Ok((s, None));
    }

    // Check if we've hit another section header
    let peeked = peek(not_line_ending)(s)?;
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let line_str = peeked.1.fragment();
      } else {
        let line_str = peeked.1;
      }
    }

    // If line doesn't start with space, it's likely a section header - stop parsing bounds
    if !line_str.starts_with(' ') {
      return Err(nom::Err::Error(nom::error::Error::new(
        s,
        nom::error::ErrorKind::Eof,
      )));
    }

    let mut p = map_res(
      terminated(preceded(tag(" "), not_line_ending), line_ending_flexible),
      |line: Span| -> Result<BoundsLine<T>> {
        // Try strict field positioning first (no comment stripping for strict)
        let strict_result = Self::parse_bounds_strict(line);

        // If strict parsing fails, try flexible whitespace-separated parsing
        if strict_result.is_err() {
          Self::parse_bounds_flexible(line)
        } else {
          strict_result
        }
      },
    );

    let (s, bounds_line) = p(s)?;
    Ok((s, Some(bounds_line)))
  }

  /// Parse bounds line using strict field positioning
  fn parse_bounds_strict(line: Span) -> Result<BoundsLine<T>> {
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let line_str = *line.fragment();
      } else {
        let line_str = line;
      }
    }
    let bytes = line_str.as_bytes();
    let length = line_str.len();
    
    // Helper: get trimmed slice from fixed positions (returns empty string for whitespace-only)
    let get_trimmed = |start: usize, end: usize| -> Option<&str> {
      let end = end.min(length);
      if end <= start { return None; }
      let s = &bytes[start..end];
      let mut first = 0;
      while first < s.len() && s[first].is_ascii_whitespace() { first += 1; }
      let mut last = s.len();
      while last > first && s[last-1].is_ascii_whitespace() { last -= 1; }
      Some(unsafe { std::str::from_utf8_unchecked(&s[first..last]) })
    };
    
    let bound_type_str = get_trimmed(L1, R1).ok_or_eyre("")?;
    let bound_type = BoundType::try_from(bound_type_str)?;
    
    Ok(match bound_type {
      BoundType::Fr | BoundType::Pl => BoundsLine::<T> {
        bound_type,
        bound_name: get_trimmed(L2, R2).ok_or_eyre("")?,
        column_name: get_trimmed(L3, cmp::min(length, R3)).ok_or_eyre("")?,
        value: None,
      },
      _ => {
        // Check for sign character just before the value field
        if L4 > 0 && length > L4 {
          let prev = bytes[L4 - 1];
          if prev == b'-' || prev == b'+' {
            return Err(eyre!("potential sign character excluded from value field"));
          }
        }
        BoundsLine::<T> {
          bound_type,
          bound_name: get_trimmed(L2, R2).ok_or_eyre("")?,
          column_name: get_trimmed(L3, R3).ok_or_eyre("")?,
          value: Some(fast_float2::parse(
            get_trimmed(L4, cmp::min(length, R4)).ok_or_eyre("")?,
          )?),
        }
      }
    })
  }

  /// Parse bounds line using flexible whitespace-separated format
  /// Optimized: zero-allocation parser using span slicing
  fn parse_bounds_flexible(line: Span) -> Result<BoundsLine<T>> {
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let line_str = line.fragment();
      } else {
        let line_str = line;
      }
    }

    // For flexible parsing, only strip comments if $ appears after significant whitespace
    let line_str = if let Some(pos) = line_str.find("  $") {
      &line_str[..pos]
    } else if let Some(pos) = line_str.find("\t$") {
      &line_str[..pos]
    } else {
      line_str
    };

    // Zero-allocation flexible parser
    let bytes = line_str.as_bytes();
    let len = bytes.len();
    let mut pos = 0;

    // Skip leading whitespace
    while pos < len && bytes[pos].is_ascii_whitespace() {
      pos += 1;
    }

    // Field 1: bound type
    let bt_start = pos;
    while pos < len && !bytes[pos].is_ascii_whitespace() {
      pos += 1;
    }
    if pos == bt_start {
      return Err(eyre!("insufficient fields in bounds line"));
    }
    let bound_type_str = &line_str[bt_start..pos];
    let bound_type = BoundType::try_from(bound_type_str)?;

    // Skip whitespace to field 2
    while pos < len && bytes[pos].is_ascii_whitespace() {
      pos += 1;
    }
    let bn_start = pos;
    while pos < len && !bytes[pos].is_ascii_whitespace() {
      pos += 1;
    }
    let bound_name = &line_str[bn_start..pos];

    // Skip whitespace to field 3
    while pos < len && bytes[pos].is_ascii_whitespace() {
      pos += 1;
    }
    let cn_start = pos;
    while pos < len && !bytes[pos].is_ascii_whitespace() {
      pos += 1;
    }
    let column_name = &line_str[cn_start..pos];

    // Skip whitespace to check for field 4 (value)
    while pos < len && bytes[pos].is_ascii_whitespace() {
      pos += 1;
    }

    Ok(match bound_type {
      BoundType::Fr | BoundType::Pl => BoundsLine::<T> {
        bound_type,
        bound_name,
        column_name,
        value: None,
      },
      _ => {
        let value = if pos < len {
          let val_start = pos;
          while pos < len && !bytes[pos].is_ascii_whitespace() {
            pos += 1;
          }
          Some(fast_float2::parse(&line_str[val_start..pos])?)
        } else {
          None
        };
        BoundsLine::<T> {
          bound_type,
          bound_name,
          column_name,
          value,
        }
      }
    })
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn bounds(s: Span) -> IResult<Span, Vec<BoundsLine<T>>> {
    // Parse BOUNDS header with optional trailing spaces
    let (s, _) = tag("BOUNDS")(s)?;
    let (s, _) = space0(s)?; // Skip optional trailing spaces
    let (s, _) = line_ending_flexible(s)?;

    let mut p = map(
      many0(Self::bounds_line),
      |lines: Vec<Option<BoundsLine<T>>>| {
        // Filter out None values (comment/empty lines)
        lines.into_iter().flatten().collect()
      },
    );
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  // ============================================================================
  // MIP/QP Extension Parsers
  // ============================================================================

  #[doc(hidden)]
  #[tracable_parser]
  pub fn usercuts(s: Span) -> IResult<Span, Vec<RowLine>> {
    let (s, _) = tag("USERCUTS")(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = line_ending_flexible(s)?;

    // USERCUTS section has same format as ROWS section
    let mut p = map(
      many0(Self::row_line_or_end),
      |lines: Vec<Option<RowLine>>| lines.into_iter().flatten().collect(),
    );

    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn indicators_line(s: Span) -> IResult<Span, Option<IndicatorLine>> {
    // Try to skip comment or empty lines first
    if let Ok((s, _)) = alt((Self::comment_line, Self::empty_line))(s) {
      return Ok((s, None));
    }

    // Check if we've hit another section header
    let peeked = peek(not_line_ending)(s)?;
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let line_str = peeked.1.fragment();
      } else {
        let line_str = peeked.1;
      }
    }

    // If line doesn't start with space, it's likely a section header
    if !line_str.starts_with(' ') {
      return Err(nom::Err::Error(nom::error::Error::new(
        s,
        nom::error::ErrorKind::Eof,
      )));
    }

    let mut p = map_res(
      terminated(preceded(tag(" "), not_line_ending), line_ending_flexible),
      |line: Span| -> Result<IndicatorLine> {
        cfg_if::cfg_if! {
          if #[cfg(feature = "trace")] {
            let line_str = line.fragment();
          } else {
            let line_str = line;
          }
        }

        // Format: IF constraint_name binary_var_name trigger_value
        // Zero-allocation parser
        let bytes = line_str.as_bytes();
        let len = bytes.len();
        let mut pos = 0;
        
        // Skip leading whitespace
        while pos < len && bytes[pos].is_ascii_whitespace() {
          pos += 1;
        }
        
        // Check for "IF" prefix
        if pos + 2 > len || &line_str[pos..pos+2] != "IF" {
          return Err(eyre!("invalid indicator line format"));
        }
        pos += 2;
        
        // Skip whitespace
        while pos < len && bytes[pos].is_ascii_whitespace() {
          pos += 1;
        }
        let c1_start = pos;
        while pos < len && !bytes[pos].is_ascii_whitespace() {
          pos += 1;
        }
        let constraint_name = &line_str[c1_start..pos];
        
        // Skip whitespace
        while pos < len && bytes[pos].is_ascii_whitespace() {
          pos += 1;
        }
        let bv_start = pos;
        while pos < len && !bytes[pos].is_ascii_whitespace() {
          pos += 1;
        }
        let binary_var = &line_str[bv_start..pos];
        
        // Skip whitespace
        while pos < len && bytes[pos].is_ascii_whitespace() {
          pos += 1;
        }
        let tv_start = pos;
        while pos < len && !bytes[pos].is_ascii_whitespace() {
          pos += 1;
        }
        let trigger_str = &line_str[tv_start..pos];
        
        let trigger_value = match trigger_str {
          "0" => 0,
          "1" => 1,
          _ => return Err(eyre!("indicator trigger must be 0 or 1")),
        };

        Ok(IndicatorLine {
          binary_var,
          trigger_value,
          constraint_name,
        })
      },
    );

    let (s, indicator) = p(s)?;
    Ok((s, Some(indicator)))
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn indicators(s: Span) -> IResult<Span, Vec<IndicatorLine>> {
    let (s, _) = tag("INDICATORS")(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = line_ending_flexible(s)?;

    let mut p = map(
      many0(Self::indicators_line),
      |lines: Vec<Option<IndicatorLine>>| lines.into_iter().flatten().collect(),
    );
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn lazycons_line(s: Span) -> IResult<Span, Option<LazyConstraintLine>> {
    // Try to skip comment or empty lines first
    if let Ok((s, _)) = alt((Self::comment_line, Self::empty_line))(s) {
      return Ok((s, None));
    }

    // Check if we've hit another section header
    let peeked = peek(not_line_ending)(s)?;
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let line_str = peeked.1.fragment();
      } else {
        let line_str = peeked.1;
      }
    }

    if !line_str.starts_with(' ') {
      return Err(nom::Err::Error(nom::error::Error::new(
        s,
        nom::error::ErrorKind::Eof,
      )));
    }

    let mut p = map_res(
      terminated(preceded(tag(" "), not_line_ending), line_ending_flexible),
      |line: Span| -> Result<LazyConstraintLine> {
        cfg_if::cfg_if! {
          if #[cfg(feature = "trace")] {
            let line_str = line.fragment();
          } else {
            let line_str = line;
          }
        }

        // Zero-allocation parser for lazy constraint line
        // Format: [priority] row_name
        let bytes = line_str.as_bytes();
        let len = bytes.len();
        let mut pos = 0;
        
        // Skip leading whitespace
        while pos < len && bytes[pos].is_ascii_whitespace() {
          pos += 1;
        }
        
        if pos >= len {
          return Err(eyre!("empty lazy constraint line"));
        }
        
        // Try to parse priority (first field)
        let first_start = pos;
        while pos < len && !bytes[pos].is_ascii_whitespace() {
          pos += 1;
        }
        let first_field = &line_str[first_start..pos];
        
        // Skip whitespace
        while pos < len && bytes[pos].is_ascii_whitespace() {
          pos += 1;
        }
        
        // Parse row name (second field, or first if no priority)
        let row_start = pos;
        while pos < len && !bytes[pos].is_ascii_whitespace() {
          pos += 1;
        }
        let row_name = &line_str[row_start..pos];
        
        let (priority, row_name) = if first_field.parse::<i32>().is_ok() && pos < len {
          // First field was priority, we have a row name
          (Some(first_field.parse::<i32>().unwrap()), row_name)
        } else {
          // First field was row name (no priority)
          (None, first_field)
        };

        Ok(LazyConstraintLine { priority, row_name })
      },
    );

    let (s, lazy_constraint) = p(s)?;
    Ok((s, Some(lazy_constraint)))
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn lazycons(s: Span) -> IResult<Span, Vec<LazyConstraintLine>> {
    let (s, _) = tag("LAZYCONS")(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = line_ending_flexible(s)?;

    let mut p = map(
      many0(Self::lazycons_line),
      |lines: Vec<Option<LazyConstraintLine>>| {
        lines.into_iter().flatten().collect()
      },
    );
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn quadobj_line(
    s: Span,
  ) -> IResult<Span, Option<QuadraticObjectiveTerm<T>>> {
    // Try to skip comment or empty lines first
    if let Ok((s, _)) = alt((Self::comment_line, Self::empty_line))(s) {
      return Ok((s, None));
    }

    let peeked = peek(not_line_ending)(s)?;
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let line_str = peeked.1.fragment();
      } else {
        let line_str = peeked.1;
      }
    }

    if !line_str.starts_with(' ') {
      return Err(nom::Err::Error(nom::error::Error::new(
        s,
        nom::error::ErrorKind::Eof,
      )));
    }

    let mut p = map_res(
      terminated(preceded(tag(" "), not_line_ending), line_ending_flexible),
      |line: Span| -> Result<QuadraticObjectiveTerm<T>> {
        cfg_if::cfg_if! {
          if #[cfg(feature = "trace")] {
            let line_str = line.fragment();
          } else {
            let line_str = line;
          }
        }

        let len = line_str.len();
        let var1 = line_str
          .get(L2..R2.min(len))
          .ok_or_eyre("missing var1 field in quadobj line")?
          .trim();
        let var2 = line_str
          .get(L3..R3.min(len))
          .ok_or_eyre("missing var2 field in quadobj line")?
          .trim();
        // Extend coefficient field to end-of-line to capture unpadded values
        let val_str = line_str
          .get(L4..len)
          .ok_or_eyre("missing coefficient field in quadobj line")?
          .trim();

        if var1.is_empty() || var2.is_empty() || val_str.is_empty() {
          return Err(eyre!("empty field in quadobj line"));
        }

        Ok(QuadraticObjectiveTerm {
          var1,
          var2,
          coefficient: fast_float2::parse(val_str)?,
        })
      },
    );

    let (s, term) = p(s)?;
    Ok((s, Some(term)))
  }

  /// Parses the QSECTION (quadratic objective) section.
  ///
  /// QSECTION is an alternative format for specifying quadratic objective coefficients.
  /// It is functionally equivalent to QUADOBJ but may be used interchangeably.
  /// Both specify quadratic terms in the objective function as: coefficient * var1 * var2
  ///
  /// Per CPLEX MPS specification:
  /// - Field 2: First variable name (must be in COLUMNS section)
  /// - Field 3: Second variable name (must be in COLUMNS section)
  /// - Field 4: Coefficient value (non-zero coefficients only)
  /// - Zero entries and duplicate (i,j) and (j,i) pairs should be avoided
  #[doc(hidden)]
  #[tracable_parser]
  pub fn qsection(s: Span) -> IResult<Span, Vec<QuadraticObjectiveTerm<T>>> {
    let (s, _) = tag("QSECTION")(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = line_ending_flexible(s)?;

    let mut p = map(
      many0(Self::quadobj_line),
      |lines: Vec<Option<QuadraticObjectiveTerm<T>>>| {
        lines.into_iter().flatten().collect()
      },
    );
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  /// Parses the QUADOBJ (quadratic objective) section.
  ///
  /// QUADOBJ specifies quadratic objective coefficients using only upper diagonal
  /// elements of the Q matrix. It is functionally equivalent to QSECTION.
  /// Both specify quadratic terms in the objective function as: coefficient * var1 * var2
  ///
  /// Per CPLEX MPS specification:
  /// - Field 2: First variable name (must be in COLUMNS section)
  /// - Field 3: Second variable name (must be in COLUMNS section)
  /// - Field 4: Coefficient value (non-zero coefficients only)
  /// - Only upper diagonal elements should be specified
  #[doc(hidden)]
  #[tracable_parser]
  pub fn quadobj(s: Span) -> IResult<Span, Vec<QuadraticObjectiveTerm<T>>> {
    let (s, _) = tag("QUADOBJ")(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = line_ending_flexible(s)?;

    let mut p = map(
      many0(Self::quadobj_line),
      |lines: Vec<Option<QuadraticObjectiveTerm<T>>>| {
        lines.into_iter().flatten().collect()
      },
    );
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  /// Parses a single SOS member line (variable and weight).
  ///
  /// Format: var_name weight
  #[doc(hidden)]
  #[tracable_parser]
  pub fn sos_member_line(s: Span) -> IResult<Span, Option<SOSMember<T>>> {
    // Try to skip comment or empty lines first
    if let Ok((s, _)) = alt((Self::comment_line, Self::empty_line))(s) {
      return Ok((s, None));
    }

    let peeked = peek(not_line_ending)(s)?;
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let line_str = peeked.1.fragment();
      } else {
        let line_str = peeked.1;
      }
    }

    // If line doesn't start with space or is a section header, we're done
    if !line_str.starts_with(' ')
      || line_str.trim_start().starts_with("S1")
      || line_str.trim_start().starts_with("S2")
    {
      return Err(nom::Err::Error(nom::error::Error::new(
        s,
        nom::error::ErrorKind::Eof,
      )));
    }

    let mut p = map_res(
      terminated(preceded(tag(" "), not_line_ending), line_ending_flexible),
      |line: Span| -> Result<SOSMember<T>> {
        cfg_if::cfg_if! {
          if #[cfg(feature = "trace")] {
            let line_str = line.fragment();
          } else {
            let line_str = line;
          }
        }

        // Zero-allocation parser for SOS member line
        // Format: var_name weight
        let bytes = line_str.as_bytes();
        let len = bytes.len();
        let mut pos = 0;
        
        // Skip leading whitespace
        while pos < len && bytes[pos].is_ascii_whitespace() {
          pos += 1;
        }
        
        let var_start = pos;
        while pos < len && !bytes[pos].is_ascii_whitespace() {
          pos += 1;
        }
        if pos == var_start {
          return Err(eyre!("SOS member line requires variable name and weight"));
        }
        let var_name = &line_str[var_start..pos];
        
        // Skip whitespace
        while pos < len && bytes[pos].is_ascii_whitespace() {
          pos += 1;
        }
        
        let weight_start = pos;
        while pos < len && !bytes[pos].is_ascii_whitespace() {
          pos += 1;
        }
        
        if pos == weight_start {
          return Err(eyre!("SOS member line requires variable name and weight"));
        }
        let weight_str = &line_str[weight_start..pos];

        Ok(SOSMember {
          var_name,
          weight: fast_float2::parse(weight_str)?,
        })
      },
    );

    let (s, member) = p(s)?;
    Ok((s, Some(member)))
  }

  /// Parses a single SOS set definition line and its members.
  ///
  /// Format:
  /// S1 set_name
  ///   var1 weight1
  ///   var2 weight2
  ///   ...
  #[doc(hidden)]
  #[tracable_parser]
  pub fn sos_line(s: Span) -> IResult<Span, Option<SOSLine<T>>> {
    // Try to skip comment or empty lines first
    if let Ok((s, _)) = alt((Self::comment_line, Self::empty_line))(s) {
      return Ok((s, None));
    }

    let peeked = peek(not_line_ending)(s)?;
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let line_str = peeked.1.fragment();
      } else {
        let line_str = peeked.1;
      }
    }

    if !line_str.starts_with(' ') {
      return Err(nom::Err::Error(nom::error::Error::new(
        s,
        nom::error::ErrorKind::Eof,
      )));
    }

    // Parse the set definition line
    let (s, set_def_line) =
      terminated(preceded(tag(" "), not_line_ending), line_ending_flexible)(s)?;

    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let set_def_str = set_def_line.fragment();
      } else {
        let set_def_str = set_def_line;
      }
    }

    // Zero-allocation parser for SOS set definition line
    // Format: S1/S2 set_name
    let bytes = set_def_str.as_bytes();
    let len = bytes.len();
    let mut pos = 0;
    
    // Skip leading whitespace
    while pos < len && bytes[pos].is_ascii_whitespace() {
      pos += 1;
    }
    
    let type_start = pos;
    while pos < len && !bytes[pos].is_ascii_whitespace() {
      pos += 1;
    }
    if pos == type_start {
      return Err(nom::Err::Error(nom::error::Error::new(
        s,
        nom::error::ErrorKind::Fail,
      )));
    }
    let type_str = &set_def_str[type_start..pos];
    
    // Skip whitespace
    while pos < len && bytes[pos].is_ascii_whitespace() {
      pos += 1;
    }
    
    let name_start = pos;
    while pos < len && !bytes[pos].is_ascii_whitespace() {
      pos += 1;
    }
    if pos == name_start {
      return Err(nom::Err::Error(nom::error::Error::new(
        s,
        nom::error::ErrorKind::Fail,
      )));
    }
    let set_name = &set_def_str[name_start..pos];

    let sos_type = match SOSType::try_from(type_str) {
      Ok(t) => t,
      Err(_) => {
        return Err(nom::Err::Error(nom::error::Error::new(
          s,
          nom::error::ErrorKind::Fail,
        )))
      }
    };

    // Now collect member lines until we hit a line that doesn't start with space
    let (s, member_lines) = many0(Self::sos_member_line)(s)?;
    let members: Vec<SOSMember<T>> =
      member_lines.into_iter().flatten().collect();

    Ok((
      s,
      Some(SOSLine {
        sos_type,
        set_name,
        members,
      }),
    ))
  }

  /// Parses the SOS section.
  ///
  /// Per CPLEX MPS specification: defines special ordered sets for integer programming.
  #[doc(hidden)]
  #[tracable_parser]
  pub fn sos(s: Span) -> IResult<Span, Vec<SOSLine<T>>> {
    let (s, _) = tag("SOS")(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = line_ending_flexible(s)?;

    let mut p = map(many0(Self::sos_line), |lines: Vec<Option<SOSLine<T>>>| {
      lines.into_iter().flatten().collect()
    });
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn qmatrix_line(s: Span) -> IResult<Span, Option<QuadraticTerm<T>>> {
    // Try to skip comment or empty lines first
    if let Ok((s, _)) = alt((Self::comment_line, Self::empty_line))(s) {
      return Ok((s, None));
    }

    let peeked = peek(not_line_ending)(s)?;
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let line_str = peeked.1.fragment();
      } else {
        let line_str = peeked.1;
      }
    }

    if !line_str.starts_with(' ') {
      return Err(nom::Err::Error(nom::error::Error::new(
        s,
        nom::error::ErrorKind::Eof,
      )));
    }

    let mut p = map_res(
      terminated(preceded(tag(" "), not_line_ending), line_ending_flexible),
      |line: Span| -> Result<QuadraticTerm<T>> {
        cfg_if::cfg_if! {
          if #[cfg(feature = "trace")] {
            let line_str = line.fragment();
          } else {
            let line_str = line;
          }
        }

        let len = line_str.len();
        let var1 = line_str
          .get(L2..R2.min(len))
          .ok_or_eyre("missing var1 field in QMATRIX line")?
          .trim();
        let var2 = line_str
          .get(L3..R3.min(len))
          .ok_or_eyre("missing var2 field in QMATRIX line")?
          .trim();
        // Extend coefficient field to end-of-line to capture unpadded values
        let val_str = line_str
          .get(L4..len)
          .ok_or_eyre("missing coefficient field in QMATRIX line")?
          .trim();

        if var1.is_empty() || var2.is_empty() || val_str.is_empty() {
          return Err(eyre!("empty field in QMATRIX line"));
        }

        Ok(QuadraticTerm {
          var1,
          var2,
          coefficient: fast_float2::parse(val_str)?,
        })
      },
    );

    let (s, term) = p(s)?;
    Ok((s, Some(term)))
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn qmatrix(s: Span) -> IResult<Span, Vec<QuadraticConstraint<T>>> {
    let (s, _) = tag("QMATRIX")(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = line_ending_flexible(s)?;

    let mut p = map(
      many0(Self::qmatrix_line),
      |lines: Vec<Option<QuadraticTerm<T>>>| {
        let terms: Vec<QuadraticTerm<T>> =
          lines.into_iter().flatten().collect();
        vec![QuadraticConstraint {
          row_name: "OBJ",
          terms,
        }]
      },
    );

    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn qcmatrix(s: Span) -> IResult<Span, Vec<QuadraticConstraint<T>>> {
    let (s, _) = tag("QCMATRIX")(s)?;
    let (s, _) = space0(s)?;
    let (s, constraint_name) = map(
      opt(not_line_ending),
      |opt_name: Option<Span>| match opt_name {
        Some(name) => {
          cfg_if::cfg_if! {
            if #[cfg(feature = "trace")] {
              name.fragment().trim()
            } else {
              name.trim()
            }
          }
        }
        None => "QC",
      },
    )(s)?;
    let (s, _) = line_ending_flexible(s)?;

    let mut p = map(
      many0(Self::qmatrix_line),
      move |lines: Vec<Option<QuadraticTerm<T>>>| {
        let terms: Vec<QuadraticTerm<T>> =
          lines.into_iter().flatten().collect();
        vec![QuadraticConstraint {
          row_name: constraint_name,
          terms,
        }]
      },
    );

    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn csection_line(s: Span<'_>) -> IResult<Span<'_>, Option<&str>> {
    // Try to skip comment or empty lines first
    if let Ok((s, _)) = alt((Self::comment_line, Self::empty_line))(s) {
      return Ok((s, None));
    }

    let peeked = peek(not_line_ending)(s)?;
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let line_str = peeked.1.fragment();
      } else {
        let line_str = peeked.1;
      }
    }

    if !line_str.starts_with(' ') {
      return Err(nom::Err::Error(nom::error::Error::new(
        s,
        nom::error::ErrorKind::Eof,
      )));
    }

    let mut p = map(
      terminated(preceded(tag(" "), not_line_ending), line_ending_flexible),
      |line: Span| {
        cfg_if::cfg_if! {
          if #[cfg(feature = "trace")] {
            Some(line.fragment().trim())
          } else {
            Some(line.trim())
          }
        }
      },
    );

    p(s)
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn csection(s: Span) -> IResult<Span, Vec<ConeConstraint<T>>> {
    let (s, _) = tag("CSECTION")(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = line_ending_flexible(s)?;

    // Try to parse cone type (QUAD or RQUAD) on first line
    let mut cone_type = ConeType::Quad;
    let mut current = s;

    // Check if first content line is a cone type or a variable
    let peeked = peek(opt(preceded(tag(" "), alpha1)))(current)?;
    if let (_, Some(type_str)) = peeked {
      cfg_if::cfg_if! {
        if #[cfg(feature = "trace")] {
          let type_str_val = *type_str.fragment();
        } else {
          let type_str_val = type_str;
        }
      }

      if type_str_val == "QUAD" || type_str_val == "RQUAD" {
        cone_type = if type_str_val == "QUAD" {
          ConeType::Quad
        } else {
          ConeType::RQuad
        };
        // Consume the cone type line
        let (next, _) = terminated(
          preceded(tag(" "), not_line_ending),
          line_ending_flexible,
        )(current)?;
        current = next;
      }
    }

    // Parse the variable names
    let (s, lines) = many0(Self::csection_line)(current)?;
    let members: Vec<ConeMember<T>> = lines
      .into_iter()
      .flatten()
      .map(|var_name| ConeMember {
        var_name,
        coefficient: None,
      })
      .collect();
    let result = vec![ConeConstraint {
      cone_name: "CONE",
      cone_type,
      members,
    }];

    Ok((s, result))
  }

  /// Parses a single branching priority line.
  ///
  /// Format: [direction] [var_name] [priority]
  /// Direction can be: UP, DN, RD, CB (or blank for auto)
  #[doc(hidden)]
  #[tracable_parser]
  pub fn branch_line(s: Span) -> IResult<Span, Option<BranchPriority>> {
    // Try to skip comment or empty lines first
    if let Ok((s, _)) = alt((Self::comment_line, Self::empty_line))(s) {
      return Ok((s, None));
    }

    // Check if we've hit another section header
    let peeked = peek(not_line_ending)(s)?;
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let line_str = peeked.1.fragment();
      } else {
        let line_str = peeked.1;
      }
    }

    // If line doesn't start with space, it's likely a section header
    if !line_str.starts_with(' ') {
      return Err(nom::Err::Error(nom::error::Error::new(
        s,
        nom::error::ErrorKind::Eof,
      )));
    }

    let mut p = map_res(
      terminated(preceded(tag(" "), not_line_ending), line_ending_flexible),
      |line: Span| -> Result<BranchPriority> {
        cfg_if::cfg_if! {
          if #[cfg(feature = "trace")] {
            let line_str = line.fragment();
          } else {
            let line_str = line;
          }
        }

        // Zero-allocation parser for branch line
        // Format: [direction] var_name priority
        let bytes = line_str.as_bytes();
        let len = bytes.len();
        let mut pos = 0;
        let mut fields: [&str; 3] = ["", "", ""];
        let mut field_count = 0;
        
        // Skip leading whitespace
        while pos < len && bytes[pos].is_ascii_whitespace() {
          pos += 1;
        }
        
        while pos < len && field_count < 3 {
          let field_start = pos;
          while pos < len && !bytes[pos].is_ascii_whitespace() {
            pos += 1;
          }
          fields[field_count] = &line_str[field_start..pos];
          field_count += 1;
          
          // Skip whitespace
          while pos < len && bytes[pos].is_ascii_whitespace() {
            pos += 1;
          }
        }
        
        if field_count == 0 {
          return Err(eyre!("empty branch line"));
        }

        // Parse: [direction] var_name priority
        let (direction, var_name, priority_str) = if field_count >= 3 {
          // Try to parse first field as direction
          match BranchDirection::try_from(fields[0]) {
            Ok(dir) => (dir, fields[1], fields[2]),
            Err(_) => (BranchDirection::Auto, fields[0], fields[1]),
          }
        } else if field_count == 2 {
          (BranchDirection::Auto, fields[0], fields[1])
        } else {
          return Err(eyre!("insufficient fields in branch line"));
        };

        let priority_val = priority_str.parse::<i32>()?;
        if priority_val < 0 {
          return Err(eyre!("branch priority must be non-negative"));
        }

        Ok(BranchPriority {
          var_name,
          priority: priority_val,
          direction,
        })
      },
    );

    let (s, branch) = p(s)?;
    Ok((s, Some(branch)))
  }

  /// Parses the BRANCH section (branching priorities).
  ///
  /// Per CPLEX MPS specification, specifies branching priorities and directions
  /// to guide the branch-and-bound algorithm for integer programming problems.
  /// Variables with higher priorities are branched first.
  #[doc(hidden)]
  #[tracable_parser]
  pub fn branch(s: Span) -> IResult<Span, Vec<BranchPriority>> {
    let (s, _) = tag("BRANCH")(s)?;
    let (s, _) = space0(s)?;
    let (s, _) = line_ending_flexible(s)?;

    let mut p = map(
      many0(Self::branch_line),
      |lines: Vec<Option<BranchPriority>>| {
        lines.into_iter().flatten().collect()
      },
    );
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  #[doc(hidden)]
  pub fn endata(s: Span<'_>) -> IResult<Span<'_>, &str> {
    let p = tag("ENDATA");
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x.fragment()))
      } else { p(s) }
    }
  }
}
