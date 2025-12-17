//! ERFA Precession, Nutation, and Polar Motion Functions

use crate::raw::precnutpolar::*;

/// Frame bias components of IAU 2000 precession-nutation models; part of the
/// Mathews-Herring-Buffett (MHB2000) nutation series, with additions.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/bi00.c)
pub fn Bi00() -> (f64, f64, f64) {
    let mut dpsibi: f64 = 0.0;
    let mut depsbi: f64 = 0.0;
    let mut dra: f64 = 0.0;
    unsafe {
        eraBi00(&mut dpsibi, &mut depsbi, &mut dra);
    }

    return (dpsibi, depsbi, dra);
}

/// Frame bias and precession, IAU 2000.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/bp00.c)
pub fn Bp00(date1: f64, date2: f64, rb: &mut [f64; 9], rp: &mut [f64; 9], rbp: &mut [f64; 9]) {
    unsafe {
        eraBp00(date1, date2, rb, rp, rbp);
    }
}

/// Frame bias and precession, IAU 2006.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/bp06.c)
pub fn Bp06(date1: f64, date2: f64, rb: &mut [f64; 9], rp: &mut [f64; 9], rbp: &mut [f64; 9]) {
    unsafe {
        eraBp06(date1, date2, rb, rp, rbp);
    }
}

/// Extract from the bias-precession-nutation matrix the X,Y coordinates of the
/// Celestial Intermediate Pole.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/bpn2xy.c)
pub fn Bpn2xy(rbpn: &[f64; 9]) -> (f64, f64) {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    unsafe {
        eraBpn2xy(rbpn, &mut x, &mut y);
    }

    return (x, y);
}

/// Form the celestial-to-intermediate matrix for a given date using the IAU
/// 2000A precession-nutation model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/c2i00a.c)
pub fn C2i00a(date1: f64, date2: f64, rc2i: &mut [f64; 9]) {
    unsafe {
        eraC2i00a(date1, date2, rc2i);
    }
}

/// Form the celestial-to-intermediate matrix for a given date using the IAU
/// 2000B precession-nutation model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/c2i00b.c)
pub fn C2i00b(date1: f64, date2: f64, rc2i: &mut [f64; 9]) {
    unsafe {
        eraC2i00b(date1, date2, rc2i);
    }
}

/// Form the celestial-to-intermediate matrix for a given date using the IAU
/// 2006 precession and IAU 2000A nutation models.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/c2i06a.c)
pub fn C2i06a(date1: f64, date2: f64, rc2i: &mut [f64; 9]) {
    unsafe {
        eraC2i06a(date1, date2, rc2i);
    }
}

/// Form the celestial-to-intermediate matrix for a given date given the bias-
/// precession-nutation matrix. IAU 2000.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/c2ibpn.c)
pub fn C2ibpn(date1: f64, date2: f64, rbpn: &[f64; 9], rc2i: &mut [f64; 9]) {
    unsafe {
        eraC2ibpn(date1, date2, rbpn, rc2i);
    }
}

/// Form the celestial to intermediate-frame-of-date matrix for a given date
/// when the CIP X,Y coordinates are known. IAU 2000.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/c2ixy.c)
pub fn C2ixy(date1: f64, date2: f64, x: f64, y: f64, rc2i: &mut [f64; 9]) {
    unsafe {
        eraC2ixy(date1, date2, x, y, rc2i);
    }
}

/// Form the celestial to intermediate-frame-of-date matrix given the CIP X,Y
/// and the CIO locator s.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/c2ixys.c)
pub fn C2ixys(x: f64, y: f64, s: f64, rc2i: &mut [f64; 9]) {
    unsafe {
        eraC2ixys(x, y, s, rc2i);
    }
}

