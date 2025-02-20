//! ERFA Gnomonic Projection Functions

use crate::{raw::gnomonic::*, unexpected_val_err, ERFAError};

pub fn Tpors(xi: f64, eta: f64, a: f64, b: f64) -> (f64, f64, f64, f64) {
    let mut a01: f64 = 0.0;
    let mut b01: f64 = 0.0;
    let mut a02: f64 = 0.0;
    let mut b02: f64 = 0.0;

    unsafe {
        _ = eraTpors(xi, eta, a, b, &mut a01, &mut b01, &mut a02, &mut b02)
    }

    return (a01, b01, a02, b02)
}

pub fn Tporv(xi: f64, eta: f64, v: &[f64; 3]) -> ([f64; 3], [f64; 3]) {
    let mut v01: [f64; 3] = [0.0; 3];
    let mut v02: [f64; 3] = [0.0; 3];

    unsafe {
        _ = eraTporv(xi, eta, v, &mut v01, &mut v02)
    }

    return (v01, v02)
}

pub fn Tpsts(xi: f64, eta: f64, a0: f64, b0: f64) -> (f64, f64) {
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;

    unsafe {
        eraTpsts(xi, eta, a0, b0, &mut a, &mut b);
    }

    return (a, b)
}

pub fn Tpstv(xi: f64, eta: f64, v0: &[f64; 3]) -> [f64; 3] {
    let mut v: [f64; 3] = [0.0; 3];

    unsafe {
        eraTpstv(xi, eta, v0, &mut v);
    }

    return v
}

pub fn Tpxes(a: f64, b: f64, a0: f64, b0: f64) -> Result<(f64, f64), ERFAError> {
    let mut xi: f64 = 0.0;
    let mut eta: f64 = 0.0;
    let err: i32;

    unsafe {
        err = eraTpxes(a, b, a0, b0, &mut xi, &mut eta);
    }

    match err {
        0 => Ok((xi, eta)),
        1 => Err(ERFAError::ERFABadInputValue),
        2 => Err(ERFAError::ERFABadInputValue),
        3 => Err(ERFAError::ERFABadInputValue),
        _ => unexpected_val_err!(eraTpxes)
    }
}

pub fn Tpxev(v: &[f64; 3], v0: &[f64; 3]) -> Result<(f64, f64), ERFAError> {
    let mut xi: f64 = 0.0;
    let mut eta: f64 = 0.0;
    let err: i32;

    unsafe {
        err = eraTpxev(v, v0, &mut xi, &mut eta);
    }

    match err {
        0 => Ok((xi, eta)),
        1 => Err(ERFAError::ERFABadInputValue),
        2 => Err(ERFAError::ERFABadInputValue),
        3 => Err(ERFAError::ERFABadInputValue),
        _ => unexpected_val_err!(eraTpxev)
    }
}