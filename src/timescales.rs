//! ERFA Timescales Functions

use std::ffi::CString;

use crate::{raw::timescales::*, unexpected_val_err, ERFAError};

pub fn D2dtf(utc: bool, ndp: i32, d1: f64, d2: f64) -> Result<(i32, i32, i32, i32, i32, i32, i32), ERFAError> {
    let mut year: i32 = 0;
    let mut month: i32 = 0;
    let mut day: i32 = 0;
    let mut hmsf: [i32; 4] = [0; 4];
    let scale = if utc {CString::new("UTC")} else {CString::new("NA")};
    let err: i32;
    unsafe{ err = eraD2dtf(scale.unwrap().as_ptr(), ndp, d1, d2, &mut year, &mut month, &mut day, &mut hmsf) };
    match err {
        -1 => Err(ERFAError::ERFABadDate),
        0 => Ok((year, month, day, hmsf[0], hmsf[1], hmsf[2], hmsf[3])),
        1 => Ok((year, month, day, hmsf[0], hmsf[1], hmsf[2], hmsf[3])),
        _ => unexpected_val_err!(eraD2dtf)
    }
}

pub fn Dat(year: i32, month: i32, day: i32, fd: f64) -> Result<f64, ERFAError> {
    let mut deltat: f64 = 0.0;
    let err: i32;
    unsafe{ err = eraDat(year, month, day, fd, &mut deltat) }
    match err {
        -5 => Err(ERFAError::ERFAInternalError),
        -4 => Err(ERFAError::ERFABadFraction),
        -3 => Err(ERFAError::ERFABadDay),
        -2 => Err(ERFAError::ERFABadMonth),
        -1 => Err(ERFAError::ERFABadYear),
        0 => Ok(deltat),
        1 => Ok(deltat),
        _ => unexpected_val_err!(eraDat)
    }
}

pub fn Dtdb(date1: f64, date2: f64, ut: f64, elong: f64, u: f64, v: f64) -> f64 {
    return unsafe{ eraDtdb(date1, date2, ut, elong, u, v) }
}

pub fn Dtf2d(utc: bool, year: i32, month: i32, day: i32, hour: i32, minute: i32, seconds: f64) -> Result<(f64, f64), ERFAError> {
    let mut date1: f64 = 0.0;
    let mut date2: f64 = 0.0;
    let scale = if utc {CString::new("UTC")} else {CString::new("NA")};
    let err: i32;
    unsafe{ err = eraDtf2d(scale.unwrap().as_ptr(), year, month, day, hour, minute, seconds, &mut date1, &mut date2) }

    match err {
        -6 => Err(ERFAError::ERFABadSecond),
        -5 => Err(ERFAError::ERFABadMinute),
        -4 => Err(ERFAError::ERFABadHour),
        -3 => Err(ERFAError::ERFABadDay),
        -2 => Err(ERFAError::ERFABadMonth),
        -1 => Err(ERFAError::ERFABadYear),
        0 => Ok((date1, date2)),
        1 => Ok((date1, date2)),
        2 => Ok((date1, date2)),
        3 => Ok((date1, date2)),
        _ => unexpected_val_err!(eraDtf2d)
    }
}