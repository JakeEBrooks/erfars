//! ERFA Geodetic/Geocentric Functions

use crate::raw::geodeticgeocentric::*;
use crate::{ERFAError, unexpected_val_err};

///  Earth reference ellipsoids.
///
///  Given:
///     n    int         ellipsoid identifier (Note 1)
///
///  Returned:
///     a    double      equatorial radius (meters, Note 2)
///     f    double      flattening (Note 2)
///
///  Returned (function value):
///          int         status:  0 = OK
///                              -1 = illegal identifier (Note 3)
///
///  Notes:
///
///  1) The identifier n is a number that specifies the choice of
///     reference ellipsoid.  The following are supported:
///
///        n    ellipsoid
///
///        1     ERFA_WGS84
///        2     ERFA_GRS80
///        3     ERFA_WGS72
///
///     The n value has no significance outside the ERFA software.  For
///     convenience, symbols ERFA_WGS84 etc. are defined in erfam.h.
///
///  2) The ellipsoid parameters are returned in the form of equatorial
///     radius in meters (a) and flattening (f).  The latter is a number
///     around 0.00335, i.e. around 1/298.
///
///  3) For the case where an unsupported n value is supplied, zero a and
///     f are returned, as well as error status.
///
///  References:
///
///     Department of Defense World Geodetic System 1984, National
///     Imagery and Mapping Agency Technical Report 8350.2, Third
///     Edition, p3-2.
///
///     Moritz, H., Bull. Geodesique 66-2, 187 (1992).
///
///     The Department of Defense World Geodetic System 1972, World
///     Geodetic System Committee, May 1974.
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992),
///     p220.
pub fn Eform(n: i32) -> Result<(f64, f64), ERFAError> {
    let mut a: f64 = 0.0;
    let mut f: f64 = 0.0;
    let err: i32;
    unsafe { err = eraEform(n, &mut a, &mut f) }
    match err {
        0 => Ok((a, f)),
        -1 => Err(ERFAError::ERFABadInputValue),
        _ => unexpected_val_err!(eraEform),
    }
}

///  Transform geocentric coordinates to geodetic using the specified
///  reference ellipsoid.
///
///  Given:
///     n       int        ellipsoid identifier (Note 1)
///     xyz     double[3]  geocentric vector (Note 2)
///
///  Returned:
///     elong   double     longitude (radians, east +ve, Note 3)
///     phi     double     latitude (geodetic, radians, Note 3)
///     height  double     height above ellipsoid (geodetic, Notes 2,3)
///
///  Returned (function value):
///            int         status:  0 = OK
///                                -1 = illegal identifier (Note 3)
///                                -2 = internal error (Note 3)
///
///  Notes:
///
///  1) The identifier n is a number that specifies the choice of
///     reference ellipsoid.  The following are supported:
///
///        n    ellipsoid
///
///        1     ERFA_WGS84
///        2     ERFA_GRS80
///        3     ERFA_WGS72
///
///     The n value has no significance outside the ERFA software.  For
///     convenience, symbols ERFA_WGS84 etc. are defined in erfam.h.
///
///  2) The geocentric vector (xyz, given) and height (height, returned)
///     are in meters.
///
///  3) An error status -1 means that the identifier n is illegal.  An
///     error status -2 is theoretically impossible.  In all error cases,
///     all three results are set to -1e9.
///
///  4) The inverse transformation is performed in the function eraGd2gc.
///
///  Called:
///     eraEform     Earth reference ellipsoids
///     eraGc2gde    geocentric to geodetic transformation, general
pub fn Gc2gd(n: i32, xyz: &[f64; 3]) -> Result<(f64, f64, f64), ERFAError> {
    let mut elong: f64 = 0.0;
    let mut phi: f64 = 0.0;
    let mut height: f64 = 0.0;
    let err: i32;
    unsafe { err = eraGc2gd(n, xyz, &mut elong, &mut phi, &mut height) }

    match err {
        0 => Ok((elong, phi, height)),
        -1 => Err(ERFAError::ERFABadInputValue),
        -2 => Err(ERFAError::ERFAInternalError),
        _ => unexpected_val_err!(eraGc2gd),
    }
}

///  Transform geocentric coordinates to geodetic for a reference
///  ellipsoid of specified form.
///
///  Given:
///     a       double     equatorial radius (Notes 2,4)
///     f       double     flattening (Note 3)
///     xyz     double[3]  geocentric vector (Note 4)
///
///  Returned:
///     elong   double     longitude (radians, east +ve)
///     phi     double     latitude (geodetic, radians)
///     height  double     height above ellipsoid (geodetic, Note 4)
///
///  Returned (function value):
///             int        status:  0 = OK
///                                -1 = illegal f
///                                -2 = illegal a
///
///  Notes:
///
///  1) This function is based on the GCONV2H Fortran subroutine by
///     Toshio Fukushima (see reference).
///
///  2) The equatorial radius, a, can be in any units, but meters is
///     the conventional choice.
///
///  3) The flattening, f, is (for the Earth) a value around 0.00335,
///     i.e. around 1/298.
///
///  4) The equatorial radius, a, and the geocentric vector, xyz,
///     must be given in the same units, and determine the units of
///     the returned height, height.
///
///  5) If an error occurs (status < 0), elong, phi and height are
///     unchanged.
///
///  6) The inverse transformation is performed in the function
///     eraGd2gce.
///
///  7) The transformation for a standard ellipsoid (such as ERFA_WGS84) can
///     more conveniently be performed by calling eraGc2gd, which uses a
///     numerical code to identify the required A and F values.
///
///  Reference:
///
///     Fukushima, T., "Transformation from Cartesian to geodetic
///     coordinates accelerated by Halley's method", J.Geodesy (2006)
///     79: 689-693
pub fn Gc2gde(a: f64, f: f64, xyz: &[f64; 3]) -> Result<(f64, f64, f64), ERFAError> {
    let mut elong: f64 = 0.0;
    let mut phi: f64 = 0.0;
    let mut height: f64 = 0.0;
    let err: i32;
    unsafe { err = eraGc2gde(a, f, xyz, &mut elong, &mut phi, &mut height) }

    match err {
        0 => Ok((elong, phi, height)),
        -1 => Err(ERFAError::ERFABadInputValue),
        -2 => Err(ERFAError::ERFABadInputValue),
        _ => unexpected_val_err!(eraGc2gde),
    }
}

