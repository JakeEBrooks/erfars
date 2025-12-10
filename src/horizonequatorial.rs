//! ERFA Horizon/Equatorial Functions

use crate::raw::horizonequatorial::*;

///  Horizon to equatorial coordinates:  transform azimuth and altitude
///  to hour angle and declination.
///
///  Given:
///     az       double       azimuth
///     el       double       altitude (informally, elevation)
///     phi      double       site latitude
///
///  Returned:
///     ha       double       hour angle (local)
///     dec      double       declination
///
///  Notes:
///
///  1)  All the arguments are angles in radians.
///
///  2)  The sign convention for azimuth is north zero, east +pi/2.
///
///  3)  HA is returned in the range +/-pi.  Declination is returned in
///      the range +/-pi/2.
///
///  4)  The latitude phi is pi/2 minus the angle between the Earth's
///      rotation axis and the adopted zenith.  In many applications it
///      will be sufficient to use the published geodetic latitude of the
///      site.  In very precise (sub-arcsecond) applications, phi can be
///      corrected for polar motion.
///
///  5)  The azimuth az must be with respect to the rotational north pole,
///      as opposed to the ITRS pole, and an azimuth with respect to north
///      on a map of the Earth's surface will need to be adjusted for
///      polar motion if sub-arcsecond accuracy is required.
///
///  6)  Should the user wish to work with respect to the astronomical
///      zenith rather than the geodetic zenith, phi will need to be
///      adjusted for deflection of the vertical (often tens of
///      arcseconds), and the zero point of ha will also be affected.
///
///  7)  The transformation is the same as Ve = Ry(phi-pi/2)*Rz(pi)*Vh,
///      where Ve and Vh are lefthanded unit vectors in the (ha,dec) and
///      (az,el) systems respectively and Rz and Ry are rotations about
///      first the z-axis and then the y-axis.  (n.b. Rz(pi) simply
///      reverses the signs of the x and y components.)  For efficiency,
///      the algorithm is written out rather than calling other utility
///      functions.  For applications that require even greater
///      efficiency, additional savings are possible if constant terms
///      such as functions of latitude are computed once and for all.
///
///  8)  Again for efficiency, no range checking of arguments is carried
///      out.
pub fn Ae2hd(az: f64, el: f64, phi: f64) -> (f64, f64) {
    let mut ha: f64 = 0.0;
    let mut dec: f64 = 0.0;

    unsafe {
        eraAe2hd(az, el, phi, &mut ha, &mut dec);
    }

    return (ha, dec);
}

///  Equatorial to horizon coordinates:  transform hour angle and
///  declination to azimuth and altitude.
///
///  Given:
///     ha       double       hour angle (local)
///     dec      double       declination
///     phi      double       site latitude
///
///  Returned:
///     *az      double       azimuth
///     *el      double       altitude (informally, elevation)
///
///  Notes:
///
///  1)  All the arguments are angles in radians.
///
///  2)  Azimuth is returned in the range 0-2pi;  north is zero, and east
///      is +pi/2.  Altitude is returned in the range +/- pi/2.
///
///  3)  The latitude phi is pi/2 minus the angle between the Earth's
///      rotation axis and the adopted zenith.  In many applications it
///      will be sufficient to use the published geodetic latitude of the
///      site.  In very precise (sub-arcsecond) applications, phi can be
///      corrected for polar motion.
///
///  4)  The returned azimuth az is with respect to the rotational north
///      pole, as opposed to the ITRS pole, and for sub-arcsecond
///      accuracy will need to be adjusted for polar motion if it is to
///      be with respect to north on a map of the Earth's surface.
///
///  5)  Should the user wish to work with respect to the astronomical
///      zenith rather than the geodetic zenith, phi will need to be
///      adjusted for deflection of the vertical (often tens of
///      arcseconds), and the zero point of the hour angle ha will also
///      be affected.
///
///  6)  The transformation is the same as Vh = Rz(pi)*Ry(pi/2-phi)*Ve,
///      where Vh and Ve are lefthanded unit vectors in the (az,el) and
///      (ha,dec) systems respectively and Ry and Rz are rotations about
///      first the y-axis and then the z-axis.  (n.b. Rz(pi) simply
///      reverses the signs of the x and y components.)  For efficiency,
///      the algorithm is written out rather than calling other utility
///      functions.  For applications that require even greater
///      efficiency, additional savings are possible if constant terms
///      such as functions of latitude are computed once and for all.
///
///  7)  Again for efficiency, no range checking of arguments is carried
///      out.
pub fn Hd2ae(ha: f64, dec: f64, phi: f64) -> (f64, f64) {
    let mut az: f64 = 0.0;
    let mut el: f64 = 0.0;

    unsafe {
        eraHd2ae(ha, dec, phi, &mut az, &mut el);
    }

    return (az, el);
}

///  Parallactic angle for a given hour angle and declination.
///
///  Given:
///     ha     double     hour angle
///     dec    double     declination
///     phi    double     site latitude
///
///  Returned (function value):
///            double     parallactic angle
///
///  Notes:
///
///  1)  All the arguments are angles in radians.
///
///  2)  The parallactic angle at a point in the sky is the position
///      angle of the vertical, i.e. the angle between the directions to
///      the north celestial pole and to the zenith respectively.
///
///  3)  The result is returned in the range -pi to +pi.
///
///  4)  At the pole itself a zero result is returned.
///
///  5)  The latitude phi is pi/2 minus the angle between the Earth's
///      rotation axis and the adopted zenith.  In many applications it
///      will be sufficient to use the published geodetic latitude of the
///      site.  In very precise (sub-arcsecond) applications, phi can be
///      corrected for polar motion.
///
///  6)  Should the user wish to work with respect to the astronomical
///      zenith rather than the geodetic zenith, phi will need to be
///      adjusted for deflection of the vertical (often tens of
///      arcseconds), and the zero point of the hour angle ha will also
///      be affected.
///
///  Reference:
///     Smart, W.M., "Spherical Astronomy", Cambridge University Press,
///     6th edition (Green, 1977), p49.
pub fn Hd2pa(ha: f64, dec: f64, phi: f64) -> f64 {
    return unsafe { eraHd2pa(ha, dec, phi) };
}
