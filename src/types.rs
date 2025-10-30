use color_eyre::{eyre::eyre, Result};
use fast_float2::FastFloat;
#[cfg(feature = "serde")]
use serde::Serialize;

cfg_if::cfg_if! {
  if #[cfg(feature = "trace")] {
    use nom_locate::LocatedSpan;
    use nom_tracable::TracableInfo;

    /// Conditionally compiles different `Span` types based on the crate's feature flags.
    ///
    /// When the `trace` feature is enabled, it utilizes `nom_locate::LocatedSpan` and `nom_tracable::TracableInfo`
    /// to provide enhanced error reporting and debugging capabilities. This allows the parser to track the location
    /// of tokens within the input, making it easier to diagnose parsing errors and understand the parsing flow.
    ///
    /// Without the `trace` feature, a simpler `Span` type is used, which is a reference to a slice of the input string.
    /// This simpler span is more performant but lacks the detailed tracking and error reporting capabilities.
    /// A `Span` type that includes location and tracing information.
    /// Used when the `trace` feature is enabled.
    pub type Span<'a> = LocatedSpan<&'a str, TracableInfo>;
  } else {
    /// A simple `Span` type representing a reference to a slice of the input string.
    /// Used when the `trace` feature is not enabled.
    pub type Span<'a> = &'a str;
  }
}

/// The primary structure for parsing MPS (Mathematical Programming System) data.
///
/// `Parser` holds the structured representation of a parsed MPS file, supporting both standard
/// MPS format and CPLEX extensions for mixed-integer programming (MIP) and quadratic programming (QP).
///
/// # Type Parameters
///
/// * `'a`: Lifetime parameter indicating that fields hold references to the input string data.
/// * `T`: Numeric type bounded by `FastFloat` (f32, f64, etc.) for floating-point values.
///
/// # Fields - Core Sections
///
/// * `name`: Problem name from NAME section
/// * `rows`: Row definitions (constraints) from ROWS section
/// * `columns`: Column definitions (variables) and coefficients from COLUMNS section
/// * `rhs`: Right-hand side values from optional RHS section
/// * `ranges`: Range constraints from optional RANGES section
/// * `bounds`: Variable bounds from optional BOUNDS section
///
/// # Fields - CPLEX Extensions (MIP/QP)
///
/// * `objective_sense`: Optimization direction (MIN or MAX) from optional OBJSENSE section
/// * `objective_name`: Explicit objective row name from optional OBJNAME section
/// * `reference_row`: Reference row for SOS weights from optional REFROW section
/// * `user_cuts`: User-defined cuts from optional USERCUTS section
/// * `special_ordered_sets`: SOS definitions from optional SOS section (must follow BOUNDS)
/// * `quadratic_objective`: Quadratic objective terms from QSECTION/QUADOBJ/QMATRIX sections
/// * `quadratic_constraints`: Quadratic constraint terms from optional QCMATRIX sections
/// * `indicators`: Indicator constraints from optional INDICATORS section
/// * `lazy_constraints`: Lazy constraints from optional LAZYCONS section
/// * `cone_constraints`: Second-order cone constraints from optional CSECTION
///
/// # Section Ordering
///
/// The parser enforces CPLEX MPS format section ordering to ensure spec compliance:
/// NAME → [OBJSENSE] → [OBJNAME] → [REFROW] → ROWS → [USERCUTS] → COLUMNS → [RHS] →
/// [RANGES] → [BOUNDS] → [SOS] → [QSECTION/QUADOBJ/QMATRIX] → [QCMATRIX]* → [CSECTION] →
/// [INDICATORS] → [LAZYCONS] → ENDATA
///
/// # Example
///
/// ```ignore
/// let parser: Parser<f64> = Parser::parse(mps_content)?;
/// println!("Problem: {}", parser.name);
/// println!("Rows: {}", parser.rows.len());
/// println!("Columns: {}", parser.columns.len());
/// if let Some(obj) = &parser.quadratic_objective {
///   println!("Quadratic objective terms: {}", obj.len());
/// }
/// ```
#[derive(Debug, Default, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Parser<'a, T: FastFloat> {
  /// Problem name from NAME section
  pub name: &'a str,
  /// Objective sense (MIN/MAX) from optional OBJSENSE section
  pub objective_sense: Option<ObjectiveSense>,
  /// Objective function row name from optional OBJNAME section
  pub objective_name: Option<&'a str>,
  /// Reference row for SOS weighting from optional REFROW section
  pub reference_row: Option<&'a str>,
  /// Row constraints from ROWS section
  pub rows: Rows<'a>,
  /// Column variables from COLUMNS section
  pub columns: Columns<'a, T>,
  /// Right-hand side values from optional RHS section
  pub rhs: Option<Rhs<'a, T>>,
  /// Range constraints from optional RANGES section
  pub ranges: Option<Ranges<'a, T>>,
  /// Variable bounds from optional BOUNDS section
  pub bounds: Option<Bounds<'a, T>>,
  /// User-defined cuts from optional USERCUTS section
  pub user_cuts: Option<UserCuts<'a>>,
  /// Special ordered sets from optional SOS section
  pub special_ordered_sets: Option<SpecialOrderedSets<'a, T>>,
  /// Quadratic objective terms from QSECTION/QUADOBJ/QMATRIX sections
  pub quadratic_objective: Option<QuadraticObjective<'a, T>>,
  /// Quadratic constraint terms from optional QCMATRIX sections
  pub quadratic_constraints: Option<QuadraticConstraints<'a, T>>,
  /// Indicator constraints from optional INDICATORS section
  pub indicators: Option<Indicators<'a>>,
  /// Lazy constraints from optional LAZYCONS section
  pub lazy_constraints: Option<LazyConstraints<'a>>,
  /// Second-order cone constraints from optional CSECTION
  pub cone_constraints: Option<ConeConstraints<'a, T>>,
  /// Branching priorities from optional BRANCH section
  pub branch_priorities: Option<BranchPriorities<'a>>,
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
#[cfg_attr(feature = "serde", derive(Serialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize))]
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

