//! ERFA Precession, Nutation, and Polar Motion Functions

use crate::raw::precnutpolar::*;

///  Frame bias components of IAU 2000 precession-nutation models;  part
///  of the Mathews-Herring-Buffett (MHB2000) nutation series, with
///  additions.
///
///  Returned:
///     dpsibi,depsbi  double  longitude and obliquity corrections
///     dra            double  the ICRS RA of the J2000.0 mean equinox
///
///  Notes:
///
///  1) The frame bias corrections in longitude and obliquity (radians)
///     are required in order to correct for the offset between the GCRS
///     pole and the mean J2000.0 pole.  They define, with respect to the
///     GCRS frame, a J2000.0 mean pole that is consistent with the rest
///     of the IAU 2000A precession-nutation model.
///
///  2) In addition to the displacement of the pole, the complete
///     description of the frame bias requires also an offset in right
///     ascension.  This is not part of the IAU 2000A model, and is from
///     Chapront et al. (2002).  It is returned in radians.
///
///  3) This is a supplemented implementation of one aspect of the IAU
///     2000A nutation model, formally adopted by the IAU General
///     Assembly in 2000, namely MHB2000 (Mathews et al. 2002).
///
///  References:
///
///     Chapront, J., Chapront-Touze, M. & Francou, G., Astron.
///     Astrophys., 387, 700, 2002.
///
///     Mathews, P.M., Herring, T.A., Buffet, B.A., "Modeling of nutation
///     and precession:  New nutation series for nonrigid Earth and
///     insights into the Earth's interior", J.Geophys.Res., 107, B4,
///     2002.  The MHB2000 code itself was obtained on 2002 September 9
///     from ftp://maia.usno.navy.mil/conv2000/chapter5/IAU2000A.
pub fn Bi00() -> (f64, f64, f64) {
    let mut dpsibi: f64 = 0.0;
    let mut depsbi: f64 = 0.0;
    let mut dra: f64 = 0.0;
    unsafe {
        eraBi00(&mut dpsibi, &mut depsbi, &mut dra);
    }

    return (dpsibi, depsbi, dra);
}

///  Frame bias and precession, IAU 2000.
///
///  Given:
///     date1,date2  double         TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     rb           double[3][3]   frame bias matrix (Note 2)
///     rp           double[3][3]   precession matrix (Note 3)
///     rbp          double[3][3]   bias-precession matrix (Note 4)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///             date1         date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The matrix rb transforms vectors from GCRS to mean J2000.0 by
///     applying frame bias.
///
///  3) The matrix rp transforms vectors from J2000.0 mean equator and
///     equinox to mean equator and equinox of date by applying
///     precession.
///
///  4) The matrix rbp transforms vectors from GCRS to mean equator and
///     equinox of date by applying frame bias then precession.  It is
///     the product rp x rb.
///
///  5) It is permissible to re-use the same array in the returned
///     arguments.  The arrays are filled in the order given.
///
///  Called:
///     eraBi00      frame bias components, IAU 2000
///     eraPr00      IAU 2000 precession adjustments
///     eraIr        initialize r-matrix to identity
///     eraRx        rotate around X-axis
///     eraRy        rotate around Y-axis
///     eraRz        rotate around Z-axis
///     eraCr        copy r-matrix
///     eraRxr       product of two r-matrices
///
///  Reference:
///     "Expressions for the Celestial Intermediate Pole and Celestial
///     Ephemeris Origin consistent with the IAU 2000A precession-
///     nutation model", Astron.Astrophys. 400, 1145-1154 (2003)
///
///     n.b. The celestial ephemeris origin (CEO) was renamed "celestial
///          intermediate origin" (CIO) by IAU 2006 Resolution 2.
pub fn Bp00(date1: f64, date2: f64, rb: &mut [f64; 9], rp: &mut [f64; 9], rbp: &mut [f64; 9]) {
    unsafe {
        eraBp00(date1, date2, rb, rp, rbp);
    }
}

///  Frame bias and precession, IAU 2006.
///
///  Given:
///     date1,date2  double         TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     rb           double[3][3]   frame bias matrix (Note 2)
///     rp           double[3][3]   precession matrix (Note 3)
///     rbp          double[3][3]   bias-precession matrix (Note 4)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///             date1         date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The matrix rb transforms vectors from GCRS to mean J2000.0 by
///     applying frame bias.
///
///  3) The matrix rp transforms vectors from mean J2000.0 to mean of
///     date by applying precession.
///
///  4) The matrix rbp transforms vectors from GCRS to mean of date by
///     applying frame bias then precession.  It is the product rp x rb.
///
///  5) It is permissible to re-use the same array in the returned
///     arguments.  The arrays are filled in the order given.
///
///  Called:
///     eraPfw06     bias-precession F-W angles, IAU 2006
///     eraFw2m      F-W angles to r-matrix
///     eraPmat06    PB matrix, IAU 2006
///     eraTr        transpose r-matrix
///     eraRxr       product of two r-matrices
///     eraCr        copy r-matrix
///
///  References:
///
///     Capitaine, N. & Wallace, P.T., 2006, Astron.Astrophys. 450, 855
///
///     Wallace, P.T. & Capitaine, N., 2006, Astron.Astrophys. 459, 981
pub fn Bp06(date1: f64, date2: f64, rb: &mut [f64; 9], rp: &mut [f64; 9], rbp: &mut [f64; 9]) {
    unsafe {
        eraBp06(date1, date2, rb, rp, rbp);
    }
}

///  Extract from the bias-precession-nutation matrix the X,Y coordinates
///  of the Celestial Intermediate Pole.
///
///  Given:
///     rbpn      double[3][3]  celestial-to-true matrix (Note 1)
///
///  Returned:
///     x,y       double        Celestial Intermediate Pole (Note 2)
///
///  Notes:
///
///  1) The matrix rbpn transforms vectors from GCRS to true equator (and
///     CIO or equinox) of date, and therefore the Celestial Intermediate
///     Pole unit vector is the bottom row of the matrix.
///
///  2) The arguments x,y are components of the Celestial Intermediate
///     Pole unit vector in the Geocentric Celestial Reference System.
///
///  Reference:
///
///     "Expressions for the Celestial Intermediate Pole and Celestial
///     Ephemeris Origin consistent with the IAU 2000A precession-
///     nutation model", Astron.Astrophys. 400, 1145-1154
///     (2003)
///
///     n.b. The celestial ephemeris origin (CEO) was renamed "celestial
///          intermediate origin" (CIO) by IAU 2006 Resolution 2.
pub fn Bpn2xy(rbpn: &[f64; 9]) -> (f64, f64) {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    unsafe {
        eraBpn2xy(rbpn, &mut x, &mut y);
    }

    return (x, y);
}

///  Form the celestial-to-intermediate matrix for a given date using the
///  IAU 2000A precession-nutation model.
///
///  Given:
///     date1,date2 double       TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     rc2i        double[3][3] celestial-to-intermediate matrix (Note 2)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The matrix rc2i is the first stage in the transformation from
///     celestial to terrestrial coordinates:
///
///        [TRS]  =  RPOM * R_3(ERA) * rc2i * [CRS]
///
///               =  rc2t * [CRS]
///
///     where [CRS] is a vector in the Geocentric Celestial Reference
///     System and [TRS] is a vector in the International Terrestrial
///     Reference System (see IERS Conventions 2003), ERA is the Earth
///     Rotation Angle and RPOM is the polar motion matrix.
///
///  3) A faster, but slightly less accurate, result (about 1 mas) can be
///     obtained by using instead the eraC2i00b function.
///
///  Called:
///     eraPnm00a    classical NPB matrix, IAU 2000A
///     eraC2ibpn    celestial-to-intermediate matrix, given NPB matrix
///
///  References:
///
///     "Expressions for the Celestial Intermediate Pole and Celestial
///     Ephemeris Origin consistent with the IAU 2000A precession-
///     nutation model", Astron.Astrophys. 400, 1145-1154
///     (2003)
///
///     n.b. The celestial ephemeris origin (CEO) was renamed "celestial
///          intermediate origin" (CIO) by IAU 2006 Resolution 2.
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
pub fn C2i00a(date1: f64, date2: f64, rc2i: &mut [f64; 9]) {
    unsafe {
        eraC2i00a(date1, date2, rc2i);
    }
}

///  Form the celestial-to-intermediate matrix for a given date using the
///  IAU 2000B precession-nutation model.
///
///  Given:
///     date1,date2 double       TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     rc2i        double[3][3] celestial-to-intermediate matrix (Note 2)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The matrix rc2i is the first stage in the transformation from
///     celestial to terrestrial coordinates:
///
///        [TRS]  =  RPOM * R_3(ERA) * rc2i * [CRS]
///
///               =  rc2t * [CRS]
///
///     where [CRS] is a vector in the Geocentric Celestial Reference
///     System and [TRS] is a vector in the International Terrestrial
///     Reference System (see IERS Conventions 2003), ERA is the Earth
///     Rotation Angle and RPOM is the polar motion matrix.
///
///  3) The present function is faster, but slightly less accurate (about
///     1 mas), than the eraC2i00a function.
///
///  Called:
///     eraPnm00b    classical NPB matrix, IAU 2000B
///     eraC2ibpn    celestial-to-intermediate matrix, given NPB matrix
///
///  References:
///
///     "Expressions for the Celestial Intermediate Pole and Celestial
///     Ephemeris Origin consistent with the IAU 2000A precession-
///     nutation model", Astron.Astrophys. 400, 1145-1154
///     (2003)
///
///     n.b. The celestial ephemeris origin (CEO) was renamed "celestial
///          intermediate origin" (CIO) by IAU 2006 Resolution 2.
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
pub fn C2i00b(date1: f64, date2: f64, rc2i: &mut [f64; 9]) {
    unsafe {
        eraC2i00b(date1, date2, rc2i);
    }
}

///  Form the celestial-to-intermediate matrix for a given date using the
///  IAU 2006 precession and IAU 2000A nutation models.
///
///  Given:
///     date1,date2 double       TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     rc2i        double[3][3] celestial-to-intermediate matrix (Note 2)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The matrix rc2i is the first stage in the transformation from
///     celestial to terrestrial coordinates:
///
///        [TRS]  =  RPOM * R_3(ERA) * rc2i * [CRS]
///
///               =  RC2T * [CRS]
///
///     where [CRS] is a vector in the Geocentric Celestial Reference
///     System and [TRS] is a vector in the International Terrestrial
///     Reference System (see IERS Conventions 2003), ERA is the Earth
///     Rotation Angle and RPOM is the polar motion matrix.
///
///  Called:
///     eraPnm06a    classical NPB matrix, IAU 2006/2000A
///     eraBpn2xy    extract CIP X,Y coordinates from NPB matrix
///     eraS06       the CIO locator s, given X,Y, IAU 2006
///     eraC2ixys    celestial-to-intermediate matrix, given X,Y and s
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), 2004, IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG
pub fn C2i06a(date1: f64, date2: f64, rc2i: &mut [f64; 9]) {
    unsafe {
        eraC2i06a(date1, date2, rc2i);
    }
}

///  Form the celestial-to-intermediate matrix for a given date given
///  the bias-precession-nutation matrix.  IAU 2000.
///
///  Given:
///     date1,date2 double       TT as a 2-part Julian Date (Note 1)
///     rbpn        double[3][3] celestial-to-true matrix (Note 2)
///
///  Returned:
///     rc2i        double[3][3] celestial-to-intermediate matrix (Note 3)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The matrix rbpn transforms vectors from GCRS to true equator (and
///     CIO or equinox) of date.  Only the CIP (bottom row) is used.
///
///  3) The matrix rc2i is the first stage in the transformation from
///     celestial to terrestrial coordinates:
///
///        [TRS] = RPOM * R_3(ERA) * rc2i * [CRS]
///
///              = RC2T * [CRS]
///
///     where [CRS] is a vector in the Geocentric Celestial Reference
///     System and [TRS] is a vector in the International Terrestrial
///     Reference System (see IERS Conventions 2003), ERA is the Earth
///     Rotation Angle and RPOM is the polar motion matrix.
///
///  4) Although its name does not include "00", This function is in fact
///     specific to the IAU 2000 models.
///
///  Called:
///     eraBpn2xy    extract CIP X,Y coordinates from NPB matrix
///     eraC2ixy     celestial-to-intermediate matrix, given X,Y
///
///  References:
///     "Expressions for the Celestial Intermediate Pole and Celestial
///     Ephemeris Origin consistent with the IAU 2000A precession-
///     nutation model", Astron.Astrophys. 400, 1145-1154 (2003)
///
///     n.b. The celestial ephemeris origin (CEO) was renamed "celestial
///          intermediate origin" (CIO) by IAU 2006 Resolution 2.
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
pub fn C2ibpn(date1: f64, date2: f64, rbpn: &[f64; 9], rc2i: &mut [f64; 9]) {
    unsafe {
        eraC2ibpn(date1, date2, rbpn, rc2i);
    }
}

///  Form the celestial to intermediate-frame-of-date matrix for a given
///  date when the CIP X,Y coordinates are known.  IAU 2000.
///
///  Given:
///     date1,date2 double       TT as a 2-part Julian Date (Note 1)
///     x,y         double       Celestial Intermediate Pole (Note 2)
///
///  Returned:
///     rc2i        double[3][3] celestial-to-intermediate matrix (Note 3)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The Celestial Intermediate Pole coordinates are the x,y components
///     of the unit vector in the Geocentric Celestial Reference System.
///
///  3) The matrix rc2i is the first stage in the transformation from
///     celestial to terrestrial coordinates:
///
///        [TRS] = RPOM * R_3(ERA) * rc2i * [CRS]
///
///              = RC2T * [CRS]
///
///     where [CRS] is a vector in the Geocentric Celestial Reference
///     System and [TRS] is a vector in the International Terrestrial
///     Reference System (see IERS Conventions 2003), ERA is the Earth
///     Rotation Angle and RPOM is the polar motion matrix.
///
///  4) Although its name does not include "00", This function is in fact
///     specific to the IAU 2000 models.
///
///  Called:
///     eraC2ixys    celestial-to-intermediate matrix, given X,Y and s
///     eraS00       the CIO locator s, given X,Y, IAU 2000A
///
///  Reference:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
pub fn C2ixy(date1: f64, date2: f64, x: f64, y: f64, rc2i: &mut [f64; 9]) {
    unsafe {
        eraC2ixy(date1, date2, x, y, rc2i);
    }
}

///  Form the celestial to intermediate-frame-of-date matrix given the CIP
///  X,Y and the CIO locator s.
///
///  Given:
///     x,y      double         Celestial Intermediate Pole (Note 1)
///     s        double         the CIO locator s (Note 2)
///
///  Returned:
///     rc2i     double[3][3]   celestial-to-intermediate matrix (Note 3)
///
///  Notes:
///
///  1) The Celestial Intermediate Pole coordinates are the x,y
///     components of the unit vector in the Geocentric Celestial
///     Reference System.
///
///  2) The CIO locator s (in radians) positions the Celestial
///     Intermediate Origin on the equator of the CIP.
///
///  3) The matrix rc2i is the first stage in the transformation from
///     celestial to terrestrial coordinates:
///
///        [TRS] = RPOM * R_3(ERA) * rc2i * [CRS]
///
///              = RC2T * [CRS]
///
///     where [CRS] is a vector in the Geocentric Celestial Reference
///     System and [TRS] is a vector in the International Terrestrial
///     Reference System (see IERS Conventions 2003), ERA is the Earth
///     Rotation Angle and RPOM is the polar motion matrix.
///
///  Called:
///     eraIr        initialize r-matrix to identity
///     eraRz        rotate around Z-axis
///     eraRy        rotate around Y-axis
///
///  Reference:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
pub fn C2ixys(x: f64, y: f64, s: f64, rc2i: &mut [f64; 9]) {
    unsafe {
        eraC2ixys(x, y, s, rc2i);
    }
}