/// Form the celestial to terrestrial matrix given the date, the UT1 and the
/// polar motion, using the IAU 2000A precession-nutation model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/c2t00a.c)
pub fn C2t00a(tta: f64, ttb: f64, uta: f64, utb: f64, xp: f64, yp: f64, rc2t: &mut [f64; 9]) {
    unsafe {
        eraC2t00a(tta, ttb, uta, utb, xp, yp, rc2t);
    }
}

/// Form the celestial to terrestrial matrix given the date, the UT1 and the
/// polar motion, using the IAU 2000B precession-nutation model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/c2t00b.c)
pub fn C2t00b(tta: f64, ttb: f64, uta: f64, utb: f64, xp: f64, yp: f64, rc2t: &mut [f64; 9]) {
    unsafe {
        eraC2t00b(tta, ttb, uta, utb, xp, yp, rc2t);
    }
}

/// Form the celestial to terrestrial matrix given the date, the UT1 and the
/// polar motion, using the IAU 2006/2000A precession-nutation model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/c2t06a.c)
pub fn C2t06a(tta: f64, ttb: f64, uta: f64, utb: f64, xp: f64, yp: f64, rc2t: &mut [f64; 9]) {
    unsafe {
        eraC2t06a(tta, ttb, uta, utb, xp, yp, rc2t);
    }
}

/// Assemble the celestial to terrestrial matrix from CIO-based components (the
/// celestial-to-intermediate matrix, the Earth Rotation Angle and the polar
/// motion matrix).
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/c2tcio.c)
pub fn C2tcio(rc2i: &[f64; 9], era: f64, rpom: &[f64; 9], rc2t: &mut [f64; 9]) {
    unsafe {
        eraC2tcio(rc2i, era, rpom, rc2t);
    }
}

/// Assemble the celestial to terrestrial matrix from equinox-based components
/// (the celestial-to-true matrix, the Greenwich Apparent Sidereal Time and the
/// polar motion matrix).
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/c2teqx.c)
pub fn C2teqx(rbpn: &[f64; 9], gst: f64, rpom: &[f64; 9], rc2t: &mut [f64; 9]) {
    unsafe {
        eraC2teqx(rbpn, gst, rpom, rc2t);
    }
}

/// Form the celestial to terrestrial matrix given the date, the UT1, the
/// nutation and the polar motion. IAU 2000.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/c2tpe.c)
pub fn C2tpe(tta: f64, ttb: f64, uta: f64, utb: f64, dpsi: f64, deps: f64, xp: f64, yp: f64, rc2t: &mut [f64; 9]) {
    unsafe {
        eraC2tpe(tta, ttb, uta, utb, dpsi, deps, xp, yp, rc2t);
    }
}

/// Form the celestial to terrestrial matrix given the date, the UT1, the CIP
/// coordinates and the polar motion. IAU 2000.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/c2txy.c)
pub fn C2txy(tta: f64, ttb: f64, uta: f64, utb: f64, x: f64, y: f64, xp: f64, yp: f64, rc2t: &mut [f64; 9]) {
    unsafe {
        eraC2txy(tta, ttb, uta, utb, x, y, xp, yp, rc2t);
    }
}

/// Equation of the origins, IAU 2006 precession and IAU 2000A nutation.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/eo06a.c)
pub fn Eo06a(date1: f64, date2: f64) -> f64 {
    return unsafe { eraEo06a(date1, date2) };
}

/// Equation of the origins, given the classical NPB matrix and the quantity s.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/eors.c)
pub fn Eors(rnpb: &[f64; 9], s: f64) -> f64 {
    return unsafe { eraEors(rnpb, s) };
}

/// Form rotation matrix given the Fukushima-Williams angles.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/fw2m.c)
pub fn Fw2m(gamb: f64, phib: f64, psi: f64, eps: f64, r: &mut [f64; 9]) {
    unsafe {
        eraFw2m(gamb, phib, psi, eps, r);
    }
}

/// CIP X,Y given Fukushima-Williams bias-precession-nutation angles.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/fw2xy.c)
pub fn Fw2xy(gamb: f64, phib: f64, psi: f64, eps: f64) -> (f64, f64) {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    unsafe {
        eraFw2xy(gamb, phib, psi, eps, &mut x, &mut y);
    }

    return (x, y);
}