/// Enumeration representing the objective function sense (minimize or maximize)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum ObjectiveSense {
  /// Minimize the objective function
  Min,
  /// Maximize the objective function
  Max,
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
#[cfg_attr(feature = "serde", derive(Serialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize))]
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
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct BoundsLine<'a, T> {
  pub bound_type: BoundType,
  pub bound_name: &'a str,
  pub column_name: &'a str,
  pub value: Option<T>,
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
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum BoundType {
  #[default]
  Lo, // lower bound     :  l_j <= x_j <= inf
  Up, // upper bound     :    0 <= x_j <= u_j
  Fx, // fixed variable  :  l_j == x_j == u_j
  Fr, // free variable   : -inf <= x_j <= inf
  Mi, // Unbounded below : -inf <= x_j <= 0
  Pl, // Unbounded above :    0 <= x_j <= inf
  Bv, // Binary variable :  x_j in {0, 1}
  Li, // Lower integer   :  l_j <= x_j <= inf, x_j integer
  Ui, // Upper integer   :    0 <= x_j <= u_j, x_j integer
  Sc, // Semi-continuous :  x_j = 0 or l_j <= x_j <= u_j
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
      "BV" => Ok(BoundType::Bv),
      "LI" => Ok(BoundType::Li),
      "UI" => Ok(BoundType::Ui),
      "SC" => Ok(BoundType::Sc),
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
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum RangeType {
  #[default]
  _Le, // Less than or Equal
  _Ge, // Greater than or Equal
  _Ep, // Equality with positive R_i
  _Em, // Equality with negative R_i
  _Ez, // Equality with unspecified R_i
}

// ============================================================================
// MIP/QP Extension Types
// ============================================================================

/// Represents an indicator constraint in MIP problems
/// Format: IF binary_var = 0/1 THEN constraint is active
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct IndicatorLine<'a> {
  /// Binary variable name
  pub binary_var: &'a str,
  /// Value that triggers the constraint (0 or 1)
  pub trigger_value: u8,
  /// Row/constraint name that is activated
  pub constraint_name: &'a str,
}

/// Collection of indicator constraints
pub type Indicators<'a> = Vec<IndicatorLine<'a>>;

/// Represents a lazy constraint (constraint that's only added when violated)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct LazyConstraintLine<'a> {
  /// Priority level (higher = checked first)
  pub priority: Option<i32>,
  /// Constraint row name
  pub row_name: &'a str,
}

/// Collection of lazy constraints
pub type LazyConstraints<'a> = Vec<LazyConstraintLine<'a>>;

/// Collection of user-defined cuts (same format as rows)
pub type UserCuts<'a> = Vec<RowLine<'a>>;

/// Represents a quadratic term in the objective function
/// For term: coefficient * var1 * var2
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct QuadraticObjectiveTerm<'a, T: FastFloat> {
  /// First variable in the quadratic term
  pub var1: &'a str,
  /// Second variable in the quadratic term
  pub var2: &'a str,
  /// Coefficient of the quadratic term
  pub coefficient: T,
}

/// Collection of quadratic objective terms
pub type QuadraticObjective<'a, T> = Vec<QuadraticObjectiveTerm<'a, T>>;