///  Form the celestial to terrestrial matrix given the date, the UT1 and
///  the polar motion, using the IAU 2000A precession-nutation model.
///
///  Given:
///     tta,ttb  double         TT as a 2-part Julian Date (Note 1)
///     uta,utb  double         UT1 as a 2-part Julian Date (Note 1)
///     xp,yp    double         CIP coordinates (radians, Note 2)
///
///  Returned:
///     rc2t     double[3][3]   celestial-to-terrestrial matrix (Note 3)
///
///  Notes:
///
///  1) The TT and UT1 dates tta+ttb and uta+utb are Julian Dates,
///     apportioned in any convenient way between the arguments uta and
///     utb.  For example, JD(UT1)=2450123.7 could be expressed in any of
///     these ways, among others:
///
///             uta            utb
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution is
///     acceptable.  The J2000 and MJD methods are good compromises
///     between resolution and convenience.  In the case of uta,utb, the
///     date & time method is best matched to the Earth rotation angle
///     algorithm used:  maximum precision is delivered when the uta
///     argument is for 0hrs UT1 on the day in question and the utb
///     argument lies in the range 0 to 1, or vice versa.
///
///  2) The arguments xp and yp are the coordinates (in radians) of the
///     Celestial Intermediate Pole with respect to the International
///     Terrestrial Reference System (see IERS Conventions 2003),
///     measured along the meridians 0 and 90 deg west respectively.
///
///  3) The matrix rc2t transforms from celestial to terrestrial
///     coordinates:
///
///        [TRS] = RPOM * R_3(ERA) * RC2I * [CRS]
///
///              = rc2t * [CRS]
///
///     where [CRS] is a vector in the Geocentric Celestial Reference
///     System and [TRS] is a vector in the International Terrestrial
///     Reference System (see IERS Conventions 2003), RC2I is the
///     celestial-to-intermediate matrix, ERA is the Earth rotation
///     angle and RPOM is the polar motion matrix.
///
///  4) A faster, but slightly less accurate, result (about 1 mas) can
///     be obtained by using instead the eraC2t00b function.
///
///  Called:
///     eraC2i00a    celestial-to-intermediate matrix, IAU 2000A
///     eraEra00     Earth rotation angle, IAU 2000
///     eraSp00      the TIO locator s', IERS 2000
///     eraPom00     polar motion matrix
///     eraC2tcio    form CIO-based celestial-to-terrestrial matrix
///
///  Reference:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
pub fn C2t00a(tta: f64, ttb: f64, uta: f64, utb: f64, xp: f64, yp: f64, rc2t: &mut [f64; 9]) {
    unsafe {
        eraC2t00a(tta, ttb, uta, utb, xp, yp, rc2t);
    }
}

///  Form the celestial to terrestrial matrix given the date, the UT1 and
///  the polar motion, using the IAU 2000B precession-nutation model.
///
///  Given:
///     tta,ttb  double         TT as a 2-part Julian Date (Note 1)
///     uta,utb  double         UT1 as a 2-part Julian Date (Note 1)
///     xp,yp    double         coordinates of the pole (radians, Note 2)
///
///  Returned:
///     rc2t     double[3][3]   celestial-to-terrestrial matrix (Note 3)
///
///  Notes:
///
///  1) The TT and UT1 dates tta+ttb and uta+utb are Julian Dates,
///     apportioned in any convenient way between the arguments uta and
///     utb.  For example, JD(UT1)=2450123.7 could be expressed in any of
///     these ways, among others:
///
///             uta            utb
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution is
///     acceptable.  The J2000 and MJD methods are good compromises
///     between resolution and convenience.  In the case of uta,utb, the
///     date & time method is best matched to the Earth rotation angle
///     algorithm used:  maximum precision is delivered when the uta
///     argument is for 0hrs UT1 on the day in question and the utb
///     argument lies in the range 0 to 1, or vice versa.
///
///  2) The arguments xp and yp are the coordinates (in radians) of the
///     Celestial Intermediate Pole with respect to the International
///     Terrestrial Reference System (see IERS Conventions 2003),
///     measured along the meridians 0 and 90 deg west respectively.
///
///  3) The matrix rc2t transforms from celestial to terrestrial
///     coordinates:
///
///        [TRS] = RPOM * R_3(ERA) * RC2I * [CRS]
///
///              = rc2t * [CRS]
///
///     where [CRS] is a vector in the Geocentric Celestial Reference
///     System and [TRS] is a vector in the International Terrestrial
///     Reference System (see IERS Conventions 2003), RC2I is the
///     celestial-to-intermediate matrix, ERA is the Earth rotation
///     angle and RPOM is the polar motion matrix.
///
///  4) The present function is faster, but slightly less accurate (about
///     1 mas), than the eraC2t00a function.
///
///  Called:
///     eraC2i00b    celestial-to-intermediate matrix, IAU 2000B
///     eraEra00     Earth rotation angle, IAU 2000
///     eraPom00     polar motion matrix
///     eraC2tcio    form CIO-based celestial-to-terrestrial matrix
///
///  Reference:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
pub fn C2t00b(tta: f64, ttb: f64, uta: f64, utb: f64, xp: f64, yp: f64, rc2t: &mut [f64; 9]) {
    unsafe {
        eraC2t00b(tta, ttb, uta, utb, xp, yp, rc2t);
    }
}

///  Form the celestial to terrestrial matrix given the date, the UT1 and
///  the polar motion, using the IAU 2006/2000A precession-nutation
///  model.
///
///  Given:
///     tta,ttb  double         TT as a 2-part Julian Date (Note 1)
///     uta,utb  double         UT1 as a 2-part Julian Date (Note 1)
///     xp,yp    double         coordinates of the pole (radians, Note 2)
///
///  Returned:
///     rc2t     double[3][3]   celestial-to-terrestrial matrix (Note 3)
///
///  Notes:
///
///  1) The TT and UT1 dates tta+ttb and uta+utb are Julian Dates,
///     apportioned in any convenient way between the two arguments.  For
///     example, JD(UT1)=2450123.7 could be expressed in any of
///     these ways, among others:
///
///             uta            utb
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution is
///     acceptable.  The J2000 and MJD methods are good compromises
///     between resolution and convenience.  In the case of uta,utb, the
///     date & time method is best matched to the Earth rotation angle
///     algorithm used:  maximum precision is delivered when the uta
///     argument is for 0hrs UT1 on the day in question and the utb
///     argument lies in the range 0 to 1, or vice versa.
///
///  2) The arguments xp and yp are the coordinates (in radians) of the
///     Celestial Intermediate Pole with respect to the International
///     Terrestrial Reference System (see IERS Conventions 2003),
///     measured along the meridians 0 and 90 deg west respectively.
///
///  3) The matrix rc2t transforms from celestial to terrestrial
///     coordinates:
///
///        [TRS] = RPOM * R_3(ERA) * RC2I * [CRS]
///
///              = rc2t * [CRS]
///
///     where [CRS] is a vector in the Geocentric Celestial Reference
///     System and [TRS] is a vector in the International Terrestrial
///     Reference System (see IERS Conventions 2003), RC2I is the
///     celestial-to-intermediate matrix, ERA is the Earth rotation
///     angle and RPOM is the polar motion matrix.
///
///  Called:
///     eraC2i06a    celestial-to-intermediate matrix, IAU 2006/2000A
///     eraEra00     Earth rotation angle, IAU 2000
///     eraSp00      the TIO locator s', IERS 2000
///     eraPom00     polar motion matrix
///     eraC2tcio    form CIO-based celestial-to-terrestrial matrix
///
///  Reference:
///
///     McCarthy, D. D., Petit, G. (eds.), 2004, IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG
pub fn C2t06a(tta: f64, ttb: f64, uta: f64, utb: f64, xp: f64, yp: f64, rc2t: &mut [f64; 9]) {
    unsafe {
        eraC2t06a(tta, ttb, uta, utb, xp, yp, rc2t);
    }
}

///  Assemble the celestial to terrestrial matrix from CIO-based
///  components (the celestial-to-intermediate matrix, the Earth Rotation
///  Angle and the polar motion matrix).
///
///  Given:
///     rc2i     double[3][3]    celestial-to-intermediate matrix
///     era      double          Earth rotation angle (radians)
///     rpom     double[3][3]    polar-motion matrix
///
///  Returned:
///     rc2t     double[3][3]    celestial-to-terrestrial matrix
///
///  Notes:
///
///  1) This function constructs the rotation matrix that transforms
///     vectors in the celestial system into vectors in the terrestrial
///     system.  It does so starting from precomputed components, namely
///     the matrix which rotates from celestial coordinates to the
///     intermediate frame, the Earth rotation angle and the polar motion
///     matrix.  One use of the present function is when generating a
///     series of celestial-to-terrestrial matrices where only the Earth
///     Rotation Angle changes, avoiding the considerable overhead of
///     recomputing the precession-nutation more often than necessary to
///     achieve given accuracy objectives.
///
///  2) The relationship between the arguments is as follows:
///
///        [TRS] = RPOM * R_3(ERA) * rc2i * [CRS]
///
///              = rc2t * [CRS]
///
///     where [CRS] is a vector in the Geocentric Celestial Reference
///     System and [TRS] is a vector in the International Terrestrial
///     Reference System (see IERS Conventions 2003).
///
///  Called:
///     eraCr        copy r-matrix
///     eraRz        rotate around Z-axis
///     eraRxr       product of two r-matrices
///
///  Reference:
///
///     McCarthy, D. D., Petit, G. (eds.), 2004, IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG
pub fn C2tcio(rc2i: &[f64; 9], era: f64, rpom: &[f64; 9], rc2t: &mut [f64; 9]) {
    unsafe {
        eraC2tcio(rc2i, era, rpom, rc2t);
    }
}

///  Assemble the celestial to terrestrial matrix from equinox-based
///  components (the celestial-to-true matrix, the Greenwich Apparent
///  Sidereal Time and the polar motion matrix).
///
///  Given:
///     rbpn   double[3][3]  celestial-to-true matrix
///     gst    double        Greenwich (apparent) Sidereal Time (radians)
///     rpom   double[3][3]  polar-motion matrix
///
///  Returned:
///     rc2t   double[3][3]  celestial-to-terrestrial matrix (Note 2)
///
///  Notes:
///
///  1) This function constructs the rotation matrix that transforms
///     vectors in the celestial system into vectors in the terrestrial
///     system.  It does so starting from precomputed components, namely
///     the matrix which rotates from celestial coordinates to the
///     true equator and equinox of date, the Greenwich Apparent Sidereal
///     Time and the polar motion matrix.  One use of the present function
///     is when generating a series of celestial-to-terrestrial matrices
///     where only the Sidereal Time changes, avoiding the considerable
///     overhead of recomputing the precession-nutation more often than
///     necessary to achieve given accuracy objectives.
///
///  2) The relationship between the arguments is as follows:
///
///        [TRS] = rpom * R_3(gst) * rbpn * [CRS]
///
///              = rc2t * [CRS]
///
///     where [CRS] is a vector in the Geocentric Celestial Reference
///     System and [TRS] is a vector in the International Terrestrial
///     Reference System (see IERS Conventions 2003).
///
///  Called:
///     eraCr        copy r-matrix
///     eraRz        rotate around Z-axis
///     eraRxr       product of two r-matrices
///
///  Reference:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
pub fn C2teqx(rbpn: &[f64; 9], gst: f64, rpom: &[f64; 9], rc2t: &mut [f64; 9]) {
    unsafe {
        eraC2teqx(rbpn, gst, rpom, rc2t);
    }
}

///  Form the celestial to terrestrial matrix given the date, the UT1,
///  the nutation and the polar motion.  IAU 2000.
///
///  Given:
///     tta,ttb    double        TT as a 2-part Julian Date (Note 1)
///     uta,utb    double        UT1 as a 2-part Julian Date (Note 1)
///     dpsi,deps  double        nutation (Note 2)
///     xp,yp      double        coordinates of the pole (radians, Note 3)
///
///  Returned:
///     rc2t       double[3][3]  celestial-to-terrestrial matrix (Note 4)
///
///  Notes:
///
///  1) The TT and UT1 dates tta+ttb and uta+utb are Julian Dates,
///     apportioned in any convenient way between the arguments uta and
///     utb.  For example, JD(UT1)=2450123.7 could be expressed in any of
///     these ways, among others:
///
///             uta            utb
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution is
///     acceptable.  The J2000 and MJD methods are good compromises
///     between resolution and convenience.  In the case of uta,utb, the
///     date & time method is best matched to the Earth rotation angle
///     algorithm used:  maximum precision is delivered when the uta
///     argument is for 0hrs UT1 on the day in question and the utb
///     argument lies in the range 0 to 1, or vice versa.
///
///  2) The caller is responsible for providing the nutation components;
///     they are in longitude and obliquity, in radians and are with
///     respect to the equinox and ecliptic of date.  For high-accuracy
///     applications, free core nutation should be included as well as
///     any other relevant corrections to the position of the CIP.
///
///  3) The arguments xp and yp are the coordinates (in radians) of the
///     Celestial Intermediate Pole with respect to the International
///     Terrestrial Reference System (see IERS Conventions 2003),
///     measured along the meridians 0 and 90 deg west respectively.
///
///  4) The matrix rc2t transforms from celestial to terrestrial
///     coordinates:
///
///        [TRS] = RPOM * R_3(GST) * RBPN * [CRS]
///
///              = rc2t * [CRS]
///
///     where [CRS] is a vector in the Geocentric Celestial Reference
///     System and [TRS] is a vector in the International Terrestrial
///     Reference System (see IERS Conventions 2003), RBPN is the
///     bias-precession-nutation matrix, GST is the Greenwich (apparent)
///     Sidereal Time and RPOM is the polar motion matrix.
///
///  5) Although its name does not include "00", This function is in fact
///     specific to the IAU 2000 models.
///
///  Called:
///     eraPn00      bias/precession/nutation results, IAU 2000
///     eraGmst00    Greenwich mean sidereal time, IAU 2000
///     eraSp00      the TIO locator s', IERS 2000
///     eraEe00      equation of the equinoxes, IAU 2000
///     eraPom00     polar motion matrix
///     eraC2teqx    form equinox-based celestial-to-terrestrial matrix
///
///  Reference:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
pub fn C2tpe(
    tta: f64,
    ttb: f64,
    uta: f64,
    utb: f64,
    dpsi: f64,
    deps: f64,
    xp: f64,
    yp: f64,
    rc2t: &mut [f64; 9],
) {
    unsafe {
        eraC2tpe(tta, ttb, uta, utb, dpsi, deps, xp, yp, rc2t);
    }
}

