use crate::types::*;
use color_eyre::{eyre::OptionExt, Result};
use nom::{
  branch::alt,
  bytes::complete::{tag, take_while1},
  character::complete::*,
  combinator::{map, map_res, opt, peek},
  multi::{count, many0},
  number::complete::float,
  sequence::{preceded, separated_pair, terminated, tuple},
  IResult,
};
use nom_tracable::tracable_parser;
use num_traits::float::Float;
cfg_if::cfg_if! {
  if #[cfg(feature = "trace")] {
    use nom_locate::LocatedSpan;
    use nom_tracable::TracableInfo;
  }
}

/// Parses a sequence of non-whitespace characters from the given input span.
///
/// This function is a utility to extract continuous non-whitespace characters
/// from the input, returning the parsed string along with the remaining input.
/// It utilizes `take_while1` from `nom` to achieve this.
///
/// # Arguments
///
/// * `s` - The input span from which to parse. The type of `Span` depends on the
///         compilation feature. It's either a basic string slice or a `LocatedSpan`
///         for enhanced error reporting and debugging.
///
/// # Returns
///
/// This function returns an `IResult` with the parsed non-whitespace string and
/// the remaining input. In case of parsing errors, it returns appropriate `nom` errors.
///
/// # Feature-Dependent Behavior
///
/// When compiled with the `trace` feature, this function extracts the fragment
/// from the `LocatedSpan` after parsing, providing additional context like line and
/// column information in case of errors. Without this feature, it behaves as a
/// standard `nom` parser function.
fn not_whitespace1(s: Span) -> IResult<Span, &str> {
  let p = take_while1(|c: char| !c.is_whitespace());
  cfg_if::cfg_if! {
    if #[cfg(feature = "trace")] {
      let (s, x) = p(s)?;
      Ok((s, x.fragment()))
    } else { p(s) }
  }
}