/// Type of Special Ordered Set
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum SOSType {
  /// Type 1: At most one variable can be non-zero
  S1,
  /// Type 2: At most two adjacent variables can be non-zero
  S2,
}

impl TryFrom<&str> for SOSType {
  type Error = color_eyre::Report;

  fn try_from(s: &str) -> Result<Self> {
    match s {
      "S1" => Ok(SOSType::S1),
      "S2" => Ok(SOSType::S2),
      _ => Err(eyre!("invalid SOS type: {}", s)),
    }
  }
}

/// Special Ordered Set definition
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SOSLine<'a, T: FastFloat> {
  /// Type of SOS (S1 or S2)
  pub sos_type: SOSType,
  /// Name of the SOS set
  pub set_name: &'a str,
  /// Variables in the set with their weights
  pub members: Vec<SOSMember<'a, T>>,
}

/// Member of a Special Ordered Set
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct SOSMember<'a, T: FastFloat> {
  /// Variable name
  pub var_name: &'a str,
  /// Weight/priority in the SOS
  pub weight: T,
}

/// Collection of Special Ordered Sets
pub type SpecialOrderedSets<'a, T> = Vec<SOSLine<'a, T>>;

/// Quadratic constraint in the form: x'Qx + c'x <= b
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct QuadraticConstraint<'a, T: FastFloat> {
  /// Name of the constraint row
  pub row_name: &'a str,
  /// Quadratic terms in the constraint
  pub terms: Vec<QuadraticTerm<'a, T>>,
}

/// Single quadratic term in a constraint
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct QuadraticTerm<'a, T: FastFloat> {
  /// First variable
  pub var1: &'a str,
  /// Second variable
  pub var2: &'a str,
  /// Coefficient
  pub coefficient: T,
}

/// Collection of quadratic constraints
pub type QuadraticConstraints<'a, T> = Vec<QuadraticConstraint<'a, T>>;

/// Type of cone constraint
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum ConeType {
  /// Quadratic/Second-order cone
  Quad,
  /// Rotated quadratic cone
  RQuad,
}

impl TryFrom<&str> for ConeType {
  type Error = color_eyre::Report;

  fn try_from(s: &str) -> Result<Self> {
    match s {
      "QUAD" => Ok(ConeType::Quad),
      "RQUAD" => Ok(ConeType::RQuad),
      _ => Err(eyre!("invalid cone type: {}", s)),
    }
  }
}

/// Second-order cone constraint
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ConeConstraint<'a, T: FastFloat> {
  /// Name of the cone constraint
  pub cone_name: &'a str,
  /// Type of cone
  pub cone_type: ConeType,
  /// Variables in the cone
  pub members: Vec<ConeMember<'a, T>>,
}

/// Member variable of a cone constraint
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct ConeMember<'a, T: FastFloat> {
  /// Variable name
  pub var_name: &'a str,
  /// Coefficient (optional, defaults to 1.0)
  pub coefficient: Option<T>,
}

/// Collection of cone constraints
pub type ConeConstraints<'a, T> = Vec<ConeConstraint<'a, T>>;

// ============================================================================
// Branching Directives
// ============================================================================

/// Branching direction for integer variables
///
/// Specifies the direction preference for branch-and-bound when exploring the search tree.
/// Variables with higher priorities are branched first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub enum BranchDirection {
  /// Branch up first (prefer increasing variable values)
  Up,
  /// Branch down first (prefer decreasing variable values)
  Down,
  /// Use rounding heuristic
  Rounding,
  /// Branch toward closest bound
  ClosestBound,
  /// Let solver decide (automatic)
  Auto,
}

impl TryFrom<&str> for BranchDirection {
  type Error = color_eyre::Report;

  fn try_from(s: &str) -> Result<Self> {
    match s {
      "UP" => Ok(BranchDirection::Up),
      "DN" => Ok(BranchDirection::Down),
      "RD" => Ok(BranchDirection::Rounding),
      "CB" => Ok(BranchDirection::ClosestBound),
      "" => Ok(BranchDirection::Auto),
      _ => Err(eyre!("invalid branch direction: {}", s)),
    }
  }
}

/// Branching priority specification for an integer variable
///
/// Per CPLEX MPS specification: specifies branching priorities and directions
/// to guide the branch-and-bound algorithm. Variables with higher priorities
/// are branched on first. Direction specifies which branch to explore first.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct BranchPriority<'a> {
  /// Variable name
  pub var_name: &'a str,
  /// Priority value (0 = default/lowest, higher = branch first)
  /// Must be non-negative
  pub priority: i32,
  /// Direction preference for branching
  pub direction: BranchDirection,
}

/// Collection of branching priority specifications
pub type BranchPriorities<'a> = Vec<BranchPriority<'a>>;
