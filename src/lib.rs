#![warn(missing_docs, rust_2018_idioms)]
#![cfg_attr(docsrs, feature(doc_cfg))]

//! CMAC-based utilities for imputing missing floating-point observations.
//!
//! # Examples
//!
//! ```
//! use cmac_rust::Cmac;
//! use std::f64;
//!
//! let data = vec![1.0, f64::NAN, 1.0];
//! let mut cmac = Cmac::new(data, 1);
//! let filled = cmac.impute(0.5, 4);
//!
//! assert!(!filled[1].is_nan());
//! ```

pub mod cmac;

// Re-export explicitly from the module to avoid ambiguity between the
// `cmac` module and the `Cmac` type when the crate is used from other
// contexts (this mirrors `crate::cmac::Cmac`).
pub use crate::cmac::Cmac;

#[allow(clippy::upper_case_acronyms)]
/// Backwards-compatible alias that preserves the original type name.
pub type CMAC = crate::cmac::Cmac;

/// Prelude containing the primary types exported by this crate.
pub mod prelude {
    pub use crate::cmac::Cmac;
}
