//! ERFA Star Catalog Functions

use crate::{ERFAError, raw::starcatalogs::*, unexpected_val_err};

///
///  - - - - - - - - -
///   e r a F k 4 2 5
///  - - - - - - - - -
///
///  Convert B1950.0 FK4 star catalog data to J2000.0 FK5.
///
///  This function converts a star's catalog data from the old FK4
///  (Bessel-Newcomb) system to the later IAU 1976 FK5 (Fricke) system.
///
///  Given: (all B1950.0, FK4)
///     r1950,d1950    double   B1950.0 RA,Dec (rad)
///     dr1950,dd1950  double   B1950.0 proper motions (rad/trop.yr)
///     p1950          double   parallax (arcsec)
///     v1950          double   radial velocity (km/s, +ve = moving away)
///
///  Returned: (all J2000.0, FK5)
///     r2000,d2000    double   J2000.0 RA,Dec (rad)
///     dr2000,dd2000  double   J2000.0 proper motions (rad/Jul.yr)
///     p2000          double   parallax (arcsec)
///     v2000          double   radial velocity (km/s, +ve = moving away)
///
///  Notes:
///
///  1) The proper motions in RA are dRA/dt rather than cos(Dec)*dRA/dt,
///     and are per year rather than per century.
///
///  2) The conversion is somewhat complicated, for several reasons:
///
///     . Change of standard epoch from B1950.0 to J2000.0.
///
///     . An intermediate transition date of 1984 January 1.0 TT.
///
///     . A change of precession model.
///
///     . Change of time unit for proper motion (tropical to Julian).
///
///     . FK4 positions include the E-terms of aberration, to simplify
///       the hand computation of annual aberration.  FK5 positions
///       assume a rigorous aberration computation based on the Earth's
///       barycentric velocity.
///
///     . The E-terms also affect proper motions, and in particular cause
///       objects at large distances to exhibit fictitious proper
///       motions.
///
///     The algorithm is based on Smith et al. (1989) and Yallop et al.
///     (1989), which presented a matrix method due to Standish (1982) as
///     developed by Aoki et al. (1983), using Kinoshita's development of
///     Andoyer's post-Newcomb precession.  The numerical constants from
///     Seidelmann (1992) are used canonically.
///
///  3) Conversion from B1950.0 FK4 to J2000.0 FK5 only is provided for.
///     Conversions for different epochs and equinoxes would require
///     additional treatment for precession, proper motion and E-terms.
///
///  4) In the FK4 catalog the proper motions of stars within 10 degrees
///     of the poles do not embody differential E-terms effects and
///     should, strictly speaking, be handled in a different manner from
///     stars outside these regions.  However, given the general lack of
///     homogeneity of the star data available for routine astrometry,
///     the difficulties of handling positions that may have been
///     determined from astrometric fields spanning the polar and non-
///     polar regions, the likelihood that the differential E-terms
///     effect was not taken into account when allowing for proper motion
///     in past astrometry, and the undesirability of a discontinuity in
///     the algorithm, the decision has been made in this ERFA algorithm
///     to include the effects of differential E-terms on the proper
///     motions for all stars, whether polar or not.  At epoch J2000.0,
///     and measuring "on the sky" rather than in terms of RA change, the
///     errors resulting from this simplification are less than
///     1 milliarcsecond in position and 1 milliarcsecond per century in
///     proper motion.
///
///  Called:
///     eraAnp       normalize angle into range 0 to 2pi
///     eraPv2s      pv-vector to spherical coordinates
///     eraPdp       scalar product of two p-vectors
///     eraPvmpv     pv-vector minus pv_vector
///     eraPvppv     pv-vector plus pv_vector
///     eraS2pv      spherical coordinates to pv-vector
///     eraSxp       multiply p-vector by scalar
///
///  References:
///
///     Aoki, S. et al., 1983, "Conversion matrix of epoch B1950.0
///     FK4-based positions of stars to epoch J2000.0 positions in
///     accordance with the new IAU resolutions".  Astron.Astrophys.
///     128, 263-267.
///
///     Seidelmann, P.K. (ed), 1992, "Explanatory Supplement to the
///     Astronomical Almanac", ISBN 0-935702-68-7.
///
///     Smith, C.A. et al., 1989, "The transformation of astrometric
///     catalog systems to the equinox J2000.0".  Astron.J. 97, 265.
///
///     Standish, E.M., 1982, "Conversion of positions and proper motions
///     from B1950.0 to the IAU system at J2000.0".  Astron.Astrophys.,
///     115, 1, 20-22.
///
///     Yallop, B.D. et al., 1989, "Transformation of mean star places
///     from FK4 B1950.0 to FK5 J2000.0 using matrices in 6-space".
///     Astron.J. 97, 274.
///
///  This revision:   2023 March 20
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Fk425(
    r1950: f64,
    d1950: f64,
    dr1950: f64,
    dd1950: f64,
    p1950: f64,
    v1950: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    let mut r2000: f64 = 0.0;
    let mut d2000: f64 = 0.0;
    let mut dr2000: f64 = 0.0;
    let mut dd2000: f64 = 0.0;
    let mut p2000: f64 = 0.0;
    let mut v2000: f64 = 0.0;

    unsafe {
        eraFk425(
            r1950,
            d1950,
            dr1950,
            dd1950,
            p1950,
            v1950,
            &mut r2000,
            &mut d2000,
            &mut dr2000,
            &mut dd2000,
            &mut p2000,
            &mut v2000,
        );
    }

    return (r2000, d2000, dr2000, dd2000, p2000, v2000);
}

