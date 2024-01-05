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
pub mod model;
pub mod parse;
pub mod types;
pub use crate::types::Parser;
