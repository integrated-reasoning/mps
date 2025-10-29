use crate::types::*;
use color_eyre::{eyre::eyre, eyre::OptionExt, Result};
use fast_float::FastFloat;
use nom::{
  branch::alt,
  bytes::complete::tag,
  character::complete::*,
  combinator::{map, map_res, opt, peek},
  multi::many0,
  sequence::{preceded, terminated, tuple},
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
  #[tracable_parser]
  pub fn mps_file(s: Span<'a>) -> IResult<Span<'a>, Parser<'a, T>> {
    let mut p = map(
      tuple((
        many0(Self::skip_line),
        Self::name,
        many0(Self::skip_line),
        Self::rows,
        many0(Self::skip_line),
        Self::columns,
        many0(Self::skip_line),
        opt(Self::rhs),
        many0(Self::skip_line),
        opt(Self::ranges),
        many0(Self::skip_line),
        opt(Self::bounds),
        many0(Self::skip_line),
        many0(Self::skip_unknown_section),
        Self::endata,
      )),
      |(
        _,
        name,
        _,
        rows,
        _,
        columns,
        _,
        rhs,
        _,
        ranges,
        _,
        bounds,
        _,
        _,
        _,
      )| Parser {
        name: name.trim(),
        rows,
        columns,
        rhs,
        ranges,
        bounds,
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
  #[tracable_parser]
  pub fn skip_unknown_section(s: Span) -> IResult<Span, ()> {
    // Skip sections like INDICATORS, LAZYCONS, etc.
    // First try to match a known section header
    let (s, _) = alt((
      tag("INDICATORS"),
      tag("LAZYCONS"),
      tag("QUADOBJ"),
      tag("SOS"),
      tag("QSECTION"),
      tag("QMATRIX"),
      tag("CSECTION"),
    ))(s)?;

    // Skip to the end of the line
    let (s, _) = opt(not_line_ending)(s)?;
    let (s, _) = line_ending_flexible(s)?;

    // Skip all lines that start with a space (section content)
    // Continue until we hit another section header or ENDATA
    let mut current = s;
    loop {
      // Check if we've hit another section header or ENDATA
      // Use a simpler check to avoid type inference issues
      cfg_if::cfg_if! {
        if #[cfg(feature = "trace")] {
          let test_str = current.fragment();
        } else {
          let test_str = current;
        }
      }

      if test_str.starts_with("COLUMNS")
        || test_str.starts_with("RHS")
        || test_str.starts_with("RANGES")
        || test_str.starts_with("BOUNDS")
        || test_str.starts_with("ENDATA")
        || test_str.starts_with("INDICATORS")
        || test_str.starts_with("LAZYCONS")
        || test_str.starts_with("QUADOBJ")
        || test_str.starts_with("SOS")
        || test_str.starts_with("QSECTION")
        || test_str.starts_with("QMATRIX")
        || test_str.starts_with("CSECTION")
      {
        break;
      }

      // Try to consume a line
      match terminated(not_line_ending, line_ending_flexible)(current) {
        Ok((next, _)) => current = next,
        Err(_) => break,
      }
    }

    Ok((current, ()))
  }

  #[doc(hidden)]
  pub fn name(s: Span) -> IResult<Span, &str> {
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

        // Skip leading spaces and get the content
        let content = line_str.trim_start();
        if content.is_empty() {
          return Err(eyre!("empty row line"));
        }

        // First character should be the row type (E, L, G, N)
        let first_char = content.chars().next().ok_or_eyre("no row type")?;
        let row_type = RowType::try_from(first_char)?;

        // Rest is the row name - skip all spaces after the type character
        let rest = &content[1..];
        let row_name = rest.trim();

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
        let strict_result = (|| -> Result<WideLine<T>> {
          let first_pair = RowValuePair {
            row_name: line_str.get(L3..R3).ok_or_eyre("")?.trim(),
            value: fast_float::parse(
              line_str.get(L4..R4).ok_or_eyre("")?.trim(),
            )?,
          };
          let second_pair = match line_str.get(L5..R5) {
            Some(row_name) => {
              let row_name = row_name.trim();
              if row_name.is_empty() {
                None
              } else {
                Some(RowValuePair {
                  row_name,
                  value: fast_float::parse(
                    line_str.get(L6..R6).ok_or_eyre("")?.trim(),
                  )?,
                })
              }
            }
            None => None,
          };
          Ok(WideLine::<T> {
            name: line_str.get(L2..R2).ok_or_eyre("")?.trim(),
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

    let parts: Vec<&str> = line_str.split_whitespace().collect();

    // Minimum: column_name row_name value
    if parts.len() < 3 {
      return Err(eyre!("insufficient fields in line"));
    }

    let name = parts[0];
    let first_pair = RowValuePair {
      row_name: parts[1],
      value: fast_float::parse(parts[2])?,
    };

    // Check if there's a second pair (row_name value)
    let second_pair = if parts.len() >= 5 {
      Some(RowValuePair {
        row_name: parts[3],
        value: fast_float::parse(parts[4])?,
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
    let length = line.len();
    let bound_type = BoundType::try_from(line.get(L1..R1).ok_or_eyre("")?)?;
    Ok(match bound_type {
      BoundType::Fr | BoundType::Pl => BoundsLine::<T> {
        bound_type,
        bound_name: line.get(L2..R2).ok_or_eyre("")?.trim(),
        column_name: line.get(L3..cmp::min(length, R3)).ok_or_eyre("")?.trim(),
        value: None,
      },
      _ => BoundsLine::<T> {
        bound_type,
        bound_name: line.get(L2..R2).ok_or_eyre("")?.trim(),
        column_name: line.get(L3..R3).ok_or_eyre("")?.trim(),
        value: Some(fast_float::parse(
          line.get(L4..cmp::min(length, R4)).ok_or_eyre("")?.trim(),
        )?),
      },
    })
  }

  /// Parse bounds line using flexible whitespace-separated format
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

    let parts: Vec<&str> = line_str.split_whitespace().collect();

    // Minimum: bound_type bound_name column_name [value]
    if parts.len() < 3 {
      return Err(eyre!("insufficient fields in bounds line"));
    }

    let bound_type = BoundType::try_from(parts[0])?;
    let bound_name = parts[1];
    let column_name = parts[2];

    Ok(match bound_type {
      BoundType::Fr | BoundType::Pl => BoundsLine::<T> {
        bound_type,
        bound_name,
        column_name,
        value: None,
      },
      _ => {
        let value = if parts.len() >= 4 {
          Some(fast_float::parse(parts[3])?)
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

  #[doc(hidden)]
  pub fn endata(s: Span) -> IResult<Span, &str> {
    let p = tag("ENDATA");
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x.fragment()))
      } else { p(s) }
    }
  }
}