///
///  - - - - - - - - -
///   e r a F k 4 5 z
///  - - - - - - - - -
///
///  Convert a B1950.0 FK4 star position to J2000.0 FK5, assuming zero
///  proper motion in the FK5 system.
///
///  This function converts a star's catalog data from the old FK4
///  (Bessel-Newcomb) system to the later IAU 1976 FK5 (Fricke) system,
///  in such a way that the FK5 proper motion is zero.  Because such a
///  star has, in general, a non-zero proper motion in the FK4 system,
///  the function requires the epoch at which the position in the FK4
///  system was determined.
///
///  Given:
///     r1950,d1950    double   B1950.0 FK4 RA,Dec at epoch (rad)
///     bepoch         double   Besselian epoch (e.g. 1979.3)
///
///  Returned:
///     r2000,d2000    double   J2000.0 FK5 RA,Dec (rad)
///
///  Notes:
///
///  1) The epoch bepoch is strictly speaking Besselian, but if a
///     Julian epoch is supplied the result will be affected only to a
///     negligible extent.
///
///  2) The method is from Appendix 2 of Aoki et al. (1983), but using
///     the constants of Seidelmann (1992).  See the function eraFk425
///     for a general introduction to the FK4 to FK5 conversion.
///
///  3) Conversion from equinox B1950.0 FK4 to equinox J2000.0 FK5 only
///     is provided for.  Conversions for different starting and/or
///     ending epochs would require additional treatment for precession,
///     proper motion and E-terms.
///
///  4) In the FK4 catalog the proper motions of stars within 10 degrees
///     of the poles do not embody differential E-terms effects and
///     should, strictly speaking, be handled in a different manner from
///     stars outside these regions.  However, given the general lack of
///     homogeneity of the star data available for routine astrometry,
///     the difficulties of handling positions that may have been
///     determined from astrometric fields spanning the polar and non-
///     polar regions, the likelihood that the differential E-terms
///     effect was not taken into account when allowing for proper motion
///     in past astrometry, and the undesirability of a discontinuity in
///     the algorithm, the decision has been made in this ERFA algorithm
///     to include the effects of differential E-terms on the proper
///     motions for all stars, whether polar or not.  At epoch J2000.0,
///     and measuring "on the sky" rather than in terms of RA change, the
///     errors resulting from this simplification are less than
///     1 milliarcsecond in position and 1 milliarcsecond per century in
///     proper motion.
///
///  References:
///
///     Aoki, S. et al., 1983, "Conversion matrix of epoch B1950.0
///     FK4-based positions of stars to epoch J2000.0 positions in
///     accordance with the new IAU resolutions".  Astron.Astrophys.
///     128, 263-267.
///
///     Seidelmann, P.K. (ed), 1992, "Explanatory Supplement to the
///     Astronomical Almanac", ISBN 0-935702-68-7.
///
///  Called:
///     eraAnp       normalize angle into range 0 to 2pi
///     eraC2s       p-vector to spherical
///     eraEpb2jd    Besselian epoch to Julian date
///     eraEpj       Julian date to Julian epoch
///     eraPdp       scalar product of two p-vectors
///     eraPmp       p-vector minus p-vector
///     eraPpsp      p-vector plus scaled p-vector
///     eraPvu       update a pv-vector
///     eraS2c       spherical to p-vector
///
///  This revision:   2023 March 4
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Fk45z(r1950: f64, d1950: f64, bepoch: f64) -> (f64, f64) {
    let mut r2000: f64 = 0.0;
    let mut d2000: f64 = 0.0;

    unsafe {
        eraFk45z(r1950, d1950, bepoch, &mut r2000, &mut d2000);
    }

    return (r2000, d2000);
}