///  Form the celestial to terrestrial matrix given the date, the UT1,
///  the CIP coordinates and the polar motion.  IAU 2000.
///
///  Given:
///     tta,ttb  double         TT as a 2-part Julian Date (Note 1)
///     uta,utb  double         UT1 as a 2-part Julian Date (Note 1)
///     x,y      double         Celestial Intermediate Pole (Note 2)
///     xp,yp    double         coordinates of the pole (radians, Note 3)
///
///  Returned:
///     rc2t     double[3][3]   celestial-to-terrestrial matrix (Note 4)
///
///  Notes:
///
///  1) The TT and UT1 dates tta+ttb and uta+utb are Julian Dates,
///     apportioned in any convenient way between the arguments uta and
///     utb.  For example, JD(UT1)=2450123.7 could be expressed in any o
///     these ways, among others:
///
///             uta            utb
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution is
///     acceptable.  The J2000 and MJD methods are good compromises
///     between resolution and convenience.  In the case of uta,utb, the
///     date & time method is best matched to the Earth rotation angle
///     algorithm used:  maximum precision is delivered when the uta
///     argument is for 0hrs UT1 on the day in question and the utb
///     argument lies in the range 0 to 1, or vice versa.
///
///  2) The Celestial Intermediate Pole coordinates are the x,y
///     components of the unit vector in the Geocentric Celestial
///     Reference System.
///
///  3) The arguments xp and yp are the coordinates (in radians) of the
///     Celestial Intermediate Pole with respect to the International
///     Terrestrial Reference System (see IERS Conventions 2003),
///     measured along the meridians 0 and 90 deg west respectively.
///
///  4) The matrix rc2t transforms from celestial to terrestrial
///     coordinates:
///
///        [TRS] = RPOM * R_3(ERA) * RC2I * [CRS]
///
///              = rc2t * [CRS]
///
///     where [CRS] is a vector in the Geocentric Celestial Reference
///     System and [TRS] is a vector in the International Terrestrial
///     Reference System (see IERS Conventions 2003), ERA is the Earth
///     Rotation Angle and RPOM is the polar motion matrix.
///
///  5) Although its name does not include "00", This function is in fact
///     specific to the IAU 2000 models.
///
///  Called:
///     eraC2ixy     celestial-to-intermediate matrix, given X,Y
///     eraEra00     Earth rotation angle, IAU 2000
///     eraSp00      the TIO locator s', IERS 2000
///     eraPom00     polar motion matrix
///     eraC2tcio    form CIO-based celestial-to-terrestrial matrix
///
/// Reference:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
pub fn C2txy(
    tta: f64,
    ttb: f64,
    uta: f64,
    utb: f64,
    x: f64,
    y: f64,
    xp: f64,
    yp: f64,
    rc2t: &mut [f64; 9],
) {
    unsafe {
        eraC2txy(tta, ttb, uta, utb, x, y, xp, yp, rc2t);
    }
}

///  Equation of the origins, IAU 2006 precession and IAU 2000A nutation.
///
///  Given:
///     date1,date2  double    TT as a 2-part Julian Date (Note 1)
///
///  Returned (function value):
///                  double    the equation of the origins in radians
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The equation of the origins is the distance between the true
///     equinox and the celestial intermediate origin and, equivalently,
///     the difference between Earth rotation angle and Greenwich
///     apparent sidereal time (ERA-GST).  It comprises the precession
///     (since J2000.0) in right ascension plus the equation of the
///     equinoxes (including the small correction terms).
///
///  Called:
///     eraPnm06a    classical NPB matrix, IAU 2006/2000A
///     eraBpn2xy    extract CIP X,Y coordinates from NPB matrix
///     eraS06       the CIO locator s, given X,Y, IAU 2006
///     eraEors      equation of the origins, given NPB matrix and s
///
///  References:
///
///     Capitaine, N. & Wallace, P.T., 2006, Astron.Astrophys. 450, 855
///
///     Wallace, P.T. & Capitaine, N., 2006, Astron.Astrophys. 459, 981
pub fn Eo06a(date1: f64, date2: f64) -> f64 {
    return unsafe { eraEo06a(date1, date2) };
}

///  Equation of the origins, given the classical NPB matrix and the
///  quantity s.
///
///  Given:
///     rnpb  double[3][3]  classical nutation x precession x bias matrix
///     s     double        the quantity s (the CIO locator) in radians
///
///  Returned (function value):
///           double        the equation of the origins in radians
///
///  Notes:
///
///  1)  The equation of the origins is the distance between the true
///      equinox and the celestial intermediate origin and, equivalently,
///      the difference between Earth rotation angle and Greenwich
///      apparent sidereal time (ERA-GST).  It comprises the precession
///      (since J2000.0) in right ascension plus the equation of the
///      equinoxes (including the small correction terms).
///
///  2)  The algorithm is from Wallace & Capitaine (2006).
///
/// References:
///
///     Capitaine, N. & Wallace, P.T., 2006, Astron.Astrophys. 450, 855
///
///     Wallace, P. & Capitaine, N., 2006, Astron.Astrophys. 459, 981
pub fn Eors(rnpb: &[f64; 9], s: f64) -> f64 {
    return unsafe { eraEors(rnpb, s) };
}

///  Form rotation matrix given the Fukushima-Williams angles.
///
///  Given:
///     gamb     double         F-W angle gamma_bar (radians)
///     phib     double         F-W angle phi_bar (radians)
///     psi      double         F-W angle psi (radians)
///     eps      double         F-W angle epsilon (radians)
///
///  Returned:
///     r        double[3][3]   rotation matrix
///
///  Notes:
///
///  1) Naming the following points:
///
///           e = J2000.0 ecliptic pole,
///           p = GCRS pole,
///           E = ecliptic pole of date,
///     and   P = CIP,
///
///     the four Fukushima-Williams angles are as follows:
///
///        gamb = gamma = epE
///        phib = phi = pE
///        psi = psi = pEP
///        eps = epsilon = EP
///
///  2) The matrix representing the combined effects of frame bias,
///     precession and nutation is:
///
///        NxPxB = R_1(-eps).R_3(-psi).R_1(phib).R_3(gamb)
///
///  3) The present function can construct three different matrices,
///     depending on which angles are supplied as the arguments gamb,
///     phib, psi and eps:
///
///     o  To obtain the nutation x precession x frame bias matrix,
///        first generate the four precession angles known conventionally
///        as gamma_bar, phi_bar, psi_bar and epsilon_A, then generate
///        the nutation components Dpsi and Depsilon and add them to
///        psi_bar and epsilon_A, and finally call the present function
///        using those four angles as arguments.
///
///     o  To obtain the precession x frame bias matrix, generate the
///        four precession angles and call the present function.
///
///     o  To obtain the frame bias matrix, generate the four precession
///        angles for date J2000.0 and call the present function.
///
///     The nutation-only and precession-only matrices can if necessary
///     be obtained by combining these three appropriately.
///
///  Called:
///     eraIr        initialize r-matrix to identity
///     eraRz        rotate around Z-axis
///     eraRx        rotate around X-axis
///
///  References:
///
///     Capitaine, N. & Wallace, P.T., 2006, Astron.Astrophys. 450, 855
///
///     Hilton, J. et al., 2006, Celest.Mech.Dyn.Astron. 94, 351
pub fn Fw2m(gamb: f64, phib: f64, psi: f64, eps: f64, r: &mut [f64; 9]) {
    unsafe {
        eraFw2m(gamb, phib, psi, eps, r);
    }
}

///  CIP X,Y given Fukushima-Williams bias-precession-nutation angles.
///
///  Given:
///     gamb     double    F-W angle gamma_bar (radians)
///     phib     double    F-W angle phi_bar (radians)
///     psi      double    F-W angle psi (radians)
///     eps      double    F-W angle epsilon (radians)
///
///  Returned:
///     x,y      double    CIP unit vector X,Y
///
///  Notes:
///
///  1) Naming the following points:
///
///           e = J2000.0 ecliptic pole,
///           p = GCRS pole
///           E = ecliptic pole of date,
///     and   P = CIP,
///
///     the four Fukushima-Williams angles are as follows:
///
///        gamb = gamma = epE
///        phib = phi = pE
///        psi = psi = pEP
///        eps = epsilon = EP
///
///  2) The matrix representing the combined effects of frame bias,
///     precession and nutation is:
///
///        NxPxB = R_1(-epsA).R_3(-psi).R_1(phib).R_3(gamb)
///
///     The returned values x,y are elements [2][0] and [2][1] of the
///     matrix.  Near J2000.0, they are essentially angles in radians.
///
///  Called:
///     eraFw2m      F-W angles to r-matrix
///     eraBpn2xy    extract CIP X,Y coordinates from NPB matrix
///
///  Reference:
///
///     Hilton, J. et al., 2006, Celest.Mech.Dyn.Astron. 94, 351
pub fn Fw2xy(gamb: f64, phib: f64, psi: f64, eps: f64) -> (f64, f64) {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    unsafe {
        eraFw2xy(gamb, phib, psi, eps, &mut x, &mut y);
    }

    return (x, y);
}

///  Long-term precession matrix.
///
///  Given:
///     epj     double         Julian epoch (TT)
///
///  Returned:
///     rp      double[3][3]   precession matrix, J2000.0 to date
///
///  Notes:
///
///  1) The matrix is in the sense
///
///        P_date = rp x P_J2000,
///
///     where P_J2000 is a vector with respect to the J2000.0 mean
///     equator and equinox and P_date is the same vector with respect to
///     the mean equator and equinox of epoch epj.
///
///  2) The Vondrak et al. (2011, 2012) 400 millennia precession model
///     agrees with the IAU 2006 precession at J2000.0 and stays within
///     100 microarcseconds during the 20th and 21st centuries.  It is
///     accurate to a few arcseconds throughout the historical period,
///     worsening to a few tenths of a degree at the end of the
///     +/- 200,000 year time span.
///
///  Called:
///     eraLtpequ    equator pole, long term
///     eraLtpecl    ecliptic pole, long term
///     eraPxp       vector product
///     eraPn        normalize vector
///
///  References:
///
///    Vondrak, J., Capitaine, N. and Wallace, P., 2011, New precession
///    expressions, valid for long time intervals, Astron.Astrophys. 534,
///    A22
///
///    Vondrak, J., Capitaine, N. and Wallace, P., 2012, New precession
///    expressions, valid for long time intervals (Corrigendum),
///    Astron.Astrophys. 541, C1
pub fn Ltp(epj: f64, rp: &mut [f64; 9]) {
    unsafe {
        eraLtp(epj, rp);
    }
}

///  Long-term precession matrix, including ICRS frame bias.
///
///  Given:
///     epj     double         Julian epoch (TT)
///
///  Returned:
///     rpb     double[3][3]   precession+bias matrix, J2000.0 to date
///
///  Notes:
///
///  1) The matrix is in the sense
///
///        P_date = rpb x P_ICRS,
///
///     where P_ICRS is a vector in the Geocentric Celestial Reference
///     System, and P_date is the vector with respect to the Celestial
///     Intermediate Reference System at that date but with nutation
///     neglected.
///
///  2) A first order frame bias formulation is used, of sub-
///     microarcsecond accuracy compared with a full 3D rotation.
///
///  3) The Vondrak et al. (2011, 2012) 400 millennia precession model
///     agrees with the IAU 2006 precession at J2000.0 and stays within
///     100 microarcseconds during the 20th and 21st centuries.  It is
///     accurate to a few arcseconds throughout the historical period,
///     worsening to a few tenths of a degree at the end of the
///     +/- 200,000 year time span.
///
///  References:
///
///    Vondrak, J., Capitaine, N. and Wallace, P., 2011, New precession
///    expressions, valid for long time intervals, Astron.Astrophys. 534,
///    A22
///
///    Vondrak, J., Capitaine, N. and Wallace, P., 2012, New precession
///    expressions, valid for long time intervals (Corrigendum),
///    Astron.Astrophys. 541, C1
pub fn Ltpb(epj: f64, rpb: &mut [f64; 9]) {
    unsafe {
        eraLtpb(epj, rpb);
    }
}

///  Long-term precession of the ecliptic.
///
///  Given:
///     epj     double         Julian epoch (TT)
///
///  Returned:
///     vec     double[3]      ecliptic pole unit vector
///
///  Notes:
///
///  1) The returned vector is with respect to the J2000.0 mean equator
///     and equinox.
///
///  2) The Vondrak et al. (2011, 2012) 400 millennia precession model
///     agrees with the IAU 2006 precession at J2000.0 and stays within
///     100 microarcseconds during the 20th and 21st centuries.  It is
///     accurate to a few arcseconds throughout the historical period,
///     worsening to a few tenths of a degree at the end of the
///     +/- 200,000 year time span.
///
///  References:
///
///    Vondrak, J., Capitaine, N. and Wallace, P., 2011, New precession
///    expressions, valid for long time intervals, Astron.Astrophys. 534,
///    A22
///
///    Vondrak, J., Capitaine, N. and Wallace, P., 2012, New precession
///    expressions, valid for long time intervals (Corrigendum),
///    Astron.Astrophys. 541, C1
pub fn Ltpecl(epj: f64) -> [f64; 3] {
    let mut vec: [f64; 3] = [0.0; 3];
    unsafe {
        eraLtpecl(epj, &mut vec);
    }

    return vec;
}

///  Long-term precession of the equator.
///
///  Given:
///     epj     double         Julian epoch (TT)
///
///  Returned:
///     veq     double[3]      equator pole unit vector
///
///  Notes:
///
///  1) The returned vector is with respect to the J2000.0 mean equator
///     and equinox.
///
///  2) The Vondrak et al. (2011, 2012) 400 millennia precession model
///     agrees with the IAU 2006 precession at J2000.0 and stays within
///     100 microarcseconds during the 20th and 21st centuries.  It is
///     accurate to a few arcseconds throughout the historical period,
///     worsening to a few tenths of a degree at the end of the
///     +/- 200,000 year time span.
///
///  References:
///
///    Vondrak, J., Capitaine, N. and Wallace, P., 2011, New precession
///    expressions, valid for long time intervals, Astron.Astrophys. 534,
///    A22
///
///    Vondrak, J., Capitaine, N. and Wallace, P., 2012, New precession
///    expressions, valid for long time intervals (Corrigendum),
///    Astron.Astrophys. 541, C1
pub fn Ltpequ(epj: f64) -> [f64; 3] {
    let mut veq: [f64; 3] = [0.0; 3];
    unsafe {
        eraLtpequ(epj, &mut veq);
    }

    return veq;
}

///  Form the matrix of nutation for a given date, IAU 2000A model.
///
///  Given:
///     date1,date2  double          TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     rmatn        double[3][3]    nutation matrix
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The matrix operates in the sense V(true) = rmatn * V(mean), where
///     the p-vector V(true) is with respect to the true equatorial triad
///     of date and the p-vector V(mean) is with respect to the mean
///     equatorial triad of date.
///
///  3) A faster, but slightly less accurate, result (about 1 mas) can be
///     obtained by using instead the eraNum00b function.
///
///  Called:
///     eraPn00a     bias/precession/nutation, IAU 2000A
///
///  Reference:
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992),
///     Section 3.222-3 (p114).
pub fn Num00a(date1: f64, date2: f64, rmatn: &mut [f64; 9]) {
    unsafe {
        eraNum00a(date1, date2, rmatn);
    }
}

