//! ERFA Calendar Functions

use crate::{raw::calendar::*, unexpected_val_err, ERFAError};

///  Gregorian Calendar to Julian Date.
///
///  Given:
///     iy,im,id  int     year, month, day in Gregorian calendar (Note 1)
///
///  Returned:
///     djm0      double  MJD zero-point: always 2400000.5
///     djm       double  Modified Julian Date for 0 hrs
///
///  Returned (function value):
///               int     status:
///                           0 = OK
///                          -1 = bad year   (Note 3: JD not computed)
///                          -2 = bad month  (JD not computed)
///                          -3 = bad day    (JD computed)
///
///  Notes:
///
///  1) The algorithm used is valid from -4800 March 1, but this
///     implementation rejects dates before -4799 January 1.
///
///  2) The Julian Date is returned in two pieces, in the usual ERFA
///     manner, which is designed to preserve time resolution.  The
///     Julian Date is available as a single number by adding djm0 and
///     djm.
///
///  3) In early eras the conversion is from the "Proleptic Gregorian
///     Calendar";  no account is taken of the date(s) of adoption of
///     the Gregorian Calendar, nor is the AD/BC numbering convention
///     observed.
///
///  Reference:
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992),
///     Section 12.92 (p604).
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

///  Julian Date to Besselian Epoch.
///
///  Given:
///     dj1,dj2    double     Julian Date (Notes 3,4)
///
///  Returned (function value):
///                double     Besselian Epoch.
///
///  Notes:
///
///  1) Besselian Epoch is a method of expressing a moment in time as a
///     year plus fraction.  It was superseded by Julian Year (see the
///     function eraEpj).
///
///  2) The start of a Besselian year is when the right ascension of
///     the fictitious mean Sun is 18h 40m, and the unit is the tropical
///     year.  The conventional definition (see Lieske 1979) is that
///     Besselian Epoch B1900.0 is JD 2415020.31352 and the length of the
///     year is 365.242198781 days.
///
///  3) The time scale for the JD, originally Ephemeris Time, is TDB,
///     which for all practical purposes in the present context is
///     indistinguishable from TT.
///
///  4) The Julian Date is supplied in two pieces, in the usual ERFA
///     manner, which is designed to preserve time resolution.  The
///     Julian Date is available as a single number by adding dj1 and
///     dj2.  The maximum resolution is achieved if dj1 is 2451545.0
///     (J2000.0).
///
///  Reference:
///
///     Lieske, J.H., 1979. Astron.Astrophys., 73, 282.
pub fn Epb(jd0: f64, jd1: f64) -> f64 {
    return unsafe { eraEpb(jd0, jd1) };
}

///  Besselian Epoch to Julian Date.
///
///  Given:
///     epb      double    Besselian Epoch (e.g. 1957.3)
///
///  Returned:
///     djm0     double    MJD zero-point: always 2400000.5
///     djm      double    Modified Julian Date
///
///  Note:
///
///     The Julian Date is returned in two pieces, in the usual ERFA
///     manner, which is designed to preserve time resolution.  The
///     Julian Date is available as a single number by adding djm0 and
///     djm.
///
///  Reference:
///
///     Lieske, J.H., 1979, Astron.Astrophys. 73, 282.
pub fn Epb2jd(epoch: f64) -> (f64, f64) {
    let mut djm0: f64 = 0.0;
    let mut djm: f64 = 0.0;
    unsafe {
        eraEpb2jd(epoch, &mut djm0, &mut djm);
    }

    return (djm0, djm);
}

///  Julian Date to Julian Epoch.
///
///  Given:
///     dj1,dj2    double     Julian Date (Note 4)
///
///  Returned (function value):
///                double     Julian Epoch
///
///  Notes:
///
///  1) Julian Epoch is a method of expressing a moment in time as a
///     year plus fraction.
///
///  2) Julian Epoch J2000.0 is 2000 Jan 1.5, and the length of the year
///     is 365.25 days.
///
///  3) For historical reasons, the time scale formally associated with
///     Julian Epoch is TDB (or TT, near enough).  However, Julian Epoch
///     can be used more generally as a calendrical convention to
///     represent other time scales such as TAI and TCB.  This is
///     analogous to Julian Date, which was originally defined
///     specifically as a way of representing Universal Times but is now
///     routinely used for any of the regular time scales.
///
///  4) The Julian Date is supplied in two pieces, in the usual ERFA
///     manner, which is designed to preserve time resolution.  The
///     Julian Date is available as a single number by adding dj1 and
///     dj2.  The maximum resolution is achieved if dj1 is 2451545.0
///     (J2000.0).
///
///  Reference:
///
///     Lieske, J.H., 1979, Astron.Astrophys. 73, 282.
pub fn Epj(jd0: f64, jd1: f64) -> f64 {
    return unsafe { eraEpj(jd0, jd1) };
}