impl<'a, T: Float> Parser<'a, T> {
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
  ) -> Result<Parser<'a, f32>, nom::error::Error<String>> {
    cfg_if::cfg_if! {
        if #[cfg(feature = "trace")] {
            let info = TracableInfo::new().forward(false).backward(false);
            let input = LocatedSpan::new_extra(input, info);
        }
    }
    let (_, parsed) = Parser::<f32>::mps_file(input).map_err(|_| {
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
  pub fn mps_file(s: Span<'a>) -> IResult<Span<'a>, Parser<'a, f32>> {
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
        name,
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

  /// Parses the name section of an MPS file.
  ///
  /// This function extracts the name of the MPS problem defined in the MPS file. The name section
  /// is expected to start with the keyword "NAME", followed by the actual name of the problem.
  /// This parser function specifically looks for this pattern and extracts the problem name.
  ///
  /// # Arguments
  ///
  /// * `s`: A `Span` representing the part of the MPS file to be parsed. The `Span` can either be
  ///        a simple string slice or a `LocatedSpan` depending on the `trace` feature.
  ///
  /// # Returns
  ///
  /// Returns an `IResult<Span, &str>`:
  /// - On success: Contains the parsed problem name and the remaining unparsed input.
  /// - On failure: Contains a parsing error.
  ///
  /// # Errors
  ///
  /// Errors occur if the input does not start with the "NAME" keyword or if the problem name is not properly defined.
  ///
  /// # Features
  ///
  /// - Without `trace`: Basic parsing with standard error information.
  /// - With `trace`: Enhanced error reporting including detailed location tracking.
  ///
  /// # Example
  ///
  /// ```ignore
  /// use mps::Parser;
  /// let contents = "NAME          SAMPLE_PROBLEM\n...";
  /// match Parser::<f32>::name(contents) {
  ///     Ok((remaining, name)) => println!("Problem name: {}", name),
  ///     Err(e) => eprintln!("Error parsing name section: {}", e),
  /// }
  /// ```
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

  /// Parses a single row line from the ROWS section of an MPS file.
  ///
  /// This function is designed to extract individual row information from the MPS format.
  /// It looks for a row type indicator (one of "E", "L", "G", "N" for equality, less than or equal,
  /// greater than or equal, and non-standard types respectively), followed by the row name.
  ///
  /// # Arguments
  ///
  /// * `s`: A `Span` representing the part of the MPS file to be parsed. This can be a simple string slice
  ///        or a `LocatedSpan` if the `trace` feature is enabled.
  ///
  /// # Returns
  ///
  /// Returns an `IResult<Span, RowLine>`:
  /// - On success: Contains a `RowLine` struct representing the parsed row, along with the remaining input.
  /// - On failure: Contains a parsing error.
  ///
  /// # Errors
  ///
  /// Errors occur if the input does not conform to the expected format for a row line in MPS files.
  ///
  /// # Features
  ///
  /// - Without `trace`: Standard parsing with basic error information.
  /// - With `trace`: Enhanced error reporting including detailed location tracking.
  ///
  /// # Example
  ///
  /// ```ignore
  /// use mps::{Parser, RowLine};
  /// let row_data = " E  ROW_NAME\n";
  /// match Parser::<f32>::row_line(row_data) {
  ///     Ok((remaining, row_line)) => println!("Parsed row: {:?}", row_line),
  ///     Err(e) => eprintln!("Error parsing row line: {}", e),
  /// }
  /// ```
  ///
  /// The function uses `nom`'s combinators to parse and transform the input into a `RowLine` struct.
  /// It employs `map_res` for mapping the parsing result to a `RowLine` and handling any conversion errors.
  #[tracable_parser]
  pub fn row_line(s: Span) -> IResult<Span, RowLine> {
    let mut p = map_res(
      preceded(
        tag(" "),
        terminated(
          separated_pair(one_of("ELGN"), multispace1, not_whitespace1),
          newline,
        ),
      ),
      |(t, n)| -> Result<RowLine> {
        Ok(RowLine {
          row_type: RowType::try_from(t)?,
          row_name: n,
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

  /// Parses the ROWS section of an MPS file.
  ///
  /// This function is responsible for parsing multiple row lines from the ROWS section
  /// of the MPS format. It sequentially processes each row line, extracting the row type
  /// and row name, and collects them into a vector.
  ///
  /// # Arguments
  ///
  /// * `s`: A `Span` representing the part of the MPS file to be parsed. Depending on the
  ///        compilation feature, this can be either a simple string slice or a `LocatedSpan`
  ///        for enhanced error reporting.
  ///
  /// # Returns
  ///
  /// Returns an `IResult<Span, Vec<RowLine>>`:
  /// - On success: Contains a vector of `RowLine` structs representing the parsed rows, along
  ///   with the remaining unparsed input.
  /// - On failure: Contains a parsing error.
  ///
  /// # Errors
  ///
  /// Errors can occur if the input does not conform to the expected format for the ROWS section,
  /// or if individual row lines are malformed.
  ///
  /// # Features
  ///
  /// - Without `trace`: Performs standard parsing with basic error information.
  /// - With `trace`: Offers enhanced error reporting, including detailed location tracking.
  ///
  /// # Example
  ///
  /// ```ignore
  /// use mps::Parser;
  /// let rows_section = "ROWS\n E  ROW1\n G  ROW2\n N  ROW3\n";
  /// match Parser::<f32>::rows(rows_section) {
  ///     Ok((remaining, rows)) => println!("Parsed rows: {:?}", rows),
  ///     Err(e) => eprintln!("Error parsing ROWS section: {}", e),
  /// }
  /// ```
  ///
  /// The function employs `many0` to parse row lines and collects them into a vector.
  /// It uses `terminated` to delineate the end of the ROWS section, allowing for seamless transition
  /// to subsequent sections of the MPS file.
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

  /// Parses a single line in various sections (like COLUMNS, RHS) of an MPS file.
  ///
  /// This function is designed to extract data from lines that typically represent
  /// column entries or values associated with rows. It handles lines that may span
  /// multiple columns, capturing both the primary and optional secondary data.
  ///
  /// # Arguments
  ///
  /// * `s`: A `Span` representing the part of the MPS file to be parsed. The type of `Span` can
  ///        either be a simple string slice or a `LocatedSpan` if the `trace` feature is enabled.
  ///
  /// # Returns
  ///
  /// Returns an `IResult<Span, WideLine<f32>>`:
  /// - On success: Contains a `WideLine<f32>` struct representing the parsed line, along with
  ///   the remaining unparsed input.
  /// - On failure: Contains a parsing error.
  ///
  /// # Errors
  ///
  /// Errors occur if the input does not follow the expected format for a line in these sections.
  ///
  /// # Features
  ///
  /// - Without `trace`: Performs standard parsing with basic error information.
  /// - With `trace`: Enhances error reporting with detailed location tracking.
  ///
  /// # Example
  ///
  /// ```ignore
  /// use mps::{Parser, WideLine};
  /// let line_data = "    COL_NAME ROW_NAME 3.5\n";
  /// match Parser::<f32>::line(line_data) {
  ///     Ok((remaining, line)) => println!("Parsed line: {:?}", line),
  ///     Err(e) => eprintln!("Error parsing line: {}", e),
  /// }
  /// ```
  ///
  /// The function uses a combination of `nom`'s combinators like `map`, `preceded`, `terminated`,
  /// and `tuple` to parse the line and construct a `WideLine<f32>` structure. It's capable of
  /// handling optional secondary data pairs if present in the line.
  #[tracable_parser]
  pub fn line(s: Span) -> IResult<Span, WideLine<f32>> {
    let mut p = map_res(
      terminated(preceded(tag(" "), not_line_ending), newline),
      |line: Span| -> Result<WideLine<f32>> {
        let first_pair = RowValuePair {
          row_name: line.get(13..21).ok_or_eyre("incomplete")?,
          value: fast_float::parse(
            line.get(23..35).ok_or_eyre("incomplete")?.trim(),
          )?,
        };
        let second_pair = match line.get(38..46) {
          Some(row_name) => Some(RowValuePair {
            row_name,
            value: fast_float::parse(
              line.get(48..60).ok_or_eyre("incomplete")?.trim(),
            )?,
          }),
          None => None,
        };
        Ok(WideLine::<f32> {
          name: line.get(3..11).ok_or_eyre("incomplete")?,
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

  /// Parses a single line from the COLUMNS section of an MPS file.
  ///
  /// This function specializes in parsing lines within the COLUMNS section, which typically
  /// represent the column entries in the MPS format. It delegates the actual parsing task to
  /// the `line` function and is specifically intended for parsing within the COLUMNS context.
  ///
  /// # Arguments
  ///
  /// * `s`: A `Span` representing the part of the MPS file to be parsed. The type of `Span` can
  ///        vary based on the compilation feature; it can be a simple string slice or a
  ///        `LocatedSpan` for enhanced error reporting with the `trace` feature.
  ///
  /// # Returns
  ///
  /// Returns an `IResult<Span, WideLine<f32>>`:
  /// - On success: Contains a `WideLine<f32>` struct representing the parsed column line,
  ///   along with the remaining unparsed input.
  /// - On failure: Contains a parsing error.
  ///
  /// # Errors
  ///
  /// Errors occur if the input does not conform to the expected format for a column line in the MPS file.
  ///
  /// # Features
  ///
  /// - Without `trace`: Standard parsing with basic error information.
  /// - With `trace`: Enhanced error reporting, including detailed location tracking.
  ///
  /// # Example
  ///
  /// ```ignore
  /// use mps::{Parser, WideLine};
  /// let column_line_data = "    COLUMN_NAME ROW_NAME 4.2\n";
  /// match Parser::<f32>::columns_line(column_line_data) {
  ///     Ok((remaining, column_line)) => println!("Parsed column line: {:?}", column_line),
  ///     Err(e) => eprintln!("Error parsing column line: {}", e),
  /// }
  /// ```
  ///
  /// The function essentially wraps the `line` function, providing a context-specific interface
  /// for parsing lines within the COLUMNS section of an MPS file.
  #[tracable_parser]
  pub fn columns_line(s: Span) -> IResult<Span, WideLine<f32>> {
    let p = Self::line;
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  /// Parses the COLUMNS section of an MPS file.
  ///
  /// This function is responsible for parsing the COLUMNS section, which typically contains
  /// detailed information about each column of the MPS problem, including column names and
  /// associated values. It processes multiple lines, each representing a column entry.
  ///
  /// # Arguments
  ///
  /// * `s`: A `Span` representing the part of the MPS file to be parsed. Depending on the
  ///        compilation feature, this can be either a simple string slice or a `LocatedSpan`
  ///        for enhanced error reporting with the `trace` feature.
  ///
  /// # Returns
  ///
  /// Returns an `IResult<Span, Vec<WideLine<f32>>>`:
  /// - On success: Contains a vector of `WideLine<f32>` structs representing the parsed column
  ///   entries, along with the remaining unparsed input.
  /// - On failure: Contains a parsing error.
  ///
  /// # Errors
  ///
  /// Errors can occur if the input does not conform to the expected format for the COLUMNS section
  /// or if individual column lines are malformed.
  ///
  /// # Features
  ///
  /// - Without `trace`: Performs standard parsing with basic error information.
  /// - With `trace`: Provides enhanced error reporting, including detailed location tracking.
  ///
  /// # Example
  ///
  /// ```ignore
  /// use mps::Parser;
  /// let columns_section = "COLUMNS\n    COL1 ROW1 5.0\n    COL1 ROW2 2.5\n";
  /// match Parser::<f32>::columns(columns_section) {
  ///     Ok((remaining, columns)) => println!("Parsed columns: {:?}", columns),
  ///     Err(e) => eprintln!("Error parsing COLUMNS section: {}", e),
  /// }
  /// ```
  ///
  /// The function employs `many0` to parse column lines and collects them into a vector.
  /// It uses `terminated` and `preceded` combinators to delineate the start and end of the COLUMNS section.
  #[tracable_parser]
  pub fn columns(s: Span) -> IResult<Span, Vec<WideLine<f32>>> {
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

  /// Parses a single line from the RHS (Right-Hand Side) section of an MPS file.
  ///
  /// This function is specialized for parsing lines within the RHS section, which typically
  /// contain values associated with the right-hand side of constraints in the MPS format.
  /// It delegates the actual parsing task to the `line` function and is specifically intended
  /// for parsing within the RHS context.
  ///
  /// # Arguments
  ///
  /// * `s`: A `Span` representing the part of the MPS file to be parsed. The type of `Span` can
  ///        vary based on the compilation feature; it can be a simple string slice or a
  ///        `LocatedSpan` for enhanced error reporting with the `trace` feature.
  ///
  /// # Returns
  ///
  /// Returns an `IResult<Span, WideLine<f32>>`:
  /// - On success: Contains a `WideLine<f32>` struct representing the parsed RHS line,
  ///   along with the remaining unparsed input.
  /// - On failure: Contains a parsing error.
  ///
  /// # Errors
  ///
  /// Errors occur if the input does not follow the expected format for an RHS line in the MPS file.
  ///
  /// # Features
  ///
  /// - Without `trace`: Standard parsing with basic error information.
  /// - With `trace`: Enhanced error reporting, including detailed location tracking.
  ///
  /// # Example
  ///
  /// ```ignore
  /// use mps::{Parser, WideLine};
  /// let rhs_line_data = "    RHS_NAME ROW_NAME 7.5\n";
  /// match Parser::<f32>::rhs_line(rhs_line_data) {
  ///     Ok((remaining, rhs_line)) => println!("Parsed RHS line: {:?}", rhs_line),
  ///     Err(e) => eprintln!("Error parsing RHS line: {}", e),
  /// }
  /// ```
  ///
  /// The function essentially wraps the `line` function, providing a context-specific interface
  /// for parsing lines within the RHS section of an MPS file.
  #[tracable_parser]
  pub fn rhs_line(s: Span) -> IResult<Span, WideLine<f32>> {
    let p = Self::line;
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  /// Parses the RHS (Right-Hand Side) section of an MPS file.
  ///
  /// This function is responsible for parsing the RHS section, which typically contains
  /// values associated with the right-hand side of constraints in the MPS format.
  /// It processes multiple lines, each representing an RHS entry, and collects them into a vector.
  ///
  /// # Arguments
  ///
  /// * `s`: A `Span` representing the part of the MPS file to be parsed. Depending on the
  ///        compilation feature, this can be either a simple string slice or a `LocatedSpan`
  ///        for enhanced error reporting with the `trace` feature.
  ///
  /// # Returns
  ///
  /// Returns an `IResult<Span, Vec<WideLine<f32>>>`:
  /// - On success: Contains a vector of `WideLine<f32>` structs representing the parsed RHS
  ///   entries, along with the remaining unparsed input.
  /// - On failure: Contains a parsing error.
  ///
  /// # Errors
  ///
  /// Errors can occur if the input does not conform to the expected format for the RHS section
  /// or if individual RHS lines are malformed.
  ///
  /// # Features
  ///
  /// - Without `trace`: Performs standard parsing with basic error information.
  /// - With `trace`: Provides enhanced error reporting, including detailed location tracking.
  ///
  /// # Example
  ///
  /// ```ignore
  /// use mps::Parser;
  /// let rhs_section = "RHS\n    RHS_NAME ROW1 10.0\n    RHS_NAME ROW2 -5.5\n";
  /// match Parser::<f32>::rhs(rhs_section) {
  ///     Ok((remaining, rhs)) => println!("Parsed RHS: {:?}", rhs),
  ///     Err(e) => eprintln!("Error parsing RHS section: {}", e),
  /// }
  /// ```
  ///
  /// The function employs `many0` to parse RHS lines and collects them into a vector.
  /// It uses `terminated` and `preceded` combinators to delineate the start and end of the RHS section.
  #[tracable_parser]
  pub fn rhs(s: Span) -> IResult<Span, Vec<WideLine<f32>>> {
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

  /// Parses a single line from the RANGES section of an MPS file.
  ///
  /// This function is designed to parse lines within the RANGES section, which specify
  /// additional constraints for rows in the MPS format. It reuses the `line` function
  /// for actual parsing, tailoring it for the RANGES section context.
  ///
  /// # Arguments
  ///
  /// * `s`: A `Span` representing the part of the MPS file to be parsed. The type of `Span`
  ///        can vary based on the compilation feature; it can be a simple string slice or a
  ///        `LocatedSpan` for enhanced error reporting with the `trace` feature.
  ///
  /// # Returns
  ///
  /// Returns an `IResult<Span, WideLine<f32>>`:
  /// - On success: Contains a `WideLine<f32>` struct representing the parsed range line,
  ///   along with the remaining unparsed input.
  /// - On failure: Contains a parsing error.
  ///
  /// # Errors
  ///
  /// Errors occur if the input does not follow the expected format for a range line in the MPS file.
  ///
  /// # Features
  ///
  /// - Without `trace`: Standard parsing with basic error information.
  /// - With `trace`: Enhanced error reporting, including detailed location tracking.
  ///
  /// # Example
  ///
  /// ```ignore
  /// use mps::{Parser, WideLine};
  /// let range_line_data = "    RANGE_NAME ROW_NAME 15.0\n";
  /// match Parser::<f32>::ranges_line(range_line_data) {
  ///     Ok((remaining, range_line)) => println!("Parsed range line: {:?}", range_line),
  ///     Err(e) => eprintln!("Error parsing range line: {}", e),
  /// }
  /// ```
  ///
  /// This function wraps the `line` function to provide a context-specific interface for parsing
  /// lines within the RANGES section of an MPS file.
  #[tracable_parser]
  pub fn ranges_line(s: Span) -> IResult<Span, WideLine<f32>> {
    let p = Self::line;
    cfg_if::cfg_if! {
      if #[cfg(feature = "trace")] {
        let (s, x) = p(s)?;
        Ok((s, x))
      } else { p(s) }
    }
  }

  /// Parses the RANGES section of an MPS file.
  ///
  /// This function is dedicated to parsing the entire RANGES section of an MPS format file.
  /// It repeatedly applies `ranges_line` to parse individual lines, thereby collecting all the range definitions
  /// within this section.
  ///
  /// # Arguments
  ///
  /// * `s`: A `Span` that represents the part of the MPS file being parsed.
  ///
  /// # Returns
  ///
  /// Returns an `IResult<Span, Vec<WideLine<f32>>>`:
  /// - On success: Provides a vector of `WideLine<f32>` structs, each representing a parsed range line,
  ///   along with the remaining unparsed input.
  /// - On failure: Results in a parsing error.
  ///
  /// # Errors
  ///
  /// This function may return errors if the input does not match the expected format of the RANGES section in the MPS file.
  ///
  /// # Features
  ///
  /// - `trace` disabled: Standard parsing with basic error information.
  /// - `trace` enabled: Enhanced error reporting, including detailed location tracking.
  ///
  /// # Example
  ///
  /// ```ignore
  /// use mps::{Parser, WideLine};
  /// let mps_data = "..."; // A string slice of the MPS data including the RANGES section
  /// match Parser::<f32>::ranges(mps_data) {
  ///     Ok((remaining, ranges)) => println!("Parsed ranges: {:?}", ranges),
  ///     Err(e) => eprintln!("Error parsing ranges: {}", e),
  /// }
  /// ```
  ///
  /// This function uses `ranges_line` in a loop to parse and accumulate all range lines within the RANGES section.
  #[tracable_parser]
  pub fn ranges(s: Span) -> IResult<Span, Vec<WideLine<f32>>> {
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

  /// Parses a bound type identifier from a string in an MPS file.
  ///
  /// This function is responsible for parsing a bound type identifier (like "LO", "UP", etc.) from a given `Span`.
  /// The bound type identifiers correspond to different types of bounds that can be applied to variables in linear
  /// programming models defined in MPS format.
  ///
  /// # Arguments
  ///
  /// * `s`: A `Span` representing the part of the MPS file being parsed.
  ///
  /// # Returns
  ///
  /// Returns an `IResult<Span, BoundType>`:
  /// - On success: Contains a `BoundType` enum value representing the parsed bound type, along with the remaining unparsed input.
  /// - On failure: Contains a parsing error.
  ///
  /// # Errors
  ///
  /// Errors occur if the input does not match any of the known bound type identifiers.
  ///
  /// # Features
  ///
  /// - Without `trace`: Performs standard parsing.
  /// - With `trace`: Enhanced parsing with detailed error reporting, leveraging `LocatedSpan`.
  ///
  /// # Example
  ///
  /// ```ignore
  /// use mps::{Parser, BoundType};
  /// let bound_type_data = "LO";
  /// match Parser::<f32>::bound_type(bound_type_data) {
  ///     Ok((remaining, bound_type)) => println!("Parsed bound type: {:?}", bound_type),
  ///     Err(e) => eprintln!("Error parsing bound type: {}", e),
  /// }
  /// ```
  ///
  /// This function identifies and parses bound type identifiers, crucial for interpreting constraints in MPS files.
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

  /// Parses a single line from the BOUNDS section of an MPS file.
  ///
  /// This function interprets a line within the BOUNDS section of an MPS format file. Each line in this section
  /// defines a bound for a variable in the linear programming model. The parsing process extracts the bound type,
  /// bound name, column (variable) name, and the bound value.
  ///
  /// # Arguments
  ///
  /// * `s`: A `Span` representing the part of the MPS file to be parsed. This should be a single line from the BOUNDS section.
  ///
  /// # Returns
  ///
  /// Returns an `IResult<Span, BoundsLine<f32>>`:
  /// - On success: Contains a `BoundsLine<f32>` struct representing the parsed bounds line, along with the remaining unparsed input.
  /// - On failure: Contains a parsing error.
  ///
  /// # Errors
  ///
  /// Errors occur if the input does not follow the expected format for a bounds line in the MPS file.
  ///
  /// # Features
  ///
  /// - Without `trace`: Standard parsing with basic error information.
  /// - With `trace`: Enhanced error reporting, including detailed location tracking.
  ///
  /// # Example
  ///
  /// ```ignore
  /// use mps::{Parser, BoundsLine};
  /// let bounds_line_data = " LO BOUND_NAME COLUMN_NAME 10.0\n";
  /// match Parser::<f32>::bounds_line(bounds_line_data) {
  ///     Ok((remaining, bounds_line)) => println!("Parsed bounds line: {:?}", bounds_line),
  ///     Err(e) => eprintln!("Error parsing bounds line: {}", e),
  /// }
  /// ```
  ///
  /// This function provides a specialized method for parsing lines in the BOUNDS section of an MPS file.
  #[tracable_parser]
  pub fn bounds_line(s: Span) -> IResult<Span, BoundsLine<f32>> {
    let mut p = map_res(
      preceded(
        tag(" "),
        terminated(
          tuple((
            terminated(
              Self::bound_type,
              multispace1,
            ),
            terminated(not_whitespace1, multispace1),
            terminated(not_whitespace1, multispace1),
            float,
          )),
          newline,
        ),
      ),
      |(bound_type, bound_name, column_name, value)| -> Result<BoundsLine<f32>> {
        Ok(BoundsLine {
          bound_type,
          bound_name,
          column_name,
          value,
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

  /// Parses the BOUNDS section of an MPS file.
  ///
  /// This function is responsible for parsing the entire BOUNDS section in an MPS format file. It utilizes `bounds_line`
  /// to parse each individual line within this section. The BOUNDS section defines the bounds for variables in the
  /// linear programming model, and this function aggregates all such definitions.
  ///
  /// # Arguments
  ///
  /// * `s`: A `Span` representing the part of the MPS file being parsed. This should include the entire BOUNDS section.
  ///
  /// # Returns
  ///
  /// Returns an `IResult<Span, Vec<BoundsLine<f32>>>`:
  /// - On success: Contains a vector of `BoundsLine<f32>` structs, each representing a parsed bounds line,
  ///   along with the remaining unparsed input.
  /// - On failure: Contains a parsing error.
  ///
  /// # Errors
  ///
  /// Errors can occur if the input does not conform to the expected format of the BOUNDS section in the MPS file.
  ///
  /// # Features
  ///
  /// - Without `trace`: Performs standard parsing with basic error information.
  /// - With `trace`: Offers enhanced error reporting, including detailed location tracking.
  ///
  /// # Example
  ///
  /// ```ignore
  /// use mps::{Parser, BoundsLine};
  /// let mps_data = "..."; // A string slice of the MPS data including the BOUNDS section
  /// match Parser::<f32>::bounds(mps_data) {
  ///     Ok((remaining, bounds)) => println!("Parsed bounds: {:?}", bounds),
  ///     Err(e) => eprintln!("Error parsing bounds: {}", e),
  /// }
  /// ```
  ///
  /// This function sequentially parses each line in the BOUNDS section to build a comprehensive list of bounds for the model.
  #[tracable_parser]
  pub fn bounds(s: Span) -> IResult<Span, Vec<BoundsLine<f32>>> {
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

  /// Parses the ENDATA marker in an MPS file.
  ///
  /// This function is used to identify and parse the "ENDATA" marker in an MPS format file. The "ENDATA" marker
  /// signifies the end of the data section in an MPS file. This is crucial for parsers to recognize the completion
  /// of the file parsing process.
  ///
  /// # Arguments
  ///
  /// * `s`: A `Span` representing the part of the MPS file being parsed, expected to contain the "ENDATA" marker.
  ///
  /// # Returns
  ///
  /// Returns an `IResult<Span, &str>`:
  /// - On success: Contains the matched "ENDATA" string, along with the remaining unparsed input.
  /// - On failure: Contains a parsing error.
  ///
  /// # Errors
  ///
  /// Errors occur if the "ENDATA" marker is not found where expected.
  ///
  /// # Features
  ///
  /// - Without `trace`: Standard parsing.
  /// - With `trace`: Enhanced parsing with detailed error reporting.
  ///
  /// # Example
  ///
  /// ```ignore
  /// use mps::Parser;
  /// let mps_data = "ENDATA";
  /// match Parser::<f32>::endata(mps_data) {
  ///     Ok((remaining, endata_marker)) => println!("Found ENDATA marker: {:?}", endata_marker),
  ///     Err(e) => eprintln!("Error: ENDATA marker not found"),
  /// }
  /// ```
  ///
  /// This function specifically targets the "ENDATA" marker, indicating the end of an MPS file's data section.
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