///  Form the matrix of nutation for a given date, IAU 2000B model.
///
///  Given:
///     date1,date2  double         TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     rmatn        double[3][3]   nutation matrix
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The matrix operates in the sense V(true) = rmatn * V(mean), where
///     the p-vector V(true) is with respect to the true equatorial triad
///     of date and the p-vector V(mean) is with respect to the mean
///     equatorial triad of date.
///
///  3) The present function is faster, but slightly less accurate (about
///     1 mas), than the eraNum00a function.
///
///  Called:
///     eraPn00b     bias/precession/nutation, IAU 2000B
///
///  Reference:
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992),
///     Section 3.222-3 (p114).
pub fn Num00b(date1: f64, date2: f64, rmatn: &mut [f64; 9]) {
    unsafe {
        eraNum00b(date1, date2, rmatn);
    }
}

///  Form the matrix of nutation for a given date, IAU 2006/2000A model.
///
///  Given:
///     date1,date2   double          TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     rmatn         double[3][3]    nutation matrix
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The matrix operates in the sense V(true) = rmatn * V(mean), where
///     the p-vector V(true) is with respect to the true equatorial triad
///     of date and the p-vector V(mean) is with respect to the mean
///     equatorial triad of date.
///
///  Called:
///     eraObl06     mean obliquity, IAU 2006
///     eraNut06a    nutation, IAU 2006/2000A
///     eraNumat     form nutation matrix
///
///  Reference:
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992),
///     Section 3.222-3 (p114).
pub fn Num06a(date1: f64, date2: f64, rmatn: &mut [f64; 9]) {
    unsafe {
        eraNum06a(date1, date2, rmatn);
    }
}

///  Form the matrix of nutation.
///
///  Given:
///     epsa        double         mean obliquity of date (Note 1)
///     dpsi,deps   double         nutation (Note 2)
///
///  Returned:
///     rmatn       double[3][3]   nutation matrix (Note 3)
///
///  Notes:
///
///
///  1) The supplied mean obliquity epsa, must be consistent with the
///     precession-nutation models from which dpsi and deps were obtained.
///
///  2) The caller is responsible for providing the nutation components;
///     they are in longitude and obliquity, in radians and are with
///     respect to the equinox and ecliptic of date.
///
///  3) The matrix operates in the sense V(true) = rmatn * V(mean),
///     where the p-vector V(true) is with respect to the true
///     equatorial triad of date and the p-vector V(mean) is with
///     respect to the mean equatorial triad of date.
///
///  Called:
///     eraIr        initialize r-matrix to identity
///     eraRx        rotate around X-axis
///     eraRz        rotate around Z-axis
///
///  Reference:
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992),
///     Section 3.222-3 (p114).
pub fn Numat(epsa: f64, dpsi: f64, deps: f64, rmatn: &mut [f64; 9]) {
    unsafe {
        eraNumat(epsa, dpsi, deps, rmatn);
    }
}

///  Nutation, IAU 2000A model (MHB2000 luni-solar and planetary nutation
///  with free core nutation omitted).
///
///  Given:
///     date1,date2   double   TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     dpsi,deps     double   nutation, luni-solar + planetary (Note 2)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The nutation components in longitude and obliquity are in radians
///     and with respect to the equinox and ecliptic of date.  The
///     obliquity at J2000.0 is assumed to be the Lieske et al. (1977)
///     value of 84381.448 arcsec.
///
///     Both the luni-solar and planetary nutations are included.  The
///     latter are due to direct planetary nutations and the
///     perturbations of the lunar and terrestrial orbits.
///
///  3) The function computes the MHB2000 nutation series with the
///     associated corrections for planetary nutations.  It is an
///     implementation of the nutation part of the IAU 2000A precession-
///     nutation model, formally adopted by the IAU General Assembly in
///     2000, namely MHB2000 (Mathews et al. 2002), but with the free
///     core nutation (FCN - see Note 4) omitted.
///
///  4) The full MHB2000 model also contains contributions to the
///     nutations in longitude and obliquity due to the free-excitation
///     of the free-core-nutation during the period 1979-2000.  These FCN
///     terms, which are time-dependent and unpredictable, are NOT
///     included in the present function and, if required, must be
///     independently computed.  With the FCN corrections included, the
///     present function delivers a pole which is at current epochs
///     accurate to a few hundred microarcseconds.  The omission of FCN
///     introduces further errors of about that size.
///
///  5) The present function provides classical nutation.  The MHB2000
///     algorithm, from which it is adapted, deals also with (i) the
///     offsets between the GCRS and mean poles and (ii) the adjustments
///     in longitude and obliquity due to the changed precession rates.
///     These additional functions, namely frame bias and precession
///     adjustments, are supported by the ERFA functions eraBi00  and
///     eraPr00.
///
///  6) The MHB2000 algorithm also provides "total" nutations, comprising
///     the arithmetic sum of the frame bias, precession adjustments,
///     luni-solar nutation and planetary nutation.  These total
///     nutations can be used in combination with an existing IAU 1976
///     precession implementation, such as eraPmat76,  to deliver GCRS-
///     to-true predictions of sub-mas accuracy at current dates.
///     However, there are three shortcomings in the MHB2000 model that
///     must be taken into account if more accurate or definitive results
///     are required (see Wallace 2002):
///
///       (i) The MHB2000 total nutations are simply arithmetic sums,
///           yet in reality the various components are successive Euler
///           rotations.  This slight lack of rigor leads to cross terms
///           that exceed 1 mas after a century.  The rigorous procedure
///           is to form the GCRS-to-true rotation matrix by applying the
///           bias, precession and nutation in that order.
///
///      (ii) Although the precession adjustments are stated to be with
///           respect to Lieske et al. (1977), the MHB2000 model does
///           not specify which set of Euler angles are to be used and
///           how the adjustments are to be applied.  The most literal
///           and straightforward procedure is to adopt the 4-rotation
///           epsilon_0, psi_A, omega_A, xi_A option, and to add DPSIPR
///           to psi_A and DEPSPR to both omega_A and eps_A.
///
///     (iii) The MHB2000 model predates the determination by Chapront
///           et al. (2002) of a 14.6 mas displacement between the
///           J2000.0 mean equinox and the origin of the ICRS frame.  It
///           should, however, be noted that neglecting this displacement
///           when calculating star coordinates does not lead to a
///           14.6 mas change in right ascension, only a small second-
///           order distortion in the pattern of the precession-nutation
///           effect.
///
///     For these reasons, the ERFA functions do not generate the "total
///     nutations" directly, though they can of course easily be
///     generated by calling eraBi00, eraPr00 and the present function
///     and adding the results.
///
///  7) The MHB2000 model contains 41 instances where the same frequency
///     appears multiple times, of which 38 are duplicates and three are
///     triplicates.  To keep the present code close to the original MHB
///     algorithm, this small inefficiency has not been corrected.
///
///  Called:
///     eraFal03     mean anomaly of the Moon
///     eraFaf03     mean argument of the latitude of the Moon
///     eraFaom03    mean longitude of the Moon's ascending node
///     eraFame03    mean longitude of Mercury
///     eraFave03    mean longitude of Venus
///     eraFae03     mean longitude of Earth
///     eraFama03    mean longitude of Mars
///     eraFaju03    mean longitude of Jupiter
///     eraFasa03    mean longitude of Saturn
///     eraFaur03    mean longitude of Uranus
///     eraFapa03    general accumulated precession in longitude
///
///  References:
///
///     Chapront, J., Chapront-Touze, M. & Francou, G. 2002,
///     Astron.Astrophys. 387, 700
///
///     Lieske, J.H., Lederle, T., Fricke, W. & Morando, B. 1977,
///     Astron.Astrophys. 58, 1-16
///
///     Mathews, P.M., Herring, T.A., Buffet, B.A. 2002, J.Geophys.Res.
///     107, B4.  The MHB_2000 code itself was obtained on 9th September
///     2002 from ftp//maia.usno.navy.mil/conv2000/chapter5/IAU2000A.
///
///     Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M.,
///     Francou, G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
///
///     Souchay, J., Loysel, B., Kinoshita, H., Folgueira, M. 1999,
///     Astron.Astrophys.Supp.Ser. 135, 111
///
///     Wallace, P.T., "Software for Implementing the IAU 2000
///     Resolutions", in IERS Workshop 5.1 (2002)
pub fn Nut00a(date1: f64, date2: f64) -> (f64, f64) {
    let mut dpsi: f64 = 0.0;
    let mut deps: f64 = 0.0;
    unsafe {
        eraNut00a(date1, date2, &mut dpsi, &mut deps);
    }

    return (dpsi, deps);
}

///  Nutation, IAU 2000B model.
///
///  Given:
///     date1,date2   double    TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     dpsi,deps     double    nutation, luni-solar + planetary (Note 2)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The nutation components in longitude and obliquity are in radians
///     and with respect to the equinox and ecliptic of date.  The
///     obliquity at J2000.0 is assumed to be the Lieske et al. (1977)
///     value of 84381.448 arcsec.  (The errors that result from using
///     this function with the IAU 2006 value of 84381.406 arcsec can be
///     neglected.)
///
///     The nutation model consists only of luni-solar terms, but
///     includes also a fixed offset which compensates for certain long-
///     period planetary terms (Note 7).
///
///  3) This function is an implementation of the IAU 2000B abridged
///     nutation model formally adopted by the IAU General Assembly in
///     2000.  The function computes the MHB_2000_SHORT luni-solar
///     nutation series (Luzum 2001), but without the associated
///     corrections for the precession rate adjustments and the offset
///     between the GCRS and J2000.0 mean poles.
///
///  4) The full IAU 2000A (MHB2000) nutation model contains nearly 1400
///     terms.  The IAU 2000B model (McCarthy & Luzum 2003) contains only
///     77 terms, plus additional simplifications, yet still delivers
///     results of 1 mas accuracy at present epochs.  This combination of
///     accuracy and size makes the IAU 2000B abridged nutation model
///     suitable for most practical applications.
///
///     The function delivers a pole accurate to 1 mas from 1900 to 2100
///     (usually better than 1 mas, very occasionally just outside
///     1 mas).  The full IAU 2000A model, which is implemented in the
///     function eraNut00a (q.v.), delivers considerably greater accuracy
///     at current dates;  however, to realize this improved accuracy,
///     corrections for the essentially unpredictable free-core-nutation
///     (FCN) must also be included.
///
///  5) The present function provides classical nutation.  The
///     MHB_2000_SHORT algorithm, from which it is adapted, deals also
///     with (i) the offsets between the GCRS and mean poles and (ii) the
///     adjustments in longitude and obliquity due to the changed
///     precession rates.  These additional functions, namely frame bias
///     and precession adjustments, are supported by the ERFA functions
///     eraBi00  and eraPr00.
///
///  6) The MHB_2000_SHORT algorithm also provides "total" nutations,
///     comprising the arithmetic sum of the frame bias, precession
///     adjustments, and nutation (luni-solar + planetary).  These total
///     nutations can be used in combination with an existing IAU 1976
///     precession implementation, such as eraPmat76,  to deliver GCRS-
///     to-true predictions of mas accuracy at current epochs.  However,
///     for symmetry with the eraNut00a  function (q.v. for the reasons),
///     the ERFA functions do not generate the "total nutations"
///     directly.  Should they be required, they could of course easily
///     be generated by calling eraBi00, eraPr00 and the present function
///     and adding the results.
///
///  7) The IAU 2000B model includes "planetary bias" terms that are
///     fixed in size but compensate for long-period nutations.  The
///     amplitudes quoted in McCarthy & Luzum (2003), namely
///     Dpsi = -1.5835 mas and Depsilon = +1.6339 mas, are optimized for
///     the "total nutations" method described in Note 6.  The Luzum
///     (2001) values used in this ERFA implementation, namely -0.135 mas
///     and +0.388 mas, are optimized for the "rigorous" method, where
///     frame bias, precession and nutation are applied separately and in
///     that order.  During the interval 1995-2050, the ERFA
///     implementation delivers a maximum error of 1.001 mas (not
///     including FCN).
///
///  References:
///
///     Lieske, J.H., Lederle, T., Fricke, W., Morando, B., "Expressions
///     for the precession quantities based upon the IAU /1976/ system of
///     astronomical constants", Astron.Astrophys. 58, 1-2, 1-16. (1977)
///
///     Luzum, B., private communication, 2001 (Fortran code
///     MHB_2000_SHORT)
///
///     McCarthy, D.D. & Luzum, B.J., "An abridged model of the
///     precession-nutation of the celestial pole", Cel.Mech.Dyn.Astron.
///     85, 37-49 (2003)
///
///     Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M.,
///     Francou, G., Laskar, J., Astron.Astrophys. 282, 663-683 (1994)
pub fn Nut00b(date1: f64, date2: f64) -> (f64, f64) {
    let mut dpsi: f64 = 0.0;
    let mut deps: f64 = 0.0;
    unsafe {
        eraNut00b(date1, date2, &mut dpsi, &mut deps);
    }

    return (dpsi, deps);
}

///  IAU 2000A nutation with adjustments to match the IAU 2006
///  precession.
///
///  Given:
///     date1,date2   double   TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     dpsi,deps     double   nutation, luni-solar + planetary (Note 2)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The nutation components in longitude and obliquity are in radians
///     and with respect to the mean equinox and ecliptic of date,
///     IAU 2006 precession model (Hilton et al. 2006, Capitaine et al.
///     2005).
///
///  3) The function first computes the IAU 2000A nutation, then applies
///     adjustments for (i) the consequences of the change in obliquity
///     from the IAU 1980 ecliptic to the IAU 2006 ecliptic and (ii) the
///     secular variation in the Earth's dynamical form factor J2.
///
///  4) The present function provides classical nutation, complementing
///     the IAU 2000 frame bias and IAU 2006 precession.  It delivers a
///     pole which is at current epochs accurate to a few tens of
///     microarcseconds, apart from the free core nutation.
///
///  Called:
///     eraNut00a    nutation, IAU 2000A
///
///  References:
///
///     Chapront, J., Chapront-Touze, M. & Francou, G. 2002,
///     Astron.Astrophys. 387, 700
///
///     Lieske, J.H., Lederle, T., Fricke, W. & Morando, B. 1977,
///     Astron.Astrophys. 58, 1-16
///
///     Mathews, P.M., Herring, T.A., Buffet, B.A. 2002, J.Geophys.Res.
///     107, B4.  The MHB_2000 code itself was obtained on 9th September
///     2002 from ftp//maia.usno.navy.mil/conv2000/chapter5/IAU2000A.
///
///     Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M.,
///     Francou, G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
///
///     Souchay, J., Loysel, B., Kinoshita, H., Folgueira, M. 1999,
///     Astron.Astrophys.Supp.Ser. 135, 111
///
///     Wallace, P.T., "Software for Implementing the IAU 2000
///     Resolutions", in IERS Workshop 5.1 (2002)
pub fn Nut06a(date1: f64, date2: f64) -> (f64, f64) {
    let mut dpsi: f64 = 0.0;
    let mut deps: f64 = 0.0;
    unsafe {
        eraNut06a(date1, date2, &mut dpsi, &mut deps);
    }

    return (dpsi, deps);
}