///  Julian Epoch to Julian Date.
///
///  Given:
///     epj      double    Julian Epoch (e.g. 1996.8)
///
///  Returned:
///     djm0     double    MJD zero-point: always 2400000.5
///     djm      double    Modified Julian Date
///
///  Note:
///
///     The Julian Date is returned in two pieces, in the usual ERFA
///     manner, which is designed to preserve time resolution.  The
///     Julian Date is available as a single number by adding djm0 and
///     djm.
///
///  Reference:
///
///     Lieske, J.H., 1979, Astron.Astrophys. 73, 282.
pub fn Epj2jd(epoch: f64) -> (f64, f64) {
    let mut djm0: f64 = 0.0;
    let mut djm: f64 = 0.0;
    unsafe {
        eraEpj2jd(epoch, &mut djm0, &mut djm);
    }

    return (djm0, djm);
}

///  Julian Date to Gregorian year, month, day, and fraction of a day.
///
///  Given:
///     dj1,dj2   double   Julian Date (Notes 1, 2)
///
///  Returned (arguments):
///     iy        int      year
///     im        int      month
///     id        int      day
///     fd        double   fraction of day
///
///  Returned (function value):
///               int      status:
///                           0 = OK
///                          -1 = unacceptable date (Note 1)
///
///  Notes:
///
///  1) The earliest valid date is -68569.5 (-4900 March 1).  The
///     largest value accepted is 1e9.
///
///  2) The Julian Date is apportioned in any convenient way between
///     the arguments dj1 and dj2.  For example, JD=2450123.7 could
///     be expressed in any of these ways, among others:
///
///            dj1             dj2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     Separating integer and fraction uses the "compensated summation"
///     algorithm of Kahan-Neumaier to preserve as much precision as
///     possible irrespective of the jd1+jd2 apportionment.
///
///  3) In early eras the conversion is from the "proleptic Gregorian
///     calendar";  no account is taken of the date(s) of adoption of
///     the Gregorian calendar, nor is the AD/BC numbering convention
///     observed.
///
///  References:
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992),
///     Section 12.92 (p604).
///
///     Klein, A., A Generalized Kahan-Babuska-Summation-Algorithm.
///     Computing, 76, 279-293 (2006), Section 3.
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

///  Julian Date to Gregorian Calendar, expressed in a form convenient
///  for formatting messages:  rounded to a specified precision.
///
///  Given:
///     ndp       int      number of decimal places of days in fraction
///     dj1,dj2   double   dj1+dj2 = Julian Date (Note 1)
///
///  Returned:
///     iymdf     int[4]   year, month, day, fraction in Gregorian
///                        calendar
///
///  Returned (function value):
///               int      status:
///                          -1 = date out of range
///                           0 = OK
///                          +1 = ndp not 0-9 (interpreted as 0)
///
///  Notes:
///
///  1) The Julian Date is apportioned in any convenient way between
///     the arguments dj1 and dj2.  For example, JD=2450123.7 could
///     be expressed in any of these ways, among others:
///
///             dj1            dj2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///  2) In early eras the conversion is from the "Proleptic Gregorian
///     Calendar";  no account is taken of the date(s) of adoption of
///     the Gregorian Calendar, nor is the AD/BC numbering convention
///     observed.
///
///  3) See also the function eraJd2cal.
///
///  4) The number of decimal places ndp should be 4 or less if internal
///     overflows are to be avoided on platforms which use 16-bit
///     integers.
///
///  Called:
///     eraJd2cal    JD to Gregorian calendar
///
///  Reference:
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992),
///     Section 12.92 (p604).
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
