//! ERFA Ecliptic Coordinates Functions

use crate::raw::eclipticcoordinates::*;

/// Transformation from ecliptic coordinates (mean equinox and ecliptic of date)
/// to ICRS RA,Dec, using the IAU 2006 precession model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/eceq06.c)
pub fn Eceq06(date1: f64, date2: f64, dl: f64, db: f64) -> (f64, f64) {
    let mut dr: f64 = 0.0;
    let mut dd: f64 = 0.0;

    unsafe {
        eraEceq06(date1, date2, dl, db, &mut dr, &mut dd);
    }

    return (dr, dd);
}

/// ICRS equatorial to ecliptic rotation matrix, IAU 2006.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/ecm06.c)
pub fn Ecm06(date1: f64, date2: f64, rm: &mut [f64; 9]) {
    unsafe {
        eraEcm06(date1, date2, rm);
    }
}

/// Transformation from ICRS equatorial coordinates to ecliptic coordinates
/// (mean equinox and ecliptic of date) using IAU 2006 precession model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/eqec06.c)
pub fn Eqec06(date1: f64, date2: f64, dr: f64, dd: f64) -> (f64, f64) {
    let mut dl: f64 = 0.0;
    let mut db: f64 = 0.0;

    unsafe {
        eraEqec06(date1, date2, dr, dd, &mut dl, &mut db);
    }

    return (dl, db);
}

/// Transformation from ecliptic coordinates (mean equinox and ecliptic of date)
/// to ICRS RA,Dec, using a long-term precession model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/lteceq.c)
pub fn Lteceq(epj: f64, dl: f64, db: f64) -> (f64, f64) {
    let mut dr: f64 = 0.0;
    let mut dd: f64 = 0.0;

    unsafe {
        eraLteceq(epj, dl, db, &mut dr, &mut dd);
    }

    return (dr, dd);
}

/// ICRS equatorial to ecliptic rotation matrix, long-term.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/ltecm.c)
pub fn Ltecm(epj: f64, rm: &mut [f64; 9]) {
    unsafe {
        eraLtecm(epj, rm);
    }
}

/// Transformation from ICRS RA,Dec to ecliptic coordinates (mean equinox and
/// ecliptic of date), using a long-term precession model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/lteqec.c)
pub fn Lteqec(epj: f64, dr: f64, dd: f64) -> (f64, f64) {
    let mut dl: f64 = 0.0;
    let mut db: f64 = 0.0;

    unsafe {
        eraLteqec(epj, dr, dd, &mut dl, &mut db);
    }

    return (dl, db);
}
