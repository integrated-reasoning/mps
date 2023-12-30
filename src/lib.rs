//! # mps
//!
//! `mps` is a parser for the Mathematical Programming System (MPS) file format,
//! commonly used to represent optimization problems.
//!
//! ## Examples
//!
//! **Library**
//!
//! ```
//! use mps::Parser;
//!
//! let data = "
//! NAME example
//! ROWS
//!  N  OBJ
//!  L  R1
//!  L  R2
//!  E  R3
//! COLUMNS
//!     X1        OBJ       -6
//!     X1        R1        2
//!     X1        R2        1
//!     X1        R3        3
//!     X2        OBJ       7
//!     X2        R1        5
//!     X2        R2        -1
//!     X2        R3        2
//!     X3        OBJ       4
//!     X3        R1        -1
//!     X3        R2        -2
//!     X3        R3        2
//! RHS
//!     RHS1      R1        18
//!     RHS1      R2        -14
//!     RHS1      R3        26
//! BOUNDS
//!  LO BND1      X1        0
//!  LO BND1      X2        0
//!  LO BND1      X3        0
//! ENDATA";
//!
//! cfg_if::cfg_if! {
//!   if #[cfg(feature = "located")] {
//!     use nom_locate::LocatedSpan;
//!     use nom_tracable::TracableInfo;
//!     let info = TracableInfo::new().forward(true).backward(true);
//!     Parser::<f32>::parse(LocatedSpan::new_extra(data, info));
//!   } else {
//!     Parser::<f32>::parse(data);
//!   }
//! }
//! ```
//!
//! **CLI**
//!
//! ```bash
//! $ mps --input-path ./data/netlib/afiro
//! ```
//!
//! This crate provides both a library and a CLI for parsing MPS data. Key features include:
//!
//! - **Configurable Parsing**:
//!   - Supported feature flags:
//!     - `cli` - Command line interface.
//!     - `proptest` - Property testing integrations.
//!     - `trace` - Enhanced debugging and statistics via `nom_tracable` and `nom_locate`.
//! - **Robustness**: Extensively tested against [Netlib LP test suite](http://www.netlib.org/lp/data/).
//! - **Performance**: Benchmarked using [Criterion.rs](https://github.com/bheisler/criterion.rs).
//!
//! ## References
//!
//! - [Mathematical Programming System format](https://lpsolve.sourceforge.net/5.5/mps-format.htm)
//! - [NETLIB linear programming library](http://www.netlib.org/lp/)
//!
use color_eyre::{eyre::eyre, Result};
use nom::{
  branch::alt,
  bytes::complete::{tag, take_while1},
  character::complete::*,
  combinator::{map, map_res, opt, peek},
  multi::{count, many1},
  number::complete::float,
  sequence::{preceded, separated_pair, terminated, tuple},
  IResult,
};
use nom_tracable::tracable_parser;
use num_traits::float::Float;

cfg_if::cfg_if! {
  if #[cfg(feature = "located")] {
    use nom_locate::LocatedSpan;
    use nom_tracable::TracableInfo;

    /// Conditionally compiles different `Span` types based on the crate's feature flags.
    ///
    /// When the `located` feature is enabled, it utilizes `nom_locate::LocatedSpan` and `nom_tracable::TracableInfo`
    /// to provide enhanced error reporting and debugging capabilities. This allows the parser to track the location
    /// of tokens within the input, making it easier to diagnose parsing errors and understand the parsing flow.
    ///
    /// Without the `located` feature, a simpler `Span` type is used, which is a reference to a slice of the input string.
    /// This simpler span is more performant but lacks the detailed tracking and error reporting capabilities.

    /// A `Span` type that includes location and tracing information.
    /// Used when the `located` feature is enabled.
    pub type Span<'a> = LocatedSpan<&'a str, TracableInfo>;
  } else {
    /// A simple `Span` type representing a reference to a slice of the input string.
    /// Used when the `located` feature is not enabled.
    pub type Span<'a> = &'a str;
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
/// When compiled with the `located` feature, this function extracts the fragment
/// from the `LocatedSpan` after parsing, providing additional context like line and
/// column information in case of errors. Without this feature, it behaves as a
/// standard `nom` parser function.
fn not_whitespace1(s: Span) -> IResult<Span, &str> {
  let p = take_while1(|c: char| !c.is_whitespace());
  cfg_if::cfg_if! {
    if #[cfg(feature = "located")] {
      let (s, x) = p(s)?;
      Ok((s, x.fragment()))
    } else { p(s) }
  }
}

