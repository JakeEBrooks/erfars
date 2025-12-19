//! ERFA Geodetic/Geocentric Functions

use crate::raw::geodeticgeocentric::*;
use crate::{ERFAResult, unexpected_val_err};

/// Earth reference ellipsoids.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/eform.c)
pub fn Eform(n: i32) -> ERFAResult<(f64, f64)> {
    let mut a: f64 = 0.0;
    let mut f: f64 = 0.0;
    let err: i32;
    unsafe { err = eraEform(n, &mut a, &mut f) }
    match err {
        0 => Ok(((a, f), 0)),
        -1 => Err(-1),
        _ => unexpected_val_err!(eraEform),
    }
}

/// Transform geocentric coordinates to geodetic using the specified reference
/// ellipsoid.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/gc2gd.c)
pub fn Gc2gd(n: i32, xyz: &[f64; 3]) -> ERFAResult<(f64, f64, f64)> {
    let mut elong: f64 = 0.0;
    let mut phi: f64 = 0.0;
    let mut height: f64 = 0.0;
    let err: i32;
    unsafe { err = eraGc2gd(n, xyz, &mut elong, &mut phi, &mut height) }

    match err {
        0 => Ok(((elong, phi, height), 0)),
        -1 => Err(-1),
        -2 => Err(-2),
        _ => unexpected_val_err!(eraGc2gd),
    }
}

/// Transform geocentric coordinates to geodetic for a reference ellipsoid of
/// specified form.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/gc2gde.c)
pub fn Gc2gde(a: f64, f: f64, xyz: &[f64; 3]) -> ERFAResult<(f64, f64, f64)> {
    let mut elong: f64 = 0.0;
    let mut phi: f64 = 0.0;
    let mut height: f64 = 0.0;
    let err: i32;
    unsafe { err = eraGc2gde(a, f, xyz, &mut elong, &mut phi, &mut height) }

    match err {
        0 => Ok(((elong, phi, height), 0)),
        -1 => Err(-1),
        -2 => Err(-2),
        _ => unexpected_val_err!(eraGc2gde),
    }
}

/// Transform geodetic coordinates to geocentric using the specified reference
/// ellipsoid.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/gd2gc.c)
pub fn Gd2gc(n: i32, elong: f64, phi: f64, height: f64) -> ERFAResult<[f64; 3]> {
    let mut xyz: [f64; 3] = [0.0; 3];
    let err: i32;
    unsafe { err = eraGd2gc(n, elong, phi, height, &mut xyz) }

    match err {
        0 => Ok((xyz, 0)),
        -1 => Err(-1),
        -2 => Err(-2),
        _ => unexpected_val_err!(eraGd2gc),
    }
}

/// Transform geodetic coordinates to geocentric for a reference ellipsoid of
/// specified form.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/gd2gce.c)
pub fn Gd2gce(a: f64, f: f64, elong: f64, phi: f64, height: f64) -> ERFAResult<[f64; 3]> {
    let mut xyz: [f64; 3] = [0.0; 3];
    let err: i32;
    unsafe { err = eraGd2gce(a, f, elong, phi, height, &mut xyz) }

    match err {
        0 => Ok((xyz, 0)),
        -1 => Err(-1),
        _ => unexpected_val_err!(eraGd2gce),
    }
}
