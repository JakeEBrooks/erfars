use crate::raw::galacticcoordinates::*;

pub fn G2icrs(dl: f64, db: f64) -> (f64, f64) {
    let mut dr: f64 = 0.0;
    let mut dd: f64 = 0.0;
    unsafe{ eraG2icrs(dl, db, &mut dr, &mut dd) }
    return (dr, dd)
}

pub fn Icrs2g(dr: f64, dd: f64) -> (f64, f64) {
    let mut dl: f64 = 0.0;
    let mut db: f64 = 0.0;
    unsafe{ eraIcrs2g(dr, dd, &mut dl, &mut db) }
    return (dl, db)
}