///
///  - - - - - - - - -
///   e r a F k 5 2 4
///  - - - - - - - - -
///
///  Convert J2000.0 FK5 star catalog data to B1950.0 FK4.
///
///  Given: (all J2000.0, FK5)
///     r2000,d2000    double   J2000.0 RA,Dec (rad)
///     dr2000,dd2000  double   J2000.0 proper motions (rad/Jul.yr)
///     p2000          double   parallax (arcsec)
///     v2000          double   radial velocity (km/s, +ve = moving away)
///
///  Returned: (all B1950.0, FK4)
///     r1950,d1950    double   B1950.0 RA,Dec (rad)
///     dr1950,dd1950  double   B1950.0 proper motions (rad/trop.yr)
///     p1950          double   parallax (arcsec)
///     v1950          double   radial velocity (km/s, +ve = moving away)
///
///  Notes:
///
///  1) The proper motions in RA are dRA/dt rather than cos(Dec)*dRA/dt,
///     and are per year rather than per century.
///
///  2) The conversion is somewhat complicated, for several reasons:
///
///     . Change of standard epoch from J2000.0 to B1950.0.
///
///     . An intermediate transition date of 1984 January 1.0 TT.
///
///     . A change of precession model.
///
///     . Change of time unit for proper motion (Julian to tropical).
///
///     . FK4 positions include the E-terms of aberration, to simplify
///       the hand computation of annual aberration.  FK5 positions
///       assume a rigorous aberration computation based on the Earth's
///       barycentric velocity.
///
///     . The E-terms also affect proper motions, and in particular cause
///       objects at large distances to exhibit fictitious proper
///       motions.
///
///     The algorithm is based on Smith et al. (1989) and Yallop et al.
///     (1989), which presented a matrix method due to Standish (1982) as
///     developed by Aoki et al. (1983), using Kinoshita's development of
///     Andoyer's post-Newcomb precession.  The numerical constants from
///     Seidelmann (1992) are used canonically.
///
///  4) In the FK4 catalog the proper motions of stars within 10 degrees
///     of the poles do not embody differential E-terms effects and
///     should, strictly speaking, be handled in a different manner from
///     stars outside these regions.  However, given the general lack of
///     homogeneity of the star data available for routine astrometry,
///     the difficulties of handling positions that may have been
///     determined from astrometric fields spanning the polar and non-
///     polar regions, the likelihood that the differential E-terms
///     effect was not taken into account when allowing for proper motion
///     in past astrometry, and the undesirability of a discontinuity in
///     the algorithm, the decision has been made in this ERFA algorithm
///     to include the effects of differential E-terms on the proper
///     motions for all stars, whether polar or not.  At epoch J2000.0,
///     and measuring "on the sky" rather than in terms of RA change, the
///     errors resulting from this simplification are less than
///     1 milliarcsecond in position and 1 milliarcsecond per century in
///     proper motion.
///
///  Called:
///     eraAnp       normalize angle into range 0 to 2pi
///     eraPdp       scalar product of two p-vectors
///     eraPm        modulus of p-vector
///     eraPmp       p-vector minus p-vector
///     eraPpp       p-vector pluus p-vector
///     eraPv2s      pv-vector to spherical coordinates
///     eraS2pv      spherical coordinates to pv-vector
///     eraSxp       multiply p-vector by scalar
///
///  References:
///
///     Aoki, S. et al., 1983, "Conversion matrix of epoch B1950.0
///     FK4-based positions of stars to epoch J2000.0 positions in
///     accordance with the new IAU resolutions".  Astron.Astrophys.
///     128, 263-267.
///
///     Seidelmann, P.K. (ed), 1992, "Explanatory Supplement to the
///     Astronomical Almanac", ISBN 0-935702-68-7.
///
///     Smith, C.A. et al., 1989, "The transformation of astrometric
///     catalog systems to the equinox J2000.0".  Astron.J. 97, 265.
///
///     Standish, E.M., 1982, "Conversion of positions and proper motions
///     from B1950.0 to the IAU system at J2000.0".  Astron.Astrophys.,
///     115, 1, 20-22.
///
///     Yallop, B.D. et al., 1989, "Transformation of mean star places
///     from FK4 B1950.0 to FK5 J2000.0 using matrices in 6-space".
///     Astron.J. 97, 274.
///
///  This revision:   2023 March 20
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Fk524(
    r2000: f64,
    d2000: f64,
    dr2000: f64,
    dd2000: f64,
    p2000: f64,
    v2000: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    let mut r1950: f64 = 0.0;
    let mut d1950: f64 = 0.0;
    let mut dr1950: f64 = 0.0;
    let mut dd1950: f64 = 0.0;
    let mut p1950: f64 = 0.0;
    let mut v1950: f64 = 0.0;

    unsafe {
        eraFk524(
            r2000,
            d2000,
            dr2000,
            dd2000,
            p2000,
            v2000,
            &mut r1950,
            &mut d1950,
            &mut dr1950,
            &mut dd1950,
            &mut p1950,
            &mut v1950,
        );
    }

    return (r1950, d1950, dr1950, dd1950, p1950, v1950);
}

