// #![warn(missing_docs)]
//! A Rust crate that provides safe Rust bindings to the Essential Routines for Fundamental Astronomy ([ERFA](https://github.com/liberfa/erfa)) 
//! C library, which is based on the Standards of Fundamental Astronomy ([SOFA](https://www.iausofa.org/index.html)) library published by 
//! the International Astronomical Union (IAU).
//! 
//! ## Usage
//! Functions in ERFA can be called safely in Rust:
//! ```rust
//! use erfars::calendar::Cal2jd;
//! 
//! fn main() {
//!     let (jd0, jd1) = Cal2jd(2025, 1, 22).unwrap();
//!     assert_eq!(jd0+jd1, 2460697.5)
//! }
//! ```

#![allow(non_snake_case)]

// TODO: order these alphabetically
pub mod astrometry;
pub mod calendar;
pub mod ephemerides;
pub mod fundamentalargs;
pub mod precnutpolar;
pub mod rotationtime;
pub mod spacemotion;
pub mod timescales;
pub mod galacticcoordinates;

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
    /// Indicates a bad (day) fraction input
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
