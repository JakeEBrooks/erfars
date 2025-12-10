//! ERFA Timescales Functions

use std::ffi::CString;

use crate::{ERFAError, raw::timescales::*, unexpected_val_err};

///
///  - - - - - - - - -
///   e r a D 2 d t f
///  - - - - - - - - -
///
///  Format for output a 2-part Julian Date (or in the case of UTC a
///  quasi-JD form that includes special provision for leap seconds).
///
///  Given:
///     scale     char[]  time scale ID (Note 1)
///     ndp       int     resolution (Note 2)
///     d1,d2     double  time as a 2-part Julian Date (Notes 3,4)
///
///  Returned:
///     iy,im,id  int     year, month, day in Gregorian calendar (Note 5)
///     ihmsf     int[4]  hours, minutes, seconds, fraction (Note 1)
///
///  Returned (function value):
///               int     status: +1 = dubious year (Note 5)
///                                0 = OK
///                               -1 = unacceptable date (Note 6)
///
///  Notes:
///
///  1) scale identifies the time scale.  Only the value "UTC" (in upper
///     case) is significant, and enables handling of leap seconds (see
///     Note 4).
///
///  2) ndp is the number of decimal places in the seconds field, and can
///     have negative as well as positive values, such as:
///
///     ndp         resolution
///     -4            1 00 00
///     -3            0 10 00
///     -2            0 01 00
///     -1            0 00 10
///      0            0 00 01
///      1            0 00 00.1
///      2            0 00 00.01
///      3            0 00 00.001
///
///     The limits are platform dependent, but a safe range is -5 to +9.
///
///  3) d1+d2 is Julian Date, apportioned in any convenient way between
///     the two arguments, for example where d1 is the Julian Day Number
///     and d2 is the fraction of a day.  In the case of UTC, where the
///     use of JD is problematical, special conventions apply:  see the
///     next note.
///
///  4) JD cannot unambiguously represent UTC during a leap second unless
///     special measures are taken.  The ERFA internal convention is that
///     the quasi-JD day represents UTC days whether the length is 86399,
///     86400 or 86401 SI seconds.  In the 1960-1972 era there were
///     smaller jumps (in either direction) each time the linear UTC(TAI)
///     expression was changed, and these "mini-leaps" are also included
///     in the ERFA convention.
///
///  5) The warning status "dubious year" flags UTCs that predate the
///     introduction of the time scale or that are too far in the future
///     to be trusted.  See eraDat for further details.
///
///  6) For calendar conventions and limitations, see eraCal2jd.
///
///  Called:
///     eraJd2cal    JD to Gregorian calendar
///     eraD2tf      decompose days to hms
///     eraDat       delta(AT) = TAI-UTC
///
///  This revision:  2021 May 11
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn D2dtf(
    utc: bool,
    ndp: i32,
    d1: f64,
    d2: f64,
) -> Result<(i32, i32, i32, i32, i32, i32, i32), ERFAError> {
    let mut year: i32 = 0;
    let mut month: i32 = 0;
    let mut day: i32 = 0;
    let mut hmsf: [i32; 4] = [0; 4];
    let scale = if utc {
        CString::new("UTC")
    } else {
        CString::new("NA")
    };
    let err: i32;
    unsafe {
        err = eraD2dtf(
            scale.unwrap().as_ptr(),
            ndp,
            d1,
            d2,
            &mut year,
            &mut month,
            &mut day,
            &mut hmsf,
        )
    };
    match err {
        -1 => Err(ERFAError::ERFABadDate),
        0 => Ok((year, month, day, hmsf[0], hmsf[1], hmsf[2], hmsf[3])),
        1 => Ok((year, month, day, hmsf[0], hmsf[1], hmsf[2], hmsf[3])),
        _ => unexpected_val_err!(eraD2dtf),
    }
}

