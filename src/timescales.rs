//! ERFA Timescales Functions

use std::ffi::CString;

use crate::{ERFAError, raw::timescales::*, unexpected_val_err};

/// Format for output a 2-part Julian Date (or in the case of UTC a quasi-JD
/// form that includes special provision for leap seconds).
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/d2dtf.c)
pub fn D2dtf(utc: bool, ndp: i32, d1: f64, d2: f64) -> Result<(i32, i32, i32, i32, i32, i32, i32), ERFAError> {
    let mut year: i32 = 0;
    let mut month: i32 = 0;
    let mut day: i32 = 0;
    let mut hmsf: [i32; 4] = [0; 4];
    let scale = if utc { CString::new("UTC") } else { CString::new("NA") };
    let err: i32;
    unsafe { err = eraD2dtf(scale.unwrap().as_ptr(), ndp, d1, d2, &mut year, &mut month, &mut day, &mut hmsf) };
    match err {
        -1 => Err(ERFAError::ERFABadDate),
        0 => Ok((year, month, day, hmsf[0], hmsf[1], hmsf[2], hmsf[3])),
        1 => Ok((year, month, day, hmsf[0], hmsf[1], hmsf[2], hmsf[3])),
        _ => unexpected_val_err!(eraD2dtf),
    }
}

/// For a given UTC date, calculate Delta(AT) = TAI-UTC.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/dat.c)
pub fn Dat(year: i32, month: i32, day: i32, fd: f64) -> Result<f64, ERFAError> {
    let mut deltat: f64 = 0.0;
    let err: i32;
    unsafe { err = eraDat(year, month, day, fd, &mut deltat) }
    match err {
        -5 => Err(ERFAError::ERFAInternalError),
        -4 => Err(ERFAError::ERFABadFraction),
        -3 => Err(ERFAError::ERFABadDay),
        -2 => Err(ERFAError::ERFABadMonth),
        -1 => Err(ERFAError::ERFABadYear),
        0 => Ok(deltat),
        1 => Ok(deltat),
        _ => unexpected_val_err!(eraDat),
    }
}

/// An approximation to TDB-TT, the difference between barycentric dynamical
/// time and terrestrial time, for an observer on the Earth.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/dtdb.c)
pub fn Dtdb(date1: f64, date2: f64, ut: f64, elong: f64, u: f64, v: f64) -> f64 {
    return unsafe { eraDtdb(date1, date2, ut, elong, u, v) };
}

/// Encode date and time fields into 2-part Julian Date (or in the case of UTC a
/// quasi-JD form that includes special provision for leap seconds).
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/dtf2d.c)
pub fn Dtf2d(utc: bool, year: i32, month: i32, day: i32, hour: i32, minute: i32, seconds: f64) -> Result<(f64, f64), ERFAError> {
    let mut date1: f64 = 0.0;
    let mut date2: f64 = 0.0;
    let scale = if utc { CString::new("UTC") } else { CString::new("NA") };
    let err: i32;
    unsafe { err = eraDtf2d(scale.unwrap().as_ptr(), year, month, day, hour, minute, seconds, &mut date1, &mut date2) }

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
        _ => unexpected_val_err!(eraDtf2d),
    }
}

/// Time scale transformation: International Atomic Time, TAI, to Terrestrial
/// Time, TT.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/taitt.c)
pub fn Taitt(tai1: f64, tai2: f64) -> (f64, f64) {
    let mut tt1: f64 = 0.0;
    let mut tt2: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraTaitt(tai1, tai2, &mut tt1, &mut tt2);
    }

    return (tt1, tt2);
}

/// Time scale transformation: International Atomic Time, TAI, to Universal
/// Time, UT1.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/taiut1.c)
pub fn Taiut1(tai1: f64, tai2: f64, dta: f64) -> (f64, f64) {
    let mut ut11: f64 = 0.0;
    let mut ut12: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraTaiut1(tai1, tai2, dta, &mut ut11, &mut ut12)
    }

    return (ut11, ut12);
}

/// Time scale transformation: International Atomic Time, TAI, to Coordinated
/// Universal Time, UTC.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/taiutc.c)
pub fn Taiutc(tai1: f64, tai2: f64) -> Result<(f64, f64), ERFAError> {
    let mut utc1: f64 = 0.0;
    let mut utc2: f64 = 0.0;
    let err: i32;

    unsafe { err = eraTaiutc(tai1, tai2, &mut utc1, &mut utc2) }

    match err {
        -1 => Err(ERFAError::ERFABadDate),
        0 => Ok((utc1, utc2)),
        1 => Ok((utc1, utc2)),
        _ => unexpected_val_err!(eraTaiutc),
    }
}

/// Time scale transformation: Barycentric Coordinate Time, TCB, to Barycentric
/// Dynamical Time, TDB.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/tcbtdb.c)
pub fn Tcbtdb(tcb1: f64, tcb2: f64) -> (f64, f64) {
    let mut tdb1: f64 = 0.0;
    let mut tdb2: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraTcbtdb(tcb1, tcb2, &mut tdb1, &mut tdb2)
    }

    return (tdb1, tdb2);
}

/// Time scale transformation: Geocentric Coordinate Time, TCG, to Terrestrial
/// Time, TT.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/tcgtt.c)
pub fn Tcgtt(tcg1: f64, tcg2: f64) -> (f64, f64) {
    let mut tt1: f64 = 0.0;
    let mut tt2: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraTcgtt(tcg1, tcg2, &mut tt1, &mut tt2)
    }

    return (tt1, tt2);
}

