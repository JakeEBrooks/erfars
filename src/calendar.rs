//! ERFA Calendar Functions

use crate::{raw::calendar::*, unexpected_val_err, ERFAError};

pub fn Cal2jd(year: i32, month: i32, day: i32) -> Result<(f64, f64), ERFAError> {
    let mut djm0: f64 = 0.0;
    let mut djm: f64 = 0.0;
    let err: i32;
    unsafe { err = eraCal2jd(year, month, day, &mut djm0, &mut djm) };

    match err {
        0 => Ok((djm0, djm)),
        -1 => Err(ERFAError::ERFABadYear),
        -2 => Err(ERFAError::ERFABadMonth),
        -3 => Err(ERFAError::ERFABadDay),
        _ => unexpected_val_err!(eraCal2dj),
    }
}

pub fn Epb(jd0: f64, jd1: f64) -> f64 {
    return unsafe { eraEpb(jd0, jd1) };
}

pub fn Epb2jd(epoch: f64) -> (f64, f64) {
    let mut djm0: f64 = 0.0;
    let mut djm: f64 = 0.0;
    unsafe {
        eraEpb2jd(epoch, &mut djm0, &mut djm);
    }

    return (djm0, djm);
}

pub fn Epj(jd0: f64, jd1: f64) -> f64 {
    return unsafe { eraEpj(jd0, jd1) };
}

pub fn Epj2jd(epoch: f64) -> (f64, f64) {
    let mut djm0: f64 = 0.0;
    let mut djm: f64 = 0.0;
    unsafe {
        eraEpj2jd(epoch, &mut djm0, &mut djm);
    }

    return (djm0, djm);
}

pub fn Jd2cal(jd0: f64, jd1: f64) -> Result<(i32, i32, i32, f64), ERFAError> {
    let mut iy: i32 = 0;
    let mut im: i32 = 0;
    let mut id: i32 = 0;
    let mut fd: f64 = 0.0;
    let err: i32;
    unsafe { err = eraJd2cal(jd0, jd1, &mut iy, &mut im, &mut id, &mut fd) };

    match err {
        0 => Ok((iy, im, id, fd)),
        -1 => Err(ERFAError::ERFABadDate),
        _ => unexpected_val_err!(eraJd2cal),
    }
}

pub fn Jdcalf(jd0: f64, jd1: f64, ndp: i32) -> Result<(i32, i32, i32, i32), ERFAError> {
    let mut iymdf: [i32; 4] = [0, 0, 0, 0];
    let err: i32;
    unsafe { err = eraJdcalf(ndp, jd0, jd1, &mut iymdf) };

    match err {
        0 => Ok((iymdf[0], iymdf[1], iymdf[2], iymdf[3])),
        // If the ndp warning is returned, just ignore the warning
        1 => Ok((iymdf[0], iymdf[1], iymdf[2], iymdf[3])),
        -1 => Err(ERFAError::ERFABadDate),
        _ => unexpected_val_err!(eraJdcalf),
    }
}