///
///  - - - - - - -
///   e r a D a t
///  - - - - - - -
///
///  For a given UTC date, calculate Delta(AT) = TAI-UTC.
///
///     :------------------------------------------:
///     :                                          :
///     :                 IMPORTANT                :
///     :                                          :
///     :  A new version of this function must be  :
///     :  produced whenever a new leap second is  :
///     :  announced.  There are four items to     :
///     :  change on each such occasion:           :
///     :                                          :
///     :  1) A new line must be added to the set  :
///     :     of statements that initialize the    :
///     :     array "changes".                     :
///     :                                          :
///     :  2) The constant IYV must be set to the  :
///     :     current year.                        :
///     :                                          :
///     :  3) The "Latest leap second" comment     :
///     :     below must be set to the new leap    :
///     :     second date.                         :
///     :                                          :
///     :  4) The "This revision" comment, later,  :
///     :     must be set to the current date.     :
///     :                                          :
///     :  Change (2) must also be carried out     :
///     :  whenever the function is re-issued,     :
///     :  even if no leap seconds have been       :
///     :  added.                                  :
///     :                                          :
///     :  Latest leap second:  2016 December 31   :
///     :                                          :
///     :__________________________________________:
///
///  Given:
///     iy     int      UTC:  year (Notes 1 and 2)
///     im     int            month (Note 2)
///     id     int            day (Notes 2 and 3)
///     fd     double         fraction of day (Note 4)
///
///  Returned:
///     deltat double   TAI minus UTC, seconds
///
///  Returned (function value):
///            int      status (Note 5):
///                       1 = dubious year (Note 1)
///                       0 = OK
///                      -1 = bad year
///                      -2 = bad month
///                      -3 = bad day (Note 3)
///                      -4 = bad fraction (Note 4)
///                      -5 = internal error (Note 5)
///
///  Notes:
///
///  1) UTC began at 1960 January 1.0 (JD 2436934.5) and it is improper
///     to call the function with an earlier date.  If this is attempted,
///     zero is returned together with a warning status.
///
///     Because leap seconds cannot, in principle, be predicted in
///     advance, a reliable check for dates beyond the valid range is
///     impossible.  To guard against gross errors, a year five or more
///     after the release year of the present function (see the constant
///     IYV) is considered dubious.  In this case a warning status is
///     returned but the result is computed in the normal way.
///
///     For both too-early and too-late years, the warning status is +1.
///     This is distinct from the error status -1, which signifies a year
///     so early that JD could not be computed.
///
///  2) If the specified date is for a day which ends with a leap second,
///     the TAI-UTC value returned is for the period leading up to the
///     leap second.  If the date is for a day which begins as a leap
///     second ends, the TAI-UTC returned is for the period following the
///     leap second.
///
///  3) The day number must be in the normal calendar range, for example
///     1 through 30 for April.  The "almanac" convention of allowing
///     such dates as January 0 and December 32 is not supported in this
///     function, in order to avoid confusion near leap seconds.
///
///  4) The fraction of day is used only for dates before the
///     introduction of leap seconds, the first of which occurred at the
///     end of 1971.  It is tested for validity (0 to 1 is the valid
///     range) even if not used;  if invalid, zero is used and status -4
///     is returned.  For many applications, setting fd to zero is
///     acceptable;  the resulting error is always less than 3 ms (and
///     occurs only pre-1972).
///
///  5) The status value returned in the case where there are multiple
///     errors refers to the first error detected.  For example, if the
///     month and day are 13 and 32 respectively, status -2 (bad month)
///     will be returned.  The "internal error" status refers to a
///     case that is impossible but causes some compilers to issue a
///     warning.
///
///  6) In cases where a valid result is not available, zero is returned.
///
///  References:
///
///  1) For dates from 1961 January 1 onwards, the expressions from the
///     file ftp://maia.usno.navy.mil/ser7/tai-utc.dat are used.
///
///  2) The 5ms timestep at 1961 January 1 is taken from 2.58.1 (p87) of
///     the 1992 Explanatory Supplement.
///
///  Called:
///     eraCal2jd    Gregorian calendar to JD
///
///  This revision:  2023 January 17
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Dat(year: i32, month: i32, day: i32, fd: f64) -> Result<f64, ERFAError> {
    let mut deltat: f64 = 0.0;
    let err: i32;
    unsafe { err = eraDat(year, month, day, fd, &mut deltat) }
    match err {
        -5 => Err(ERFAError::ERFAInternalError),
        -4 => Err(ERFAError::ERFABadFraction),
        -3 => Err(ERFAError::ERFABadDay),
        -2 => Err(ERFAError::ERFABadMonth),
        -1 => Err(ERFAError::ERFABadYear),
        0 => Ok(deltat),
        1 => Ok(deltat),
        _ => unexpected_val_err!(eraDat),
    }
}