/// Long-term precession matrix.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/ltp.c)
pub fn Ltp(epj: f64, rp: &mut [f64; 9]) {
    unsafe {
        eraLtp(epj, rp);
    }
}

/// Long-term precession matrix, including ICRS frame bias.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/ltpb.c)
pub fn Ltpb(epj: f64, rpb: &mut [f64; 9]) {
    unsafe {
        eraLtpb(epj, rpb);
    }
}

/// Long-term precession of the ecliptic.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/ltpecl.c)
pub fn Ltpecl(epj: f64) -> [f64; 3] {
    let mut vec: [f64; 3] = [0.0; 3];
    unsafe {
        eraLtpecl(epj, &mut vec);
    }

    return vec;
}

/// Long-term precession of the equator.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/ltpequ.c)
pub fn Ltpequ(epj: f64) -> [f64; 3] {
    let mut veq: [f64; 3] = [0.0; 3];
    unsafe {
        eraLtpequ(epj, &mut veq);
    }

    return veq;
}

/// Form the matrix of nutation for a given date, IAU 2000A model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/num00a.c)
pub fn Num00a(date1: f64, date2: f64, rmatn: &mut [f64; 9]) {
    unsafe {
        eraNum00a(date1, date2, rmatn);
    }
}

/// Form the matrix of nutation for a given date, IAU 2000B model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/num00b.c)
pub fn Num00b(date1: f64, date2: f64, rmatn: &mut [f64; 9]) {
    unsafe {
        eraNum00b(date1, date2, rmatn);
    }
}

/// Form the matrix of nutation for a given date, IAU 2006/2000A model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/num06a.c)
pub fn Num06a(date1: f64, date2: f64, rmatn: &mut [f64; 9]) {
    unsafe {
        eraNum06a(date1, date2, rmatn);
    }
}

/// Form the matrix of nutation.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/numat.c)
pub fn Numat(epsa: f64, dpsi: f64, deps: f64, rmatn: &mut [f64; 9]) {
    unsafe {
        eraNumat(epsa, dpsi, deps, rmatn);
    }
}

/// Nutation, IAU 2000A model (MHB2000 luni-solar and planetary nutation with
/// free core nutation omitted).
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/nut00a.c)
pub fn Nut00a(date1: f64, date2: f64) -> (f64, f64) {
    let mut dpsi: f64 = 0.0;
    let mut deps: f64 = 0.0;
    unsafe {
        eraNut00a(date1, date2, &mut dpsi, &mut deps);
    }

    return (dpsi, deps);
}

/// Nutation, IAU 2000B model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/nut00b.c)
pub fn Nut00b(date1: f64, date2: f64) -> (f64, f64) {
    let mut dpsi: f64 = 0.0;
    let mut deps: f64 = 0.0;
    unsafe {
        eraNut00b(date1, date2, &mut dpsi, &mut deps);
    }

    return (dpsi, deps);
}

/// IAU 2000A nutation with adjustments to match the IAU 2006 precession.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/nut06a.c)
pub fn Nut06a(date1: f64, date2: f64) -> (f64, f64) {
    let mut dpsi: f64 = 0.0;
    let mut deps: f64 = 0.0;
    unsafe {
        eraNut06a(date1, date2, &mut dpsi, &mut deps);
    }

    return (dpsi, deps);
}

/// Nutation, IAU 1980 model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/nut80.c)
pub fn Nut80(date1: f64, date2: f64) -> (f64, f64) {
    let mut dpsi: f64 = 0.0;
    let mut deps: f64 = 0.0;
    unsafe {
        eraNut80(date1, date2, &mut dpsi, &mut deps);
    }

    return (dpsi, deps);
}