///
///  - - - - - - - - -
///   e r a F k 5 2 h
///  - - - - - - - - -
///
///  Transform FK5 (J2000.0) star data into the Hipparcos system.
///
///  Given (all FK5, equinox J2000.0, epoch J2000.0):
///     r5      double    RA (radians)
///     d5      double    Dec (radians)
///     dr5     double    proper motion in RA (dRA/dt, rad/Jyear)
///     dd5     double    proper motion in Dec (dDec/dt, rad/Jyear)
///     px5     double    parallax (arcsec)
///     rv5     double    radial velocity (km/s, positive = receding)
///
///  Returned (all Hipparcos, epoch J2000.0):
///     rh      double    RA (radians)
///     dh      double    Dec (radians)
///     drh     double    proper motion in RA (dRA/dt, rad/Jyear)
///     ddh     double    proper motion in Dec (dDec/dt, rad/Jyear)
///     pxh     double    parallax (arcsec)
///     rvh     double    radial velocity (km/s, positive = receding)
///
///  Notes:
///
///  1) This function transforms FK5 star positions and proper motions
///     into the system of the Hipparcos catalog.
///
///  2) The proper motions in RA are dRA/dt rather than
///     cos(Dec)*dRA/dt, and are per year rather than per century.
///
///  3) The FK5 to Hipparcos transformation is modeled as a pure
///     rotation and spin;  zonal errors in the FK5 catalog are not
///     taken into account.
///
///  4) See also eraH2fk5, eraFk5hz, eraHfk5z.
///
///  Called:
///     eraStarpv    star catalog data to space motion pv-vector
///     eraFk5hip    FK5 to Hipparcos rotation and spin
///     eraRxp       product of r-matrix and p-vector
///     eraPxp       vector product of two p-vectors
///     eraPpp       p-vector plus p-vector
///     eraPvstar    space motion pv-vector to star catalog data
///
///  Reference:
///
///     F.Mignard & M.Froeschle, Astron.Astrophys., 354, 732-739 (2000).
///
///  This revision:  2021 May 11
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Fk52h(
    r5: f64,
    d5: f64,
    dr5: f64,
    dd5: f64,
    px5: f64,
    rv5: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    let mut rh: f64 = 0.0;
    let mut dh: f64 = 0.0;
    let mut drh: f64 = 0.0;
    let mut ddh: f64 = 0.0;
    let mut pxh: f64 = 0.0;
    let mut rvh: f64 = 0.0;

    unsafe {
        eraFk52h(
            r5, d5, dr5, dd5, px5, rv5, &mut rh, &mut dh, &mut drh, &mut ddh, &mut pxh, &mut rvh,
        );
    }

    return (rh, dh, drh, ddh, pxh, rvh);
}