///
///  - - - - - - - -
///   e r a D t d b
///  - - - - - - - -
///
///  An approximation to TDB-TT, the difference between barycentric
///  dynamical time and terrestrial time, for an observer on the Earth.
///
///  The different time scales - proper, coordinate and realized - are
///  related to each other:
///
///            TAI             <-  physically realized
///             :
///          offset            <-  observed (nominally +32.184s)
///             :
///            TT              <-  terrestrial time
///             :
///    rate adjustment (L_G)   <-  definition of TT
///             :
///            TCG             <-  time scale for GCRS
///             :
///      "periodic" terms      <-  eraDtdb  is an implementation
///             :
///    rate adjustment (L_C)   <-  function of solar-system ephemeris
///             :
///            TCB             <-  time scale for BCRS
///             :
///    rate adjustment (-L_B)  <-  definition of TDB
///             :
///            TDB             <-  TCB scaled to track TT
///             :
///      "periodic" terms      <-  -eraDtdb is an approximation
///             :
///            TT              <-  terrestrial time
///
///  Adopted values for the various constants can be found in the IERS
///  Conventions (McCarthy & Petit 2003).
///
///  Given:
///     date1,date2   double  date, TDB (Notes 1-3)
///     ut            double  universal time (UT1, fraction of one day)
///     elong         double  longitude (east positive, radians)
///     u             double  distance from Earth spin axis (km)
///     v             double  distance north of equatorial plane (km)
///
///  Returned (function value):
///                   double  TDB-TT (seconds)
///
///  Notes:
///
///  1) The date date1+date2 is a Julian Date, apportioned in any
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
///     Although the date is, formally, barycentric dynamical time (TDB),
///     the terrestrial dynamical time (TT) can be used with no practical
///     effect on the accuracy of the prediction.
///
///  2) TT can be regarded as a coordinate time that is realized as an
///     offset of 32.184s from International Atomic Time, TAI.  TT is a
///     specific linear transformation of geocentric coordinate time TCG,
///     which is the time scale for the Geocentric Celestial Reference
///     System, GCRS.
///
///  3) TDB is a coordinate time, and is a specific linear transformation
///     of barycentric coordinate time TCB, which is the time scale for
///     the Barycentric Celestial Reference System, BCRS.
///
///  4) The difference TCG-TCB depends on the masses and positions of the
///     bodies of the solar system and the velocity of the Earth.  It is
///     dominated by a rate difference, the residual being of a periodic
///     character.  The latter, which is modeled by the present function,
///     comprises a main (annual) sinusoidal term of amplitude
///     approximately 0.00166 seconds, plus planetary terms up to about
///     20 microseconds, and lunar and diurnal terms up to 2 microseconds.
///     These effects come from the changing transverse Doppler effect
///     and gravitational red-shift as the observer (on the Earth's
///     surface) experiences variations in speed (with respect to the
///     BCRS) and gravitational potential.
///
///  5) TDB can be regarded as the same as TCB but with a rate adjustment
///     to keep it close to TT, which is convenient for many applications.
///     The history of successive attempts to define TDB is set out in
///     Resolution 3 adopted by the IAU General Assembly in 2006, which
///     defines a fixed TDB(TCB) transformation that is consistent with
///     contemporary solar-system ephemerides.  Future ephemerides will
///     imply slightly changed transformations between TCG and TCB, which
///     could introduce a linear drift between TDB and TT;  however, any
///     such drift is unlikely to exceed 1 nanosecond per century.
///
///  6) The geocentric TDB-TT model used in the present function is that of
///     Fairhead & Bretagnon (1990), in its full form.  It was originally
///     supplied by Fairhead (private communications with P.T.Wallace,
///     1990) as a Fortran subroutine.  The present C function contains an
///     adaptation of the Fairhead code.  The numerical results are
///     essentially unaffected by the changes, the differences with
///     respect to the Fairhead & Bretagnon original being at the 1e-20 s
///     level.
///
///     The topocentric part of the model is from Moyer (1981) and
///     Murray (1983), with fundamental arguments adapted from
///     Simon et al. 1994.  It is an approximation to the expression
///     ( v / c ) . ( r / c ), where v is the barycentric velocity of
///     the Earth, r is the geocentric position of the observer and
///     c is the speed of light.
///
///     By supplying zeroes for u and v, the topocentric part of the
///     model can be nullified, and the function will return the Fairhead
///     & Bretagnon result alone.
///
///  7) During the interval 1950-2050, the absolute accuracy is better
///     than +/- 3 nanoseconds relative to time ephemerides obtained by
///     direct numerical integrations based on the JPL DE405 solar system
///     ephemeris.
///
///  8) It must be stressed that the present function is merely a model,
///     and that numerical integration of solar-system ephemerides is the
///     definitive method for predicting the relationship between TCG and
///     TCB and hence between TT and TDB.
///
///  References:
///
///     Fairhead, L., & Bretagnon, P., Astron.Astrophys., 229, 240-247
///     (1990).
///
///     IAU 2006 Resolution 3.
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     Moyer, T.D., Cel.Mech., 23, 33 (1981).
///
///     Murray, C.A., Vectorial Astrometry, Adam Hilger (1983).
///
///     Seidelmann, P.K. et al., Explanatory Supplement to the
///     Astronomical Almanac, Chapter 2, University Science Books (1992).
///
///     Simon, J.L., Bretagnon, P., Chapront, J., Chapront-Touze, M.,
///     Francou, G. & Laskar, J., Astron.Astrophys., 282, 663-683 (1994).
///
///  This revision:  2021 May 11
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Dtdb(date1: f64, date2: f64, ut: f64, elong: f64, u: f64, v: f64) -> f64 {
    return unsafe { eraDtdb(date1, date2, ut, elong, u, v) };
}

///
///  - - - - - - - - -
///   e r a D t f 2 d
///  - - - - - - - - -
///
///  Encode date and time fields into 2-part Julian Date (or in the case
///  of UTC a quasi-JD form that includes special provision for leap
///  seconds).
///
///  Given:
///     scale     char[]  time scale ID (Note 1)
///     iy,im,id  int     year, month, day in Gregorian calendar (Note 2)
///     ihr,imn   int     hour, minute
///     sec       double  seconds
///
///  Returned:
///     d1,d2     double  2-part Julian Date (Notes 3,4)
///
///  Returned (function value):
///               int     status: +3 = both of next two
///                               +2 = time is after end of day (Note 5)
///                               +1 = dubious year (Note 6)
///                                0 = OK
///                               -1 = bad year
///                               -2 = bad month
///                               -3 = bad day
///                               -4 = bad hour
///                               -5 = bad minute
///                               -6 = bad second (<0)
///
///  Notes:
///
///  1) scale identifies the time scale.  Only the value "UTC" (in upper
///     case) is significant, and enables handling of leap seconds (see
///     Note 4).
///
///  2) For calendar conventions and limitations, see eraCal2jd.
///
///  3) The sum of the results, d1+d2, is Julian Date, where normally d1
///     is the Julian Day Number and d2 is the fraction of a day.  In the
///     case of UTC, where the use of JD is problematical, special
///     conventions apply:  see the next note.
///
///  4) JD cannot unambiguously represent UTC during a leap second unless
///     special measures are taken.  The ERFA internal convention is that
///     the quasi-JD day represents UTC days whether the length is 86399,
///     86400 or 86401 SI seconds.  In the 1960-1972 era there were
///     smaller jumps (in either direction) each time the linear UTC(TAI)
///     expression was changed, and these "mini-leaps" are also included
///     in the ERFA convention.
///
///  5) The warning status "time is after end of day" usually means that
///     the sec argument is greater than 60.0.  However, in a day ending
///     in a leap second the limit changes to 61.0 (or 59.0 in the case
///     of a negative leap second).
///
///  6) The warning status "dubious year" flags UTCs that predate the
///     introduction of the time scale or that are too far in the future
///     to be trusted.  See eraDat for further details.
///
///  7) Only in the case of continuous and regular time scales (TAI, TT,
///     TCG, TCB and TDB) is the result d1+d2 a Julian Date, strictly
///     speaking.  In the other cases (UT1 and UTC) the result must be
///     used with circumspection;  in particular the difference between
///     two such results cannot be interpreted as a precise time
///     interval.
///
///  Called:
///     eraCal2jd    Gregorian calendar to JD
///     eraDat       delta(AT) = TAI-UTC
///     eraJd2cal    JD to Gregorian calendar
///
///  This revision:  2023 May 6
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Dtf2d(
    utc: bool,
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    minute: i32,
    seconds: f64,
) -> Result<(f64, f64), ERFAError> {
    let mut date1: f64 = 0.0;
    let mut date2: f64 = 0.0;
    let scale = if utc {
        CString::new("UTC")
    } else {
        CString::new("NA")
    };
    let err: i32;
    unsafe {
        err = eraDtf2d(
            scale.unwrap().as_ptr(),
            year,
            month,
            day,
            hour,
            minute,
            seconds,
            &mut date1,
            &mut date2,
        )
    }

    match err {
        -6 => Err(ERFAError::ERFABadSecond),
        -5 => Err(ERFAError::ERFABadMinute),
        -4 => Err(ERFAError::ERFABadHour),
        -3 => Err(ERFAError::ERFABadDay),
        -2 => Err(ERFAError::ERFABadMonth),
        -1 => Err(ERFAError::ERFABadYear),
        0 => Ok((date1, date2)),
        1 => Ok((date1, date2)),
        2 => Ok((date1, date2)),
        3 => Ok((date1, date2)),
        _ => unexpected_val_err!(eraDtf2d),
    }
}

