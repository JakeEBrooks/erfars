//! ERFA Astrometry Functions

use std::ffi::CString;

use crate::{Astrom, ERFAResult, LDBody, raw::astrometry::*, unexpected_val_err};

/// Apply aberration to transform natural direction into proper direction.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/ab.c)
pub fn Ab(pnat: &[f64; 3], v: &[f64; 3], s: f64, bm1: f64) -> [f64; 3] {
    let mut ppr: [f64; 3] = [0.0; 3];
    unsafe { eraAb(pnat, v, s, bm1, &mut ppr) }
    return ppr;
}

/// For a geocentric observer, prepare star-independent astrometry parameters
/// for transformations between ICRS and GCRS coordinates. The Earth ephemeris
/// is supplied by the caller.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/apcg.c)
pub fn Apcg(date1: f64, date2: f64, ebpv: &[f64; 6], ehp: &[f64; 3], astrom: &mut Astrom) {
    unsafe { eraApcg(date1, date2, ebpv, ehp, astrom) }
}

/// For a geocentric observer, prepare star-independent astrometry parameters
/// for transformations between ICRS and GCRS coordinates. The caller supplies
/// the date, and ERFA models are used to predict the Earth ephemeris.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/apcg13.c)
pub fn Apcg13(date1: f64, date2: f64, astrom: &mut Astrom) {
    unsafe { eraApcg13(date1, date2, astrom) }
}

/// For a terrestrial observer, prepare star-independent astrometry parameters
/// for transformations between ICRS and geocentric CIRS coordinates. The Earth
/// ephemeris and CIP/CIO are supplied by the caller.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/apci.c)
pub fn Apci(date1: f64, date2: f64, ebpv: &[f64; 6], ehp: &[f64; 3], x: f64, y: f64, s: f64, astrom: &mut Astrom) {
    unsafe { eraApci(date1, date2, ebpv, ehp, x, y, s, astrom) }
}

/// For a terrestrial observer, prepare star-independent astrometry parameters
/// for transformations between ICRS and geocentric CIRS coordinates. The caller
/// supplies the date, and ERFA models are used to predict the Earth ephemeris
/// and CIP/CIO.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/apci13.c)
pub fn Apci13(date1: f64, date2: f64, astrom: &mut Astrom) -> f64 {
    let mut eo: f64 = 0.0;
    unsafe { eraApci13(date1, date2, astrom, &mut eo) }
    return eo;
}

/// For a terrestrial observer, prepare star-independent astrometry parameters
/// for transformations between ICRS and observed coordinates. The caller
/// supplies the Earth ephemeris, the Earth rotation information and the
/// refraction constants as well as the site coordinates.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/apco.c)
pub fn Apco(
    date1: f64,
    date2: f64,
    ebpv: &[f64; 6],
    ehp: &[f64; 3],
    x: f64,
    y: f64,
    s: f64,
    theta: f64,
    elong: f64,
    phi: f64,
    hm: f64,
    xp: f64,
    yp: f64,
    sp: f64,
    refa: f64,
    refb: f64,
    astrom: &mut Astrom,
) {
    unsafe { eraApco(date1, date2, ebpv, ehp, x, y, s, theta, elong, phi, hm, xp, yp, sp, refa, refb, astrom) }
}

/// For a terrestrial observer, prepare star-independent astrometry parameters
/// for transformations between ICRS and observed coordinates. The caller
/// supplies UTC, site coordinates, ambient air conditions and observing
/// wavelength, and ERFA models are used to obtain the Earth ephemeris, CIP/CIO
/// and refraction constants.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/apco13.c)
pub fn Apco13(utc1: f64, utc2: f64, dut1: f64, elong: f64, phi: f64, hm: f64, xp: f64, yp: f64, phpa: f64, tc: f64, rh: f64, w1: f64, astrom: &mut Astrom) -> ERFAResult<f64> {
    let mut eo: f64 = 0.0;
    let err: i32;
    unsafe { err = eraApco13(utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, w1, astrom, &mut eo) }

    match err {
        1 => Ok((eo, 1)),
        0 => Ok((eo, 0)),
        -1 => Err(-1),
        _ => unexpected_val_err!(eraApco13),
    }
}

