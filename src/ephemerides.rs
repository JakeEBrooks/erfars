//! ERFA Ephemerides Functions

use crate::{ERFAResult, raw::ephemerides::*, unexpected_val_err};

/// Earth position and velocity, heliocentric and barycentric, with respect to
/// the Barycentric Celestial Reference System.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/epv00.c)
pub fn Epv00(date1: f64, date2: f64) -> ERFAResult<([f64; 6], [f64; 6])> {
    let mut pvh: [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let mut pvb: [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let err: i32;
    unsafe { err = eraEpv00(date1, date2, &mut pvh, &mut pvb) }

    match err {
        0 => Ok(((pvh, pvb), 0)),
        1 => Ok(((pvh, pvb), 1)),
        _ => unexpected_val_err!(eraEpv00),
    }
}

/// Approximate geocentric position and velocity of the Moon.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/moon98.c)
pub fn Moon98(date1: f64, date2: f64) -> [f64; 6] {
    let mut pv: [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    unsafe { eraMoon98(date1, date2, &mut pv) }

    return pv;
}

/// Approximate heliocentric position and velocity of a nominated planet:
/// Mercury, Venus, EMB, Mars, Jupiter, Saturn, Uranus or Neptune (but not the
/// Earth itself).
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/plan94.c)
pub fn Plan94(date1: f64, date2: f64, planet: i32) -> ERFAResult<[f64; 6]> {
    let mut pv: [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let err: i32;
    unsafe { err = eraPlan94(date1, date2, planet, &mut pv) }

    match err {
        0 => Ok((pv, 0)),
        1 => Ok((pv, 1)),
        2 => Ok((pv, 2)),
        -1 => Err(-1),
        _ => unexpected_val_err!(eraPlan94),
    }
}