/// The primary structure for parsing MPS (Mathematical Programming System) data.
///
/// `Parser` is responsible for holding the structured representation of the parsed MPS file.
/// It encapsulates all the main components of an MPS file, including rows, columns, and various
/// optional sections like right-hand side (RHS) values, ranges, and bounds.
///
/// # Type Parameters
///
/// * `'a`: Lifetime parameter, indicating that the fields in `Parser` hold references to the string data.
/// * `T`: A type parameter bounded by the `Float` trait, representing the numeric type used for values in the MPS data.
///
/// # Fields
///
/// * `name`: A string slice holding the name of the MPS problem.
/// * `rows`: A vector of `RowLine` instances, representing the rows defined in the MPS file.
/// * `columns`: A vector of `WideLine` instances, representing the columns and their associated data.
/// * `rhs`: An optional vector of `WideLine` instances, representing the right-hand side values for constraints.
/// * `ranges`: An optional vector representing the ranges associated with constraints.
/// * `bounds`: An optional vector representing the bounds on the variables.
///
/// Each of these fields corresponds to a specific section of the MPS format, allowing for a comprehensive
/// representation of the MPS file structure.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Parser<'a, T: Float> {
  pub name: &'a str,
  pub rows: Rows<'a>,
  pub columns: Columns<'a, T>,
  pub rhs: Option<Rhs<'a, T>>,
  pub ranges: Option<Ranges<'a, T>>,
  pub bounds: Option<Bounds<'a, T>>,
}

/// Represents a single row in an MPS (Mathematical Programming System) file.
///
/// This struct is a key component in representing the structure of an MPS file,
/// capturing the details of a single row as defined in the ROWS section of the file.
///
/// # Fields
///
/// * `row_type`: An enumeration of type `RowType` indicating the nature of the row
///   (equality, inequality, etc.).
/// * `row_name`: A string slice referring to the name of the row.
///
/// The combination of `row_type` and `row_name` allows for precise definition and
/// identification of constraints within linear programming models.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct RowLine<'a> {
  pub row_type: RowType,
  pub row_name: &'a str,
}

/// Enumeration representing the type of a row in an MPS file.
///
/// This enum categorizes rows into different types, each corresponding to a specific
/// kind of constraint or equation in linear programming models.
///
/// # Variants
///
/// * `Eq`: Represents an equality constraint (`E` in MPS format).
/// * `Leq`: Represents a less than or equal to constraint (`L` in MPS format).
/// * `Geq`: Represents a greater than or equal to constraint (`G` in MPS format).
/// * `Nr`: Represents a special type or non-standard row (`N` in MPS format).
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum RowType {
  #[default]
  Eq,
  Leq,
  Geq,
  Nr,
}

impl TryFrom<char> for RowType {
  type Error = color_eyre::Report;

  /// Attempts to convert a single character into a `RowType`.
  ///
  /// This function is essential for parsing the row type indicators in MPS files.
  /// It maps specific characters to their corresponding `RowType` variants.
  ///
  /// # Arguments
  ///
  /// * `c`: A character representing the row type in the MPS file format.
  ///
  /// # Errors
  ///
  /// Returns an error if the character does not correspond to a valid `RowType`.
  fn try_from(c: char) -> Result<Self> {
    match c {
      'E' => Ok(RowType::Eq),
      'L' => Ok(RowType::Leq),
      'G' => Ok(RowType::Geq),
      'N' => Ok(RowType::Nr),
      _ => Err(eyre!("invalid row type")),
    }
  }
}

/// Type alias for a collection of `RowLine` instances.
///
/// This type represents the ROWS section of an MPS file, containing all the row
/// definitions within the problem.
pub type Rows<'a> = Vec<RowLine<'a>>;

/// Type alias for a collection of `WideLine` instances.
///
/// This type is used to represent collections of columns (and associated data),
/// RHS values, ranges, or bounds, each represented as a `WideLine`.
pub type Columns<'a, T> = Vec<WideLine<'a, T>>;