///
///  - - - - - - - - -
///   e r a T a i t t
///  - - - - - - - - -
///
///  Time scale transformation:  International Atomic Time, TAI, to
///  Terrestrial Time, TT.
///
///  Given:
///     tai1,tai2  double    TAI as a 2-part Julian Date
///
///  Returned:
///     tt1,tt2    double    TT as a 2-part Julian Date
///
///  Returned (function value):
///                int       status:  0 = OK
///
///  Note:
///
///     tai1+tai2 is Julian Date, apportioned in any convenient way
///     between the two arguments, for example where tai1 is the Julian
///     Day Number and tai2 is the fraction of a day.  The returned
///     tt1,tt2 follow suit.
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992)
///
///  This revision:  2021 May 11
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Taitt(tai1: f64, tai2: f64) -> (f64, f64) {
    let mut tt1: f64 = 0.0;
    let mut tt2: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraTaitt(tai1, tai2, &mut tt1, &mut tt2);
    }

    return (tt1, tt2);
}

///
///  - - - - - - - - - -
///   e r a T a i u t 1
///  - - - - - - - - - -
///
///  Time scale transformation:  International Atomic Time, TAI, to
///  Universal Time, UT1.
///
///  Given:
///     tai1,tai2  double    TAI as a 2-part Julian Date
///     dta        double    UT1-TAI in seconds
///
///  Returned:
///     ut11,ut12  double    UT1 as a 2-part Julian Date
///
///  Returned (function value):
///                int       status:  0 = OK
///
///  Notes:
///
///  1) tai1+tai2 is Julian Date, apportioned in any convenient way
///     between the two arguments, for example where tai1 is the Julian
///     Day Number and tai2 is the fraction of a day.  The returned
///     UT11,UT12 follow suit.
///
///  2) The argument dta, i.e. UT1-TAI, is an observed quantity, and is
///     available from IERS tabulations.
///
///  Reference:
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992)
///
///  This revision:  2021 May 11
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Taiut1(tai1: f64, tai2: f64, dta: f64) -> (f64, f64) {
    let mut ut11: f64 = 0.0;
    let mut ut12: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraTaiut1(tai1, tai2, dta, &mut ut11, &mut ut12)
    }

    return (ut11, ut12);
}

///
///  - - - - - - - - - -
///   e r a T a i u t c
///  - - - - - - - - - -
///
///  Time scale transformation:  International Atomic Time, TAI, to
///  Coordinated Universal Time, UTC.
///
///  Given:
///     tai1,tai2  double   TAI as a 2-part Julian Date (Note 1)
///
///  Returned:
///     utc1,utc2  double   UTC as a 2-part quasi Julian Date (Notes 1-3)
///
///  Returned (function value):
///                int      status: +1 = dubious year (Note 4)
///                                  0 = OK
///                                 -1 = unacceptable date
///
///  Notes:
///
///  1) tai1+tai2 is Julian Date, apportioned in any convenient way
///     between the two arguments, for example where tai1 is the Julian
///     Day Number and tai2 is the fraction of a day.  The returned utc1
///     and utc2 form an analogous pair, except that a special convention
///     is used, to deal with the problem of leap seconds - see the next
///     note.
///
///  2) JD cannot unambiguously represent UTC during a leap second unless
///     special measures are taken.  The convention in the present
///     function is that the JD day represents UTC days whether the
///     length is 86399, 86400 or 86401 SI seconds.  In the 1960-1972 era
///     there were smaller jumps (in either direction) each time the
///     linear UTC(TAI) expression was changed, and these "mini-leaps"
///     are also included in the ERFA convention.
///
///  3) The function eraD2dtf can be used to transform the UTC quasi-JD
///     into calendar date and clock time, including UTC leap second
///     handling.
///
///  4) The warning status "dubious year" flags UTCs that predate the
///     introduction of the time scale or that are too far in the future
///     to be trusted.  See eraDat for further details.
///
///  Called:
///     eraUtctai    UTC to TAI
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992)
///
///  This revision:  2021 May 11
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Taiutc(tai1: f64, tai2: f64) -> Result<(f64, f64), ERFAError> {
    let mut utc1: f64 = 0.0;
    let mut utc2: f64 = 0.0;
    let err: i32;

    unsafe { err = eraTaiutc(tai1, tai2, &mut utc1, &mut utc2) }

    match err {
        -1 => Err(ERFAError::ERFABadDate),
        0 => Ok((utc1, utc2)),
        1 => Ok((utc1, utc2)),
        _ => unexpected_val_err!(eraTaiutc),
    }
}

