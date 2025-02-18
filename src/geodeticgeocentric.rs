//! ERFA Geodetic/Geocentric Functions

use crate::{unexpected_val_err, ERFAError};
use crate::raw::geodeticgeocentric::*;

pub fn Eform(n: i32) -> Result<(f64, f64), ERFAError> {
    let mut a: f64 = 0.0;
    let mut f: f64 = 0.0;
    let err: i32;
    unsafe{
        err = eraEform(n, &mut a, &mut f)
    }
    match err {
        0 => Ok((a, f)),
        -1 => Err(ERFAError::ERFABadInputValue),
        _ => unexpected_val_err!(eraEform)
    }
}

pub fn Gc2gd(n: i32, xyz: &[f64; 3]) -> Result<(f64, f64, f64), ERFAError> {
    let mut elong: f64 = 0.0;
    let mut phi: f64 = 0.0;
    let mut height: f64 = 0.0;
    let err: i32;
    unsafe{
        err = eraGc2gd(n, xyz, &mut elong, &mut phi, &mut height)
    }

    match err {
        0 => Ok((elong, phi, height)),
        -1 => Err(ERFAError::ERFABadInputValue),
        -2 => Err(ERFAError::ERFAInternalError),
        _ => unexpected_val_err!(eraGc2gd)
    }
}

pub fn Gc2gde(a: f64, f: f64, xyz: &[f64; 3]) -> Result<(f64, f64, f64), ERFAError> {
    let mut elong: f64 = 0.0;
    let mut phi: f64 = 0.0;
    let mut height: f64 = 0.0;
    let err: i32;
    unsafe{
        err = eraGc2gde(a, f, xyz, &mut elong, &mut phi, &mut height)
    }

    match err {
        0 => Ok((elong, phi, height)),
        -1 => Err(ERFAError::ERFABadInputValue),
        -2 => Err(ERFAError::ERFABadInputValue),
        _ => unexpected_val_err!(eraGc2gde)
    }
}

pub fn Gd2gc(n: i32, elong: f64, phi: f64, height: f64) -> Result<[f64; 3], ERFAError> {
    let mut xyz: [f64; 3] = [0.0; 3];
    let err: i32;
    unsafe {
        err = eraGd2gc(n, elong, phi, height, &mut xyz)
    }

    match err {
        0 => Ok(xyz),
        -1 => Err(ERFAError::ERFABadInputValue),
        -2 => Err(ERFAError::ERFABadInputValue),
        _ => unexpected_val_err!(eraGd2gc)
    }
}

pub fn Gd2gce(a: f64, f: f64, elong: f64, phi: f64, height: f64) -> Result<[f64; 3], ERFAError> {
    let mut xyz: [f64; 3] = [0.0; 3];
    let err: i32;
    unsafe {
        err = eraGd2gce(a, f, elong, phi, height, &mut xyz)
    }

    match err {
        0 => Ok(xyz),
        -1 => Err(ERFAError::ERFABadInputValue),
        _ => unexpected_val_err!(eraGd2gce)
    }
}