/// Form the matrix of nutation for a given date, IAU 1980 model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/nutm80.c)
pub fn Nutm80(date1: f64, date2: f64, rmatn: &mut [f64; 9]) {
    unsafe {
        eraNutm80(date1, date2, rmatn);
    }
}

/// Mean obliquity of the ecliptic, IAU 2006 precession model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/obl06.c)
pub fn Obl06(date1: f64, date2: f64) -> f64 {
    return unsafe { eraObl06(date1, date2) };
}

/// Mean obliquity of the ecliptic, IAU 1980 model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/obl80.c)
pub fn Obl80(date1: f64, date2: f64) -> f64 {
    return unsafe { eraObl80(date1, date2) };
}

/// Precession angles, IAU 2006, equinox based.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/p06e.c)
pub fn P06e(date1: f64, date2: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
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

    return (eps0, psia, oma, bpa, bqa, pia, bpia, epsa, chia, za, zetaa, thetaa, pa, gam, phi, psi);
}

/// This function forms three Euler angles which implement general precession
/// from epoch J2000.0, using the IAU 2006 model. Frame bias (the offset between
/// ICRS and mean J2000.0) is included.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/pb06.c)
pub fn Pb06(date1: f64, date2: f64) -> (f64, f64, f64) {
    let mut bzeta: f64 = 0.0;
    let mut bz: f64 = 0.0;
    let mut btheta: f64 = 0.0;

    unsafe {
        eraPb06(date1, date2, &mut bzeta, &mut bz, &mut btheta);
    }

    return (bzeta, bz, btheta);
}

/// Precession angles, IAU 2006 (Fukushima-Williams 4-angle formulation).
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/pfw06.c)
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

/// Precession matrix (including frame bias) from GCRS to a specified date, IAU
/// 2000 model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/pmat00.c)
pub fn Pmat00(date1: f64, date2: f64, rbp: &mut [f64; 9]) {
    unsafe {
        eraPmat00(date1, date2, rbp);
    }
}

/// Precession matrix (including frame bias) from GCRS to a specified date, IAU
/// 2006 model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/pmat06.c)
pub fn Pmat06(date1: f64, date2: f64, rbp: &mut [f64; 9]) {
    unsafe {
        eraPmat06(date1, date2, rbp);
    }
}

/// Precession matrix from J2000.0 to a specified date, IAU 1976 model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/pmat76.c)
pub fn Pmat76(date1: f64, date2: f64, rmatp: &mut [f64; 9]) {
    unsafe {
        eraPmat76(date1, date2, rmatp);
    }
}

/// Precession-nutation, IAU 2000 model: a multi-purpose function, supporting
/// classical (equinox-based) use directly and CIO-based use indirectly.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/pn00.c)
pub fn Pn00(date1: f64, date2: f64, dpsi: f64, deps: f64, epsa: &mut f64, rb: &mut [f64; 9], rp: &mut [f64; 9], rbp: &mut [f64; 9], rn: &mut [f64; 9], rbpn: &mut [f64; 9]) {
    unsafe {
        eraPn00(date1, date2, dpsi, deps, epsa, rb, rp, rbp, rn, rbpn);
    }
}

/// Precession-nutation, IAU 2000A model: a multi-purpose function, supporting
/// classical (equinox-based) use directly and CIO-based use indirectly.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/pn00a.c)
pub fn Pn00a(date1: f64, date2: f64, dpsi: &mut f64, deps: &mut f64, epsa: &mut f64, rb: &mut [f64; 9], rp: &mut [f64; 9], rbp: &mut [f64; 9], rn: &mut [f64; 9], rbpn: &mut [f64; 9]) {
    unsafe {
        eraPn00a(date1, date2, dpsi, deps, epsa, rb, rp, rbp, rn, rbpn);
    }
}