/// Represents a pairing of a row name with a numeric value.
///
/// This struct is utilized in `WideLine` to represent data associated with rows
/// in various sections of an MPS file (e.g., COLUMNS, RHS).
///
/// # Type Parameters
///
/// * `T`: Numeric type for the value associated with the row.
///
/// # Fields
///
/// * `row_name`: A string slice referring to the name of the row.
/// * `value`: A numeric value associated with the row.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct RowValuePair<'a, T> {
  pub row_name: &'a str,
  pub value: T,
}

/// Represents a line in an MPS file that can span across multiple columns.
///
/// This struct is a key component for representing data in sections like COLUMNS,
/// RHS, RANGES, and BOUNDS. Each `WideLine` can hold up to two `RowValuePair`
/// instances, allowing it to represent complex data structures in the MPS format.
///
/// # Type Parameters
///
/// * `T`: Numeric type for the values associated with the rows.
///
/// # Fields
///
/// * `name`: Name of the column or the identifier for the data line.
/// * `first_pair`: The first `RowValuePair` representing the primary data.
/// * `second_pair`: An optional second `RowValuePair`, used when the line spans multiple rows.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct WideLine<'a, T> {
  pub name: &'a str,
  pub first_pair: RowValuePair<'a, T>,
  pub second_pair: Option<RowValuePair<'a, T>>,
}

/// Type alias for a collection of `WideLine` instances representing the RHS (Right-Hand Side) values.
///
/// This type is specifically used to represent the RHS section of an MPS file,
/// where each `WideLine` contains data associated with the RHS values of constraints.
pub type Rhs<'a, T> = Vec<WideLine<'a, T>>;

/// Type alias for a collection of `WideLine` instances representing ranges.
///
/// In the context of MPS files, ranges define additional constraints for rows.
/// Each `WideLine` in this collection represents a range associated with a row.
pub type Ranges<'a, T> = Vec<WideLine<'a, T>>;

/// Represents a single line in the BOUNDS section of an MPS file.
///
/// This struct captures the details of a bound, which can be applied to variables
/// in linear programming models. Each bound has a type, a name, a column (variable) it
/// applies to, and a numeric value.
///
/// # Type Parameters
///
/// * `T`: Numeric type for the value of the bound.
///
/// # Fields
///
/// * `bound_type`: The type of the bound (e.g., upper, lower, fixed).
/// * `bound_name`: A string slice representing the name of the bound.
/// * `column_name`: A string slice representing the name of the column to which the bound applies.
/// * `value`: The numeric value of the bound.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct BoundsLine<'a, T> {
  pub bound_type: BoundType,
  pub bound_name: &'a str,
  pub column_name: &'a str,
  pub value: T,
}

/// Type alias for a collection of `BoundsLine` instances.
///
/// Represents the BOUNDS section of an MPS file, where each `BoundsLine`
/// defines a bound for a particular variable (column) in the linear programming model.
pub type Bounds<'a, T> = Vec<BoundsLine<'a, T>>;

/// Enumeration representing types of bounds in an MPS (Mathematical Programming System) file.
///
/// Each variant corresponds to a specific type of bound that can be applied to a variable
/// in linear programming models. These bounds define the permissible range of values for
/// variables within the constraints of the model.
///
/// # Variants
///
/// * `Lo`: Lower Bound (denoted as `l_j <= x_j <= inf` in MPS format).
///   Indicates that the variable should be greater than or equal to a specified lower bound.
///
/// * `Up`: Upper Bound (denoted as `0 <= x_j <= u_j` in MPS format).
///   Specifies that the variable should be less than or equal to a given upper bound.
///
/// * `Fx`: Fixed Variable (denoted as `l_j == x_j == u_j` in MPS format).
///   Implies that the variable is fixed to a certain value.
///
/// * `Fr`: Free Variable (denoted as `-inf <= x_j <= inf` in MPS format).
///   Represents a variable without any bounds.
///
/// * `Mi`: Unbounded Below (denoted as `-inf <= x_j <= 0` in MPS format).
///   Indicates that the variable has no lower bound but is bounded above by zero.
///
/// * `Pl`: Unbounded Above (denoted as `0 <= x_j <= inf` in MPS format).
///   Specifies that the variable has no upper bound but is bounded below by zero.
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum BoundType {
  #[default]
  Lo, // lower bound     :  l_j <= x_j <= inf
  Up, // upper bound     :    0 <= x_j <= u_j
  Fx, // fixed variable  :  l_j == x_j == u_j
  Fr, // free variable   : -inf <= x_j <= inf
  Mi, // Unbounded below : -inf <= x_j <= 0
  Pl, // Unbounded above :    0 <= x_j <= inf
}

