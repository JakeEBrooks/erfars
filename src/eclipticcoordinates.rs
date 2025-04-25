//! ERFA Ecliptic Coordinates Functions

use crate::raw::eclipticcoordinates::*;

///  Transformation from ecliptic coordinates (mean equinox and ecliptic
///  of date) to ICRS RA,Dec, using the IAU 2006 precession model.
///
///  Given:
///     date1,date2 double TT as a 2-part Julian date (Note 1)
///     dl,db       double ecliptic longitude and latitude (radians)
///
///  Returned:
///     dr,dd       double ICRS right ascension and declination (radians)
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
///  2) No assumptions are made about whether the coordinates represent
///     starlight and embody astrometric effects such as parallax or
///     aberration.
///
///  3) The transformation is approximately that from ecliptic longitude
///     and latitude (mean equinox and ecliptic of date) to mean J2000.0
///     right ascension and declination, with only frame bias (always
///     less than 25 mas) to disturb this classical picture.
pub fn Eceq06(date1: f64, date2: f64, dl: f64, db: f64) -> (f64, f64) {
    let mut dr: f64 = 0.0;
    let mut dd: f64 = 0.0;

    unsafe {
        eraEceq06(date1, date2, dl, db, &mut dr, &mut dd);
    }

    return (dr, dd)
}

///  ICRS equatorial to ecliptic rotation matrix, IAU 2006.
///
///  Given:
///     date1,date2  double         TT as a 2-part Julian date (Note 1)
///
///  Returned:
///     rm           double[3][3]   ICRS to ecliptic rotation matrix
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
///  2) The matrix is in the sense
///
///        E_ep = rm x P_ICRS,
///
///     where P_ICRS is a vector with respect to ICRS right ascension
///     and declination axes and E_ep is the same vector with respect to
///     the (inertial) ecliptic and equinox of date.
///
///     P_ICRS is a free vector, merely a direction, typically of unit
///     magnitude, and not bound to any particular spatial origin, such
///     as the Earth, Sun or SSB.  No assumptions are made about whether
///     it represents starlight and embodies astrometric effects such as
///     parallax or aberration.  The transformation is approximately that
///     between mean J2000.0 right ascension and declination and ecliptic
///     longitude and latitude, with only frame bias (always less than
///     25 mas) to disturb this classical picture.
pub fn Ecm06(date1: f64, date2: f64, rm: &mut [f64; 9]) {
    unsafe {
        eraEcm06(date1, date2, rm);
    }
}

///  Transformation from ICRS equatorial coordinates to ecliptic
///  coordinates (mean equinox and ecliptic of date) using IAU 2006
///  precession model.
///
///  Given:
///     date1,date2 double TT as a 2-part Julian date (Note 1)
///     dr,dd       double ICRS right ascension and declination (radians)
///
///  Returned:
///     dl,db       double ecliptic longitude and latitude (radians)
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
///  2) No assumptions are made about whether the coordinates represent
///     starlight and embody astrometric effects such as parallax or
///     aberration.
///
///  3) The transformation is approximately that from mean J2000.0 right
///     ascension and declination to ecliptic longitude and latitude
///     (mean equinox and ecliptic of date), with only frame bias (always
///     less than 25 mas) to disturb this classical picture.
pub fn Eqec06(date1: f64, date2: f64, dr: f64, dd: f64) -> (f64, f64) {
    let mut dl: f64 = 0.0;
    let mut db: f64 = 0.0;

    unsafe {
        eraEqec06(date1, date2, dr, dd, &mut dl, &mut db);
    }

    return (dl, db)
}

