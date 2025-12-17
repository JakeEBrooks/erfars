//! ERFA Earth Rotation Angle and Sidereal Time Functions

use crate::raw::rotationtime::*;

/// The equation of the equinoxes, compatible with IAU 2000 resolutions, given
/// the nutation in longitude and the mean obliquity.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/ee00.c)
pub fn Ee00(date1: f64, date2: f64, epsa: f64, dpsi: f64) -> f64 {
    return unsafe { eraEe00(date1, date2, epsa, dpsi) };
}

/// Equation of the equinoxes, compatible with IAU 2000 resolutions.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/ee00a.c)
pub fn Ee00a(date1: f64, date2: f64) -> f64 {
    return unsafe { eraEe00a(date1, date2) };
}

/// Equation of the equinoxes, compatible with IAU 2000 resolutions but using
/// the truncated nutation model IAU 2000B.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/ee00b.c)
pub fn Ee00b(date1: f64, date2: f64) -> f64 {
    return unsafe { eraEe00b(date1, date2) };
}

/// Equation of the equinoxes, compatible with IAU 2000 resolutions and IAU
/// 2006/2000A precession-nutation.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/ee06a.c)
pub fn Ee06a(date1: f64, date2: f64) -> f64 {
    return unsafe { eraEe06a(date1, date2) };
}

/// Equation of the equinoxes complementary terms, consistent with IAU 2000
/// resolutions.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/eect00.c)
pub fn Eect00(date1: f64, date2: f64) -> f64 {
    return unsafe { eraEect00(date1, date2) };
}

/// Equation of the equinoxes, IAU 1994 model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/eqeq94.c)
pub fn Eqeq94(date1: f64, date2: f64) -> f64 {
    return unsafe { eraEqeq94(date1, date2) };
}

/// Earth rotation angle (IAU 2000 model).
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/era00.c)
pub fn Era00(dj1: f64, dj2: f64) -> f64 {
    return unsafe { eraEra00(dj1, dj2) };
}

/// Greenwich mean sidereal time (model consistent with IAU 2000 resolutions).
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/gmst00.c)
pub fn Gmst00(uta: f64, utb: f64, tta: f64, ttb: f64) -> f64 {
    return unsafe { eraGmst00(uta, utb, tta, ttb) };
}

/// Greenwich mean sidereal time (consistent with IAU 2006 precession).
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/gmst06.c)
pub fn Gmst06(uta: f64, utb: f64, tta: f64, ttb: f64) -> f64 {
    return unsafe { eraGmst06(uta, utb, tta, ttb) };
}

/// Universal Time to Greenwich mean sidereal time (IAU 1982 model).
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/gmst82.c)
pub fn Gmst82(dj1: f64, dj2: f64) -> f64 {
    return unsafe { eraGmst82(dj1, dj2) };
}

/// Greenwich apparent sidereal time (consistent with IAU 2000 resolutions).
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/gst00a.c)
pub fn Gst00a(uta: f64, utb: f64, tta: f64, ttb: f64) -> f64 {
    return unsafe { eraGst00a(uta, utb, tta, ttb) };
}

/// Greenwich apparent sidereal time (consistent with IAU 2000 resolutions but
/// using the truncated nutation model IAU 2000B).
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/gst00b.c)
pub fn Gst00b(uta: f64, utb: f64) -> f64 {
    return unsafe { eraGst00b(uta, utb) };
}

/// Greenwich apparent sidereal time, IAU 2006, given the NPB matrix.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/gst06.c)
pub fn Gst06(uta: f64, utb: f64, tta: f64, ttb: f64, rnpb: &[f64; 9]) -> f64 {
    return unsafe { eraGst06(uta, utb, tta, ttb, rnpb) };
}

/// Greenwich apparent sidereal time (consistent with IAU 2000 and 2006
/// resolutions).
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/gst06a.c)
pub fn Gst06a(uta: f64, utb: f64, tta: f64, ttb: f64) -> f64 {
    return unsafe { eraGst06a(uta, utb, tta, ttb) };
}

/// Greenwich apparent sidereal time (consistent with IAU 1982/94 resolutions).
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/gst94.c)
pub fn Gst94(uta: f64, utb: f64) -> f64 {
    return unsafe { eraGst94(uta, utb) };
}