impl TryFrom<&str> for BoundType {
  type Error = color_eyre::Report;

  /// Attempts to convert a string slice into a `BoundType`.
  ///
  /// This function is crucial for parsing the BOUNDS section of MPS files.
  /// It maps specific string representations to their corresponding `BoundType` variants.
  ///
  /// # Arguments
  ///
  /// * `s`: A string slice representing the bound type in MPS file format.
  ///
  /// # Returns
  ///
  /// Returns `Ok(BoundType)` variant corresponding to the input string.
  /// In cases where the string does not match any known bound types or is not yet implemented,
  /// it returns an error.
  ///
  /// # Errors
  ///
  /// Returns an error if the input string does not correspond to a valid `BoundType`,
  /// or if the bound type is recognized but not yet implemented in this crate.
  ///
  /// # Supported Bound Types
  ///
  /// * `"LO"` - Lower Bound
  /// * `"UP"` - Upper Bound
  /// * `"FX"` - Fixed Variable
  /// * `"FR"` - Free Variable
  /// * `"MI"` - Unbounded Below
  /// * `"PL"` - Unbounded Above
  ///
  /// # Not Yet Implemented
  ///
  /// * `"BV"` - Binary Variable
  /// * `"LI"` - Lower Integer Bound
  /// * `"UI"` - Upper Integer Bound
  /// * `"SC"` - Semi-Continuous Variable
  fn try_from(s: &str) -> Result<Self> {
    match s {
      "LO" => Ok(BoundType::Lo),
      "UP" => Ok(BoundType::Up),
      "FX" => Ok(BoundType::Fx),
      "FR" => Ok(BoundType::Fr),
      "MI" => Ok(BoundType::Mi),
      "PL" => Ok(BoundType::Pl),
      "BV" => unimplemented!("Binary Variable not yet implemented"),
      "LI" => unimplemented!("Lower Integer Bound not yet implemented"),
      "UI" => unimplemented!("Upper Integer Bound not yet implemented"),
      "SC" => unimplemented!("Semi-Continuous Variable not yet implemented"),
      _ => Err(eyre!("invalid bound type")),
    }
  }
}

/// Enumeration representing range types in an MPS (Mathematical Programming System) file.
///
/// These types correspond to different rules for applying ranges to rows in the RANGES section
/// of an MPS file. Each variant represents a specific calculation for the lower (L_i) and upper (U_i)
/// limits of a row's range, based on its type and the sign of R_i (the range value).
///
/// The behavior of each range type is derived from the U_i L_i Limit Table, as follows:
///
/// | Range Type | Row Type | Sign of R_i | Lower Limit L_i | Upper Limit U_i |
/// |------------|----------|-------------|-----------------|-----------------|
/// | `_Le`      | LE (<=)  | + or -      | b_i - \|R_i\|   | b_i             |
/// | `_Ge`      | GE (>=)  | + or -      | b_i             | b_i + \|R_i\|   |
/// | `_Ep`      | EP (==)  | +           | b_i             | b_i + \|R_i\|   |
/// | `_Em`      | EM (==)  | -           | b_i - \|R_i\|   | b_i             |
/// | `_Ez`      | EZ (==)  | -           | b_i             | b_i             |
///
/// Note: In cases where R_i is zero, both L_i and U_i are set to the respective Rhs value b_i, as per Maros CTSM p.91.
///
/// Reference: Maros, I. Computational Techniques of the Simplex Method (CTSM).
#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub enum RangeType {
  #[default]
  _Le, // Less than or Equal
  _Ge, // Greater than or Equal
  _Ep, // Equality with positive R_i
  _Em, // Equality with negative R_i
  _Ez, // Equality with unspecified R_i
}