///  Nutation, IAU 1980 model.
///
///  Given:
///     date1,date2   double    TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     dpsi          double    nutation in longitude (radians)
///     deps          double    nutation in obliquity (radians)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The nutation components are with respect to the ecliptic of
///     date.
///
///  Called:
///     eraAnpm      normalize angle into range +/- pi
///
///  Reference:
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992),
///     Section 3.222 (p111).
pub fn Nut80(date1: f64, date2: f64) -> (f64, f64) {
    let mut dpsi: f64 = 0.0;
    let mut deps: f64 = 0.0;
    unsafe {
        eraNut80(date1, date2, &mut dpsi, &mut deps);
    }

    return (dpsi, deps);
}

///  Form the matrix of nutation for a given date, IAU 1980 model.
///
///  Given:
///     date1,date2    double          TDB date (Note 1)
///
///  Returned:
///     rmatn          double[3][3]    nutation matrix
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The matrix operates in the sense V(true) = rmatn * V(mean),
///     where the p-vector V(true) is with respect to the true
///     equatorial triad of date and the p-vector V(mean) is with
///     respect to the mean equatorial triad of date.
///
///  Called:
///     eraNut80     nutation, IAU 1980
///     eraObl80     mean obliquity, IAU 1980
///     eraNumat     form nutation matrix
pub fn Nutm80(date1: f64, date2: f64, rmatn: &mut [f64; 9]) {
    unsafe {
        eraNutm80(date1, date2, rmatn);
    }
}

///  Mean obliquity of the ecliptic, IAU 2006 precession model.
///
///  Given:
///     date1,date2  double   TT as a 2-part Julian Date (Note 1)
///
///  Returned (function value):
///                  double   obliquity of the ecliptic (radians, Note 2)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The result is the angle between the ecliptic and mean equator of
///     date date1+date2.
///
///  Reference:
///
///     Hilton, J. et al., 2006, Celest.Mech.Dyn.Astron. 94, 351
pub fn Obl06(date1: f64, date2: f64) -> f64 {
    return unsafe { eraObl06(date1, date2) };
}

///  Mean obliquity of the ecliptic, IAU 1980 model.
///
///  Given:
///     date1,date2   double    TT as a 2-part Julian Date (Note 1)
///
///  Returned (function value):
///                   double    obliquity of the ecliptic (radians, Note 2)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The result is the angle between the ecliptic and mean equator of
///     date date1+date2.
///
///  Reference:
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992),
///     Expression 3.222-1 (p114).
pub fn Obl80(date1: f64, date2: f64) -> f64 {
    return unsafe { eraObl80(date1, date2) };
}

///  Precession angles, IAU 2006, equinox based.
///
///  Given:
///     date1,date2   double   TT as a 2-part Julian Date (Note 1)
///
///  Returned (see Note 2):
///     eps0          double   epsilon_0
///     psia          double   psi_A
///     oma           double   omega_A
///     bpa           double   P_A
///     bqa           double   Q_A
///     pia           double   pi_A
///     bpia          double   Pi_A
///     epsa          double   obliquity epsilon_A
///     chia          double   chi_A
///     za            double   z_A
///     zetaa         double   zeta_A
///     thetaa        double   theta_A
///     pa            double   p_A
///     gam           double   F-W angle gamma_J2000
///     phi           double   F-W angle phi_J2000
///     psi           double   F-W angle psi_J2000
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) This function returns the set of equinox based angles for the
///     Capitaine et al. "P03" precession theory, adopted by the IAU in
///     2006.  The angles are set out in Table 1 of Hilton et al. (2006):
///
///     eps0   epsilon_0   obliquity at J2000.0
///     psia   psi_A       luni-solar precession
///     oma    omega_A     inclination of equator wrt J2000.0 ecliptic
///     bpa    P_A         ecliptic pole x, J2000.0 ecliptic triad
///     bqa    Q_A         ecliptic pole -y, J2000.0 ecliptic triad
///     pia    pi_A        angle between moving and J2000.0 ecliptics
///     bpia   Pi_A        longitude of ascending node of the ecliptic
///     epsa   epsilon_A   obliquity of the ecliptic
///     chia   chi_A       planetary precession
///     za     z_A         equatorial precession: -3rd 323 Euler angle
///     zetaa  zeta_A      equatorial precession: -1st 323 Euler angle
///     thetaa theta_A     equatorial precession: 2nd 323 Euler angle
///     pa     p_A         general precession (n.b. see below)
///     gam    gamma_J2000 J2000.0 RA difference of ecliptic poles
///     phi    phi_J2000   J2000.0 codeclination of ecliptic pole
///     psi    psi_J2000   longitude difference of equator poles, J2000.0
///
///     The returned values are all radians.
///
///     Note that the t^5 coefficient in the series for p_A from
///     Capitaine et al. (2003) is incorrectly signed in Hilton et al.
///     (2006).
///
///  3) Hilton et al. (2006) Table 1 also contains angles that depend on
///     models distinct from the P03 precession theory itself, namely the
///     IAU 2000A frame bias and nutation.  The quoted polynomials are
///     used in other ERFA functions:
///
///     . eraXy06  contains the polynomial parts of the X and Y series.
///
///     . eraS06  contains the polynomial part of the s+XY/2 series.
///
///     . eraPfw06  implements the series for the Fukushima-Williams
///       angles that are with respect to the GCRS pole (i.e. the variants
///       that include frame bias).
///
///  4) The IAU resolution stipulated that the choice of parameterization
///     was left to the user, and so an IAU compliant precession
///     implementation can be constructed using various combinations of
///     the angles returned by the present function.
///
///  5) The parameterization used by ERFA is the version of the Fukushima-
///     Williams angles that refers directly to the GCRS pole.  These
///     angles may be calculated by calling the function eraPfw06.  ERFA
///     also supports the direct computation of the CIP GCRS X,Y by
///     series, available by calling eraXy06.
///
///  6) The agreement between the different parameterizations is at the
///     1 microarcsecond level in the present era.
///
///  7) When constructing a precession formulation that refers to the GCRS
///     pole rather than the dynamical pole, it may (depending on the
///     choice of angles) be necessary to introduce the frame bias
///     explicitly.
///
///  8) It is permissible to re-use the same variable in the returned
///     arguments.  The quantities are stored in the stated order.
///
///  References:
///
///     Capitaine, N., Wallace, P.T. & Chapront, J., 2003,
///     Astron.Astrophys., 412, 567
///
///     Hilton, J. et al., 2006, Celest.Mech.Dyn.Astron. 94, 351
///
///  Called:
///     eraObl06     mean obliquity, IAU 2006
pub fn P06e(
    date1: f64,
    date2: f64,
) -> (
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
    f64,
) {
    let mut eps0: f64 = 0.0;
    let mut psia: f64 = 0.0;
    let mut oma: f64 = 0.0;
    let mut bpa: f64 = 0.0;
    let mut bqa: f64 = 0.0;
    let mut pia: f64 = 0.0;
    let mut bpia: f64 = 0.0;
    let mut epsa: f64 = 0.0;
    let mut chia: f64 = 0.0;
    let mut za: f64 = 0.0;
    let mut zetaa: f64 = 0.0;
    let mut thetaa: f64 = 0.0;
    let mut pa: f64 = 0.0;
    let mut gam: f64 = 0.0;
    let mut phi: f64 = 0.0;
    let mut psi: f64 = 0.0;

    unsafe {
        eraP06e(
            date1,
            date2,
            &mut eps0,
            &mut psia,
            &mut oma,
            &mut bpa,
            &mut bqa,
            &mut pia,
            &mut bpia,
            &mut epsa,
            &mut chia,
            &mut za,
            &mut zetaa,
            &mut thetaa,
            &mut pa,
            &mut gam,
            &mut phi,
            &mut psi,
        );
    }

    return (
        eps0, psia, oma, bpa, bqa, pia, bpia, epsa, chia, za, zetaa, thetaa, pa, gam, phi, psi,
    );
}

///  This function forms three Euler angles which implement general
///  precession from epoch J2000.0, using the IAU 2006 model.  Frame
///  bias (the offset between ICRS and mean J2000.0) is included.
///
///  Given:
///     date1,date2  double   TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     bzeta        double   1st rotation: radians cw around z
///     bz           double   3rd rotation: radians cw around z
///     btheta       double   2nd rotation: radians ccw around y
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The traditional accumulated precession angles zeta_A, z_A,
///     theta_A cannot be obtained in the usual way, namely through
///     polynomial expressions, because of the frame bias.  The latter
///     means that two of the angles undergo rapid changes near this
///     date.  They are instead the results of decomposing the
///     precession-bias matrix obtained by using the Fukushima-Williams
///     method, which does not suffer from the problem.  The
///     decomposition returns values which can be used in the
///     conventional formulation and which include frame bias.
///
///  3) The three angles are returned in the conventional order, which
///     is not the same as the order of the corresponding Euler
///     rotations.  The precession-bias matrix is
///     R_3(-z) x R_2(+theta) x R_3(-zeta).
///
///  4) Should zeta_A, z_A, theta_A angles be required that do not
///     contain frame bias, they are available by calling the ERFA
///     function eraP06e.
///
///  Called:
///     eraPmat06    PB matrix, IAU 2006
///     eraRz        rotate around Z-axis
pub fn Pb06(date1: f64, date2: f64) -> (f64, f64, f64) {
    let mut bzeta: f64 = 0.0;
    let mut bz: f64 = 0.0;
    let mut btheta: f64 = 0.0;

    unsafe {
        eraPb06(date1, date2, &mut bzeta, &mut bz, &mut btheta);
    }

    return (bzeta, bz, btheta);
}

///  Precession angles, IAU 2006 (Fukushima-Williams 4-angle formulation).
///
///  Given:
///     date1,date2  double   TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     gamb         double   F-W angle gamma_bar (radians)
///     phib         double   F-W angle phi_bar (radians)
///     psib         double   F-W angle psi_bar (radians)
///     epsa         double   F-W angle epsilon_A (radians)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) Naming the following points:
///
///           e = J2000.0 ecliptic pole,
///           p = GCRS pole,
///           E = mean ecliptic pole of date,
///     and   P = mean pole of date,
///
///     the four Fukushima-Williams angles are as follows:
///
///        gamb = gamma_bar = epE
///        phib = phi_bar = pE
///        psib = psi_bar = pEP
///        epsa = epsilon_A = EP
///
///  3) The matrix representing the combined effects of frame bias and
///     precession is:
///
///        PxB = R_1(-epsa).R_3(-psib).R_1(phib).R_3(gamb)
///
///  4) The matrix representing the combined effects of frame bias,
///     precession and nutation is simply:
///
///        NxPxB = R_1(-epsa-dE).R_3(-psib-dP).R_1(phib).R_3(gamb)
///
///     where dP and dE are the nutation components with respect to the
///     ecliptic of date.
///
///  Reference:
///
///     Hilton, J. et al., 2006, Celest.Mech.Dyn.Astron. 94, 351
///
///  Called:
///     eraObl06     mean obliquity, IAU 2006
pub fn Pfw06(date1: f64, date2: f64) -> (f64, f64, f64, f64) {
    let mut gamb: f64 = 0.0;
    let mut phib: f64 = 0.0;
    let mut psib: f64 = 0.0;
    let mut epsa: f64 = 0.0;

    unsafe {
        eraPfw06(date1, date2, &mut gamb, &mut phib, &mut psib, &mut epsa);
    }

    return (gamb, phib, psib, epsa);
}

///  Precession matrix (including frame bias) from GCRS to a specified
///  date, IAU 2000 model.
///
///  Given:
///     date1,date2  double          TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     rbp          double[3][3]    bias-precession matrix (Note 2)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The matrix operates in the sense V(date) = rbp * V(GCRS), where
///     the p-vector V(GCRS) is with respect to the Geocentric Celestial
///     Reference System (IAU, 2000) and the p-vector V(date) is with
///     respect to the mean equatorial triad of the given date.
///
///  Called:
///     eraBp00      frame bias and precession matrices, IAU 2000
///
///  Reference:
///
///     IAU: Trans. International Astronomical Union, Vol. XXIVB;  Proc.
///     24th General Assembly, Manchester, UK.  Resolutions B1.3, B1.6.
///     (2000)
pub fn Pmat00(date1: f64, date2: f64, rbp: &mut [f64; 9]) {
    unsafe {
        eraPmat00(date1, date2, rbp);
    }
}

///  Precession matrix (including frame bias) from GCRS to a specified
///  date, IAU 2006 model.
///
///  Given:
///     date1,date2  double          TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     rbp          double[3][3]    bias-precession matrix (Note 2)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The matrix operates in the sense V(date) = rbp * V(GCRS), where
///     the p-vector V(GCRS) is with respect to the Geocentric Celestial
///     Reference System (IAU, 2000) and the p-vector V(date) is with
///     respect to the mean equatorial triad of the given date.
///
///  Called:
///     eraPfw06     bias-precession F-W angles, IAU 2006
///     eraFw2m      F-W angles to r-matrix
///
///  References:
///
///     Capitaine, N. & Wallace, P.T., 2006, Astron.Astrophys. 450, 855
///
///     IAU: Trans. International Astronomical Union, Vol. XXIVB;  Proc.
///     24th General Assembly, Manchester, UK.  Resolutions B1.3, B1.6.
///     (2000)
///
///     Wallace, P.T. & Capitaine, N., 2006, Astron.Astrophys. 459, 981
pub fn Pmat06(date1: f64, date2: f64, rbp: &mut [f64; 9]) {
    unsafe {
        eraPmat06(date1, date2, rbp);
    }
}

///  Precession matrix from J2000.0 to a specified date, IAU 1976 model.
///
///  Given:
///     date1,date2 double       ending date, TT (Note 1)
///
///  Returned:
///     rmatp       double[3][3] precession matrix, J2000.0 -> date1+date2
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The matrix operates in the sense V(date) = RMATP * V(J2000),
///     where the p-vector V(J2000) is with respect to the mean
///     equatorial triad of epoch J2000.0 and the p-vector V(date)
///     is with respect to the mean equatorial triad of the given
///     date.
///
///  3) Though the matrix method itself is rigorous, the precession
///     angles are expressed through canonical polynomials which are
///     valid only for a limited time span.  In addition, the IAU 1976
///     precession rate is known to be imperfect.  The absolute accuracy
///     of the present formulation is better than 0.1 arcsec from
///     1960AD to 2040AD, better than 1 arcsec from 1640AD to 2360AD,
///     and remains below 3 arcsec for the whole of the period
///     500BC to 3000AD.  The errors exceed 10 arcsec outside the
///     range 1200BC to 3900AD, exceed 100 arcsec outside 4200BC to
///     5600AD and exceed 1000 arcsec outside 6800BC to 8200AD.
///
///  Called:
///     eraPrec76    accumulated precession angles, IAU 1976
///     eraIr        initialize r-matrix to identity
///     eraRz        rotate around Z-axis
///     eraRy        rotate around Y-axis
///     eraCr        copy r-matrix
///
///  References:
///
///     Lieske, J.H., 1979, Astron.Astrophys. 73, 282.
///      equations (6) & (7), p283.
///
///     Kaplan,G.H., 1981. USNO circular no. 163, pA2.
pub fn Pmat76(date1: f64, date2: f64, rmatp: &mut [f64; 9]) {
    unsafe {
        eraPmat76(date1, date2, rmatp);
    }
}

