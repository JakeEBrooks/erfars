//! ERFA Gnomonic Projection Functions

use crate::{ERFAError, raw::gnomonic::*, unexpected_val_err};

///  In the tangent plane projection, given the rectangular coordinates
///  of a star and its spherical coordinates, determine the spherical
///  coordinates of the tangent point.
///
///  Given:
///     xi,eta     double  rectangular coordinates of star image (Note 2)
///     a,b        double  star's spherical coordinates (Note 3)
///
///  Returned:
///     *a01,*b01  double  tangent point's spherical coordinates, Soln. 1
///     *a02,*b02  double  tangent point's spherical coordinates, Soln. 2
///
///  Returned (function value):
///                int     number of solutions:
///                        0 = no solutions returned (Note 5)
///                        1 = only the first solution is useful (Note 6)
///                        2 = both solutions are useful (Note 6)
///
///  Notes:
///
///  1) The tangent plane projection is also called the "gnomonic
///     projection" and the "central projection".
///
///  2) The eta axis points due north in the adopted coordinate system.
///     If the spherical coordinates are observed (RA,Dec), the tangent
///     plane coordinates (xi,eta) are conventionally called the
///     "standard coordinates".  If the spherical coordinates are with
///     respect to a right-handed triad, (xi,eta) are also right-handed.
///     The units of (xi,eta) are, effectively, radians at the tangent
///     point.
///
///  3) All angular arguments are in radians.
///
///  4) The angles a01 and a02 are returned in the range 0-2pi.  The
///     angles b01 and b02 are returned in the range +/-pi, but in the
///     usual, non-pole-crossing, case, the range is +/-pi/2.
///
///  5) Cases where there is no solution can arise only near the poles.
///     For example, it is clearly impossible for a star at the pole
///     itself to have a non-zero xi value, and hence it is meaningless
///     to ask where the tangent point would have to be to bring about
///     this combination of xi and dec.
///
///  6) Also near the poles, cases can arise where there are two useful
///     solutions.  The return value indicates whether the second of the
///     two solutions returned is useful;  1 indicates only one useful
///     solution, the usual case.
///
///  7) The basis of the algorithm is to solve the spherical triangle PSC,
///     where P is the north celestial pole, S is the star and C is the
///     tangent point.  The spherical coordinates of the tangent point are
///     [a0,b0];  writing rho^2 = (xi^2+eta^2) and r^2 = (1+rho^2), side c
///     is then (pi/2-b), side p is sqrt(xi^2+eta^2) and side s (to be
///     found) is (pi/2-b0).  Angle C is given by sin(C) = xi/rho and
///     cos(C) = eta/rho.  Angle P (to be found) is the longitude
///     difference between star and tangent point (a-a0).
///
///  8) This function is a member of the following set:
///
///         spherical      vector         solve for
///
///         eraTpxes      eraTpxev         xi,eta
///         eraTpsts      eraTpstv          star
///       > eraTpors <    eraTporv         origin
///
///  Called:
///     eraAnp       normalize angle into range 0 to 2pi
///
///  References:
///
///     Calabretta M.R. & Greisen, E.W., 2002, "Representations of
///     celestial coordinates in FITS", Astron.Astrophys. 395, 1077
///
///     Green, R.M., "Spherical Astronomy", Cambridge University Press,
///     1987, Chapter 13.
pub fn Tpors(xi: f64, eta: f64, a: f64, b: f64) -> (f64, f64, f64, f64) {
    let mut a01: f64 = 0.0;
    let mut b01: f64 = 0.0;
    let mut a02: f64 = 0.0;
    let mut b02: f64 = 0.0;

    unsafe { _ = eraTpors(xi, eta, a, b, &mut a01, &mut b01, &mut a02, &mut b02) }

    return (a01, b01, a02, b02);
}