impl<'a, T: Float> Parser<'a, T> {
  /// Parses an MPS (Mathematical Programming System) formatted string into a `Parser` instance.
  ///
  /// This method is the primary interface for converting MPS formatted data into a structured format.
  /// It sequentially processes different sections of the MPS file (name, rows, columns, rhs, ranges, bounds),
  /// integrating them into a single `Parser` object.
  ///
  /// # Arguments
  ///
  /// * `s`: A `Span` representing the input MPS data. The type of `Span` depends on the compilation feature.
  ///        With the `located` feature, it includes additional context for precise error reporting.
  ///
  /// # Returns
  ///
  /// Returns an `IResult<Span, Parser<f32>>`:
  /// - On success: Contains the parsed `Parser` instance and the remaining unparsed input.
  /// - On failure: Contains a parsing error, with detailed information if `located` is enabled.
  ///
  /// # Errors
  ///
  /// Errors occur if the input does not conform to the expected MPS format or if issues arise in any parsing stages.
  ///
  /// # Features
  ///
  /// - Without `located`: Standard parsing with basic error information.
  /// - With `located`: Enhanced error reporting including detailed location tracking.
  ///
  /// # Examples
  ///
  /// Basic usage without tracing:
  /// ```ignore
  /// use mps::Parser;
  /// let contents = "MPS data here...";
  /// match Parser::<f32>::parse(&contents) {
  ///     Ok((_, parsed)) => println!("{:#?}", parsed),
  ///     Err(e) => eprintln!("Error parsing MPS file: {}", e),
  /// }
  /// ```
  ///
  /// Usage with tracing enabled (`located` feature):
  /// ```ignore
  /// use mps::Parser;
  /// use nom_locate::LocatedSpan;
  /// use nom_tracable::{TracableInfo, cumulative_histogram};
  /// let contents = "MPS data with tracing...";
  /// let info = TracableInfo::new().forward(true).backward(true);
  /// match Parser::<f32>::parse(LocatedSpan::new_extra(&contents, info)) {
  ///     Ok((_, parsed)) => println!("{:#?}", parsed),
  ///     Err(e) => eprintln!("Error parsing MPS file with tracing: {}", e),
  /// }
  /// cumulative_histogram();
  /// ```
  ///
  /// The `cumulative_histogram` function from `nom_tracable` can be used to obtain cumulative
  /// parser invocation statistics, providing insights into the parsing process.
  ///
  #[tracable_parser]
  pub fn parse(s: Span<'a>) -> IResult<Span<'a>, Parser<'a, f32>> {
    let mut p = map(
      tuple((
        Self::name,
        Self::rows,
        Self::columns,
        opt(Self::rhs),
        opt(Self::ranges),
        opt(Self::bounds),
        opt(Self::endata),
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
        if #[cfg(feature = "located")] {
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
  ///        a simple string slice or a `LocatedSpan` depending on the `located` feature.
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
  /// - Without `located`: Basic parsing with standard error information.
  /// - With `located`: Enhanced error reporting including detailed location tracking.
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
      if #[cfg(feature = "located")] {
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
  ///        or a `LocatedSpan` if the `located` feature is enabled.
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
  /// - Without `located`: Standard parsing with basic error information.
  /// - With `located`: Enhanced error reporting including detailed location tracking.
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
      if #[cfg(feature = "located")] {
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
  /// - Without `located`: Performs standard parsing with basic error information.
  /// - With `located`: Offers enhanced error reporting, including detailed location tracking.
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
  /// The function employs `many1` to parse one or more row lines and collects them into a vector.
  /// It uses `terminated` to delineate the end of the ROWS section, allowing for seamless transition
  /// to subsequent sections of the MPS file.
  #[tracable_parser]
  pub fn rows(s: Span) -> IResult<Span, Vec<RowLine>> {
    let mut p = terminated(
      preceded(terminated(tag("ROWS"), newline), many1(Self::row_line)),
      peek(anychar),
    );
    cfg_if::cfg_if! {
      if #[cfg(feature = "located")] {
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
  ///        either be a simple string slice or a `LocatedSpan` if the `located` feature is enabled.
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
  /// - Without `located`: Performs standard parsing with basic error information.
  /// - With `located`: Enhances error reporting with detailed location tracking.
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
    let mut p = map(
      preceded(
        multispace1,
        terminated(
          tuple((
            terminated(not_whitespace1, multispace1),
            terminated(not_whitespace1, multispace1),
            float,
            opt(preceded(
              multispace1,
              tuple((terminated(not_whitespace1, multispace1), float)),
            )),
          )),
          newline,
        ),
      ),
      |(column_name, row_name, value, opt)| WideLine::<f32> {
        name: column_name,
        first_pair: RowValuePair { row_name, value },
        second_pair: opt.map(|(opt_row_name, opt_value)| RowValuePair {
          row_name: opt_row_name,
          value: opt_value,
        }),
      },
    );
    cfg_if::cfg_if! {
      if #[cfg(feature = "located")] {
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
  ///        `LocatedSpan` for enhanced error reporting with the `located` feature.
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
  /// - Without `located`: Standard parsing with basic error information.
  /// - With `located`: Enhanced error reporting, including detailed location tracking.
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
      if #[cfg(feature = "located")] {
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
  ///        for enhanced error reporting with the `located` feature.
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
  /// - Without `located`: Performs standard parsing with basic error information.
  /// - With `located`: Provides enhanced error reporting, including detailed location tracking.
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
  /// The function employs `many1` to parse one or more column lines and collects them into a vector.
  /// It uses `terminated` and `preceded` combinators to delineate the start and end of the COLUMNS section.
  #[tracable_parser]
  pub fn columns(s: Span) -> IResult<Span, Vec<WideLine<f32>>> {
    let mut p = terminated(
      preceded(
        terminated(tag("COLUMNS"), newline),
        many1(Self::columns_line),
      ),
      peek(anychar),
    );
    cfg_if::cfg_if! {
      if #[cfg(feature = "located")] {
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
  ///        `LocatedSpan` for enhanced error reporting with the `located` feature.
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
  /// - Without `located`: Standard parsing with basic error information.
  /// - With `located`: Enhanced error reporting, including detailed location tracking.
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
      if #[cfg(feature = "located")] {
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
  ///        for enhanced error reporting with the `located` feature.
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
  /// - Without `located`: Performs standard parsing with basic error information.
  /// - With `located`: Provides enhanced error reporting, including detailed location tracking.
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
  /// The function employs `many1` to parse one or more RHS lines and collects them into a vector.
  /// It uses `terminated` and `preceded` combinators to delineate the start and end of the RHS section.
  #[tracable_parser]
  pub fn rhs(s: Span) -> IResult<Span, Vec<WideLine<f32>>> {
    let mut p = terminated(
      preceded(terminated(tag("RHS"), newline), many1(Self::rhs_line)),
      peek(anychar),
    );
    cfg_if::cfg_if! {
      if #[cfg(feature = "located")] {
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
  ///        `LocatedSpan` for enhanced error reporting with the `located` feature.
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
  /// - Without `located`: Standard parsing with basic error information.
  /// - With `located`: Enhanced error reporting, including detailed location tracking.
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
      if #[cfg(feature = "located")] {
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
  /// - `located` disabled: Standard parsing with basic error information.
  /// - `located` enabled: Enhanced error reporting, including detailed location tracking.
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
      preceded(terminated(tag("RANGES"), newline), many1(Self::ranges_line)),
      peek(anychar),
    );
    cfg_if::cfg_if! {
      if #[cfg(feature = "located")] {
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
  /// - Without `located`: Performs standard parsing.
  /// - With `located`: Enhanced parsing with detailed error reporting, leveraging `LocatedSpan`.
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
      if #[cfg(feature = "located")] {
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
      if #[cfg(feature = "located")] {
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
  /// - Without `located`: Standard parsing with basic error information.
  /// - With `located`: Enhanced error reporting, including detailed location tracking.
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
      if #[cfg(feature = "located")] {
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
  /// - Without `located`: Performs standard parsing with basic error information.
  /// - With `located`: Offers enhanced error reporting, including detailed location tracking.
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
      preceded(terminated(tag("BOUNDS"), newline), many1(Self::bounds_line)),
      peek(anychar),
    );
    cfg_if::cfg_if! {
      if #[cfg(feature = "located")] {
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
  /// - Without `located`: Standard parsing.
  /// - With `located`: Enhanced parsing with detailed error reporting.
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
      if #[cfg(feature = "located")] {
        let (s, x) = p(s)?;
        Ok((s, x.fragment()))
      } else { p(s) }
    }
  }
}