///
///  - - - - - - - - -
///   e r a F k 5 4 z
///  - - - - - - - - -
///
///  Convert a J2000.0 FK5 star position to B1950.0 FK4, assuming zero
///  proper motion in FK5 and parallax.
///
///  Given:
///     r2000,d2000    double   J2000.0 FK5 RA,Dec (rad)
///     bepoch         double   Besselian epoch (e.g. 1950.0)
///
///  Returned:
///     r1950,d1950    double   B1950.0 FK4 RA,Dec (rad) at epoch BEPOCH
///     dr1950,dd1950  double   B1950.0 FK4 proper motions (rad/trop.yr)
///
///  Notes:
///
///  1) In contrast to the eraFk524 function, here the FK5 proper
///     motions, the parallax and the radial velocity are presumed zero.
///
///  2) This function converts a star position from the IAU 1976 FK5
///    (Fricke) system to the former FK4 (Bessel-Newcomb) system, for
///     cases such as distant radio sources where it is presumed there is
///     zero parallax and no proper motion.  Because of the E-terms of
///     aberration, such objects have (in general) non-zero proper motion
///     in FK4, and the present function returns those fictitious proper
///     motions.
///
///  3) Conversion from J2000.0 FK5 to B1950.0 FK4 only is provided for.
///     Conversions involving other equinoxes would require additional
///     treatment for precession.
///
///  4) The position returned by this function is in the B1950.0 FK4
///     reference system but at Besselian epoch bepoch.  For comparison
///     with catalogs the bepoch argument will frequently be 1950.0. (In
///     this context the distinction between Besselian and Julian epoch
///     is insignificant.)
///
///  5) The RA component of the returned (fictitious) proper motion is
///     dRA/dt rather than cos(Dec)*dRA/dt.
///
///  Called:
///     eraAnp       normalize angle into range 0 to 2pi
///     eraC2s       p-vector to spherical
///     eraFk524     FK4 to FK5
///     eraS2c       spherical to p-vector
///
///  This revision:   2023 March 5
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Fk54z(r2000: f64, d2000: f64, bepoch: f64) -> (f64, f64, f64, f64) {
    let mut r1950: f64 = 0.0;
    let mut d1950: f64 = 0.0;
    let mut dr1950: f64 = 0.0;
    let mut dd1950: f64 = 0.0;

    unsafe {
        eraFk54z(
            r2000,
            d2000,
            bepoch,
            &mut r1950,
            &mut d1950,
            &mut dr1950,
            &mut dd1950,
        );
    }

    return (r1950, d1950, dr1950, dd1950);
}

///
///  - - - - - - - - - -
///   e r a F k 5 h i p
///  - - - - - - - - - -
///
///  FK5 to Hipparcos rotation and spin.
///
///  Returned:
///     r5h   double[3][3]  r-matrix: FK5 rotation wrt Hipparcos (Note 2)
///     s5h   double[3]     r-vector: FK5 spin wrt Hipparcos (Note 3)
///
///  Notes:
///
///  1) This function models the FK5 to Hipparcos transformation as a
///     pure rotation and spin;  zonal errors in the FK5 catalog are not
///     taken into account.
///
///  2) The r-matrix r5h operates in the sense:
///
///           P_Hipparcos = r5h x P_FK5
///
///     where P_FK5 is a p-vector in the FK5 frame, and P_Hipparcos is
///     the equivalent Hipparcos p-vector.
///
///  3) The r-vector s5h represents the time derivative of the FK5 to
///     Hipparcos rotation.  The units are radians per year (Julian,
///     TDB).
///
///  Called:
///     eraRv2m      r-vector to r-matrix
///
///  Reference:
///
///     F.Mignard & M.Froeschle, Astron.Astrophys., 354, 732-739 (2000).
///
///  This revision:  2023 March 6
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Fk5hip() -> ([f64; 9], [f64; 3]) {
    let mut r5h: [f64; 9] = [0.0; 9];
    let mut s5h: [f64; 3] = [0.0; 3];

    unsafe {
        eraFk5hip(&mut r5h, &mut s5h);
    }

    return (r5h, s5h);
}