///
///  - - - - - - - - - -
///   e r a T c b t d b
///  - - - - - - - - - -
///
///  Time scale transformation:  Barycentric Coordinate Time, TCB, to
///  Barycentric Dynamical Time, TDB.
///
///  Given:
///     tcb1,tcb2  double    TCB as a 2-part Julian Date
///
///  Returned:
///     tdb1,tdb2  double    TDB as a 2-part Julian Date
///
///  Returned (function value):
///                int       status:  0 = OK
///
///  Notes:
///
///  1) tcb1+tcb2 is Julian Date, apportioned in any convenient way
///     between the two arguments, for example where tcb1 is the Julian
///     Day Number and tcb2 is the fraction of a day.  The returned
///     tdb1,tdb2 follow suit.
///
///  2) The 2006 IAU General Assembly introduced a conventional linear
///     transformation between TDB and TCB.  This transformation
///     compensates for the drift between TCB and terrestrial time TT,
///     and keeps TDB approximately centered on TT.  Because the
///     relationship between TT and TCB depends on the adopted solar
///     system ephemeris, the degree of alignment between TDB and TT over
///     long intervals will vary according to which ephemeris is used.
///     Former definitions of TDB attempted to avoid this problem by
///     stipulating that TDB and TT should differ only by periodic
///     effects.  This is a good description of the nature of the
///     relationship but eluded precise mathematical formulation.  The
///     conventional linear relationship adopted in 2006 sidestepped
///     these difficulties whilst delivering a TDB that in practice was
///     consistent with values before that date.
///
///  3) TDB is essentially the same as Teph, the time argument for the
///     JPL solar system ephemerides.
///
///  Reference:
///
///     IAU 2006 Resolution B3
///
///  This revision:  2021 May 11
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Tcbtdb(tcb1: f64, tcb2: f64) -> (f64, f64) {
    let mut tdb1: f64 = 0.0;
    let mut tdb2: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraTcbtdb(tcb1, tcb2, &mut tdb1, &mut tdb2)
    }

    return (tdb1, tdb2);
}

///
///  - - - - - - - - -
///   e r a T c g t t
///  - - - - - - - - -
///
///  Time scale transformation:  Geocentric Coordinate Time, TCG, to
///  Terrestrial Time, TT.
///
///  Given:
///     tcg1,tcg2  double    TCG as a 2-part Julian Date
///
///  Returned:
///     tt1,tt2    double    TT as a 2-part Julian Date
///
///  Returned (function value):
///                int       status:  0 = OK
///
///  Note:
///
///     tcg1+tcg2 is Julian Date, apportioned in any convenient way
///     between the two arguments, for example where tcg1 is the Julian
///     Day Number and tcg22 is the fraction of a day.  The returned
///     tt1,tt2 follow suit.
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     IAU 2000 Resolution B1.9
///
///  This revision:  2021 May 11
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Tcgtt(tcg1: f64, tcg2: f64) -> (f64, f64) {
    let mut tt1: f64 = 0.0;
    let mut tt2: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraTcgtt(tcg1, tcg2, &mut tt1, &mut tt2)
    }

    return (tt1, tt2);
}

///
///  - - - - - - - - - -
///   e r a T d b t c b
///  - - - - - - - - - -
///
///  Time scale transformation:  Barycentric Dynamical Time, TDB, to
///  Barycentric Coordinate Time, TCB.
///
///  Given:
///     tdb1,tdb2  double    TDB as a 2-part Julian Date
///
///  Returned:
///     tcb1,tcb2  double    TCB as a 2-part Julian Date
///
///  Returned (function value):
///                int       status:  0 = OK
///
///  Notes:
///
///  1) tdb1+tdb2 is Julian Date, apportioned in any convenient way
///     between the two arguments, for example where tdb1 is the Julian
///     Day Number and tdb2 is the fraction of a day.  The returned
///     tcb1,tcb2 follow suit.
///
///  2) The 2006 IAU General Assembly introduced a conventional linear
///     transformation between TDB and TCB.  This transformation
///     compensates for the drift between TCB and terrestrial time TT,
///     and keeps TDB approximately centered on TT.  Because the
///     relationship between TT and TCB depends on the adopted solar
///     system ephemeris, the degree of alignment between TDB and TT over
///     long intervals will vary according to which ephemeris is used.
///     Former definitions of TDB attempted to avoid this problem by
///     stipulating that TDB and TT should differ only by periodic
///     effects.  This is a good description of the nature of the
///     relationship but eluded precise mathematical formulation.  The
///     conventional linear relationship adopted in 2006 sidestepped
///     these difficulties whilst delivering a TDB that in practice was
///     consistent with values before that date.
///
///  3) TDB is essentially the same as Teph, the time argument for the
///     JPL solar system ephemerides.
///
///  Reference:
///
///     IAU 2006 Resolution B3
///
///  This revision:  2021 May 11
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Tdbtcb(tdb1: f64, tdb2: f64) -> (f64, f64) {
    let mut tcb1: f64 = 0.0;
    let mut tcb2: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraTdbtcb(tdb1, tdb2, &mut tcb1, &mut tcb2)
    }

    return (tcb1, tcb2);
}

