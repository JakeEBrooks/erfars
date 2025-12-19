#![warn(missing_docs)]
#![allow(clippy::needless_return)]
#![deny(clippy::implicit_return)]
//! A Rust crate that provides safe Rust bindings to the Essential Routines for Fundamental Astronomy ([ERFA](https://github.com/liberfa/erfa))
//! C library, which is based on the Standards of Fundamental Astronomy ([SOFA](https://www.iausofa.org/index.html)) library published by
//! the International Astronomical Union (IAU).
//!
//! ## Usage
//! With this crate, functions in ERFA can be called safely in Rust:
//! ```rust
//! use erfars::calendar::Cal2jd;
//!
//! fn main() {
//!     let ((jd0, jd1), _) = Cal2jd(2025, 1, 22).unwrap();
//!     assert_eq!(jd0+jd1, 2460697.5)
//! }
//! ```
//!
//! ### A note on array arguments in functions
//! Many of the ERFA C functions pass data in the form of multidimensional arrays. For example, a `double[3][3]`
//! represents a 3x3 matrix of double-precision floating-point values. However, Rust does not have
//! an equivalent and clean way of representing this in code. A `double[3][3]` is stored in row-major order,
//! so the Rust equivalent in memory is simply an array `[f64; 9]`. More specifically, the following
//! variable declarations are equivalent:
//!
//! In C:
//! ```c
//! double xyz[3][3] = { {0.0, 0.0, 0.0}, {0.0, 0.0, 0.0}, {0.0, 0.0, 0.0} };
//! ```
//!
//! In Rust:
//! ```rust
//! let xyz: [f64; 9] = [0.0; 9];
//! ```
//!
//! Where the first three elements of the Rust array are the first row of the C array, and the second group
//! of three elements in the Rust array are the second row of the C array, and so on.
//!
//! For the user, this means that wherever you see something like a `double[2][3]` or a `double[3][3]` in
//! the ERFA C API, you can safely pass a `[f64; 6]` or a `[f64; 9]` Rust value.
//!
//! ### A note on return values for some ERFA functions
//! Some of the safe Rust ERFA functions return a [`ERFAResult`], which encapsulates both warnings and errors returned from ERFA.
//! The [`Ok`] value of this result contains both the actual
//! result and the warning code as a tuple. The warning code is equal to zero if all is ok.
//! Warning codes from ERFA are always greater than zero, which means you can handle errors using something like:
//!
//! ```rust
//! use erfars::calendar::Jdcalf;
//!
//! fn main() {
//!     // Panics on error
//!     // If a warning is raised, `warn` is > 0
//!     let (res, warn) = Jdcalf(2460697.0, 0.5, 2).unwrap();
//!     if warn > 0 {
//!         // Do something
//!     }
//! }

//! ```
//!
//! The meaning of the warning/error codes can be found in the ERFA docs for the relevant function.

// Stop the linter from complaining about the ERFA function names
#![allow(non_snake_case)]

pub mod astrometry;
pub mod calendar;
pub mod eclipticcoordinates;
pub mod ephemerides;
pub mod fundamentalargs;
pub mod galacticcoordinates;
pub mod geodeticgeocentric;
pub mod gnomonic;
pub mod horizonequatorial;
pub mod precnutpolar;
pub mod rotationtime;
pub mod spacemotion;
pub mod starcatalogs;
pub mod timescales;

pub mod constants;
pub mod raw;

mod structs;
pub use structs::{Astrom, LDBody};

/// A return type for ERFA functions which contains both a warning code and an error code.
pub type ERFAResult<T> = Result<(T, i32), i32>;

macro_rules! unexpected_val_err {
    ($fname:ident) => {
        panic!(concat!("Unexpected value received from ERFA function", stringify!($fname)))
    };
}
pub(crate) use unexpected_val_err;