///  Precession-nutation, IAU 2000 model:  a multi-purpose function,
///  supporting classical (equinox-based) use directly and CIO-based
///  use indirectly.
///
///  Given:
///     date1,date2  double          TT as a 2-part Julian Date (Note 1)
///     dpsi,deps    double          nutation (Note 2)
///
///  Returned:
///     epsa         double          mean obliquity (Note 3)
///     rb           double[3][3]    frame bias matrix (Note 4)
///     rp           double[3][3]    precession matrix (Note 5)
///     rbp          double[3][3]    bias-precession matrix (Note 6)
///     rn           double[3][3]    nutation matrix (Note 7)
///     rbpn         double[3][3]    GCRS-to-true matrix (Note 8)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The caller is responsible for providing the nutation components;
///     they are in longitude and obliquity, in radians and are with
///     respect to the equinox and ecliptic of date.  For high-accuracy
///     applications, free core nutation should be included as well as
///     any other relevant corrections to the position of the CIP.
///
///  3) The returned mean obliquity is consistent with the IAU 2000
///     precession-nutation models.
///
///  4) The matrix rb transforms vectors from GCRS to J2000.0 mean
///     equator and equinox by applying frame bias.
///
///  5) The matrix rp transforms vectors from J2000.0 mean equator and
///     equinox to mean equator and equinox of date by applying
///     precession.
///
///  6) The matrix rbp transforms vectors from GCRS to mean equator and
///     equinox of date by applying frame bias then precession.  It is
///     the product rp x rb.
///
///  7) The matrix rn transforms vectors from mean equator and equinox of
///     date to true equator and equinox of date by applying the nutation
///     (luni-solar + planetary).
///
///  8) The matrix rbpn transforms vectors from GCRS to true equator and
///     equinox of date.  It is the product rn x rbp, applying frame
///     bias, precession and nutation in that order.
///
///  9) It is permissible to re-use the same array in the returned
///     arguments.  The arrays are filled in the order given.
///
///  Called:
///     eraPr00      IAU 2000 precession adjustments
///     eraObl80     mean obliquity, IAU 1980
///     eraBp00      frame bias and precession matrices, IAU 2000
///     eraCr        copy r-matrix
///     eraNumat     form nutation matrix
///     eraRxr       product of two r-matrices
///
///  Reference:
///
///     Capitaine, N., Chapront, J., Lambert, S. and Wallace, P.,
///     "Expressions for the Celestial Intermediate Pole and Celestial
///     Ephemeris Origin consistent with the IAU 2000A precession-
///     nutation model", Astron.Astrophys. 400, 1145-1154 (2003)
///
///     n.b. The celestial ephemeris origin (CEO) was renamed "celestial
///          intermediate origin" (CIO) by IAU 2006 Resolution 2.
pub fn Pn00(
    date1: f64,
    date2: f64,
    dpsi: f64,
    deps: f64,
    epsa: &mut f64,
    rb: &mut [f64; 9],
    rp: &mut [f64; 9],
    rbp: &mut [f64; 9],
    rn: &mut [f64; 9],
    rbpn: &mut [f64; 9],
) {
    unsafe {
        eraPn00(date1, date2, dpsi, deps, epsa, rb, rp, rbp, rn, rbpn);
    }
}

///  Precession-nutation, IAU 2000A model:  a multi-purpose function,
///  supporting classical (equinox-based) use directly and CIO-based
///  use indirectly.
///
///  Given:
///     date1,date2  double          TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     dpsi,deps    double          nutation (Note 2)
///     epsa         double          mean obliquity (Note 3)
///     rb           double[3][3]    frame bias matrix (Note 4)
///     rp           double[3][3]    precession matrix (Note 5)
///     rbp          double[3][3]    bias-precession matrix (Note 6)
///     rn           double[3][3]    nutation matrix (Note 7)
///     rbpn         double[3][3]    GCRS-to-true matrix (Notes 8,9)
///
///  Notes:
///
///  1)  The TT date date1+date2 is a Julian Date, apportioned in any
///      convenient way between the two arguments.  For example,
///      JD(TT)=2450123.7 could be expressed in any of these ways,
///      among others:
///
///             date1          date2
///
///          2450123.7           0.0       (JD method)
///          2451545.0       -1421.3       (J2000 method)
///          2400000.5       50123.2       (MJD method)
///          2450123.5           0.2       (date & time method)
///
///      The JD method is the most natural and convenient to use in
///      cases where the loss of several decimal digits of resolution
///      is acceptable.  The J2000 method is best matched to the way
///      the argument is handled internally and will deliver the
///      optimum resolution.  The MJD method and the date & time methods
///      are both good compromises between resolution and convenience.
///
///  2)  The nutation components (luni-solar + planetary, IAU 2000A) in
///      longitude and obliquity are in radians and with respect to the
///      equinox and ecliptic of date.  Free core nutation is omitted;
///      for the utmost accuracy, use the eraPn00 function, where the
///      nutation components are caller-specified.  For faster but
///      slightly less accurate results, use the eraPn00b function.
///
///  3)  The mean obliquity is consistent with the IAU 2000 precession.
///
///  4)  The matrix rb transforms vectors from GCRS to J2000.0 mean
///      equator and equinox by applying frame bias.
///
///  5)  The matrix rp transforms vectors from J2000.0 mean equator and
///      equinox to mean equator and equinox of date by applying
///      precession.
///
///  6)  The matrix rbp transforms vectors from GCRS to mean equator and
///      equinox of date by applying frame bias then precession.  It is
///      the product rp x rb.
///
///  7)  The matrix rn transforms vectors from mean equator and equinox
///      of date to true equator and equinox of date by applying the
///      nutation (luni-solar + planetary).
///
///  8)  The matrix rbpn transforms vectors from GCRS to true equator and
///      equinox of date.  It is the product rn x rbp, applying frame
///      bias, precession and nutation in that order.
///
///  9)  The X,Y,Z coordinates of the IAU 2000A Celestial Intermediate
///      Pole are elements (3,1-3) of the GCRS-to-true matrix,
///      i.e. rbpn[2][0-2].
///
///  10) It is permissible to re-use the same array in the returned
///      arguments.  The arrays are filled in the stated order.
///
///  Called:
///     eraNut00a    nutation, IAU 2000A
///     eraPn00      bias/precession/nutation results, IAU 2000
///
///  Reference:
///
///     Capitaine, N., Chapront, J., Lambert, S. and Wallace, P.,
///     "Expressions for the Celestial Intermediate Pole and Celestial
///     Ephemeris Origin consistent with the IAU 2000A precession-
///     nutation model", Astron.Astrophys. 400, 1145-1154 (2003)
///
///     n.b. The celestial ephemeris origin (CEO) was renamed "celestial
///          intermediate origin" (CIO) by IAU 2006 Resolution 2.
pub fn Pn00a(
    date1: f64,
    date2: f64,
    dpsi: &mut f64,
    deps: &mut f64,
    epsa: &mut f64,
    rb: &mut [f64; 9],
    rp: &mut [f64; 9],
    rbp: &mut [f64; 9],
    rn: &mut [f64; 9],
    rbpn: &mut [f64; 9],
) {
    unsafe {
        eraPn00a(date1, date2, dpsi, deps, epsa, rb, rp, rbp, rn, rbpn);
    }
}

///  Precession-nutation, IAU 2000B model:  a multi-purpose function,
///  supporting classical (equinox-based) use directly and CIO-based
///  use indirectly.
///
///  Given:
///     date1,date2  double          TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     dpsi,deps    double          nutation (Note 2)
///     epsa         double          mean obliquity (Note 3)
///     rb           double[3][3]    frame bias matrix (Note 4)
///     rp           double[3][3]    precession matrix (Note 5)
///     rbp          double[3][3]    bias-precession matrix (Note 6)
///     rn           double[3][3]    nutation matrix (Note 7)
///     rbpn         double[3][3]    GCRS-to-true matrix (Notes 8,9)
///
///  Notes:
///
///  1)  The TT date date1+date2 is a Julian Date, apportioned in any
///      convenient way between the two arguments.  For example,
///      JD(TT)=2450123.7 could be expressed in any of these ways,
///      among others:
///
///             date1          date2
///
///          2450123.7           0.0       (JD method)
///          2451545.0       -1421.3       (J2000 method)
///          2400000.5       50123.2       (MJD method)
///          2450123.5           0.2       (date & time method)
///
///      The JD method is the most natural and convenient to use in
///      cases where the loss of several decimal digits of resolution
///      is acceptable.  The J2000 method is best matched to the way
///      the argument is handled internally and will deliver the
///      optimum resolution.  The MJD method and the date & time methods
///      are both good compromises between resolution and convenience.
///
///  2)  The nutation components (luni-solar + planetary, IAU 2000B) in
///      longitude and obliquity are in radians and with respect to the
///      equinox and ecliptic of date.  For more accurate results, but
///      at the cost of increased computation, use the eraPn00a function.
///      For the utmost accuracy, use the eraPn00 function, where the
///      nutation components are caller-specified.
///
///  3)  The mean obliquity is consistent with the IAU 2000 precession.
///
///  4)  The matrix rb transforms vectors from GCRS to J2000.0 mean
///      equator and equinox by applying frame bias.
///
///  5)  The matrix rp transforms vectors from J2000.0 mean equator and
///      equinox to mean equator and equinox of date by applying
///      precession.
///
///  6)  The matrix rbp transforms vectors from GCRS to mean equator and
///      equinox of date by applying frame bias then precession.  It is
///      the product rp x rb.
///
///  7)  The matrix rn transforms vectors from mean equator and equinox
///      of date to true equator and equinox of date by applying the
///      nutation (luni-solar + planetary).
///
///  8)  The matrix rbpn transforms vectors from GCRS to true equator and
///      equinox of date.  It is the product rn x rbp, applying frame
///      bias, precession and nutation in that order.
///
///  9)  The X,Y,Z coordinates of the IAU 2000B Celestial Intermediate
///      Pole are elements (3,1-3) of the GCRS-to-true matrix,
///      i.e. rbpn[2][0-2].
///
///  10) It is permissible to re-use the same array in the returned
///      arguments.  The arrays are filled in the stated order.
///
///  Called:
///     eraNut00b    nutation, IAU 2000B
///     eraPn00      bias/precession/nutation results, IAU 2000
///
///  Reference:
///
///     Capitaine, N., Chapront, J., Lambert, S. and Wallace, P.,
///     "Expressions for the Celestial Intermediate Pole and Celestial
///     Ephemeris Origin consistent with the IAU 2000A precession-
///     nutation model", Astron.Astrophys. 400, 1145-1154 (2003).
///
///     n.b. The celestial ephemeris origin (CEO) was renamed "celestial
///          intermediate origin" (CIO) by IAU 2006 Resolution 2.
pub fn Pn00b(
    date1: f64,
    date2: f64,
    dpsi: &mut f64,
    deps: &mut f64,
    epsa: &mut f64,
    rb: &mut [f64; 9],
    rp: &mut [f64; 9],
    rbp: &mut [f64; 9],
    rn: &mut [f64; 9],
    rbpn: &mut [f64; 9],
) {
    unsafe {
        eraPn00b(date1, date2, dpsi, deps, epsa, rb, rp, rbp, rn, rbpn);
    }
}

///  Precession-nutation, IAU 2006 model:  a multi-purpose function,
///  supporting classical (equinox-based) use directly and CIO-based use
///  indirectly.
///
///  Given:
///     date1,date2  double          TT as a 2-part Julian Date (Note 1)
///     dpsi,deps    double          nutation (Note 2)
///
///  Returned:
///     epsa         double          mean obliquity (Note 3)
///     rb           double[3][3]    frame bias matrix (Note 4)
///     rp           double[3][3]    precession matrix (Note 5)
///     rbp          double[3][3]    bias-precession matrix (Note 6)
///     rn           double[3][3]    nutation matrix (Note 7)
///     rbpn         double[3][3]    GCRS-to-true matrix (Notes 8,9)
///
///  Notes:
///
///  1)  The TT date date1+date2 is a Julian Date, apportioned in any
///      convenient way between the two arguments.  For example,
///      JD(TT)=2450123.7 could be expressed in any of these ways,
///      among others:
///
///             date1          date2
///
///          2450123.7           0.0       (JD method)
///          2451545.0       -1421.3       (J2000 method)
///          2400000.5       50123.2       (MJD method)
///          2450123.5           0.2       (date & time method)
///
///      The JD method is the most natural and convenient to use in
///      cases where the loss of several decimal digits of resolution
///      is acceptable.  The J2000 method is best matched to the way
///      the argument is handled internally and will deliver the
///      optimum resolution.  The MJD method and the date & time methods
///      are both good compromises between resolution and convenience.
///
///  2)  The caller is responsible for providing the nutation components;
///      they are in longitude and obliquity, in radians and are with
///      respect to the equinox and ecliptic of date.  For high-accuracy
///      applications, free core nutation should be included as well as
///      any other relevant corrections to the position of the CIP.
///
///  3)  The returned mean obliquity is consistent with the IAU 2006
///      precession.
///
///  4)  The matrix rb transforms vectors from GCRS to J2000.0 mean
///      equator and equinox by applying frame bias.
///
///  5)  The matrix rp transforms vectors from J2000.0 mean equator and
///      equinox to mean equator and equinox of date by applying
///      precession.
///
///  6)  The matrix rbp transforms vectors from GCRS to mean equator and
///      equinox of date by applying frame bias then precession.  It is
///      the product rp x rb.
///
///  7)  The matrix rn transforms vectors from mean equator and equinox
///      of date to true equator and equinox of date by applying the
///      nutation (luni-solar + planetary).
///
///  8)  The matrix rbpn transforms vectors from GCRS to true equator and
///      equinox of date.  It is the product rn x rbp, applying frame
///      bias, precession and nutation in that order.
///
///  9)  The X,Y,Z coordinates of the Celestial Intermediate Pole are
///      elements (3,1-3) of the GCRS-to-true matrix, i.e. rbpn[2][0-2].
///
///  10) It is permissible to re-use the same array in the returned
///      arguments.  The arrays are filled in the stated order.
///
///  Called:
///     eraPfw06     bias-precession F-W angles, IAU 2006
///     eraFw2m      F-W angles to r-matrix
///     eraCr        copy r-matrix
///     eraTr        transpose r-matrix
///     eraRxr       product of two r-matrices
///
///  References:
///
///     Capitaine, N. & Wallace, P.T., 2006, Astron.Astrophys. 450, 855
///
///     Wallace, P.T. & Capitaine, N., 2006, Astron.Astrophys. 459, 981
pub fn Pn06(
    date1: f64,
    date2: f64,
    dpsi: f64,
    deps: f64,
    epsa: &mut f64,
    rb: &mut [f64; 9],
    rp: &mut [f64; 9],
    rbp: &mut [f64; 9],
    rn: &mut [f64; 9],
    rbpn: &mut [f64; 9],
) {
    unsafe {
        eraPn06(date1, date2, dpsi, deps, epsa, rb, rp, rbp, rn, rbpn);
    }
}