///  In the tangent plane projection, given the rectangular coordinates
///  of a star and its direction cosines, determine the direction
///  cosines of the tangent point.
///
///  Given:
///     xi,eta   double    rectangular coordinates of star image (Note 2)
///     v        double[3] star's direction cosines (Note 3)
///
///  Returned:
///     v01      double[3] tangent point's direction cosines, Solution 1
///     v02      double[3] tangent point's direction cosines, Solution 2
///
///  Returned (function value):
///                int     number of solutions:
///                        0 = no solutions returned (Note 4)
///                        1 = only the first solution is useful (Note 5)
///                        2 = both solutions are useful (Note 5)
///
///  Notes:
///
///  1) The tangent plane projection is also called the "gnomonic
///     projection" and the "central projection".
///
///  2) The eta axis points due north in the adopted coordinate system.
///     If the direction cosines represent observed (RA,Dec), the tangent
///     plane coordinates (xi,eta) are conventionally called the
///     "standard coordinates".  If the direction cosines are with
///     respect to a right-handed triad, (xi,eta) are also right-handed.
///     The units of (xi,eta) are, effectively, radians at the tangent
///     point.
///
///  3) The vector v must be of unit length or the result will be wrong.
///
///  4) Cases where there is no solution can arise only near the poles.
///     For example, it is clearly impossible for a star at the pole
///     itself to have a non-zero xi value, and hence it is meaningless
///     to ask where the tangent point would have to be.
///
///  5) Also near the poles, cases can arise where there are two useful
///     solutions.  The return value indicates whether the second of the
///     two solutions returned is useful;  1 indicates only one useful
///     solution, the usual case.
///
///  6) The basis of the algorithm is to solve the spherical triangle
///     PSC, where P is the north celestial pole, S is the star and C is
///     the tangent point.  Calling the celestial spherical coordinates
///     of the star and tangent point (a,b) and (a0,b0) respectively, and
///     writing rho^2 = (xi^2+eta^2) and r^2 = (1+rho^2), and
///     transforming the vector v into (a,b) in the normal way, side c is
///     then (pi/2-b), side p is sqrt(xi^2+eta^2) and side s (to be
///     found) is (pi/2-b0), while angle C is given by sin(C) = xi/rho
///     and cos(C) = eta/rho;  angle P (to be found) is (a-a0).  After
///     solving the spherical triangle, the result (a0,b0) can be
///     expressed in vector form as v0.
///
///  7) This function is a member of the following set:
///
///         spherical      vector         solve for
///
///         eraTpxes      eraTpxev         xi,eta
///         eraTpsts      eraTpstv          star
///         eraTpors    > eraTporv <       origin
///
///  References:
///
///     Calabretta M.R. & Greisen, E.W., 2002, "Representations of
///     celestial coordinates in FITS", Astron.Astrophys. 395, 1077
///
///     Green, R.M., "Spherical Astronomy", Cambridge University Press,
///     1987, Chapter 13.
pub fn Tporv(xi: f64, eta: f64, v: &[f64; 3]) -> ([f64; 3], [f64; 3]) {
    let mut v01: [f64; 3] = [0.0; 3];
    let mut v02: [f64; 3] = [0.0; 3];

    unsafe { _ = eraTporv(xi, eta, v, &mut v01, &mut v02) }

    return (v01, v02);
}

///  In the tangent plane projection, given the star's rectangular
///  coordinates and the spherical coordinates of the tangent point,
///  solve for the spherical coordinates of the star.
///
///  Given:
///     xi,eta    double  rectangular coordinates of star image (Note 2)
///     a0,b0     double  tangent point's spherical coordinates
///
///  Returned:
///     *a,*b     double  star's spherical coordinates
///
///  1) The tangent plane projection is also called the "gnomonic
///     projection" and the "central projection".
///
///  2) The eta axis points due north in the adopted coordinate system.
///     If the spherical coordinates are observed (RA,Dec), the tangent
///     plane coordinates (xi,eta) are conventionally called the
///     "standard coordinates".  If the spherical coordinates are with
///     respect to a right-handed triad, (xi,eta) are also right-handed.
///     The units of (xi,eta) are, effectively, radians at the tangent
///     point.
///
///  3) All angular arguments are in radians.
///
///  4) This function is a member of the following set:
///
///         spherical      vector         solve for
///
///         eraTpxes      eraTpxev         xi,eta
///       > eraTpsts <    eraTpstv          star
///         eraTpors      eraTporv         origin
///
///  Called:
///     eraAnp       normalize angle into range 0 to 2pi
///
///  References:
///
///     Calabretta M.R. & Greisen, E.W., 2002, "Representations of
///     celestial coordinates in FITS", Astron.Astrophys. 395, 1077
///
///     Green, R.M., "Spherical Astronomy", Cambridge University Press,
///     1987, Chapter 13.
pub fn Tpsts(xi: f64, eta: f64, a0: f64, b0: f64) -> (f64, f64) {
    let mut a: f64 = 0.0;
    let mut b: f64 = 0.0;

    unsafe {
        eraTpsts(xi, eta, a0, b0, &mut a, &mut b);
    }

    return (a, b);
}