///
///  - - - - - - - - -
///   e r a F k 5 h z
///  - - - - - - - - -
///
///  Transform an FK5 (J2000.0) star position into the system of the
///  Hipparcos catalog, assuming zero Hipparcos proper motion.
///
///  Given:
///     r5           double   FK5 RA (radians), equinox J2000.0, at date
///     d5           double   FK5 Dec (radians), equinox J2000.0, at date
///     date1,date2  double   TDB date (Notes 1,2)
///
///  Returned:
///     rh           double   Hipparcos RA (radians)
///     dh           double   Hipparcos Dec (radians)
///
///  Notes:
///
///  1) This function converts a star position from the FK5 system to
///     the Hipparcos system, in such a way that the Hipparcos proper
///     motion is zero.  Because such a star has, in general, a non-zero
///     proper motion in the FK5 system, the function requires the date
///     at which the position in the FK5 system was determined.
///
///  2) The TT date date1+date2 is a Julian Date, apportioned in any
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
///  3) The FK5 to Hipparcos transformation is modeled as a pure
///     rotation and spin;  zonal errors in the FK5 catalog are not
///     taken into account.
///
///  4) The position returned by this function is in the Hipparcos
///     reference system but at date date1+date2.
///
///  5) See also eraFk52h, eraH2fk5, eraHfk5z.
///
///  Called:
///     eraS2c       spherical coordinates to unit vector
///     eraFk5hip    FK5 to Hipparcos rotation and spin
///     eraSxp       multiply p-vector by scalar
///     eraRv2m      r-vector to r-matrix
///     eraTrxp      product of transpose of r-matrix and p-vector
///     eraPxp       vector product of two p-vectors
///     eraC2s       p-vector to spherical
///     eraAnp       normalize angle into range 0 to 2pi
///
///  Reference:
///
///     F.Mignard & M.Froeschle, 2000, Astron.Astrophys. 354, 732-739.
///
///  This revision:  2023 March 6
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Fk5hz(r5: f64, d5: f64, date1: f64, date2: f64) -> (f64, f64) {
    let mut rh: f64 = 0.0;
    let mut dh: f64 = 0.0;

    unsafe {
        eraFk5hz(r5, d5, date1, date2, &mut rh, &mut dh);
    }

    return (rh, dh);
}

///
///  - - - - - - - - -
///   e r a H 2 f k 5
///  - - - - - - - - -
///
///  Transform Hipparcos star data into the FK5 (J2000.0) system.
///
///  Given (all Hipparcos, epoch J2000.0):
///     rh      double    RA (radians)
///     dh      double    Dec (radians)
///     drh     double    proper motion in RA (dRA/dt, rad/Jyear)
///     ddh     double    proper motion in Dec (dDec/dt, rad/Jyear)
///     pxh     double    parallax (arcsec)
///     rvh     double    radial velocity (km/s, positive = receding)
///
///  Returned (all FK5, equinox J2000.0, epoch J2000.0):
///     r5      double    RA (radians)
///     d5      double    Dec (radians)
///     dr5     double    proper motion in RA (dRA/dt, rad/Jyear)
///     dd5     double    proper motion in Dec (dDec/dt, rad/Jyear)
///     px5     double    parallax (arcsec)
///     rv5     double    radial velocity (km/s, positive = receding)
///
///  Notes:
///
///  1) This function transforms Hipparcos star positions and proper
///     motions into FK5 J2000.0.
///
///  2) The proper motions in RA are dRA/dt rather than
///     cos(Dec)*dRA/dt, and are per year rather than per century.
///
///  3) The FK5 to Hipparcos transformation is modeled as a pure
///     rotation and spin;  zonal errors in the FK5 catalog are not
///     taken into account.
///
///  4) See also eraFk52h, eraFk5hz, eraHfk5z.
///
///  Called:
///     eraStarpv    star catalog data to space motion pv-vector
///     eraFk5hip    FK5 to Hipparcos rotation and spin
///     eraRv2m      r-vector to r-matrix
///     eraRxp       product of r-matrix and p-vector
///     eraTrxp      product of transpose of r-matrix and p-vector
///     eraPxp       vector product of two p-vectors
///     eraPmp       p-vector minus p-vector
///     eraPvstar    space motion pv-vector to star catalog data
///
///  Reference:
///
///     F.Mignard & M.Froeschle, Astron.Astrophys., 354, 732-739 (2000).
///
///  This revision:  2021 May 11
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn H2fk5(
    rh: f64,
    dh: f64,
    drh: f64,
    ddh: f64,
    pxh: f64,
    rvh: f64,
) -> (f64, f64, f64, f64, f64, f64) {
    let mut r5: f64 = 0.0;
    let mut d5: f64 = 0.0;
    let mut dr5: f64 = 0.0;
    let mut dd5: f64 = 0.0;
    let mut px5: f64 = 0.0;
    let mut rv5: f64 = 0.0;

    unsafe {
        eraH2fk5(
            rh, dh, drh, ddh, pxh, rvh, &mut r5, &mut d5, &mut dr5, &mut dd5, &mut px5, &mut rv5,
        );
    }

    return (r5, d5, dr5, dd5, px5, rv5);
}