///  Precession-nutation, IAU 2006/2000A models:  a multi-purpose function,
///  supporting classical (equinox-based) use directly and CIO-based use
///  indirectly.
///
///  Given:
///     date1,date2  double          TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     dpsi,deps    double          nutation (Note 2)
///     epsa         double          mean obliquity (Note 3)
///     rb           double[3][3]    frame bias matrix (Note 4)
///     rp           double[3][3]    precession matrix (Note 5)
///     rbp          double[3][3]    bias-precession matrix (Note 6)
///     rn           double[3][3]    nutation matrix (Note 7)
///     rbpn         double[3][3]    GCRS-to-true matrix (Notes 8,9)
///
///  Notes:
///
///  1)  The TT date date1+date2 is a Julian Date, apportioned in any
///      convenient way between the two arguments.  For example,
///      JD(TT)=2450123.7 could be expressed in any of these ways,
///      among others:
///
///             date1          date2
///
///          2450123.7           0.0       (JD method)
///          2451545.0       -1421.3       (J2000 method)
///          2400000.5       50123.2       (MJD method)
///          2450123.5           0.2       (date & time method)
///
///      The JD method is the most natural and convenient to use in
///      cases where the loss of several decimal digits of resolution
///      is acceptable.  The J2000 method is best matched to the way
///      the argument is handled internally and will deliver the
///      optimum resolution.  The MJD method and the date & time methods
///      are both good compromises between resolution and convenience.
///
///  2)  The nutation components (luni-solar + planetary, IAU 2000A) in
///      longitude and obliquity are in radians and with respect to the
///      equinox and ecliptic of date.  Free core nutation is omitted;
///      for the utmost accuracy, use the eraPn06 function, where the
///      nutation components are caller-specified.
///
///  3)  The mean obliquity is consistent with the IAU 2006 precession.
///
///  4)  The matrix rb transforms vectors from GCRS to mean J2000.0 by
///      applying frame bias.
///
///  5)  The matrix rp transforms vectors from mean J2000.0 to mean of
///      date by applying precession.
///
///  6)  The matrix rbp transforms vectors from GCRS to mean of date by
///      applying frame bias then precession.  It is the product rp x rb.
///
///  7)  The matrix rn transforms vectors from mean of date to true of
///      date by applying the nutation (luni-solar + planetary).
///
///  8)  The matrix rbpn transforms vectors from GCRS to true of date
///      (CIP/equinox).  It is the product rn x rbp, applying frame bias,
///      precession and nutation in that order.
///
///  9)  The X,Y,Z coordinates of the IAU 2006/2000A Celestial
///      Intermediate Pole are elements (3,1-3) of the GCRS-to-true
///      matrix, i.e. rbpn[2][0-2].
///
///  10) It is permissible to re-use the same array in the returned
///      arguments.  The arrays are filled in the stated order.
///
///  Called:
///     eraNut06a    nutation, IAU 2006/2000A
///     eraPn06      bias/precession/nutation results, IAU 2006
///
///  Reference:
///
///     Capitaine, N. & Wallace, P.T., 2006, Astron.Astrophys. 450, 855
pub fn Pn06a(
    date1: f64,
    date2: f64,
    dpsi: &mut f64,
    deps: &mut f64,
    epsa: &mut f64,
    rb: &mut [f64; 9],
    rp: &mut [f64; 9],
    rbp: &mut [f64; 9],
    rn: &mut [f64; 9],
    rbpn: &mut [f64; 9],
) {
    unsafe {
        eraPn06a(date1, date2, dpsi, deps, epsa, rb, rp, rbp, rn, rbpn);
    }
}

///  Form the matrix of precession-nutation for a given date (including
///  frame bias), equinox based, IAU 2000A model.
///
///  Given:
///     date1,date2 double       TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     rbpn        double[3][3] bias-precession-nutation matrix (Note 2)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways, among
///     others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The matrix operates in the sense V(date) = rbpn * V(GCRS), where
///     the p-vector V(date) is with respect to the true equatorial triad
///     of date date1+date2 and the p-vector V(GCRS) is with respect to
///     the Geocentric Celestial Reference System (IAU, 2000).
///
///  3) A faster, but slightly less accurate, result (about 1 mas) can be
///     obtained by using instead the eraPnm00b function.
///
///  Called:
///     eraPn00a     bias/precession/nutation, IAU 2000A
///
///  Reference:
///
///     IAU: Trans. International Astronomical Union, Vol. XXIVB;  Proc.
///     24th General Assembly, Manchester, UK.  Resolutions B1.3, B1.6.
///     (2000)
pub fn Pnm00a(date1: f64, date2: f64, rbpn: &mut [f64; 9]) {
    unsafe {
        eraPnm00a(date1, date2, rbpn);
    }
}

///  Form the matrix of precession-nutation for a given date (including
///  frame bias), equinox-based, IAU 2000B model.
///
///  Given:
///     date1,date2 double       TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     rbpn        double[3][3] bias-precession-nutation matrix (Note 2)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways, among
///     others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The matrix operates in the sense V(date) = rbpn * V(GCRS), where
///     the p-vector V(date) is with respect to the true equatorial triad
///     of date date1+date2 and the p-vector V(GCRS) is with respect to
///     the Geocentric Celestial Reference System (IAU, 2000).
///
///  3) The present function is faster, but slightly less accurate (about
///     1 mas), than the eraPnm00a function.
///
///  Called:
///     eraPn00b     bias/precession/nutation, IAU 2000B
///
///  Reference:
///
///     IAU: Trans. International Astronomical Union, Vol. XXIVB;  Proc.
///     24th General Assembly, Manchester, UK.  Resolutions B1.3, B1.6.
///     (2000)
pub fn Pnm00b(date1: f64, date2: f64, rbpn: &mut [f64; 9]) {
    unsafe {
        eraPnm00b(date1, date2, rbpn);
    }
}

///  Form the matrix of precession-nutation for a given date (including
///  frame bias), equinox based, IAU 2006 precession and IAU 2000A
///  nutation models.
///
///  Given:
///     date1,date2 double       TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     rbpn        double[3][3] bias-precession-nutation matrix (Note 2)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways, among
///     others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The matrix operates in the sense V(date) = rbpn * V(GCRS), where
///     the p-vector V(date) is with respect to the true equatorial triad
///     of date date1+date2 and the p-vector V(GCRS) is with respect to
///     the Geocentric Celestial Reference System (IAU, 2000).
///
///  Called:
///     eraPfw06     bias-precession F-W angles, IAU 2006
///     eraNut06a    nutation, IAU 2006/2000A
///     eraFw2m      F-W angles to r-matrix
///
///  Reference:
///
///     Capitaine, N. & Wallace, P.T., 2006, Astron.Astrophys. 450, 855.
pub fn Pnm06a(date1: f64, date2: f64, rbpn: &mut [f64; 9]) {
    unsafe {
        eraPnm06a(date1, date2, rbpn);
    }
}

///  Form the matrix of precession/nutation for a given date, IAU 1976
///  precession model, IAU 1980 nutation model.
///
///  Given:
///     date1,date2 double       TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     rmatpn         double[3][3]   combined precession/nutation matrix
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The matrix operates in the sense V(date) = rmatpn * V(J2000),
///     where the p-vector V(date) is with respect to the true equatorial
///     triad of date date1+date2 and the p-vector V(J2000) is with
///     respect to the mean equatorial triad of epoch J2000.0.
///
///  Called:
///     eraPmat76    precession matrix, IAU 1976
///     eraNutm80    nutation matrix, IAU 1980
///     eraRxr       product of two r-matrices
///
///  Reference:
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992),
///     Section 3.3 (p145).
pub fn Pnm80(date1: f64, date2: f64, rmatpn: &mut [f64; 9]) {
    unsafe {
        eraPnm80(date1, date2, rmatpn);
    }
}

///  Form the matrix of polar motion for a given date, IAU 2000.
///
///  Given:
///     xp,yp    double    coordinates of the pole (radians, Note 1)
///     sp       double    the TIO locator s' (radians, Note 2)
///
///  Returned:
///     rpom     double[3][3]   polar-motion matrix (Note 3)
///
///  Notes:
///
///  1) The arguments xp and yp are the coordinates (in radians) of the
///     Celestial Intermediate Pole with respect to the International
///     Terrestrial Reference System (see IERS Conventions 2003),
///     measured along the meridians 0 and 90 deg west respectively.
///
///  2) The argument sp is the TIO locator s', in radians, which
///     positions the Terrestrial Intermediate Origin on the equator.  It
///     is obtained from polar motion observations by numerical
///     integration, and so is in essence unpredictable.  However, it is
///     dominated by a secular drift of about 47 microarcseconds per
///     century, and so can be taken into account by using s' = -47*t,
///     where t is centuries since J2000.0.  The function eraSp00
///     implements this approximation.
///
///  3) The matrix operates in the sense V(TRS) = rpom * V(CIP), meaning
///     that it is the final rotation when computing the pointing
///     direction to a celestial source.
///
///  Called:
///     eraIr        initialize r-matrix to identity
///     eraRz        rotate around Z-axis
///     eraRy        rotate around Y-axis
///     eraRx        rotate around X-axis
///
///  Reference:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
pub fn Pom00(xp: f64, yp: f64, sp: f64, rpom: &mut [f64; 9]) {
    unsafe {
        eraPom00(xp, yp, sp, rpom);
    }
}

///  Precession-rate part of the IAU 2000 precession-nutation models
///  (part of MHB2000).
///
///  Given:
///     date1,date2    double  TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     dpsipr,depspr  double  precession corrections (Notes 2,3)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The precession adjustments are expressed as "nutation
///     components", corrections in longitude and obliquity with respect
///     to the J2000.0 equinox and ecliptic.
///
///  3) Although the precession adjustments are stated to be with respect
///     to Lieske et al. (1977), the MHB2000 model does not specify which
///     set of Euler angles are to be used and how the adjustments are to
///     be applied.  The most literal and straightforward procedure is to
///     adopt the 4-rotation epsilon_0, psi_A, omega_A, xi_A option, and
///     to add dpsipr to psi_A and depspr to both omega_A and eps_A.
///
///  4) This is an implementation of one aspect of the IAU 2000A nutation
///     model, formally adopted by the IAU General Assembly in 2000,
///     namely MHB2000 (Mathews et al. 2002).
///
///  References:
///
///     Lieske, J.H., Lederle, T., Fricke, W. & Morando, B., "Expressions
///     for the precession quantities based upon the IAU (1976) System of
///     Astronomical Constants", Astron.Astrophys., 58, 1-16 (1977)
///
///     Mathews, P.M., Herring, T.A., Buffet, B.A., "Modeling of nutation
///     and precession   New nutation series for nonrigid Earth and
///     insights into the Earth's interior", J.Geophys.Res., 107, B4,
///     2002.  The MHB2000 code itself was obtained on 9th September 2002
///     from ftp://maia.usno.navy.mil/conv2000/chapter5/IAU2000A.
///
///     Wallace, P.T., "Software for Implementing the IAU 2000
///     Resolutions", in IERS Workshop 5.1 (2002).
pub fn Pr00(date1: f64, date2: f64) -> (f64, f64) {
    let mut dpsipr: f64 = 0.0;
    let mut depspr: f64 = 0.0;

    unsafe {
        eraPr00(date1, date2, &mut dpsipr, &mut depspr);
    }

    return (dpsipr, depspr);
}

///  IAU 1976 precession model.
///
///  This function forms the three Euler angles which implement general
///  precession between two dates, using the IAU 1976 model (as for the
///  FK5 catalog).
///
///  Given:
///     date01,date02   double    TDB starting date (Note 1)
///     date11,date12   double    TDB ending date (Note 1)
///
///  Returned:
///     zeta            double    1st rotation: radians cw around z
///     z               double    3rd rotation: radians cw around z
///     theta           double    2nd rotation: radians ccw around y
///
///  Notes:
///
///  1) The dates date01+date02 and date11+date12 are Julian Dates,
///     apportioned in any convenient way between the arguments daten1
///     and daten2.  For example, JD(TDB)=2450123.7 could be expressed in
///     any of these ways, among others:
///
///           daten1        daten2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in cases
///     where the loss of several decimal digits of resolution is
///     acceptable.  The J2000 method is best matched to the way the
///     argument is handled internally and will deliver the optimum
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///     The two dates may be expressed using different methods, but at
///     the risk of losing some resolution.
///
///  2) The accumulated precession angles zeta, z, theta are expressed
///     through canonical polynomials which are valid only for a limited
///     time span.  In addition, the IAU 1976 precession rate is known to
///     be imperfect.  The absolute accuracy of the present formulation
///     is better than 0.1 arcsec from 1960AD to 2040AD, better than
///     1 arcsec from 1640AD to 2360AD, and remains below 3 arcsec for
///     the whole of the period 500BC to 3000AD.  The errors exceed
///     10 arcsec outside the range 1200BC to 3900AD, exceed 100 arcsec
///     outside 4200BC to 5600AD and exceed 1000 arcsec outside 6800BC to
///     8200AD.
///
///  3) The three angles are returned in the conventional order, which
///     is not the same as the order of the corresponding Euler
///     rotations.  The precession matrix is
///     R_3(-z) x R_2(+theta) x R_3(-zeta).
///
///  Reference:
///
///     Lieske, J.H., 1979, Astron.Astrophys. 73, 282, equations
///     (6) & (7), p283.
pub fn Prec76(date01: f64, date02: f64, date11: f64, date12: f64) -> (f64, f64, f64) {
    let mut zeta: f64 = 0.0;
    let mut z: f64 = 0.0;
    let mut theta: f64 = 0.0;

    unsafe {
        eraPrec76(
            date01, date02, date11, date12, &mut zeta, &mut z, &mut theta,
        );
    }

    return (zeta, z, theta);
}

///  The CIO locator s, positioning the Celestial Intermediate Origin on
///  the equator of the Celestial Intermediate Pole, given the CIP's X,Y
///  coordinates.  Compatible with IAU 2000A precession-nutation.
///
///  Given:
///     date1,date2   double    TT as a 2-part Julian Date (Note 1)
///     x,y           double    CIP coordinates (Note 3)
///
///  Returned (function value):
///                   double    the CIO locator s in radians (Note 2)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The CIO locator s is the difference between the right ascensions
///     of the same point in two systems:  the two systems are the GCRS
///     and the CIP,CIO, and the point is the ascending node of the
///     CIP equator.  The quantity s remains below 0.1 arcsecond
///     throughout 1900-2100.
///
///  3) The series used to compute s is in fact for s+XY/2, where X and Y
///     are the x and y components of the CIP unit vector;  this series
///     is more compact than a direct series for s would be.  This
///     function requires X,Y to be supplied by the caller, who is
///     responsible for providing values that are consistent with the
///     supplied date.
///
///  4) The model is consistent with the IAU 2000A precession-nutation.
///
///  Called:
///     eraFal03     mean anomaly of the Moon
///     eraFalp03    mean anomaly of the Sun
///     eraFaf03     mean argument of the latitude of the Moon
///     eraFad03     mean elongation of the Moon from the Sun
///     eraFaom03    mean longitude of the Moon's ascending node
///     eraFave03    mean longitude of Venus
///     eraFae03     mean longitude of Earth
///     eraFapa03    general accumulated precession in longitude
///
///  References:
///
///     Capitaine, N., Chapront, J., Lambert, S. and Wallace, P.,
///     "Expressions for the Celestial Intermediate Pole and Celestial
///     Ephemeris Origin consistent with the IAU 2000A precession-
///     nutation model", Astron.Astrophys. 400, 1145-1154 (2003)
///
///     n.b. The celestial ephemeris origin (CEO) was renamed "celestial
///          intermediate origin" (CIO) by IAU 2006 Resolution 2.
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
pub fn S00(date1: f64, date2: f64, x: f64, y: f64) -> f64 {
    return unsafe { eraS00(date1, date2, x, y) };
}