/// For an observer whose geocentric position and velocity are known, prepare
/// star-independent astrometry parameters for transformations between ICRS and
/// GCRS. The Earth ephemeris is supplied by the caller.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/apcs.c)
pub fn Apcs(date1: f64, date2: f64, pv: &[f64; 6], ebpv: &[f64; 6], ehp: &[f64; 3], astrom: &mut Astrom) {
    unsafe { eraApcs(date1, date2, pv, ebpv, ehp, astrom) }
}

/// For an observer whose geocentric position and velocity are known, prepare
/// star-independent astrometry parameters for transformations between ICRS and
/// GCRS. The Earth ephemeris is from ERFA models.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/apcs13.c)
pub fn Apcs13(date1: f64, date2: f64, pv: &[f64; 6], astrom: &mut Astrom) {
    unsafe { eraApcs13(date1, date2, pv, astrom) }
}

/// In the star-independent astrometry parameters, update only the Earth
/// rotation angle, supplied by the caller explicitly.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/aper.c)
pub fn Aper(theta: f64, astrom: &mut Astrom) {
    unsafe { eraAper(theta, astrom) }
}

/// In the star-independent astrometry parameters, update only the Earth
/// rotation angle. The caller provides UT1, (n.b. not UTC).
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/aper13.c)
pub fn Aper13(ut11: f64, ut12: f64, astrom: &mut Astrom) {
    unsafe { eraAper13(ut11, ut12, astrom) }
}

/// For a terrestrial observer, prepare star-independent astrometry parameters
/// for transformations between CIRS and observed coordinates. The caller
/// supplies the Earth orientation information and the refraction constants as
/// well as the site coordinates.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/apio.c)
pub fn Apio(sp: f64, theta: f64, elong: f64, phi: f64, hm: f64, xp: f64, yp: f64, refa: f64, refb: f64, astrom: &mut Astrom) {
    unsafe { eraApio(sp, theta, elong, phi, hm, xp, yp, refa, refb, astrom) }
}

/// For a terrestrial observer, prepare star-independent astrometry parameters
/// for transformations between CIRS and observed coordinates. The caller
/// supplies UTC, site coordinates, ambient air conditions and observing
/// wavelength.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/apio13.c)
pub fn Apio13(utc1: f64, utc2: f64, dut1: f64, elong: f64, phi: f64, hm: f64, xp: f64, yp: f64, phpa: f64, tc: f64, rh: f64, w1: f64, astrom: &mut Astrom) -> ERFAResult<()> {
    let err: i32;
    unsafe { err = eraApio13(utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, w1, astrom) }

    match err {
        1 => Ok(((), 1)),
        0 => Ok(((), 0)),
        -1 => Err(-1),
        _ => unexpected_val_err!(eraApio13),
    }
}

/// Transform a star's ICRS catalog entry (epoch J2000.0) into ICRS astrometric
/// place.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/atcc13.c)
pub fn Atcc13(rc: f64, dc: f64, pr: f64, pd: f64, px: f64, rv: f64, date1: f64, date2: f64) -> (f64, f64) {
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    unsafe { eraAtcc13(rc, dc, pr, pd, px, rv, date1, date2, &mut ra, &mut dec) }
    return (ra, dec);
}

/// Quick transformation of a star's ICRS catalog entry (epoch J2000.0) into
/// ICRS astrometric place, given precomputed star-independent astrometry
/// parameters.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/atccq.c)
pub fn Atccq(rc: f64, dc: f64, pr: f64, pd: f64, px: f64, rv: f64, astrom: &Astrom) -> (f64, f64) {
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    unsafe { eraAtccq(rc, dc, pr, pd, px, rv, astrom, &mut ra, &mut dec) }
    return (ra, dec);
}

/// Transform ICRS star data, epoch J2000.0, to CIRS.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/atci13.c)
pub fn Atci13(rc: f64, dc: f64, pr: f64, pd: f64, px: f64, rv: f64, date1: f64, date2: f64) -> (f64, f64, f64) {
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    let mut eo: f64 = 0.0;
    unsafe { eraAtci13(rc, dc, pr, pd, px, rv, date1, date2, &mut ra, &mut dec, &mut eo) }
    return (ra, dec, eo);
}

