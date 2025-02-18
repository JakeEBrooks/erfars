//! ERFA Star Catalog Functions

use crate::{raw::starcatalogs::*, unexpected_val_err, ERFAError};

pub fn Fk425(r1950: f64, d1950: f64, dr1950: f64, dd1950: f64, p1950: f64, v1950: f64) -> (f64, f64, f64, f64, f64, f64) {
    let mut r2000: f64 = 0.0;
    let mut d2000: f64 = 0.0;
    let mut dr2000: f64 = 0.0;
    let mut dd2000: f64 = 0.0;
    let mut p2000: f64 = 0.0;
    let mut v2000: f64 = 0.0;

    unsafe {
        eraFk425(r1950, d1950, dr1950, dd1950, p1950, v1950, &mut r2000, &mut d2000, &mut dr2000, &mut dd2000, &mut p2000, &mut v2000);
    }

    return (r2000, d2000, dr2000, dd2000, p2000, v2000)
}

pub fn Fk45z(r1950: f64, d1950: f64, bepoch: f64) -> (f64, f64) {
    let mut r2000: f64 = 0.0;
    let mut d2000: f64 = 0.0;

    unsafe {
        eraFk45z(r1950, d1950, bepoch, &mut r2000, &mut d2000);
    }

    return (r2000, d2000)
}

pub fn Fk524(r2000: f64, d2000: f64, dr2000: f64, dd2000: f64, p2000: f64, v2000: f64) -> (f64, f64, f64, f64, f64, f64) {
    let mut r1950: f64 = 0.0;
    let mut d1950: f64 = 0.0;
    let mut dr1950: f64 = 0.0;
    let mut dd1950: f64 = 0.0;
    let mut p1950: f64 = 0.0;
    let mut v1950: f64 = 0.0;

    unsafe {
        eraFk524(r2000, d2000, dr2000, dd2000, p2000, v2000, &mut r1950, &mut d1950, &mut dr1950, &mut dd1950, &mut p1950, &mut v1950);
    }

    return (r1950, d1950, dr1950, dd1950, p1950, v1950)
}

pub fn Fk52h(r5: f64, d5: f64, dr5: f64, dd5: f64, px5: f64, rv5: f64) -> (f64, f64, f64, f64, f64, f64) {
    let mut rh: f64 = 0.0;
    let mut dh: f64 = 0.0;
    let mut drh: f64 = 0.0;
    let mut ddh: f64 = 0.0;
    let mut pxh: f64 = 0.0;
    let mut rvh: f64 = 0.0;

    unsafe {
        eraFk52h(r5, d5, dr5, dd5, px5, rv5, &mut rh, &mut dh, &mut drh, &mut ddh, &mut pxh, &mut rvh);
    }

    return (rh, dh, drh, ddh, pxh, rvh)
}

pub fn Fk54z(r2000: f64, d2000: f64, bepoch: f64) -> (f64, f64, f64, f64) {
    let mut r1950: f64 = 0.0;
    let mut d1950: f64 = 0.0;
    let mut dr1950: f64 = 0.0;
    let mut dd1950: f64 = 0.0;

    unsafe {
        eraFk54z(r2000, d2000, bepoch, &mut r1950, &mut d1950, &mut dr1950, &mut dd1950);
    }

    return (r1950, d1950, dr1950, dd1950)
}

pub fn Fk5hip() -> ([f64; 9], [f64; 3]) {
    let mut r5h: [f64; 9] = [0.0; 9];
    let mut s5h: [f64; 3] = [0.0; 3];

    unsafe {
        eraFk5hip(&mut r5h, &mut s5h);
    }

    return (r5h, s5h)
}

pub fn Fk5hz(r5: f64, d5: f64, date1: f64, date2: f64) -> (f64, f64) {
    let mut rh: f64 = 0.0;
    let mut dh: f64 = 0.0;

    unsafe {
        eraFk5hz(r5, d5, date1, date2, &mut rh, &mut dh);
    }

    return (rh, dh)
}

pub fn H2fk5(rh: f64, dh: f64, drh: f64, ddh: f64, pxh: f64, rvh: f64) -> (f64, f64, f64, f64, f64, f64) {
    let mut r5: f64 = 0.0;
    let mut d5: f64 = 0.0;
    let mut dr5: f64 = 0.0;
    let mut dd5: f64 = 0.0;
    let mut px5: f64 = 0.0;
    let mut rv5: f64 = 0.0;

    unsafe {
        eraH2fk5(rh, dh, drh, ddh, pxh, rvh, &mut r5, &mut d5, &mut dr5, &mut dd5, &mut px5, &mut rv5);
    }

    return (r5, d5, dr5, dd5, px5, rv5)
}

pub fn Hfk5z(rh: f64, dh: f64, date1: f64, date2: f64) -> (f64, f64, f64, f64) {
    let mut r5: f64 = 0.0;
    let mut d5: f64 = 0.0;
    let mut dr5: f64 = 0.0;
    let mut dd5: f64 = 0.0;

    unsafe {
        eraHfk5z(rh, dh, date1, date2, &mut r5, &mut d5, &mut dr5, &mut dd5);
    }

    return (r5, d5, dr5, dd5)
}

pub fn Starpm(ra1: f64, dec1: f64, pmr1: f64, pmd1: f64, px1: f64, rv1: f64, ep1a: f64, ep1b: f64, ep2a: f64, ep2b: f64) -> Result<(f64, f64, f64, f64, f64, f64), ERFAError> {
    let mut ra2: f64 = 0.0;
    let mut dec2: f64 = 0.0;
    let mut pmr2: f64 = 0.0;
    let mut pmd2: f64 = 0.0;
    let mut px2: f64 = 0.0;
    let mut rv2: f64 = 0.0;
    let err: i32;

    unsafe {
        err = eraStarpm(ra1, dec1, pmr1, pmd1, px1, rv1, ep1a, ep1b, ep2a, ep2b, &mut ra2, &mut dec2, &mut pmr2, &mut pmd2, &mut px2, &mut rv2)
    }

    match err {
        0 => Ok((ra2, dec2, pmr2, pmd2, px2, rv2)),
        1 => Ok((ra2, dec2, pmr2, pmd2, px2, rv2)),
        2 => Ok((ra2, dec2, pmr2, pmd2, px2, rv2)),
        4 => Ok((ra2, dec2, pmr2, pmd2, px2, rv2)),
        -1 => Err(ERFAError::ERFAInternalError),
        _ => unexpected_val_err!(eraStarpm)
    }
}