///
///  - - - - - - - - -
///   e r a T d b t t
///  - - - - - - - - -
///
///  Time scale transformation:  Barycentric Dynamical Time, TDB, to
///  Terrestrial Time, TT.
///
///  Given:
///     tdb1,tdb2  double    TDB as a 2-part Julian Date
///     dtr        double    TDB-TT in seconds
///
///  Returned:
///     tt1,tt2    double    TT as a 2-part Julian Date
///
///  Returned (function value):
///                int       status:  0 = OK
///
///  Notes:
///
///  1) tdb1+tdb2 is Julian Date, apportioned in any convenient way
///     between the two arguments, for example where tdb1 is the Julian
///     Day Number and tdb2 is the fraction of a day.  The returned
///     tt1,tt2 follow suit.
///
///  2) The argument dtr represents the quasi-periodic component of the
///     GR transformation between TT and TCB.  It is dependent upon the
///     adopted solar-system ephemeris, and can be obtained by numerical
///     integration, by interrogating a precomputed time ephemeris or by
///     evaluating a model such as that implemented in the ERFA function
///     eraDtdb.   The quantity is dominated by an annual term of 1.7 ms
///     amplitude.
///
///  3) TDB is essentially the same as Teph, the time argument for the
///     JPL solar system ephemerides.
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     IAU 2006 Resolution 3
///
///  This revision:  2021 May 11
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Tdbtt(tdb1: f64, tdb2: f64, dtr: f64) -> (f64, f64) {
    let mut tt1: f64 = 0.0;
    let mut tt2: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraTdbtt(tdb1, tdb2, dtr, &mut tt1, &mut tt2)
    }

    return (tt1, tt2);
}

///
///  - - - - - - - - -
///   e r a T t t a i
///  - - - - - - - - -
///
///  Time scale transformation:  Terrestrial Time, TT, to International
///  Atomic Time, TAI.
///
///  Given:
///     tt1,tt2    double    TT as a 2-part Julian Date
///
///  Returned:
///     tai1,tai2  double    TAI as a 2-part Julian Date
///
///  Returned (function value):
///                int       status:  0 = OK
///
///  Note:
///
///     tt1+tt2 is Julian Date, apportioned in any convenient way between
///     the two arguments, for example where tt1 is the Julian Day Number
///     and tt2 is the fraction of a day.  The returned tai1,tai2 follow
///     suit.
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992)
///
///  This revision:  2021 May 11
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Tttai(tt1: f64, tt2: f64) -> (f64, f64) {
    let mut tai1: f64 = 0.0;
    let mut tai2: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraTttai(tt1, tt2, &mut tai1, &mut tai2)
    }

    return (tai1, tai2);
}

///
///  - - - - - - - - -
///   e r a T t t c g
///  - - - - - - - - -
///
///  Time scale transformation:  Terrestrial Time, TT, to Geocentric
///  Coordinate Time, TCG.
///
///  Given:
///     tt1,tt2    double    TT as a 2-part Julian Date
///
///  Returned:
///     tcg1,tcg2  double    TCG as a 2-part Julian Date
///
///  Returned (function value):
///                int       status:  0 = OK
///
///  Note:
///
///     tt1+tt2 is Julian Date, apportioned in any convenient way between
///     the two arguments, for example where tt1 is the Julian Day Number
///     and tt2 is the fraction of a day.  The returned tcg1,tcg2 follow
///     suit.
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     IAU 2000 Resolution B1.9
///
///  This revision:  2021 May 11
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Tttcg(tt1: f64, tt2: f64) -> (f64, f64) {
    let mut tcg1: f64 = 0.0;
    let mut tcg2: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraTttcg(tt1, tt2, &mut tcg1, &mut tcg2)
    }

    return (tcg1, tcg2);
}

///
///  - - - - - - - - -
///   e r a T t t d b
///  - - - - - - - - -
///
///  Time scale transformation:  Terrestrial Time, TT, to Barycentric
///  Dynamical Time, TDB.
///
///  Given:
///     tt1,tt2    double    TT as a 2-part Julian Date
///     dtr        double    TDB-TT in seconds
///
///  Returned:
///     tdb1,tdb2  double    TDB as a 2-part Julian Date
///
///  Returned (function value):
///                int       status:  0 = OK
///
///  Notes:
///
///  1) tt1+tt2 is Julian Date, apportioned in any convenient way between
///     the two arguments, for example where tt1 is the Julian Day Number
///     and tt2 is the fraction of a day.  The returned tdb1,tdb2 follow
///     suit.
///
///  2) The argument dtr represents the quasi-periodic component of the
///     GR transformation between TT and TCB.  It is dependent upon the
///     adopted solar-system ephemeris, and can be obtained by numerical
///     integration, by interrogating a precomputed time ephemeris or by
///     evaluating a model such as that implemented in the ERFA function
///     eraDtdb.   The quantity is dominated by an annual term of 1.7 ms
///     amplitude.
///
///  3) TDB is essentially the same as Teph, the time argument for the JPL
///     solar system ephemerides.
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     IAU 2006 Resolution 3
///
///  This revision:  2021 May 11
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Tttdb(tt1: f64, tt2: f64, dtr: f64) -> (f64, f64) {
    let mut tdb1: f64 = 0.0;
    let mut tdb2: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraTttdb(tt1, tt2, dtr, &mut tdb1, &mut tdb2)
    }

    return (tdb1, tdb2);
}