/// Quick ICRS, epoch J2000.0, to CIRS transformation, given precomputed star-
/// independent astrometry parameters.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/atciq.c)
pub fn Atciq(rc: f64, dc: f64, pr: f64, pd: f64, px: f64, rv: f64, astrom: &Astrom) -> (f64, f64) {
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    unsafe { eraAtciq(rc, dc, pr, pd, px, rv, astrom, &mut ra, &mut dec) }
    return (ra, dec);
}

/// Quick ICRS, epoch J2000.0, to CIRS transformation, given precomputed star-
/// independent astrometry parameters plus a list of light- deflecting bodies.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/atciqn.c)
pub fn Atciqn(rc: f64, dc: f64, pr: f64, pd: f64, px: f64, rv: f64, astrom: &Astrom, n: i32, b: &LDBody) -> (f64, f64) {
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    unsafe { eraAtciqn(rc, dc, pr, pd, px, rv, astrom, n, b, &mut ra, &mut dec) }
    return (ra, dec);
}

/// Quick ICRS to CIRS transformation, given precomputed star- independent
/// astrometry parameters, and assuming zero parallax and proper motion.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/atciqz.c)
pub fn Atciqz(rc: f64, dc: f64, astrom: &Astrom) -> (f64, f64) {
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    unsafe { eraAtciqz(rc, dc, astrom, &mut ra, &mut dec) }
    return (ra, dec);
}

/// ICRS RA,Dec to observed place. The caller supplies UTC, site coordinates,
/// ambient air conditions and observing wavelength.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/atco13.c)
pub fn Atco13(
    rc: f64,
    dc: f64,
    pr: f64,
    pd: f64,
    px: f64,
    rv: f64,
    utc1: f64,
    utc2: f64,
    dut1: f64,
    elong: f64,
    phi: f64,
    hm: f64,
    xp: f64,
    yp: f64,
    phpa: f64,
    tc: f64,
    rh: f64,
    w1: f64,
) -> ERFAResult<(f64, f64, f64, f64, f64, f64)> {
    let mut aob: f64 = 0.0;
    let mut zob: f64 = 0.0;
    let mut hob: f64 = 0.0;
    let mut dob: f64 = 0.0;
    let mut rob: f64 = 0.0;
    let mut eo: f64 = 0.0;
    let err: i32;
    unsafe {
        err = eraAtco13(
            rc, dc, pr, pd, px, rv, utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, w1, &mut aob, &mut zob, &mut hob, &mut dob, &mut rob, &mut eo,
        )
    }

    match err {
        1 => Ok(((aob, zob, hob, dob, rob, eo), 1)),
        0 => Ok(((aob, zob, hob, dob, rob, eo), 0)),
        -1 => Err(-1),
        _ => unexpected_val_err!(eraAtco13),
    }
}

/// Transform star RA,Dec from geocentric CIRS to ICRS astrometric.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/atic13.c)
pub fn Atic13(ri: f64, di: f64, date1: f64, date2: f64) -> (f64, f64, f64) {
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    let mut eo: f64 = 0.0;
    unsafe { eraAtic13(ri, di, date1, date2, &mut ra, &mut dec, &mut eo) }
    return (ra, dec, eo);
}

/// Quick CIRS RA,Dec to ICRS astrometric place, given the star- independent
/// astrometry parameters.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/aticq.c)
pub fn Aticq(ri: f64, di: f64, astrom: &Astrom) -> (f64, f64) {
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    unsafe { eraAticq(ri, di, astrom, &mut ra, &mut dec) }
    return (ra, dec);
}

/// Quick CIRS to ICRS astrometric place transformation, given the star-
/// independent astrometry parameters plus a list of light-deflecting bodies.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/aticqn.c)
pub fn Aticqn(ri: f64, di: f64, astrom: &Astrom, b: &[LDBody]) -> (f64, f64) {
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    unsafe { eraAticqn(ri, di, astrom, b.len().try_into().unwrap(), b.as_ptr(), &mut ra, &mut dec) }
    return (ra, dec);
}