///
///  - - - - - - - - -
///   e r a H f k 5 z
///  - - - - - - - - -
///
///  Transform a Hipparcos star position into FK5 J2000.0, assuming
///  zero Hipparcos proper motion.
///
///  Given:
///     rh            double    Hipparcos RA (radians)
///     dh            double    Hipparcos Dec (radians)
///     date1,date2   double    TDB date (Note 1)
///
///  Returned (all FK5, equinox J2000.0, date date1+date2):
///     r5            double    RA (radians)
///     d5            double    Dec (radians)
///     dr5           double    RA proper motion (rad/year, Note 4)
///     dd5           double    Dec proper motion (rad/year, Note 4)
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
///  2) The proper motion in RA is dRA/dt rather than cos(Dec)*dRA/dt.
///
///  3) The FK5 to Hipparcos transformation is modeled as a pure rotation
///     and spin;  zonal errors in the FK5 catalog are not taken into
///     account.
///
///  4) It was the intention that Hipparcos should be a close
///     approximation to an inertial frame, so that distant objects have
///     zero proper motion;  such objects have (in general) non-zero
///     proper motion in FK5, and this function returns those fictitious
///     proper motions.
///
///  5) The position returned by this function is in the FK5 J2000.0
///     reference system but at date date1+date2.
///
///  6) See also eraFk52h, eraH2fk5, eraFk5hz.
///
///  Called:
///     eraS2c       spherical coordinates to unit vector
///     eraFk5hip    FK5 to Hipparcos rotation and spin
///     eraRxp       product of r-matrix and p-vector
///     eraSxp       multiply p-vector by scalar
///     eraRxr       product of two r-matrices
///     eraTrxp      product of transpose of r-matrix and p-vector
///     eraPxp       vector product of two p-vectors
///     eraPv2s      pv-vector to spherical
///     eraAnp       normalize angle into range 0 to 2pi
///
///  Reference:
///
///     F.Mignard & M.Froeschle, 2000, Astron.Astrophys. 354, 732-739.
///
///  This revision:  2023 March 7
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Hfk5z(rh: f64, dh: f64, date1: f64, date2: f64) -> (f64, f64, f64, f64) {
    let mut r5: f64 = 0.0;
    let mut d5: f64 = 0.0;
    let mut dr5: f64 = 0.0;
    let mut dd5: f64 = 0.0;

    unsafe {
        eraHfk5z(rh, dh, date1, date2, &mut r5, &mut d5, &mut dr5, &mut dd5);
    }

    return (r5, d5, dr5, dd5);
}