///  Transformation from ecliptic coordinates (mean equinox and ecliptic
///  of date) to ICRS RA,Dec, using a long-term precession model.
///
///  Given:
///     epj     double     Julian epoch (TT)
///     dl,db   double     ecliptic longitude and latitude (radians)
///
///  Returned:
///     dr,dd   double     ICRS right ascension and declination (radians)
///
///  1) No assumptions are made about whether the coordinates represent
///     starlight and embody astrometric effects such as parallax or
///     aberration.
///
///  2) The transformation is approximately that from ecliptic longitude
///     and latitude (mean equinox and ecliptic of date) to mean J2000.0
///     right ascension and declination, with only frame bias (always
///     less than 25 mas) to disturb this classical picture.
///
///  3) The Vondrak et al. (2011, 2012) 400 millennia precession model
///     agrees with the IAU 2006 precession at J2000.0 and stays within
///     100 microarcseconds during the 20th and 21st centuries.  It is
///     accurate to a few arcseconds throughout the historical period,
///     worsening to a few tenths of a degree at the end of the
///     +/- 200,000 year time span.
///
///  Called:
///     eraS2c       spherical coordinates to unit vector
///     eraLtecm     J2000.0 to ecliptic rotation matrix, long term
///     eraTrxp      product of transpose of r-matrix and p-vector
///     eraC2s       unit vector to spherical coordinates
///     eraAnp       normalize angle into range 0 to 2pi
///     eraAnpm      normalize angle into range +/- pi
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
pub fn Lteceq(epj: f64, dl: f64, db: f64) -> (f64, f64) {
    let mut dr: f64 = 0.0;
    let mut dd: f64 = 0.0;

    unsafe {
        eraLteceq(epj, dl, db, &mut dr, &mut dd);
    }

    return (dr, dd)
}

///  ICRS equatorial to ecliptic rotation matrix, long-term.
///
///  Given:
///     epj     double         Julian epoch (TT)
///
///  Returned:
///     rm      double[3][3]   ICRS to ecliptic rotation matrix
///
///  Notes:
///
///  1) The matrix is in the sense
///
///        E_ep = rm x P_ICRS,
///
///     where P_ICRS is a vector with respect to ICRS right ascension
///     and declination axes and E_ep is the same vector with respect to
///     the (inertial) ecliptic and equinox of epoch epj.
///
///  2) P_ICRS is a free vector, merely a direction, typically of unit
///     magnitude, and not bound to any particular spatial origin, such
///     as the Earth, Sun or SSB.  No assumptions are made about whether
///     it represents starlight and embodies astrometric effects such as
///     parallax or aberration.  The transformation is approximately that
///     between mean J2000.0 right ascension and declination and ecliptic
///     longitude and latitude, with only frame bias (always less than
///     25 mas) to disturb this classical picture.
///
///  3) The Vondrak et al. (2011, 2012) 400 millennia precession model
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
pub fn Ltecm(epj: f64, rm: &mut [f64; 9]) {
    unsafe {
        eraLtecm(epj, rm);
    }
}

///  Transformation from ICRS RA,Dec to ecliptic coordinates (mean equinox
///  and ecliptic of date), using a long-term precession model.
///
///  Given:
///     epj     double     Julian epoch (TT)
///     dr,dd   double     ICRS right ascension and declination (radians)
///
///  Returned:
///     dl,db   double     ecliptic longitude and latitude (radians)
///
///  1) No assumptions are made about whether the coordinates represent
///     starlight and embody astrometric effects such as parallax or
///     aberration.
///
///  2) The transformation is approximately that from mean J2000.0 right
///     ascension and declination to ecliptic longitude and latitude
///     (mean equinox and ecliptic of date), with only frame bias (always
///     less than 25 mas) to disturb this classical picture.
///
///  3) The Vondrak et al. (2011, 2012) 400 millennia precession model
///     agrees with the IAU 2006 precession at J2000.0 and stays within
///     100 microarcseconds during the 20th and 21st centuries.  It is
///     accurate to a few arcseconds throughout the historical period,
///     worsening to a few tenths of a degree at the end of the
///     +/- 200,000 year time span.
///
///  Called:
///     eraS2c       spherical coordinates to unit vector
///     eraLtecm     J2000.0 to ecliptic rotation matrix, long term
///     eraRxp       product of r-matrix and p-vector
///     eraC2s       unit vector to spherical coordinates
///     eraAnp       normalize angle into range 0 to 2pi
///     eraAnpm      normalize angle into range +/- pi
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
pub fn Lteqec(epj: f64, dr: f64, dd: f64) -> (f64, f64) {
    let mut dl: f64 = 0.0;
    let mut db: f64 = 0.0;

    unsafe {
        eraLteqec(epj, dr, dd, &mut dl, &mut db);
    }

    return (dl, db)
}