/// Precession-nutation, IAU 2000B model: a multi-purpose function, supporting
/// classical (equinox-based) use directly and CIO-based use indirectly.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/pn00b.c)
pub fn Pn00b(date1: f64, date2: f64, dpsi: &mut f64, deps: &mut f64, epsa: &mut f64, rb: &mut [f64; 9], rp: &mut [f64; 9], rbp: &mut [f64; 9], rn: &mut [f64; 9], rbpn: &mut [f64; 9]) {
    unsafe {
        eraPn00b(date1, date2, dpsi, deps, epsa, rb, rp, rbp, rn, rbpn);
    }
}

/// Precession-nutation, IAU 2006 model: a multi-purpose function, supporting
/// classical (equinox-based) use directly and CIO-based use indirectly.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/pn06.c)
pub fn Pn06(date1: f64, date2: f64, dpsi: f64, deps: f64, epsa: &mut f64, rb: &mut [f64; 9], rp: &mut [f64; 9], rbp: &mut [f64; 9], rn: &mut [f64; 9], rbpn: &mut [f64; 9]) {
    unsafe {
        eraPn06(date1, date2, dpsi, deps, epsa, rb, rp, rbp, rn, rbpn);
    }
}

/// Precession-nutation, IAU 2006/2000A models: a multi-purpose function,
/// supporting classical (equinox-based) use directly and CIO-based use
/// indirectly.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/pn06a.c)
pub fn Pn06a(date1: f64, date2: f64, dpsi: &mut f64, deps: &mut f64, epsa: &mut f64, rb: &mut [f64; 9], rp: &mut [f64; 9], rbp: &mut [f64; 9], rn: &mut [f64; 9], rbpn: &mut [f64; 9]) {
    unsafe {
        eraPn06a(date1, date2, dpsi, deps, epsa, rb, rp, rbp, rn, rbpn);
    }
}

/// Form the matrix of precession-nutation for a given date (including frame
/// bias), equinox based, IAU 2000A model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/pnm00a.c)
pub fn Pnm00a(date1: f64, date2: f64, rbpn: &mut [f64; 9]) {
    unsafe {
        eraPnm00a(date1, date2, rbpn);
    }
}

/// Form the matrix of precession-nutation for a given date (including frame
/// bias), equinox-based, IAU 2000B model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/pnm00b.c)
pub fn Pnm00b(date1: f64, date2: f64, rbpn: &mut [f64; 9]) {
    unsafe {
        eraPnm00b(date1, date2, rbpn);
    }
}

/// Form the matrix of precession-nutation for a given date (including frame
/// bias), equinox based, IAU 2006 precession and IAU 2000A nutation models.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/pnm06a.c)
pub fn Pnm06a(date1: f64, date2: f64, rbpn: &mut [f64; 9]) {
    unsafe {
        eraPnm06a(date1, date2, rbpn);
    }
}

/// Form the matrix of precession/nutation for a given date, IAU 1976 precession
/// model, IAU 1980 nutation model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/pnm80.c)
pub fn Pnm80(date1: f64, date2: f64, rmatpn: &mut [f64; 9]) {
    unsafe {
        eraPnm80(date1, date2, rmatpn);
    }
}

/// Form the matrix of polar motion for a given date, IAU 2000.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/pom00.c)
pub fn Pom00(xp: f64, yp: f64, sp: f64, rpom: &mut [f64; 9]) {
    unsafe {
        eraPom00(xp, yp, sp, rpom);
    }
}

/// Precession-rate part of the IAU 2000 precession-nutation models (part of
/// MHB2000).
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/pr00.c)
pub fn Pr00(date1: f64, date2: f64) -> (f64, f64) {
    let mut dpsipr: f64 = 0.0;
    let mut depspr: f64 = 0.0;

    unsafe {
        eraPr00(date1, date2, &mut dpsipr, &mut depspr);
    }

    return (dpsipr, depspr);
}

/// IAU 1976 precession model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/prec76.c)
pub fn Prec76(date01: f64, date02: f64, date11: f64, date12: f64) -> (f64, f64, f64) {
    let mut zeta: f64 = 0.0;
    let mut z: f64 = 0.0;
    let mut theta: f64 = 0.0;

    unsafe {
        eraPrec76(date01, date02, date11, date12, &mut zeta, &mut z, &mut theta);
    }

    return (zeta, z, theta);
}