///
///  - - - - - - - - - -
///   e r a S t a r p m
///  - - - - - - - - - -
///
///  Star proper motion:  update star catalog data for space motion.
///
///  Given:
///     ra1    double     right ascension (radians), before
///     dec1   double     declination (radians), before
///     pmr1   double     RA proper motion (radians/year), before
///     pmd1   double     Dec proper motion (radians/year), before
///     px1    double     parallax (arcseconds), before
///     rv1    double     radial velocity (km/s, +ve = receding), before
///     ep1a   double     "before" epoch, part A (Note 1)
///     ep1b   double     "before" epoch, part B (Note 1)
///     ep2a   double     "after" epoch, part A (Note 1)
///     ep2b   double     "after" epoch, part B (Note 1)
///
///  Returned:
///     ra2    double     right ascension (radians), after
///     dec2   double     declination (radians), after
///     pmr2   double     RA proper motion (radians/year), after
///     pmd2   double     Dec proper motion (radians/year), after
///     px2    double     parallax (arcseconds), after
///     rv2    double     radial velocity (km/s, +ve = receding), after
///
///  Returned (function value):
///            int        status:
///                          -1 = system error (should not occur)
///                           0 = no warnings or errors
///                           1 = distance overridden (Note 6)
///                           2 = excessive velocity (Note 7)
///                           4 = solution didn't converge (Note 8)
///                        else = binary logical OR of the above warnings
///
///  Notes:
///
///  1) The starting and ending TDB dates ep1a+ep1b and ep2a+ep2b are
///     Julian Dates, apportioned in any convenient way between the two
///     parts (A and B).  For example, JD(TDB)=2450123.7 could be
///     expressed in any of these ways, among others:
///
///            epNa            epNb
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
///     good compromises between resolution and convenience.
///
///  2) In accordance with normal star-catalog conventions, the object's
///     right ascension and declination are freed from the effects of
///     secular aberration.  The frame, which is aligned to the catalog
///     equator and equinox, is Lorentzian and centered on the SSB.
///
///     The proper motions are the rate of change of the right ascension
///     and declination at the catalog epoch and are in radians per TDB
///     Julian year.
///
///     The parallax and radial velocity are in the same frame.
///
///  3) Care is needed with units.  The star coordinates are in radians
///     and the proper motions in radians per Julian year, but the
///     parallax is in arcseconds.
///
///  4) The RA proper motion is in terms of coordinate angle, not true
///     angle.  If the catalog uses arcseconds for both RA and Dec proper
///     motions, the RA proper motion will need to be divided by cos(Dec)
///     before use.
///
///  5) Straight-line motion at constant speed, in the inertial frame,
///     is assumed.
///
///  6) An extremely small (or zero or negative) parallax is interpreted
///     to mean that the object is on the "celestial sphere", the radius
///     of which is an arbitrary (large) value (see the eraStarpv
///     function for the value used).  When the distance is overridden in
///     this way, the status, initially zero, has 1 added to it.
///
///  7) If the space velocity is a significant fraction of c (see the
///     constant VMAX in the function eraStarpv), it is arbitrarily set
///     to zero.  When this action occurs, 2 is added to the status.
///
///  8) The relativistic adjustment carried out in the eraStarpv function
///     involves an iterative calculation.  If the process fails to
///     converge within a set number of iterations, 4 is added to the
///     status.
///
///  Called:
///     eraStarpv    star catalog data to space motion pv-vector
///     eraPvu       update a pv-vector
///     eraPdp       scalar product of two p-vectors
///     eraPvstar    space motion pv-vector to star catalog data
///
///  This revision:  2023 May 3
///
///  Copyright (C) 2013-2023, NumFOCUS Foundation.
///  Derived, with permission, from the SOFA library.  See notes at end of file.
///
pub fn Starpm(
    ra1: f64,
    dec1: f64,
    pmr1: f64,
    pmd1: f64,
    px1: f64,
    rv1: f64,
    ep1a: f64,
    ep1b: f64,
    ep2a: f64,
    ep2b: f64,
) -> Result<(f64, f64, f64, f64, f64, f64), ERFAError> {
    let mut ra2: f64 = 0.0;
    let mut dec2: f64 = 0.0;
    let mut pmr2: f64 = 0.0;
    let mut pmd2: f64 = 0.0;
    let mut px2: f64 = 0.0;
    let mut rv2: f64 = 0.0;
    let err: i32;

    unsafe {
        err = eraStarpm(
            ra1, dec1, pmr1, pmd1, px1, rv1, ep1a, ep1b, ep2a, ep2b, &mut ra2, &mut dec2,
            &mut pmr2, &mut pmd2, &mut px2, &mut rv2,
        )
    }

    match err {
        0 => Ok((ra2, dec2, pmr2, pmd2, px2, rv2)),
        1 => Ok((ra2, dec2, pmr2, pmd2, px2, rv2)),
        2 => Ok((ra2, dec2, pmr2, pmd2, px2, rv2)),
        4 => Ok((ra2, dec2, pmr2, pmd2, px2, rv2)),
        -1 => Err(ERFAError::ERFAInternalError),
        _ => unexpected_val_err!(eraStarpm),
    }
}