/// CIRS RA,Dec to observed place. The caller supplies UTC, site coordinates,
/// ambient air conditions and observing wavelength.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/atio13.c)
pub fn Atio13(ri: f64, di: f64, utc1: f64, utc2: f64, dut1: f64, elong: f64, phi: f64, hm: f64, xp: f64, yp: f64, phpa: f64, tc: f64, rh: f64, w1: f64) -> ERFAResult<(f64, f64, f64, f64, f64)> {
    let mut aob: f64 = 0.0;
    let mut zob: f64 = 0.0;
    let mut hob: f64 = 0.0;
    let mut dob: f64 = 0.0;
    let mut rob: f64 = 0.0;
    let err: i32;
    unsafe { err = eraAtio13(ri, di, utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, w1, &mut aob, &mut zob, &mut hob, &mut dob, &mut rob) }
    match err {
        1 => Ok(((aob, zob, hob, dob, rob), 1)),
        0 => Ok(((aob, zob, hob, dob, rob), 0)),
        -1 => Err(-1),
        _ => unexpected_val_err!(eraAtio13),
    }
}

/// Quick CIRS to observed place transformation.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/atioq.c)
pub fn Atioq(ri: f64, di: f64, astrom: &Astrom) -> (f64, f64, f64, f64, f64) {
    let mut aob: f64 = 0.0;
    let mut zob: f64 = 0.0;
    let mut hob: f64 = 0.0;
    let mut dob: f64 = 0.0;
    let mut rob: f64 = 0.0;
    unsafe { eraAtioq(ri, di, astrom, &mut aob, &mut zob, &mut hob, &mut dob, &mut rob) }
    return (aob, zob, hob, dob, rob);
}

/// Observed place at a groundbased site to to ICRS astrometric RA,Dec. The
/// caller supplies UTC, site coordinates, ambient air conditions and observing
/// wavelength.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/atoc13.c)
pub fn Atoc13(ctype: char, ob1: f64, ob2: f64, utc1: f64, utc2: f64, dut1: f64, elong: f64, phi: f64, hm: f64, xp: f64, yp: f64, phpa: f64, tc: f64, rh: f64, w1: f64) -> ERFAResult<(f64, f64)> {
    let charin = match ctype {
        'R' => CString::new("R"),
        'r' => CString::new("r"),
        'H' => CString::new("H"),
        'h' => CString::new("h"),
        _ => CString::new("A"),
    };
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    let err: i32;
    unsafe { err = eraAtoc13(charin.unwrap().as_ptr(), ob1, ob2, utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, w1, &mut ra, &mut dec) }
    match err {
        1 => Ok(((ra, dec), 1)),
        0 => Ok(((ra, dec), 0)),
        -1 => Err(-1),
        _ => unexpected_val_err!(eraAtoc13),
    }
}

/// Observed place to CIRS. The caller supplies UTC, site coordinates, ambient
/// air conditions and observing wavelength.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/atoi13.c)
pub fn Atoi13(ctype: char, ob1: f64, ob2: f64, utc1: f64, utc2: f64, dut1: f64, elong: f64, phi: f64, hm: f64, xp: f64, yp: f64, phpa: f64, tc: f64, rh: f64, w1: f64) -> ERFAResult<(f64, f64)> {
    let charin = match ctype {
        'R' => CString::new("R"),
        'r' => CString::new("r"),
        'H' => CString::new("H"),
        'h' => CString::new("h"),
        _ => CString::new("A"),
    };
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    let err: i32;
    unsafe { err = eraAtoi13(charin.unwrap().as_ptr(), ob1, ob2, utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, w1, &mut ra, &mut dec) }
    match err {
        1 => Ok(((ra, dec), 1)),
        0 => Ok(((ra, dec), 0)),
        -1 => Err(-1),
        _ => unexpected_val_err!(eraAtoi13),
    }
}