///  In the tangent plane projection, given the star's rectangular
///  coordinates and the direction cosines of the tangent point, solve
///  for the direction cosines of the star.
///
///  Given:
///     xi,eta  double     rectangular coordinates of star image (Note 2)
///     v0      double[3]  tangent point's direction cosines
///
///  Returned:
///     v       double[3]  star's direction cosines
///
///  1) The tangent plane projection is also called the "gnomonic
///     projection" and the "central projection".
///
///  2) The eta axis points due north in the adopted coordinate system.
///     If the direction cosines represent observed (RA,Dec), the tangent
///     plane coordinates (xi,eta) are conventionally called the
///     "standard coordinates".  If the direction cosines are with
///     respect to a right-handed triad, (xi,eta) are also right-handed.
///     The units of (xi,eta) are, effectively, radians at the tangent
///     point.
///
///  3) The method used is to complete the star vector in the (xi,eta)
///     based triad and normalize it, then rotate the triad to put the
///     tangent point at the pole with the x-axis aligned to zero
///     longitude.  Writing (a0,b0) for the celestial spherical
///     coordinates of the tangent point, the sequence of rotations is
///     (b-pi/2) around the x-axis followed by (-a-pi/2) around the
///     z-axis.
///
///  4) If vector v0 is not of unit length, the returned vector v will
///     be wrong.
///
///  5) If vector v0 points at a pole, the returned vector v will be
///     based on the arbitrary assumption that the longitude coordinate
///     of the tangent point is zero.
///
///  6) This function is a member of the following set:
///
///         spherical      vector         solve for
///
///         eraTpxes      eraTpxev         xi,eta
///         eraTpsts    > eraTpstv <        star
///         eraTpors      eraTporv         origin
///
///  References:
///
///     Calabretta M.R. & Greisen, E.W., 2002, "Representations of
///     celestial coordinates in FITS", Astron.Astrophys. 395, 1077
///
///     Green, R.M., "Spherical Astronomy", Cambridge University Press,
///     1987, Chapter 13.
pub fn Tpstv(xi: f64, eta: f64, v0: &[f64; 3]) -> [f64; 3] {
    let mut v: [f64; 3] = [0.0; 3];

    unsafe {
        eraTpstv(xi, eta, v0, &mut v);
    }

    return v;
}

