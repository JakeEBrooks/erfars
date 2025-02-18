//! ERFA Ecliptic Coordinates Functions

use crate::raw::eclipticcoordinates::*;

pub fn Eceq06(date1: f64, date2: f64, dl: f64, db: f64) -> (f64, f64) {
    let mut dr: f64 = 0.0;
    let mut dd: f64 = 0.0;

    unsafe {
        eraEceq06(date1, date2, dl, db, &mut dr, &mut dd);
    }

    return (dr, dd)
}

pub fn Ecm06(date1: f64, date2: f64, rm: &mut [f64; 9]) {
    unsafe {
        eraEcm06(date1, date2, rm);
    }
}

pub fn Eqec06(date1: f64, date2: f64, dr: f64, dd: f64) -> (f64, f64) {
    let mut dl: f64 = 0.0;
    let mut db: f64 = 0.0;

    unsafe {
        eraEqec06(date1, date2, dr, dd, &mut dl, &mut db);
    }

    return (dl, db)
}

pub fn Lteceq(epj: f64, dl: f64, db: f64) -> (f64, f64) {
    let mut dr: f64 = 0.0;
    let mut dd: f64 = 0.0;

    unsafe {
        eraLteceq(epj, dl, db, &mut dr, &mut dd);
    }

    return (dr, dd)
}

pub fn Ltecm(epj: f64, rm: &mut [f64; 9]) {
    unsafe {
        eraLtecm(epj, rm);
    }
}

pub fn Lteqec(epj: f64, dr: f64, dd: f64) -> (f64, f64) {
    let mut dl: f64 = 0.0;
    let mut db: f64 = 0.0;

    unsafe {
        eraLteqec(epj, dr, dd, &mut dl, &mut db);
    }

    return (dl, db)
}