/// The CIO locator s, positioning the Celestial Intermediate Origin on the
/// equator of the Celestial Intermediate Pole, given the CIP's X,Y coordinates.
/// Compatible with IAU 2000A precession-nutation.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/s00.c)
pub fn S00(date1: f64, date2: f64, x: f64, y: f64) -> f64 {
    return unsafe { eraS00(date1, date2, x, y) };
}

/// The CIO locator s, positioning the Celestial Intermediate Origin on the
/// equator of the Celestial Intermediate Pole, using the IAU 2000A precession-
/// nutation model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/s00a.c)
pub fn S00a(date1: f64, date2: f64) -> f64 {
    return unsafe { eraS00a(date1, date2) };
}

/// The CIO locator s, positioning the Celestial Intermediate Origin on the
/// equator of the Celestial Intermediate Pole, using the IAU 2000B precession-
/// nutation model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/s00b.c)
pub fn S00b(date1: f64, date2: f64) -> f64 {
    return unsafe { eraS00b(date1, date2) };
}

/// The CIO locator s, positioning the Celestial Intermediate Origin on the
/// equator of the Celestial Intermediate Pole, given the CIP's X,Y coordinates.
/// Compatible with IAU 2006/2000A precession-nutation.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/s06.c)
pub fn S06(date1: f64, date2: f64, x: f64, y: f64) -> f64 {
    return unsafe { eraS06(date1, date2, x, y) };
}

/// The CIO locator s, positioning the Celestial Intermediate Origin on the
/// equator of the Celestial Intermediate Pole, using the IAU 2006 precession
/// and IAU 2000A nutation models.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/s06a.c)
pub fn S06a(date1: f64, date2: f64) -> f64 {
    return unsafe { eraS06a(date1, date2) };
}

/// The TIO locator s', positioning the Terrestrial Intermediate Origin on the
/// equator of the Celestial Intermediate Pole.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/sp00.c)
pub fn Sp00(date1: f64, date2: f64) -> f64 {
    return unsafe { eraSp00(date1, date2) };
}

/// X,Y coordinates of celestial intermediate pole from series based on IAU 2006
/// precession and IAU 2000A nutation.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/xy06.c)
pub fn Xy06(date1: f64, date2: f64) -> (f64, f64) {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;

    unsafe {
        eraXy06(date1, date2, &mut x, &mut y);
    }

    return (x, y);
}

/// For a given TT date, compute the X,Y coordinates of the Celestial
/// Intermediate Pole and the CIO locator s, using the IAU 2000A precession-
/// nutation model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/xys00a.c)
pub fn Xys00a(date1: f64, date2: f64) -> (f64, f64, f64) {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut s: f64 = 0.0;

    unsafe {
        eraXys00a(date1, date2, &mut x, &mut y, &mut s);
    }

    return (x, y, s);
}

/// For a given TT date, compute the X,Y coordinates of the Celestial
/// Intermediate Pole and the CIO locator s, using the IAU 2000B precession-
/// nutation model.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/xys00b.c)
pub fn Xys00b(date1: f64, date2: f64) -> (f64, f64, f64) {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut s: f64 = 0.0;

    unsafe {
        eraXys00b(date1, date2, &mut x, &mut y, &mut s);
    }

    return (x, y, s);
}

/// For a given TT date, compute the X,Y coordinates of the Celestial
/// Intermediate Pole and the CIO locator s, using the IAU 2006 precession and
/// IAU 2000A nutation models.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/xys06a.c)
pub fn Xys06a(date1: f64, date2: f64) -> (f64, f64, f64) {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut s: f64 = 0.0;

    unsafe {
        eraXys06a(date1, date2, &mut x, &mut y, &mut s);
    }

    return (x, y, s);
}
