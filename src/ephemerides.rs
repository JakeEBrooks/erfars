//! ERFA Ephemerides Functions

use crate::{ERFAError, raw::ephemerides::*, unexpected_val_err};

///  Earth position and velocity, heliocentric and barycentric, with
///  respect to the Barycentric Celestial Reference System.
///
///  Given:
///     date1,date2  double        TDB date (Note 1)
///
///  Returned:
///     pvh          double[2][3]  heliocentric Earth position/velocity
///     pvb          double[2][3]  barycentric Earth position/velocity
///
///  Returned (function value):
///                  int           status: 0 = OK
///                                       +1 = warning: date outside
///                                            the range 1900-2100 AD
///
///  Notes:
///
///  1) The TDB date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TDB)=2450123.7 could be expressed in any of these ways, among
///     others:
///
///            date1          date2
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
///     resolution.  The MJD method and the date & time methods are both
///     good compromises between resolution and convenience.  However,
///     the accuracy of the result is more likely to be limited by the
///     algorithm itself than the way the date has been expressed.
///
///     n.b. TT can be used instead of TDB in most applications.
///
///  2) On return, the arrays pvh and pvb contain the following:
///
///        pvh[0][0]  x       }
///        pvh[0][1]  y       } heliocentric position, au
///        pvh[0][2]  z       }
///
///        pvh[1][0]  xdot    }
///        pvh[1][1]  ydot    } heliocentric velocity, au/d
///        pvh[1][2]  zdot    }
///
///        pvb[0][0]  x       }
///        pvb[0][1]  y       } barycentric position, au
///        pvb[0][2]  z       }
///
///        pvb[1][0]  xdot    }
///        pvb[1][1]  ydot    } barycentric velocity, au/d
///        pvb[1][2]  zdot    }
///
///     The vectors are oriented with respect to the BCRS.  The time unit
///     is one day in TDB.
///
///  3) The function is a SIMPLIFIED SOLUTION from the planetary theory
///     VSOP2000 (X. Moisson, P. Bretagnon, 2001, Celes. Mechanics &
///     Dyn. Astron., 80, 3/4, 205-213) and is an adaptation of original
///     Fortran code supplied by P. Bretagnon (private comm., 2000).
///
///  4) Comparisons over the time span 1900-2100 with this simplified
///     solution and the JPL DE405 ephemeris give the following results:
///
///                                RMS    max
///           Heliocentric:
///              position error    3.7   11.2   km
///              velocity error    1.4    5.0   mm/s
///
///           Barycentric:
///              position error    4.6   13.4   km
///              velocity error    1.4    4.9   mm/s
///
///     Comparisons with the JPL DE406 ephemeris show that by 1800 and
///     2200 the position errors are approximately double their 1900-2100
///     size.  By 1500 and 2500 the deterioration is a factor of 10 and
///     by 1000 and 3000 a factor of 60.  The velocity accuracy falls off
///     at about half that rate.
///
///  5) It is permissible to use the same array for pvh and pvb, which
///     will receive the barycentric values.
pub fn Epv00(date1: f64, date2: f64) -> ([f64; 6], [f64; 6]) {
    let mut pvh: [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let mut pvb: [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let err: i32;
    unsafe { err = eraEpv00(date1, date2, &mut pvh, &mut pvb) }

    match err {
        0 => (pvh, pvb),
        // Ignore the warning
        1 => (pvh, pvb),
        _ => unexpected_val_err!(eraEpv00),
    }
}

///  Approximate geocentric position and velocity of the Moon.
///
///  n.b. Not IAU-endorsed and without canonical status.
///
///  Given:
///     date1  double         TT date part A (Notes 1,4)
///     date2  double         TT date part B (Notes 1,4)
///
///  Returned:
///     pv     double[2][3]   Moon p,v, GCRS (au, au/d, Note 5)
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
///     The JD method is the most natural and convenient to use in cases
///     where the loss of several decimal digits of resolution is
///     acceptable.  The J2000 method is best matched to the way the
///     argument is handled internally and will deliver the optimum
///     resolution.  The MJD method and the date & time methods are both
///     good compromises between resolution and convenience.  The limited
///     accuracy of the present algorithm is such that any of the methods
///     is satisfactory.
///
///  2) This function is a full implementation of the algorithm
///     published by Meeus (see reference) except that the light-time
///     correction to the Moon's mean longitude has been omitted.
///
///  3) Comparisons with ELP/MPP02 over the interval 1950-2100 gave RMS
///     errors of 2.9 arcsec in geocentric direction, 6.1 km in position
///     and 36 mm/s in velocity.  The worst case errors were 18.3 arcsec
///     in geocentric direction, 31.7 km in position and 172 mm/s in
///     velocity.
///
///  4) The original algorithm is expressed in terms of "dynamical time",
///     which can either be TDB or TT without any significant change in
///     accuracy.  UT cannot be used without incurring significant errors
///     (30 arcsec in the present era) due to the Moon's 0.5 arcsec/sec
///     movement.
///
///  5) The result is with respect to the GCRS (the same as J2000.0 mean
///     equator and equinox to within 23 mas).
///
///  6) Velocity is obtained by a complete analytical differentiation
///     of the Meeus model.
///
///  7) The Meeus algorithm generates position and velocity in mean
///     ecliptic coordinates of date, which the present function then
///     rotates into GCRS.  Because the ecliptic system is precessing,
///     there is a coupling between this spin (about 1.4 degrees per
///     century) and the Moon position that produces a small velocity
///     contribution.  In the present function this effect is neglected
///     as it corresponds to a maximum difference of less than 3 mm/s and
///     increases the RMS error by only 0.4%.
///
///  References:
///
///     Meeus, J., Astronomical Algorithms, 2nd edition, Willmann-Bell,
///     1998, p337.
///
///     Simon, J.L., Bretagnon, P., Chapront, J., Chapront-Touze, M.,
///     Francou, G. & Laskar, J., Astron.Astrophys., 1994, 282, 663
pub fn Moon98(date1: f64, date2: f64) -> [f64; 6] {
    let mut pv: [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    unsafe { eraMoon98(date1, date2, &mut pv) }

    return pv;
}

///  Approximate heliocentric position and velocity of a nominated
///  planet:  Mercury, Venus, EMB, Mars, Jupiter, Saturn, Uranus or
///  Neptune (but not the Earth itself).
///
///  n.b. Not IAU-endorsed and without canonical status.
///
///  Given:
///     date1  double       TDB date part A (Note 1)
///     date2  double       TDB date part B (Note 1)
///     np     int          planet (1=Mercury, 2=Venus, 3=EMB, 4=Mars,
///                             5=Jupiter, 6=Saturn, 7=Uranus, 8=Neptune)
///
///  Returned (argument):
///     pv     double[2][3] planet p,v (heliocentric, J2000.0, au,au/d)
///
///  Returned (function value):
///            int          status: -1 = illegal NP (outside 1-8)
///                                  0 = OK
///                                 +1 = warning: year outside 1000-3000
///                                 +2 = warning: failed to converge
///
///  Notes:
///
///  1) The date date1+date2 is in the TDB time scale (in practice TT can
///     be used) and is a Julian Date, apportioned in any convenient way
///     between the two arguments.  For example, JD(TDB)=2450123.7 could
///     be expressed in any of these ways, among others:
///
///            date1          date2
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
///     resolution.  The MJD method and the date & time methods are both
///     good compromises between resolution and convenience.  The limited
///     accuracy of the present algorithm is such that any of the methods
///     is satisfactory.
///
///  2) If an np value outside the range 1-8 is supplied, an error status
///     (function value -1) is returned and the pv vector set to zeroes.
///
///  3) For np=3 the result is for the Earth-Moon barycenter (EMB).  To
///     obtain the heliocentric position and velocity of the Earth, use
///     instead the ERFA function eraEpv00.
///
///  4) On successful return, the array pv contains the following:
///
///        pv[0][0]   x      }
///        pv[0][1]   y      } heliocentric position, au
///        pv[0][2]   z      }
///
///        pv[1][0]   xdot   }
///        pv[1][1]   ydot   } heliocentric velocity, au/d
///        pv[1][2]   zdot   }
///
///     The reference frame is equatorial and is with respect to the
///     mean equator and equinox of epoch J2000.0.
///
///  5) The algorithm is due to J.L. Simon, P. Bretagnon, J. Chapront,
///     M. Chapront-Touze, G. Francou and J. Laskar (Bureau des
///     Longitudes, Paris, France).  From comparisons with JPL
///     ephemeris DE102, they quote the following maximum errors
///     over the interval 1800-2050:
///
///                     L (arcsec)    B (arcsec)      R (km)
///
///        Mercury          4             1             300
///        Venus            5             1             800
///        EMB              6             1            1000
///        Mars            17             1            7700
///        Jupiter         71             5           76000
///        Saturn          81            13          267000
///        Uranus          86             7          712000
///        Neptune         11             1          253000
///
///     Over the interval 1000-3000, they report that the accuracy is no
///     worse than 1.5 times that over 1800-2050.  Outside 1000-3000 the
///     accuracy declines.
///
///     Comparisons of the present function with the JPL DE200 ephemeris
///     give the following RMS errors over the interval 1960-2025:
///
///                      position (km)     velocity (m/s)
///
///        Mercury            334               0.437
///        Venus             1060               0.855
///        EMB               2010               0.815
///        Mars              7690               1.98
///        Jupiter          71700               7.70
///        Saturn          199000              19.4
///        Uranus          564000              16.4
///        Neptune         158000              14.4
///
///     Comparisons against DE200 over the interval 1800-2100 gave the
///     following maximum absolute differences (the results using
///     DE406 were essentially the same):
///
///                   L (arcsec)   B (arcsec)     R (km)   Rdot (m/s)
///
///        Mercury        7            1            500       0.7
///        Venus          7            1           1100       0.9
///        EMB            9            1           1300       1.0
///        Mars          26            1           9000       2.5
///        Jupiter       78            6          82000       8.2
///        Saturn        87           14         263000      24.6
///        Uranus        86            7         661000      27.4
///        Neptune       11            2         248000      21.4
///
///  6) The present ERFA re-implementation of the original Simon et al.
///     Fortran code differs from the original in the following respects:
///
///       *  C instead of Fortran.
///
///       *  The date is supplied in two parts.
///
///       *  The result is returned only in equatorial Cartesian form;
///          the ecliptic longitude, latitude and radius vector are not
///          returned.
///
///       *  The result is in the J2000.0 equatorial frame, not ecliptic.
///
///       *  More is done in-line: there are fewer calls to subroutines.
///
///       *  Different error/warning status values are used.
///
///       *  A different Kepler's-equation-solver is used (avoiding
///          use of double precision complex).
///
///       *  Polynomials in t are nested to minimize rounding errors.
///
///       *  Explicit double constants are used to avoid mixed-mode
///          expressions.
///
///     None of the above changes affects the result significantly.
///
///  7) The returned status indicates the most serious condition
///     encountered during execution of the function.  Illegal np is
///     considered the most serious, overriding failure to converge,
///     which in turn takes precedence over the remote date warning.
///
///  Called:
///     eraAnpm      normalize angle into range +/- pi
///
///  Reference:  Simon, J.L, Bretagnon, P., Chapront, J.,
///              Chapront-Touze, M., Francou, G., and Laskar, J.,
///              Astron.Astrophys., 282, 663 (1994).
pub fn Plan94(date1: f64, date2: f64, planet: i32) -> Result<[f64; 6], ERFAError> {
    let mut pv: [f64; 6] = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
    let err: i32;
    unsafe { err = eraPlan94(date1, date2, planet, &mut pv) }

    match err {
        0 => Ok(pv),
        // Ignore the warning
        1 => Ok(pv),
        // Ignore the warning
        2 => Ok(pv),
        -1 => Err(ERFAError::ERFABadInputValue),
        _ => unexpected_val_err!(eraPlan94),
    }
}
