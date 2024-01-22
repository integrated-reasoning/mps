use crate::types::*;
use color_eyre::{eyre::OptionExt, Result};
use fast_float::FastFloat;
use nom::{
  branch::alt,
  bytes::complete::{tag, take_while1},
  character::complete::*,
  combinator::{map, map_res, opt, peek},
  multi::{count, many0},
  sequence::{preceded, separated_pair, terminated, tuple},
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

fn not_whitespace1(s: Span) -> IResult<Span, &str> {
  let p = take_while1(|c: char| !c.is_whitespace());
  cfg_if::cfg_if! {
    if #[cfg(feature = "trace")] {
      let (s, x) = p(s)?;
      Ok((s, x.fragment()))
    } else { p(s) }
  }
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
    let (_, parsed) = Parser::<T>::mps_file(input).map_err(|_| {
      nom::error::Error::new(input.to_string(), nom::error::ErrorKind::Fail)
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
        Self::name,
        Self::rows,
        Self::columns,
        opt(Self::rhs),
        opt(Self::ranges),
        opt(Self::bounds),
        Self::endata,
      )),
      |(name, rows, columns, rhs, ranges, bounds, _)| Parser {
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
  pub fn name(s: Span) -> IResult<Span, &str> {
    let mut p = terminated(
      preceded(tag("NAME"), preceded(count(anychar, 10), not_line_ending)),
      newline,
    );
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x.fragment()))
      } else { p(s) }
    }
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn row_line(s: Span) -> IResult<Span, RowLine> {
    let mut p = map_res(
      terminated(
        preceded(tag(" "), tuple((one_of("ELGN"), not_line_ending))),
        newline,
      ),
      |t: (char, Span)| -> Result<RowLine> {
        let line = t.1;
        let row_type = RowType::try_from(t.0)?;
        let row_name = line // -1 to account for the type char
          .get((L2 - 1)..cmp::min(line.len(), R2 - 1))
          .ok_or_eyre("")?
          .trim();
        Ok(RowLine { row_type, row_name })
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
  pub fn rows(s: Span) -> IResult<Span, Vec<RowLine>> {
    let mut p = terminated(
      preceded(terminated(tag("ROWS"), newline), many0(Self::row_line)),
      peek(not_whitespace1),
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
      terminated(preceded(tag(" "), not_line_ending), newline),
      |line: Span| -> Result<WideLine<T>> {
        let first_pair = RowValuePair {
          row_name: line.get(L3..R3).ok_or_eyre("")?.trim(),
          value: fast_float::parse(line.get(L4..R4).ok_or_eyre("")?.trim())?,
        };
        let second_pair = match line.get(L5..R5) {
          Some(row_name) => Some(RowValuePair {
            row_name: row_name.trim(),
            value: fast_float::parse(line.get(L6..R6).ok_or_eyre("")?.trim())?,
          }),
          None => None,
        };
        Ok(WideLine::<T> {
          name: line.get(L2..R2).ok_or_eyre("")?.trim(),
          first_pair,
          second_pair,
        })
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
  pub fn columns_line(s: Span) -> IResult<Span, WideLine<T>> {
    let p = Self::line;
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn columns(s: Span) -> IResult<Span, Vec<WideLine<T>>> {
    let mut p = terminated(
      preceded(
        terminated(tag("COLUMNS"), newline),
        many0(Self::columns_line),
      ),
      peek(not_whitespace1),
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
  pub fn rhs_line(s: Span) -> IResult<Span, WideLine<T>> {
    let p = Self::line;
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn rhs(s: Span) -> IResult<Span, Vec<WideLine<T>>> {
    let mut p = terminated(
      preceded(terminated(tag("RHS"), newline), many0(Self::rhs_line)),
      peek(not_whitespace1),
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
  pub fn ranges_line(s: Span) -> IResult<Span, WideLine<T>> {
    let p = Self::line;
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  #[doc(hidden)]
  #[tracable_parser]
  pub fn ranges(s: Span) -> IResult<Span, Vec<WideLine<T>>> {
    let mut p = terminated(
      preceded(terminated(tag("RANGES"), newline), many0(Self::ranges_line)),
      peek(not_whitespace1),
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
  pub fn bounds_line(s: Span) -> IResult<Span, BoundsLine<T>> {
    let mut p = map_res(
      terminated(preceded(tag(" "), not_line_ending), newline),
      |line: Span| -> Result<BoundsLine<T>> {
        let length = line.len();
        let bound_type = BoundType::try_from(line.get(L1..R1).ok_or_eyre("")?)?;
        Ok(match bound_type {
          BoundType::Fr | BoundType::Pl => BoundsLine::<T> {
            bound_type,
            bound_name: line.get(L2..R2).ok_or_eyre("")?.trim(),
            column_name: line
              .get(L3..cmp::min(length, R3))
              .ok_or_eyre("")?
              .trim(),
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
  pub fn bounds(s: Span) -> IResult<Span, Vec<BoundsLine<T>>> {
    let mut p = terminated(
      preceded(terminated(tag("BOUNDS"), newline), many0(Self::bounds_line)),
      peek(not_whitespace1),
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