/// Time scale transformation: Barycentric Dynamical Time, TDB, to Barycentric
/// Coordinate Time, TCB.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/tdbtcb.c)
pub fn Tdbtcb(tdb1: f64, tdb2: f64) -> (f64, f64) {
    let mut tcb1: f64 = 0.0;
    let mut tcb2: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraTdbtcb(tdb1, tdb2, &mut tcb1, &mut tcb2)
    }

    return (tcb1, tcb2);
}

/// Time scale transformation: Barycentric Dynamical Time, TDB, to Terrestrial
/// Time, TT.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/tdbtt.c)
pub fn Tdbtt(tdb1: f64, tdb2: f64, dtr: f64) -> (f64, f64) {
    let mut tt1: f64 = 0.0;
    let mut tt2: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraTdbtt(tdb1, tdb2, dtr, &mut tt1, &mut tt2)
    }

    return (tt1, tt2);
}

/// Time scale transformation: Terrestrial Time, TT, to International Atomic
/// Time, TAI.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/tttai.c)
pub fn Tttai(tt1: f64, tt2: f64) -> (f64, f64) {
    let mut tai1: f64 = 0.0;
    let mut tai2: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraTttai(tt1, tt2, &mut tai1, &mut tai2)
    }

    return (tai1, tai2);
}

/// Time scale transformation: Terrestrial Time, TT, to Geocentric Coordinate
/// Time, TCG.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/tttcg.c)
pub fn Tttcg(tt1: f64, tt2: f64) -> (f64, f64) {
    let mut tcg1: f64 = 0.0;
    let mut tcg2: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraTttcg(tt1, tt2, &mut tcg1, &mut tcg2)
    }

    return (tcg1, tcg2);
}

/// Time scale transformation: Terrestrial Time, TT, to Barycentric Dynamical
/// Time, TDB.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/tttdb.c)
pub fn Tttdb(tt1: f64, tt2: f64, dtr: f64) -> (f64, f64) {
    let mut tdb1: f64 = 0.0;
    let mut tdb2: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraTttdb(tt1, tt2, dtr, &mut tdb1, &mut tdb2)
    }

    return (tdb1, tdb2);
}

/// Time scale transformation: Terrestrial Time, TT, to Universal Time, UT1.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/ttut1.c)
pub fn Ttut1(tt1: f64, tt2: f64, dt: f64) -> (f64, f64) {
    let mut ut11: f64 = 0.0;
    let mut ut12: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraTtut1(tt1, tt2, dt, &mut ut11, &mut ut12)
    }

    return (ut11, ut12);
}

/// Time scale transformation: Universal Time, UT1, to International Atomic
/// Time, TAI.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/ut1tai.c)
pub fn Ut1tai(ut11: f64, ut12: f64, dta: f64) -> (f64, f64) {
    let mut tai1: f64 = 0.0;
    let mut tai2: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraUt1tai(ut11, ut12, dta, &mut tai1, &mut tai2)
    }

    return (tai1, tai2);
}

/// Time scale transformation: Universal Time, UT1, to Terrestrial Time, TT.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/ut1tt.c)
pub fn Ut1tt(ut11: f64, ut12: f64, dt: f64) -> (f64, f64) {
    let mut tt1: f64 = 0.0;
    let mut tt2: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraUt1tt(ut11, ut12, dt, &mut tt1, &mut tt2)
    }

    return (tt1, tt2);
}

/// Time scale transformation: Universal Time, UT1, to Coordinated Universal
/// Time, UTC.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/ut1utc.c)
pub fn Ut1utc(ut11: f64, ut12: f64, dut1: f64) -> Result<(f64, f64), ERFAError> {
    let mut utc1: f64 = 0.0;
    let mut utc2: f64 = 0.0;
    let err: i32;

    unsafe { err = eraUt1utc(ut11, ut12, dut1, &mut utc1, &mut utc2) }

    match err {
        -1 => Err(ERFAError::ERFABadDate),
        0 => Ok((utc1, utc2)),
        1 => Ok((utc1, utc2)),
        _ => unexpected_val_err!(eraUt1utc),
    }
}

/// Time scale transformation: Coordinated Universal Time, UTC, to International
/// Atomic Time, TAI.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/utctai.c)
pub fn Utctai(utc1: f64, utc2: f64) -> Result<(f64, f64), ERFAError> {
    let mut tai1: f64 = 0.0;
    let mut tai2: f64 = 0.0;
    let err: i32;

    unsafe { err = eraUtctai(utc1, utc2, &mut tai1, &mut tai2) }

    match err {
        -1 => Err(ERFAError::ERFABadDate),
        0 => Ok((tai1, tai2)),
        1 => Ok((tai1, tai2)),
        _ => unexpected_val_err!(eraUtctai),
    }
}

/// Time scale transformation: Coordinated Universal Time, UTC, to Universal
/// Time, UT1.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/utcut1.c)
pub fn Utcut1(utc1: f64, utc2: f64, dut1: f64) -> Result<(f64, f64), ERFAError> {
    let mut ut11: f64 = 0.0;
    let mut ut12: f64 = 0.0;
    let err: i32;

    unsafe { err = eraUtcut1(utc1, utc2, dut1, &mut ut11, &mut ut12) }

    match err {
        -1 => Err(ERFAError::ERFABadDate),
        0 => Ok((ut11, ut12)),
        1 => Ok((ut11, ut12)),
        _ => unexpected_val_err!(eraUtcut1),
    }
}
