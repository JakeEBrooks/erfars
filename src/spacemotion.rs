use crate::{raw::spacemotion::*, unexpected_val_err, ERFAError};

pub fn Pvstar(pv: &[f64; 6]) -> Result<(f64, f64, f64, f64, f64, f64), ERFAError> {
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    let mut pmr: f64 = 0.0;
    let mut pmd: f64 = 0.0;
    let mut px: f64 = 0.0;
    let mut rv: f64 = 0.0;
    let err: i32;
    unsafe { err = eraPvstar(pv, &mut ra, &mut dec, &mut pmr, &mut pmd, &mut px, &mut rv) }

    match err {
        0 => Ok((ra, dec, pmr, pmd, px, rv)),
        -1 => Err(ERFAError::ERFABadSpeed),
        -2 => Err(ERFAError::ERFABadPosition),
        _ => unexpected_val_err!(eraPvstar),
    }
}

pub fn Starpv(ra: f64, dec: f64, pmr: f64, pmd: f64, px: f64, rv: f64) -> [f64; 6] {
    let mut pv: [f64; 6] = [0.0; 6];
    let err: i32;
    unsafe { err = eraStarpv(ra, dec, pmr, pmd, px, rv, &mut pv) }

    match err {
        0 => pv,
        1 => pv,
        2 => pv,
        4 => pv,
        _ => unexpected_val_err!(eraStarpv),
    }
}