/// Quick observed place to CIRS, given the star-independent astrometry
/// parameters.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/atoiq.c)
pub fn Atoiq(ctype: char, ob1: f64, ob2: f64, astrom: &Astrom) -> (f64, f64) {
    let charin = match ctype {
        'R' => CString::new("R"),
        'r' => CString::new("r"),
        'H' => CString::new("H"),
        'h' => CString::new("h"),
        _ => CString::new("A"),
    };
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    unsafe { eraAtoiq(charin.unwrap().as_ptr(), ob1, ob2, astrom, &mut ra, &mut dec) };
    return (ra, dec);
}

/// Apply light deflection by a solar-system body, as part of transforming
/// coordinate direction into natural direction.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/ld.c)
pub fn Ld(bm: f64, p: &[f64; 3], q: &[f64; 3], e: &[f64; 3], em: f64, dlim: f64) -> [f64; 3] {
    let mut p1: [f64; 3] = [0.0; 3];
    unsafe { eraLd(bm, p, q, e, em, dlim, &mut p1) }
    return p1;
}

/// For a star, apply light deflection by multiple solar-system bodies, as part
/// of transforming coordinate direction into natural direction.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/ldn.c)
pub fn Ldn(b: &[LDBody], ob: &[f64; 3], sc: &[f64; 3]) -> [f64; 3] {
    let mut sn: [f64; 3] = [0.0; 3];
    unsafe { eraLdn(b.len().try_into().unwrap(), b.as_ptr(), ob, sc, &mut sn) }
    return sn;
}

/// Deflection of starlight by the Sun.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/ldsun.c)
pub fn Ldsun(p: &[f64; 3], e: &[f64; 3], em: f64) -> [f64; 3] {
    let mut p1: [f64; 3] = [0.0; 3];
    unsafe { eraLdsun(p, e, em, &mut p1) }
    return p1;
}

/// Proper motion and parallax.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/pmpx.c)
pub fn Pmpx(rc: f64, dc: f64, pr: f64, pd: f64, px: f64, rv: f64, pmt: f64, pob: &[f64; 3]) -> [f64; 3] {
    let mut pco: [f64; 3] = [0.0; 3];
    unsafe { eraPmpx(rc, dc, pr, pd, px, rv, pmt, pob, &mut pco) }
    return pco;
}

/// Star proper motion: update star catalog data for space motion, with special
/// handling to handle the zero parallax case.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/pmsafe.c)
pub fn Pmsafe(ra: f64, dec: f64, pmr: f64, pmd: f64, px: f64, rv: f64, ep1a: f64, ep1b: f64, ep2a: f64, ep2b: f64) -> ERFAResult<(f64, f64, f64, f64, f64, f64)> {
    let mut ra2: f64 = 0.0;
    let mut dec2: f64 = 0.0;
    let mut pmr2: f64 = 0.0;
    let mut pmd2: f64 = 0.0;
    let mut px2: f64 = 0.0;
    let mut rv2: f64 = 0.0;
    let err: i32;
    unsafe { err = eraPmsafe(ra, dec, pmr, pmd, px, rv, ep1a, ep1b, ep2a, ep2b, &mut ra2, &mut dec2, &mut pmr2, &mut pmd2, &mut px2, &mut rv2) }

    return Ok(((ra2, dec2, pmr2, pmd2, px2, rv2), err));
}

/// Position and velocity of a terrestrial observing station.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/pvtob.c)
pub fn Pvtob(elong: f64, phi: f64, hm: f64, xp: f64, yp: f64, sp: f64, theta: f64) -> [f64; 6] {
    let mut pv: [f64; 6] = [0.0; 6];
    unsafe { eraPvtob(elong, phi, hm, xp, yp, sp, theta, &mut pv) }
    return pv;
}

/// Determine the constants A and B in the atmospheric refraction model dZ = A
/// tan Z + B tan^3 Z.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/refco.c)
pub fn Refco(phpa: f64, tc: f64, rh: f64, w1: f64) -> (f64, f64) {
    let mut refa: f64 = 0.0;
    let mut refb: f64 = 0.0;
    unsafe { eraRefco(phpa, tc, rh, w1, &mut refa, &mut refb) }
    return (refa, refb);
}