///  The CIO locator s, positioning the Celestial Intermediate Origin on
///  the equator of the Celestial Intermediate Pole, using the IAU 2000A
///  precession-nutation model.
///
///  Given:
///     date1,date2  double    TT as a 2-part Julian Date (Note 1)
///
///  Returned (function value):
///                  double    the CIO locator s in radians (Note 2)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The CIO locator s is the difference between the right ascensions
///     of the same point in two systems.  The two systems are the GCRS
///     and the CIP,CIO, and the point is the ascending node of the
///     CIP equator.  The CIO locator s remains a small fraction of
///     1 arcsecond throughout 1900-2100.
///
///  3) The series used to compute s is in fact for s+XY/2, where X and Y
///     are the x and y components of the CIP unit vector;  this series
///     is more compact than a direct series for s would be.  The present
///     function uses the full IAU 2000A nutation model when predicting
///     the CIP position.  Faster results, with no significant loss of
///     accuracy, can be obtained via the function eraS00b, which uses
///     instead the IAU 2000B truncated model.
///
///  Called:
///     eraPnm00a    classical NPB matrix, IAU 2000A
///     eraBnp2xy    extract CIP X,Y from the BPN matrix
///     eraS00       the CIO locator s, given X,Y, IAU 2000A
///
///  References:
///
///     Capitaine, N., Chapront, J., Lambert, S. and Wallace, P.,
///     "Expressions for the Celestial Intermediate Pole and Celestial
///     Ephemeris Origin consistent with the IAU 2000A precession-
///     nutation model", Astron.Astrophys. 400, 1145-1154 (2003)
///
///     n.b. The celestial ephemeris origin (CEO) was renamed "celestial
///          intermediate origin" (CIO) by IAU 2006 Resolution 2.
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
pub fn S00a(date1: f64, date2: f64) -> f64 {
    return unsafe { eraS00a(date1, date2) };
}

///  The CIO locator s, positioning the Celestial Intermediate Origin on
///  the equator of the Celestial Intermediate Pole, using the IAU 2000B
///  precession-nutation model.
///
///  Given:
///     date1,date2  double    TT as a 2-part Julian Date (Note 1)
///
///  Returned (function value):
///                  double    the CIO locator s in radians (Note 2)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The CIO locator s is the difference between the right ascensions
///     of the same point in two systems.  The two systems are the GCRS
///     and the CIP,CIO, and the point is the ascending node of the
///     CIP equator.  The CIO locator s remains a small fraction of
///     1 arcsecond throughout 1900-2100.
///
///  3) The series used to compute s is in fact for s+XY/2, where X and Y
///     are the x and y components of the CIP unit vector;  this series
///     is more compact than a direct series for s would be.  The present
///     function uses the IAU 2000B truncated nutation model when
///     predicting the CIP position.  The function eraS00a uses instead
///     the full IAU 2000A model, but with no significant increase in
///     accuracy and at some cost in speed.
///
///  Called:
///     eraPnm00b    classical NPB matrix, IAU 2000B
///     eraBnp2xy    extract CIP X,Y from the BPN matrix
///     eraS00       the CIO locator s, given X,Y, IAU 2000A
///
///  References:
///
///     Capitaine, N., Chapront, J., Lambert, S. and Wallace, P.,
///     "Expressions for the Celestial Intermediate Pole and Celestial
///     Ephemeris Origin consistent with the IAU 2000A precession-
///     nutation model", Astron.Astrophys. 400, 1145-1154 (2003)
///
///     n.b. The celestial ephemeris origin (CEO) was renamed "celestial
///          intermediate origin" (CIO) by IAU 2006 Resolution 2.
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
pub fn S00b(date1: f64, date2: f64) -> f64 {
    return unsafe { eraS00b(date1, date2) };
}

///  The CIO locator s, positioning the Celestial Intermediate Origin on
///  the equator of the Celestial Intermediate Pole, given the CIP's X,Y
///  coordinates.  Compatible with IAU 2006/2000A precession-nutation.
///
///  Given:
///     date1,date2   double    TT as a 2-part Julian Date (Note 1)
///     x,y           double    CIP coordinates (Note 3)
///
///  Returned (function value):
///                   double    the CIO locator s in radians (Note 2)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The CIO locator s is the difference between the right ascensions
///     of the same point in two systems:  the two systems are the GCRS
///     and the CIP,CIO, and the point is the ascending node of the
///     CIP equator.  The quantity s remains below 0.1 arcsecond
///     throughout 1900-2100.
///
///  3) The series used to compute s is in fact for s+XY/2, where X and Y
///     are the x and y components of the CIP unit vector;  this series
///     is more compact than a direct series for s would be.  This
///     function requires X,Y to be supplied by the caller, who is
///     responsible for providing values that are consistent with the
///     supplied date.
///
///  4) The model is consistent with the "P03" precession (Capitaine et
///     al. 2003), adopted by IAU 2006 Resolution 1, 2006, and the
///     IAU 2000A nutation (with P03 adjustments).
///
///  Called:
///     eraFal03     mean anomaly of the Moon
///     eraFalp03    mean anomaly of the Sun
///     eraFaf03     mean argument of the latitude of the Moon
///     eraFad03     mean elongation of the Moon from the Sun
///     eraFaom03    mean longitude of the Moon's ascending node
///     eraFave03    mean longitude of Venus
///     eraFae03     mean longitude of Earth
///     eraFapa03    general accumulated precession in longitude
///
///  References:
///
///     Capitaine, N., Wallace, P.T. & Chapront, J., 2003, Astron.
///     Astrophys. 432, 355
///
///     McCarthy, D.D., Petit, G. (eds.) 2004, IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG
pub fn S06(date1: f64, date2: f64, x: f64, y: f64) -> f64 {
    return unsafe { eraS06(date1, date2, x, y) };
}

///  The CIO locator s, positioning the Celestial Intermediate Origin on
///  the equator of the Celestial Intermediate Pole, using the IAU 2006
///  precession and IAU 2000A nutation models.
///
///  Given:
///     date1,date2  double    TT as a 2-part Julian Date (Note 1)
///
///  Returned (function value):
///                  double    the CIO locator s in radians (Note 2)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The CIO locator s is the difference between the right ascensions
///     of the same point in two systems.  The two systems are the GCRS
///     and the CIP,CIO, and the point is the ascending node of the
///     CIP equator.  The CIO locator s remains a small fraction of
///     1 arcsecond throughout 1900-2100.
///
///  3) The series used to compute s is in fact for s+XY/2, where X and Y
///     are the x and y components of the CIP unit vector;  this series is
///     more compact than a direct series for s would be.  The present
///     function uses the full IAU 2000A nutation model when predicting
///     the CIP position.
///
///  Called:
///     eraPnm06a    classical NPB matrix, IAU 2006/2000A
///     eraBpn2xy    extract CIP X,Y coordinates from NPB matrix
///     eraS06       the CIO locator s, given X,Y, IAU 2006
///
///  References:
///
///     Capitaine, N., Chapront, J., Lambert, S. and Wallace, P.,
///     "Expressions for the Celestial Intermediate Pole and Celestial
///     Ephemeris Origin consistent with the IAU 2000A precession-
///     nutation model", Astron.Astrophys. 400, 1145-1154 (2003)
///
///     n.b. The celestial ephemeris origin (CEO) was renamed "celestial
///          intermediate origin" (CIO) by IAU 2006 Resolution 2.
///
///     Capitaine, N. & Wallace, P.T., 2006, Astron.Astrophys. 450, 855
///
///     McCarthy, D. D., Petit, G. (eds.), 2004, IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG
///
///     Wallace, P.T. & Capitaine, N., 2006, Astron.Astrophys. 459, 981
pub fn S06a(date1: f64, date2: f64) -> f64 {
    return unsafe { eraS06a(date1, date2) };
}

///  The TIO locator s', positioning the Terrestrial Intermediate Origin
///  on the equator of the Celestial Intermediate Pole.
///
///  Given:
///     date1,date2  double    TT as a 2-part Julian Date (Note 1)
///
///  Returned (function value):
///                  double    the TIO locator s' in radians (Note 2)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The TIO locator s' is obtained from polar motion observations by
///     numerical integration, and so is in essence unpredictable.
///     However, it is dominated by a secular drift of about
///     47 microarcseconds per century, which is the approximation
///     evaluated by the present function.
///
///  Reference:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
pub fn Sp00(date1: f64, date2: f64) -> f64 {
    return unsafe { eraSp00(date1, date2) };
}

///  X,Y coordinates of celestial intermediate pole from series based
///  on IAU 2006 precession and IAU 2000A nutation.
///
///  Given:
///     date1,date2  double     TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     x,y          double     CIP X,Y coordinates (Note 2)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The X,Y coordinates are those of the unit vector towards the
///     celestial intermediate pole.  They represent the combined effects
///     of frame bias, precession and nutation.
///
///  3) The fundamental arguments used are as adopted in IERS Conventions
///     (2003) and are from Simon et al. (1994) and Souchay et al.
///     (1999).
///
///  4) This is an alternative to the angles-based method, via the ERFA
///     function eraFw2xy and as used in eraXys06a for example.  The two
///     methods agree at the 1 microarcsecond level (at present), a
///     negligible amount compared with the intrinsic accuracy of the
///     models.  However, it would be unwise to mix the two methods
///     (angles-based and series-based) in a single application.
///
///  Called:
///     eraFal03     mean anomaly of the Moon
///     eraFalp03    mean anomaly of the Sun
///     eraFaf03     mean argument of the latitude of the Moon
///     eraFad03     mean elongation of the Moon from the Sun
///     eraFaom03    mean longitude of the Moon's ascending node
///     eraFame03    mean longitude of Mercury
///     eraFave03    mean longitude of Venus
///     eraFae03     mean longitude of Earth
///     eraFama03    mean longitude of Mars
///     eraFaju03    mean longitude of Jupiter
///     eraFasa03    mean longitude of Saturn
///     eraFaur03    mean longitude of Uranus
///     eraFane03    mean longitude of Neptune
///     eraFapa03    general accumulated precession in longitude
///
///  References:
///
///     Capitaine, N., Wallace, P.T. & Chapront, J., 2003,
///     Astron.Astrophys., 412, 567
///
///     Capitaine, N. & Wallace, P.T., 2006, Astron.Astrophys. 450, 855
///
///     McCarthy, D. D., Petit, G. (eds.), 2004, IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG
///
///     Simon, J.L., Bretagnon, P., Chapront, J., Chapront-Touze, M.,
///     Francou, G. & Laskar, J., Astron.Astrophys., 1994, 282, 663
///
///     Souchay, J., Loysel, B., Kinoshita, H., Folgueira, M., 1999,
///     Astron.Astrophys.Supp.Ser. 135, 111
///
///     Wallace, P.T. & Capitaine, N., 2006, Astron.Astrophys. 459, 981
pub fn Xy06(date1: f64, date2: f64) -> (f64, f64) {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;

    unsafe {
        eraXy06(date1, date2, &mut x, &mut y);
    }

    return (x, y);
}

///  For a given TT date, compute the X,Y coordinates of the Celestial
///  Intermediate Pole and the CIO locator s, using the IAU 2000A
///  precession-nutation model.
///
///  Given:
///     date1,date2  double   TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     x,y          double   Celestial Intermediate Pole (Note 2)
///     s            double   the CIO locator s (Note 3)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The Celestial Intermediate Pole coordinates are the x,y
///     components of the unit vector in the Geocentric Celestial
///     Reference System.
///
///  3) The CIO locator s (in radians) positions the Celestial
///     Intermediate Origin on the equator of the CIP.
///
///  4) A faster, but slightly less accurate result (about 1 mas for
///     X,Y), can be obtained by using instead the eraXys00b function.
///
///  Called:
///     eraPnm00a    classical NPB matrix, IAU 2000A
///     eraBpn2xy    extract CIP X,Y coordinates from NPB matrix
///     eraS00       the CIO locator s, given X,Y, IAU 2000A
///
///  Reference:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
pub fn Xys00a(date1: f64, date2: f64) -> (f64, f64, f64) {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut s: f64 = 0.0;

    unsafe {
        eraXys00a(date1, date2, &mut x, &mut y, &mut s);
    }

    return (x, y, s);
}

///  For a given TT date, compute the X,Y coordinates of the Celestial
///  Intermediate Pole and the CIO locator s, using the IAU 2000B
///  precession-nutation model.
///
///  Given:
///     date1,date2  double   TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     x,y          double   Celestial Intermediate Pole (Note 2)
///     s            double   the CIO locator s (Note 3)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The Celestial Intermediate Pole coordinates are the x,y
///     components of the unit vector in the Geocentric Celestial
///     Reference System.
///
///  3) The CIO locator s (in radians) positions the Celestial
///     Intermediate Origin on the equator of the CIP.
///
///  4) The present function is faster, but slightly less accurate (about
///     1 mas in X,Y), than the eraXys00a function.
///
///  Called:
///     eraPnm00b    classical NPB matrix, IAU 2000B
///     eraBpn2xy    extract CIP X,Y coordinates from NPB matrix
///     eraS00       the CIO locator s, given X,Y, IAU 2000A
///
///  Reference:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
pub fn Xys00b(date1: f64, date2: f64) -> (f64, f64, f64) {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut s: f64 = 0.0;

    unsafe {
        eraXys00b(date1, date2, &mut x, &mut y, &mut s);
    }

    return (x, y, s);
}

///  For a given TT date, compute the X,Y coordinates of the Celestial
///  Intermediate Pole and the CIO locator s, using the IAU 2006
///  precession and IAU 2000A nutation models.
///
///  Given:
///     date1,date2  double  TT as a 2-part Julian Date (Note 1)
///
///  Returned:
///     x,y          double  Celestial Intermediate Pole (Note 2)
///     s            double  the CIO locator s (Note 3)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The Celestial Intermediate Pole coordinates are the x,y components
///     of the unit vector in the Geocentric Celestial Reference System.
///
///  3) The CIO locator s (in radians) positions the Celestial
///     Intermediate Origin on the equator of the CIP.
///
///  4) Series-based solutions for generating X and Y are also available:
///     see Capitaine & Wallace (2006) and eraXy06.
///
///  Called:
///     eraPnm06a    classical NPB matrix, IAU 2006/2000A
///     eraBpn2xy    extract CIP X,Y coordinates from NPB matrix
///     eraS06       the CIO locator s, given X,Y, IAU 2006
///
///  References:
///
///     Capitaine, N. & Wallace, P.T., 2006, Astron.Astrophys. 450, 855
///
///     Wallace, P.T. & Capitaine, N., 2006, Astron.Astrophys. 459, 981
pub fn Xys06a(date1: f64, date2: f64) -> (f64, f64, f64) {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut s: f64 = 0.0;

    unsafe {
        eraXys06a(date1, date2, &mut x, &mut y, &mut s);
    }

    return (x, y, s);
}