///
///  - - - - - - - - -
///   e r a T t u t 1
///  - - - - - - - - -
///
///  Time scale transformation:  Terrestrial Time, TT, to Universal Time,
///  UT1.
///
///  Given:
///     tt1,tt2    double    TT as a 2-part Julian Date
///     dt         double    TT-UT1 in seconds
///
///  Returned:
///     ut11,ut12  double    UT1 as a 2-part Julian Date
///
///  Returned (function value):
///                int       status:  0 = OK
///
///  Notes:
///
///  1) tt1+tt2 is Julian Date, apportioned in any convenient way between
///     the two arguments, for example where tt1 is the Julian Day Number
///     and tt2 is the fraction of a day.  The returned ut11,ut12 follow
///     suit.
///
///  2) The argument dt is classical Delta T.
///
///  Reference:
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992)
///
///  This revision:  2021 May 11
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Ttut1(tt1: f64, tt2: f64, dt: f64) -> (f64, f64) {
    let mut ut11: f64 = 0.0;
    let mut ut12: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraTtut1(tt1, tt2, dt, &mut ut11, &mut ut12)
    }

    return (ut11, ut12);
}

///
///  - - - - - - - - - -
///   e r a U t 1 t a i
///  - - - - - - - - - -
///
///  Time scale transformation:  Universal Time, UT1, to International
///  Atomic Time, TAI.
///
///  Given:
///     ut11,ut12  double    UT1 as a 2-part Julian Date
///     dta        double    UT1-TAI in seconds
///
///  Returned:
///     tai1,tai2  double    TAI as a 2-part Julian Date
///
///  Returned (function value):
///                int       status:  0 = OK
///
///  Notes:
///
///  1) ut11+ut12 is Julian Date, apportioned in any convenient way
///     between the two arguments, for example where ut11 is the Julian
///     Day Number and ut12 is the fraction of a day.  The returned
///     tai1,tai2 follow suit.
///
///  2) The argument dta, i.e. UT1-TAI, is an observed quantity, and is
///     available from IERS tabulations.
///
///  Reference:
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992)
///
///  This revision:  2021 May 11
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Ut1tai(ut11: f64, ut12: f64, dta: f64) -> (f64, f64) {
    let mut tai1: f64 = 0.0;
    let mut tai2: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraUt1tai(ut11, ut12, dta, &mut tai1, &mut tai2)
    }

    return (tai1, tai2);
}

///
///  - - - - - - - - -
///   e r a U t 1 t t
///  - - - - - - - - -
///
///  Time scale transformation:  Universal Time, UT1, to Terrestrial
///  Time, TT.
///
///  Given:
///     ut11,ut12  double    UT1 as a 2-part Julian Date
///     dt         double    TT-UT1 in seconds
///
///  Returned:
///     tt1,tt2    double    TT as a 2-part Julian Date
///
///  Returned (function value):
///                int       status:  0 = OK
///
///  Notes:
///
///  1) ut11+ut12 is Julian Date, apportioned in any convenient way
///     between the two arguments, for example where ut11 is the Julian
///     Day Number and ut12 is the fraction of a day.  The returned
///     tt1,tt2 follow suit.
///
///  2) The argument dt is classical Delta T.
///
///  Reference:
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992)
///
///  This revision:  2021 May 11
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Ut1tt(ut11: f64, ut12: f64, dt: f64) -> (f64, f64) {
    let mut tt1: f64 = 0.0;
    let mut tt2: f64 = 0.0;

    unsafe {
        // Always returns 0
        _ = eraUt1tt(ut11, ut12, dt, &mut tt1, &mut tt2)
    }

    return (tt1, tt2);
}

///
///  - - - - - - - - - -
///   e r a U t 1 u t c
///  - - - - - - - - - -
///
///  Time scale transformation:  Universal Time, UT1, to Coordinated
///  Universal Time, UTC.
///
///  Given:
///     ut11,ut12  double   UT1 as a 2-part Julian Date (Note 1)
///     dut1       double   Delta UT1: UT1-UTC in seconds (Note 2)
///
///  Returned:
///     utc1,utc2  double   UTC as a 2-part quasi Julian Date (Notes 3,4)
///
///  Returned (function value):
///                int      status: +1 = dubious year (Note 5)
///                                  0 = OK
///                                 -1 = unacceptable date
///
///  Notes:
///
///  1) ut11+ut12 is Julian Date, apportioned in any convenient way
///     between the two arguments, for example where ut11 is the Julian
///     Day Number and ut12 is the fraction of a day.  The returned utc1
///     and utc2 form an analogous pair, except that a special convention
///     is used, to deal with the problem of leap seconds - see Note 3.
///
///  2) Delta UT1 can be obtained from tabulations provided by the
///     International Earth Rotation and Reference Systems Service.  The
///     value changes abruptly by 1s at a leap second;  however, close to
///     a leap second the algorithm used here is tolerant of the "wrong"
///     choice of value being made.
///
///  3) JD cannot unambiguously represent UTC during a leap second unless
///     special measures are taken.  The convention in the present
///     function is that the returned quasi-JD UTC1+UTC2 represents UTC
///     days whether the length is 86399, 86400 or 86401 SI seconds.
///
///  4) The function eraD2dtf can be used to transform the UTC quasi-JD
///     into calendar date and clock time, including UTC leap second
///     handling.
///
///  5) The warning status "dubious year" flags UTCs that predate the
///     introduction of the time scale or that are too far in the future
///     to be trusted.  See eraDat for further details.
///
///  Called:
///     eraJd2cal    JD to Gregorian calendar
///     eraDat       delta(AT) = TAI-UTC
///     eraCal2jd    Gregorian calendar to JD
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992)
///
///  This revision:  2023 May 6
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Ut1utc(ut11: f64, ut12: f64, dut1: f64) -> Result<(f64, f64), ERFAError> {
    let mut utc1: f64 = 0.0;
    let mut utc2: f64 = 0.0;
    let err: i32;

    unsafe { err = eraUt1utc(ut11, ut12, dut1, &mut utc1, &mut utc2) }

    match err {
        -1 => Err(ERFAError::ERFABadDate),
        0 => Ok((utc1, utc2)),
        1 => Ok((utc1, utc2)),
        _ => unexpected_val_err!(eraUt1utc),
    }
}

