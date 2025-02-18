//! ERFA Horizon/Equatorial Functions

use crate::raw::horizonequatorial::*;

pub fn Ae2hd(az: f64, el: f64, phi: f64) -> (f64, f64) {
    let mut ha: f64 = 0.0;
    let mut dec: f64 = 0.0;

    unsafe {
        eraAe2hd(az, el, phi, &mut ha, &mut dec);
    }

    return (ha, dec)
}

pub fn Hd2ae(ha: f64, dec: f64, phi: f64) -> (f64, f64) {
    let mut az: f64 = 0.0;
    let mut el: f64 = 0.0;

    unsafe {
        eraHd2ae(ha, dec, phi, &mut az, &mut el);
    }

    return (az, el)
}

pub fn Hd2pa(ha: f64, dec: f64, phi: f64) -> f64 {
    return unsafe {
        eraHd2pa(ha, dec, phi)
    }
}