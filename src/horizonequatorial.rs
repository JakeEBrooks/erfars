//! ERFA Horizon/Equatorial Functions

use crate::raw::horizonequatorial::*;

/// Horizon to equatorial coordinates: transform azimuth and altitude to hour
/// angle and declination.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/ae2hd.c)
pub fn Ae2hd(az: f64, el: f64, phi: f64) -> (f64, f64) {
    let mut ha: f64 = 0.0;
    let mut dec: f64 = 0.0;

    unsafe {
        eraAe2hd(az, el, phi, &mut ha, &mut dec);
    }

    return (ha, dec);
}

/// Equatorial to horizon coordinates: transform hour angle and declination to
/// azimuth and altitude.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/hd2ae.c)
pub fn Hd2ae(ha: f64, dec: f64, phi: f64) -> (f64, f64) {
    let mut az: f64 = 0.0;
    let mut el: f64 = 0.0;

    unsafe {
        eraHd2ae(ha, dec, phi, &mut az, &mut el);
    }

    return (az, el);
}

/// Parallactic angle for a given hour angle and declination.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/hd2pa.c)
pub fn Hd2pa(ha: f64, dec: f64, phi: f64) -> f64 {
    return unsafe { eraHd2pa(ha, dec, phi) };
}
