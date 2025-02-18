// TODO: uncomment when ready
// #![warn(missing_docs)]
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
//!     let (jd0, jd1) = Cal2jd(2025, 1, 22).unwrap();
//!     assert_eq!(jd0+jd1, 2460697.5)
//! }
//! ```
//! 
//! *Note: Documentation is currently not available for the safe versions of the ERFA functions, 
//! but for now the user can consult the original ERFA/SOFA documentation as the function interfaces 
//! are quite similar.*
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
// TODO: timescales are not done
pub mod timescales;

pub mod raw;
pub mod constants;

mod structs;
pub use structs::{Astrom, LDBody};

/// An error type for error codes returned by ERFA
#[derive(Debug)]
pub enum ERFAError {
    /// Indicates a bad year input
    ERFABadYear,
    /// Indicates a bad month input
    ERFABadMonth,
    /// Indicates a bad day input
    ERFABadDay,
    /// Indicates a bad hour input
    ERFABadHour,
    /// Indicates a bad minute input
    ERFABadMinute,
    /// Indicates a bad second input
    ERFABadSecond,
    /// Indicates a bad fraction (of a day) input
    ERFABadFraction,
    /// Indicates a bad date input
    ERFABadDate,
    /// Indicates a generically bad input value
    ERFABadInputValue,
    /// Indicates a bad speed input
    ERFABadSpeed,
    /// Indicates a bad position input
    ERFABadPosition,
    /// Indicates an error within ERFA
    ERFAInternalError
}

macro_rules! unexpected_val_err {
    ($fname:ident) => {
        panic!(concat!("Unexpected value received from ", stringify!($fname)))
    };
}
pub(crate) use unexpected_val_err;