///  In the tangent plane projection, given celestial spherical
///  coordinates for a star and the tangent point, solve for the star's
///  rectangular coordinates in the tangent plane.
///
///  Given:
///     a,b       double  star's spherical coordinates
///     a0,b0     double  tangent point's spherical coordinates
///
///  Returned:
///     *xi,*eta  double  rectangular coordinates of star image (Note 2)
///
///  Returned (function value):
///               int     status:  0 = OK
///                                1 = star too far from axis
///                                2 = antistar on tangent plane
///                                3 = antistar too far from axis
///
///  Notes:
///
///  1) The tangent plane projection is also called the "gnomonic
///     projection" and the "central projection".
///
///  2) The eta axis points due north in the adopted coordinate system.
///     If the spherical coordinates are observed (RA,Dec), the tangent
///     plane coordinates (xi,eta) are conventionally called the
///     "standard coordinates".  For right-handed spherical coordinates,
///     (xi,eta) are also right-handed.  The units of (xi,eta) are,
///     effectively, radians at the tangent point.
///
///  3) All angular arguments are in radians.
///
///  4) This function is a member of the following set:
///
///         spherical      vector         solve for
///
///       > eraTpxes <    eraTpxev         xi,eta
///         eraTpsts      eraTpstv          star
///         eraTpors      eraTporv         origin
///
///  References:
///
///     Calabretta M.R. & Greisen, E.W., 2002, "Representations of
///     celestial coordinates in FITS", Astron.Astrophys. 395, 1077
///
///     Green, R.M., "Spherical Astronomy", Cambridge University Press,
///     1987, Chapter 13.
pub fn Tpxes(a: f64, b: f64, a0: f64, b0: f64) -> Result<(f64, f64), ERFAError> {
    let mut xi: f64 = 0.0;
    let mut eta: f64 = 0.0;
    let err: i32;

    unsafe {
        err = eraTpxes(a, b, a0, b0, &mut xi, &mut eta);
    }

    match err {
        0 => Ok((xi, eta)),
        1 => Err(ERFAError::ERFABadInputValue),
        2 => Err(ERFAError::ERFABadInputValue),
        3 => Err(ERFAError::ERFABadInputValue),
        _ => unexpected_val_err!(eraTpxes),
    }
}

///  In the tangent plane projection, given celestial direction cosines
///  for a star and the tangent point, solve for the star's rectangular
///  coordinates in the tangent plane.
///
///  Given:
///     v         double[3]  direction cosines of star (Note 4)
///     v0        double[3]  direction cosines of tangent point (Note 4)
///
///  Returned:
///     *xi,*eta  double     tangent plane coordinates of star
///
///  Returned (function value):
///               int        status: 0 = OK
///                                  1 = star too far from axis
///                                  2 = antistar on tangent plane
///                                  3 = antistar too far from axis
///
///  Notes:
///
///  1) The tangent plane projection is also called the "gnomonic
///     projection" and the "central projection".
///
///  2) The eta axis points due north in the adopted coordinate system.
///     If the direction cosines represent observed (RA,Dec), the tangent
///     plane coordinates (xi,eta) are conventionally called the
///     "standard coordinates".  If the direction cosines are with
///     respect to a right-handed triad, (xi,eta) are also right-handed.
///     The units of (xi,eta) are, effectively, radians at the tangent
///     point.
///
///  3) The method used is to extend the star vector to the tangent
///     plane and then rotate the triad so that (x,y) becomes (xi,eta).
///     Writing (a,b) for the celestial spherical coordinates of the
///     star, the sequence of rotations is (a+pi/2) around the z-axis
///     followed by (pi/2-b) around the x-axis.
///
///  4) If vector v0 is not of unit length, or if vector v is of zero
///     length, the results will be wrong.
///
///  5) If v0 points at a pole, the returned (xi,eta) will be based on
///     the arbitrary assumption that the longitude coordinate of the
///     tangent point is zero.
///
///  6) This function is a member of the following set:
///
///         spherical      vector         solve for
///
///         eraTpxes    > eraTpxev <       xi,eta
///         eraTpsts      eraTpstv          star
///         eraTpors      eraTporv         origin
///
///  References:
///
///     Calabretta M.R. & Greisen, E.W., 2002, "Representations of
///     celestial coordinates in FITS", Astron.Astrophys. 395, 1077
///
///     Green, R.M., "Spherical Astronomy", Cambridge University Press,
///     1987, Chapter 13.
pub fn Tpxev(v: &[f64; 3], v0: &[f64; 3]) -> Result<(f64, f64), ERFAError> {
    let mut xi: f64 = 0.0;
    let mut eta: f64 = 0.0;
    let err: i32;

    unsafe {
        err = eraTpxev(v, v0, &mut xi, &mut eta);
    }

    match err {
        0 => Ok((xi, eta)),
        1 => Err(ERFAError::ERFABadInputValue),
        2 => Err(ERFAError::ERFABadInputValue),
        3 => Err(ERFAError::ERFABadInputValue),
        _ => unexpected_val_err!(eraTpxev),
    }
}
