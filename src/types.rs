use color_eyre::{eyre::eyre, Result};
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