///
///  - - - - - - - - - -
///   e r a U t c t a i
///  - - - - - - - - - -
///
///  Time scale transformation:  Coordinated Universal Time, UTC, to
///  International Atomic Time, TAI.
///
///  Given:
///     utc1,utc2  double   UTC as a 2-part quasi Julian Date (Notes 1-4)
///
///  Returned:
///     tai1,tai2  double   TAI as a 2-part Julian Date (Note 5)
///
///  Returned (function value):
///                int      status: +1 = dubious year (Note 3)
///                                  0 = OK
///                                 -1 = unacceptable date
///
///  Notes:
///
///  1) utc1+utc2 is quasi Julian Date (see Note 2), apportioned in any
///     convenient way between the two arguments, for example where utc1
///     is the Julian Day Number and utc2 is the fraction of a day.
///
///  2) JD cannot unambiguously represent UTC during a leap second unless
///     special measures are taken.  The convention in the present
///     function is that the JD day represents UTC days whether the
///     length is 86399, 86400 or 86401 SI seconds.  In the 1960-1972 era
///     there were smaller jumps (in either direction) each time the
///     linear UTC(TAI) expression was changed, and these "mini-leaps"
///     are also included in the ERFA convention.
///
///  3) The warning status "dubious year" flags UTCs that predate the
///     introduction of the time scale or that are too far in the future
///     to be trusted.  See eraDat for further details.
///
///  4) The function eraDtf2d converts from calendar date and time of day
///     into 2-part Julian Date, and in the case of UTC implements the
///     leap-second-ambiguity convention described above.
///
///  5) The returned TAI1,TAI2 are such that their sum is the TAI Julian
///     Date.
///
///  Called:
///     eraJd2cal    JD to Gregorian calendar
///     eraDat       delta(AT) = TAI-UTC
///     eraCal2jd    Gregorian calendar to JD
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992)
///
///  This revision:  2021 May 11
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Utctai(utc1: f64, utc2: f64) -> Result<(f64, f64), ERFAError> {
    let mut tai1: f64 = 0.0;
    let mut tai2: f64 = 0.0;
    let err: i32;

    unsafe { err = eraUtctai(utc1, utc2, &mut tai1, &mut tai2) }

    match err {
        -1 => Err(ERFAError::ERFABadDate),
        0 => Ok((tai1, tai2)),
        1 => Ok((tai1, tai2)),
        _ => unexpected_val_err!(eraUtctai),
    }
}

///
///  - - - - - - - - - -
///   e r a U t c u t 1
///  - - - - - - - - - -
///
///  Time scale transformation:  Coordinated Universal Time, UTC, to
///  Universal Time, UT1.
///
///  Given:
///     utc1,utc2  double   UTC as a 2-part quasi Julian Date (Notes 1-4)
///     dut1       double   Delta UT1 = UT1-UTC in seconds (Note 5)
///
///  Returned:
///     ut11,ut12  double   UT1 as a 2-part Julian Date (Note 6)
///
///  Returned (function value):
///                int      status: +1 = dubious year (Note 3)
///                                  0 = OK
///                                 -1 = unacceptable date
///
///  Notes:
///
///  1) utc1+utc2 is quasi Julian Date (see Note 2), apportioned in any
///     convenient way between the two arguments, for example where utc1
///     is the Julian Day Number and utc2 is the fraction of a day.
///
///  2) JD cannot unambiguously represent UTC during a leap second unless
///     special measures are taken.  The convention in the present
///     function is that the JD day represents UTC days whether the
///     length is 86399, 86400 or 86401 SI seconds.
///
///  3) The warning status "dubious year" flags UTCs that predate the
///     introduction of the time scale or that are too far in the future
///     to be trusted.  See eraDat for further details.
///
///  4) The function eraDtf2d converts from calendar date and time of
///     day into 2-part Julian Date, and in the case of UTC implements
///     the leap-second-ambiguity convention described above.
///
///  5) Delta UT1 can be obtained from tabulations provided by the
///     International Earth Rotation and Reference Systems Service.
///     It is the caller's responsibility to supply a dut1 argument
///     containing the UT1-UTC value that matches the given UTC.
///
///  6) The returned ut11,ut12 are such that their sum is the UT1 Julian
///     Date.
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992)
///
///  Called:
///     eraJd2cal    JD to Gregorian calendar
///     eraDat       delta(AT) = TAI-UTC
///     eraUtctai    UTC to TAI
///     eraTaiut1    TAI to UT1
///
///  This revision:  2021 May 11
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Utcut1(utc1: f64, utc2: f64, dut1: f64) -> Result<(f64, f64), ERFAError> {
    let mut ut11: f64 = 0.0;
    let mut ut12: f64 = 0.0;
    let err: i32;

    unsafe { err = eraUtcut1(utc1, utc2, dut1, &mut ut11, &mut ut12) }

    match err {
        -1 => Err(ERFAError::ERFABadDate),
        0 => Ok((ut11, ut12)),
        1 => Ok((ut11, ut12)),
        _ => unexpected_val_err!(eraUtcut1),
    }
}
