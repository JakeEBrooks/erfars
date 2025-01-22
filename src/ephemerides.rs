//! Functions for calculating ephemerides for objects within the solar system

use crate::{raw::ephemerides::*, unexpected_val_err, ERFAError};

pub fn Epv00(date1: f64, date2: f64) -> ([f64; 6], [f64; 6]) {
    let mut pvh: [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let mut pvb: [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let err: i32;
    unsafe { err = eraEpv00(date1, date2, &mut pvh, &mut pvb) }

    match err {
        0 => (pvh, pvb),
        // Ignore the warning
        1 => (pvh, pvb),
        _ => unexpected_val_err!(eraEpv00),
    }
}

pub fn Moon98(date1: f64, date2: f64) -> [f64; 6] {
    let mut pv: [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    unsafe { eraMoon98(date1, date2, &mut pv) }

    return pv;
}

pub fn Plan94(date1: f64, date2: f64, planet: i32) -> Result<[f64; 6], ERFAError> {
    let mut pv: [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let err: i32;
    unsafe { err = eraPlan94(date1, date2, planet, &mut pv) }

    match err {
        0 => Ok(pv),
        // Ignore the warning
        1 => Ok(pv),
        // Ignore the warning
        2 => Ok(pv),
        -1 => Err(ERFAError::ERFABadInputValue),
        _ => unexpected_val_err!(eraPlan94),
    }
}
