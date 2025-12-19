//! ERFA Gnomonic Projection Functions

use crate::{ERFAResult, raw::gnomonic::*, unexpected_val_err};

/// In the tangent plane projection, given the rectangular coordinates of a star
/// and its spherical coordinates, determine the spherical coordinates of the
/// tangent point.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/tpors.c)
pub fn Tpors(xi: f64, eta: f64, a: f64, b: f64) -> ERFAResult<(f64, f64, f64, f64)> {
    let mut a01: f64 = 0.0;
    let mut b01: f64 = 0.0;
    let mut a02: f64 = 0.0;
    let mut b02: f64 = 0.0;

    let err: i32;
    unsafe { err = eraTpors(xi, eta, a, b, &mut a01, &mut b01, &mut a02, &mut b02) }

    match err {
        0 => Ok(((a01, b01, a02, b02), 0)),
        1 => Ok(((a01, b01, a02, b02), 1)),
        2 => Ok(((a01, b01, a02, b02), 2)),
        _ => unexpected_val_err!(eraTpors),
    }
}

/// In the tangent plane projection, given the rectangular coordinates of a star
/// and its direction cosines, determine the direction cosines of the tangent
/// point.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/tporv.c)
pub fn Tporv(xi: f64, eta: f64, v: &[f64; 3]) -> ERFAResult<([f64; 3], [f64; 3])> {
    let mut v01: [f64; 3] = [0.0; 3];
    let mut v02: [f64; 3] = [0.0; 3];

    let err: i32;
    unsafe { err = eraTporv(xi, eta, v, &mut v01, &mut v02) }

    match err {
        0 => Ok(((v01, v02), 0)),
        1 => Ok(((v01, v02), 1)),
        2 => Ok(((v01, v02), 2)),
        _ => unexpected_val_err!(eraTporv),
    }
}

/// In the tangent plane projection, given the star's rectangular coordinates
/// and the spherical coordinates of the tangent point, solve for the spherical
/// coordinates of the star.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/tpsts.c)
pub fn Tpsts(xi: f64, eta: f64, a0: f64, b0: f64) -> (f64, f64) {
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;

    unsafe {
        eraTpsts(xi, eta, a0, b0, &mut a, &mut b);
    }

    return (a, b);
}

/// In the tangent plane projection, given the star's rectangular coordinates
/// and the direction cosines of the tangent point, solve for the direction
/// cosines of the star.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/tpstv.c)
pub fn Tpstv(xi: f64, eta: f64, v0: &[f64; 3]) -> [f64; 3] {
    let mut v: [f64; 3] = [0.0; 3];

    unsafe {
        eraTpstv(xi, eta, v0, &mut v);
    }

    return v;
}

/// In the tangent plane projection, given celestial spherical coordinates for a
/// star and the tangent point, solve for the star's rectangular coordinates in
/// the tangent plane.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/tpxes.c)
pub fn Tpxes(a: f64, b: f64, a0: f64, b0: f64) -> ERFAResult<(f64, f64)> {
    let mut xi: f64 = 0.0;
    let mut eta: f64 = 0.0;
    let err: i32;

    unsafe {
        err = eraTpxes(a, b, a0, b0, &mut xi, &mut eta);
    }

    match err {
        0 => Ok(((xi, eta), 0)),
        1 => Ok(((xi, eta), 1)),
        2 => Ok(((xi, eta), 2)),
        3 => Ok(((xi, eta), 3)),
        _ => unexpected_val_err!(eraTpxes),
    }
}

/// In the tangent plane projection, given celestial direction cosines for a
/// star and the tangent point, solve for the star's rectangular coordinates in
/// the tangent plane.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/tpxev.c)
pub fn Tpxev(v: &[f64; 3], v0: &[f64; 3]) -> ERFAResult<(f64, f64)> {
    let mut xi: f64 = 0.0;
    let mut eta: f64 = 0.0;
    let err: i32;

    unsafe {
        err = eraTpxev(v, v0, &mut xi, &mut eta);
    }

    match err {
        0 => Ok(((xi, eta), 0)),
        1 => Ok(((xi, eta), 1)),
        2 => Ok(((xi, eta), 2)),
        3 => Ok(((xi, eta), 3)),
        _ => unexpected_val_err!(eraTpxev),
    }
}