///  Transform geodetic coordinates to geocentric using the specified
///  reference ellipsoid.
///
///  Given:
///     n       int        ellipsoid identifier (Note 1)
///     elong   double     longitude (radians, east +ve, Note 3)
///     phi     double     latitude (geodetic, radians, Note 3)
///     height  double     height above ellipsoid (geodetic, Notes 2,3)
///
///  Returned:
///     xyz     double[3]  geocentric vector (Note 2)
///
///  Returned (function value):
///             int        status:  0 = OK
///                                -1 = illegal identifier (Note 3)
///                                -2 = illegal case (Note 3)
///
///  Notes:
///
///  1) The identifier n is a number that specifies the choice of
///     reference ellipsoid.  The following are supported:
///
///        n    ellipsoid
///
///        1     ERFA_WGS84
///        2     ERFA_GRS80
///        3     ERFA_WGS72
///
///     The n value has no significance outside the ERFA software.  For
///     convenience, symbols ERFA_WGS84 etc. are defined in erfam.h.
///
///  2) The height (height, given) and the geocentric vector (xyz,
///     returned) are in meters.
///
///  3) No validation is performed on the arguments elong, phi and
///     height.  An error status -1 means that the identifier n is
///     illegal.  An error status -2 protects against cases that would
///     lead to arithmetic exceptions.  In all error cases, xyz is set
///     to zeros.
///
///  4) The inverse transformation is performed in the function eraGc2gd.
///
///  Called:
///     eraEform     Earth reference ellipsoids
///     eraGd2gce    geodetic to geocentric transformation, general
///     eraZp        zero p-vector
pub fn Gd2gc(n: i32, elong: f64, phi: f64, height: f64) -> Result<[f64; 3], ERFAError> {
    let mut xyz: [f64; 3] = [0.0; 3];
    let err: i32;
    unsafe { err = eraGd2gc(n, elong, phi, height, &mut xyz) }

    match err {
        0 => Ok(xyz),
        -1 => Err(ERFAError::ERFABadInputValue),
        -2 => Err(ERFAError::ERFABadInputValue),
        _ => unexpected_val_err!(eraGd2gc),
    }
}

///  Transform geodetic coordinates to geocentric for a reference
///  ellipsoid of specified form.
///
///  Given:
///     a       double     equatorial radius (Notes 1,3,4)
///     f       double     flattening (Notes 2,4)
///     elong   double     longitude (radians, east +ve, Note 4)
///     phi     double     latitude (geodetic, radians, Note 4)
///     height  double     height above ellipsoid (geodetic, Notes 3,4)
///
///  Returned:
///     xyz     double[3]  geocentric vector (Note 3)
///
///  Returned (function value):
///             int        status:  0 = OK
///                                -1 = illegal case (Note 4)
///  Notes:
///
///  1) The equatorial radius, a, can be in any units, but meters is
///     the conventional choice.
///
///  2) The flattening, f, is (for the Earth) a value around 0.00335,
///     i.e. around 1/298.
///
///  3) The equatorial radius, a, and the height, height, must be
///     given in the same units, and determine the units of the
///     returned geocentric vector, xyz.
///
///  4) No validation is performed on individual arguments.  The error
///     status -1 protects against (unrealistic) cases that would lead
///     to arithmetic exceptions.  If an error occurs, xyz is unchanged.
///
///  5) The inverse transformation is performed in the function
///     eraGc2gde.
///
///  6) The transformation for a standard ellipsoid (such as ERFA_WGS84) can
///     more conveniently be performed by calling eraGd2gc,  which uses a
///     numerical code to identify the required a and f values.
///
///  References:
///
///     Green, R.M., Spherical Astronomy, Cambridge University Press,
///     (1985) Section 4.5, p96.
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992),
///     Section 4.22, p202.
pub fn Gd2gce(a: f64, f: f64, elong: f64, phi: f64, height: f64) -> Result<[f64; 3], ERFAError> {
    let mut xyz: [f64; 3] = [0.0; 3];
    let err: i32;
    unsafe { err = eraGd2gce(a, f, elong, phi, height, &mut xyz) }

    match err {
        0 => Ok(xyz),
        -1 => Err(ERFAError::ERFABadInputValue),
        _ => unexpected_val_err!(eraGd2gce),
    }
}
