//! Raw bindings to the ERFA C library
#![allow(missing_docs)]

pub mod astrometry {
    use std::ffi::{c_char, c_double, c_int};

    use crate::{Astrom, LDBody};

    unsafe extern "C" {
        pub unsafe fn eraAb(pnat: *const [c_double; 3], v: *const [c_double; 3], s: c_double, bm1: c_double, ppr: *mut [c_double; 3]);
        pub unsafe fn eraApcg(date1: c_double, date2: c_double, ebpv: *const [c_double; 6], ehp: *const [c_double; 3], astrom: *mut Astrom);
        pub unsafe fn eraApcg13(date1: c_double, date2: c_double, astrom: *mut Astrom);
        pub unsafe fn eraApci(date1: c_double, date2: c_double, ebpv: *const [c_double; 6], ehp: *const [c_double; 3], x: c_double, y: c_double, s: c_double, astrom: *mut Astrom);
        pub unsafe fn eraApci13(date1: c_double, date2: c_double, astrom: *mut Astrom, eo: *mut c_double);
        pub unsafe fn eraApco(date1: c_double, date2: c_double, ebpv: *const [c_double; 6], ehp: *const [c_double; 3], x: c_double, y: c_double, s: c_double, theta: c_double, elong: c_double, phi: c_double, hm: c_double, xp: c_double, yp: c_double, sp: c_double, refa: c_double, refb: c_double, astrom: *mut Astrom);
        pub unsafe fn eraApco13(utc1: c_double, utc2: c_double, dut1: c_double, elong: c_double, phi: c_double, hm: c_double, xp: c_double, yp: c_double, phpa: c_double, tc: c_double, rh: c_double, w1: c_double, astrom: *mut Astrom, eo: *mut c_double) -> c_int;
        pub unsafe fn eraApcs(date1: c_double, date2: c_double, pv: *const [c_double; 6], ebpv: *const [c_double; 6], ehp: *const [c_double; 3], astrom: *mut Astrom);
        pub unsafe fn eraApcs13(date1: c_double, date2: c_double, pv: *const [c_double; 6], astrom: *mut Astrom);
        pub unsafe fn eraAper(theta: c_double, astrom: *mut Astrom);
        pub unsafe fn eraAper13(ut11: c_double, ut12: c_double, astrom: *mut Astrom);
        pub unsafe fn eraApio(sp: c_double, theta: c_double, elong: c_double, phi: c_double, hm: c_double, xp: c_double, yp: c_double, refa: c_double, refb: c_double, astrom: *mut Astrom);
        pub unsafe fn eraApio13(utc1: c_double, utc2: c_double, dut1: c_double, elong: c_double, phi: c_double, hm: c_double, xp: c_double, yp: c_double, phpa: c_double, tc: c_double, rh: c_double, w1: c_double, astrom: *mut Astrom) -> c_int;
        pub unsafe fn eraAtcc13(rc: c_double, dc: c_double, pr: c_double, pd: c_double, px: c_double, rv: c_double, date1: c_double, date2: c_double, ra: *mut c_double, da: *mut c_double);
        pub unsafe fn eraAtccq(rc: c_double, dc: c_double, pr: c_double, pd: c_double, px: c_double, rv: c_double, astrom: *const Astrom, ra: *mut c_double, da: *mut c_double);
        pub unsafe fn eraAtci13(rc: c_double, dc: c_double, pr: c_double, pd: c_double, px: c_double, rv: c_double, date1: c_double, date2: c_double, ri: *mut c_double, di: *mut c_double, eo: *mut c_double);
        pub unsafe fn eraAtciq(rc: c_double, dc: c_double, pr: c_double, pd: c_double, px: c_double, rv: c_double, astrom: *const Astrom, ri: *mut c_double, di: *mut c_double);
        pub unsafe fn eraAtciqn(rc: c_double, dc: c_double, pr: c_double, pd: c_double, px: c_double, rv: c_double, astrom: *const Astrom, n: c_int, b: *const LDBody, ri: *mut c_double, di: *mut c_double);
        pub unsafe fn eraAtciqz(rc: c_double, dc: c_double, astrom: *const Astrom, ri: *mut c_double, di: *mut c_double);
        pub unsafe fn eraAtco13(rc: c_double, dc: c_double, pr: c_double, pd: c_double, px: c_double, rv: c_double, utc1: c_double, utc2: c_double, dut1: c_double, elong: c_double, phi: c_double, hm: c_double, xp: c_double, yp: c_double, phpa: c_double, tc: c_double, rh: c_double, w1: c_double, aob: *mut c_double, zob: *mut c_double, hob: *mut c_double, dob: *mut c_double, rob: *mut c_double, eo: *mut c_double) -> c_int;
        pub unsafe fn eraAtic13(ri: c_double, di: c_double, date1: c_double, date2: c_double, rc: *mut c_double, dc: *mut c_double, eo: *mut c_double);
        pub unsafe fn eraAticq(ri: c_double, di: c_double, astrom: *const Astrom, rc: *mut c_double, dc: *mut c_double);
        pub unsafe fn eraAticqn(ri: c_double, di: c_double, astrom: *const Astrom, n: c_int, b: *const LDBody, rc: *mut c_double, dc: *mut c_double);
        pub unsafe fn eraAtio13(ri: c_double, di: c_double, utc1: c_double, utc2: c_double, dut1: c_double, elong: c_double, phi: c_double, hm: c_double, xp: c_double, yp: c_double, phpa: c_double, tc: c_double, rh: c_double, w1: c_double, aob: *mut c_double, zob: *mut c_double, hob: *mut c_double, dob: *mut c_double, rob: *mut c_double) -> c_int;
        pub unsafe fn eraAtioq(ri: c_double, di: c_double, astrom: *const Astrom, aob: *mut c_double, zob: *mut c_double, hob: *mut c_double, dob: *mut c_double, rob: *mut c_double);
        pub unsafe fn eraAtoc13(ctype: *const c_char, ob1: c_double, ob2: c_double, utc1: c_double, utc2: c_double, dut1: c_double, elong: c_double, phi: c_double, hm: c_double, xp: c_double, yp: c_double, phpa: c_double, tc: c_double, rh: c_double, w1: c_double, rc: *mut c_double, dc: *mut c_double) -> c_int;
        pub unsafe fn eraAtoi13(ctype: *const c_char, ob1: c_double, ob2: c_double, utc1: c_double, utc2: c_double, dut1: c_double, elong: c_double, phi: c_double, hm: c_double, xp: c_double, yp: c_double, phpa: c_double, tc: c_double, rh: c_double, w1: c_double, rc: *mut c_double, dc: *mut c_double) -> c_int;
        pub unsafe fn eraAtoiq(ctype: *const c_char, ob1: c_double, ob2: c_double, astrom: *const Astrom, ri: *mut c_double, di: *mut c_double);
        pub unsafe fn eraLd(bm: c_double, p: *const [c_double; 3], q: *const [c_double; 3], e: *const [c_double; 3], em: c_double, dlim: c_double, p1: *mut [c_double; 3]);
        pub unsafe fn eraLdn(n: c_int, b: *const LDBody, ob: *const [c_double; 3], sc: *const [c_double; 3], sn: *mut [c_double; 3]);
        pub unsafe fn eraLdsun(p: *const [c_double; 3], e: *const [c_double; 3], em: c_double, p1: *mut [c_double; 3]);
        pub unsafe fn eraPmpx(rc: c_double, dc: c_double, pr: c_double, pd: c_double, px: c_double, rv: c_double, pmt: c_double, pob: *const [c_double; 3], pco: *mut [c_double; 3]);
        pub unsafe fn eraPmsafe(ra1: c_double, dec1: c_double, pmr1: c_double, pmd1: c_double, px1: c_double, rv1: c_double, ep1a: c_double, ep1b: c_double, ep2a: c_double, ep2b: c_double, ra2: *mut c_double, dec2: *mut c_double, pmr2: *mut c_double, pmd2: *mut c_double, px2: *mut c_double, rv2: *mut c_double) -> c_int;
        pub unsafe fn eraPvtob(elong: c_double, phi: c_double, hm: c_double, xp: c_double, yp: c_double, sp: c_double, theta: c_double, pv: *mut [c_double; 6]);
        pub unsafe fn eraRefco(phpa: c_double, tc: c_double, rh: c_double, w1: c_double, refa: *mut c_double, refb: *mut c_double);
    }
}

pub mod calendar {
    use std::ffi::{c_double, c_int};

    unsafe extern "C" {
        pub unsafe fn eraCal2jd(iy: c_int, im: c_int, id: c_int, djm0: *mut c_double, djm: *mut c_double) -> c_int;
        pub unsafe fn eraEpb(dj1: c_double, dj2: c_double) -> c_double;
        pub unsafe fn eraEpb2jd(epb: c_double, djm0: *mut c_double, djm: *mut c_double);
        pub unsafe fn eraEpj(dj1: c_double, dj2: c_double) -> c_double;
        pub unsafe fn eraEpj2jd(epj: c_double, djm0: *mut c_double, djm: *mut c_double);
        pub unsafe fn eraJd2cal(dj1: c_double, dj2: c_double, iy: *mut c_int, im: *mut c_int, id: *mut c_int, fd: *mut c_double) -> c_int;
        pub unsafe fn eraJdcalf(ndp: c_int, dj1: c_double, dj2: c_double, iymdf: *mut [c_int; 4]) -> c_int;
    }
}

pub mod eclipticcoordinates {
    use std::ffi::c_double;

    unsafe extern "C" {
        pub unsafe fn eraEceq06(date1: c_double, date2: c_double, dl: c_double, db: c_double, dr: *mut c_double, dd: *mut c_double);
        pub unsafe fn eraEcm06(date1: c_double, date2: c_double, rm: *mut [c_double; 9]);
        pub unsafe fn eraEqec06(date1: c_double, date2: c_double, dr: c_double, dd: c_double, dl: *mut c_double, db: *mut c_double);
        pub unsafe fn eraLteceq(epj: c_double, dl: c_double, db: c_double, dr: *mut c_double, dd: *mut c_double);
        pub unsafe fn eraLtecm(epj: c_double, rm: *mut [c_double; 9]);
        pub unsafe fn eraLteqec(epj: c_double, dr: c_double, dd: c_double, dl: *mut c_double, db: *mut c_double);
    }
}

pub mod ephemerides {
    use std::ffi::{c_double, c_int};

    unsafe extern "C" {
        pub unsafe fn eraEpv00(date1: c_double, date2: c_double, pvh: *mut [c_double; 6], pvb: *mut [c_double; 6]) -> c_int;
        pub unsafe fn eraMoon98(date1: c_double, date2: c_double, pv: *mut [c_double; 6]);
        pub unsafe fn eraPlan94(date1: c_double, date2: c_double, np: c_int, pv: *mut [c_double; 6]) -> c_int;
    }
}

pub mod fundamentalargs {
    use std::ffi::c_double;

    unsafe extern "C" {
        pub unsafe fn eraFad03(t: c_double) -> c_double;
        pub unsafe fn eraFae03(t: c_double) -> c_double;
        pub unsafe fn eraFaf03(t: c_double) -> c_double;
        pub unsafe fn eraFaju03(t: c_double) -> c_double;
        pub unsafe fn eraFal03(t: c_double) -> c_double;
        pub unsafe fn eraFalp03(t: c_double) -> c_double;
        pub unsafe fn eraFama03(t: c_double) -> c_double;
        pub unsafe fn eraFame03(t: c_double) -> c_double;
        pub unsafe fn eraFane03(t: c_double) -> c_double;
        pub unsafe fn eraFaom03(t: c_double) -> c_double;
        pub unsafe fn eraFapa03(t: c_double) -> c_double;
        pub unsafe fn eraFasa03(t: c_double) -> c_double;
        pub unsafe fn eraFaur03(t: c_double) -> c_double;
        pub unsafe fn eraFave03(t: c_double) -> c_double;
    }
}

pub mod galacticcoordinates {
    use std::ffi::c_double;

    unsafe extern "C" {
        pub unsafe fn eraG2icrs(dl: c_double, db: c_double, dr: *mut c_double, dd: *mut c_double);
        pub unsafe fn eraIcrs2g(dr: c_double, dd: c_double, dl: *mut c_double, db: *mut c_double);
    }
}

pub mod geodeticgeocentric {
    use std::ffi::{c_double, c_int};

    unsafe extern "C" {
        pub unsafe fn eraEform(n: c_int, a: *mut c_double, f: *mut c_double) -> c_int;
        pub unsafe fn eraGc2gd(n: c_int, xyz: *const [c_double; 3], elong: *mut c_double, phi: *mut c_double, height: *mut c_double) -> c_int;
        pub unsafe fn eraGc2gde(a: c_double, f: c_double, xyz: *const [c_double; 3], elong: *mut c_double, phi: *mut c_double, height: *mut c_double) -> c_int;
        pub unsafe fn eraGd2gc(n: c_int, elong: c_double, phi: c_double, height: c_double, xyz: *mut [c_double; 3]) -> c_int;
        pub unsafe fn eraGd2gce(a: c_double, f: c_double, elong: c_double, phi: c_double, height: c_double, xyz: *mut [c_double; 3]) -> c_int;
    }
}

pub mod gnomonic {
    use std::ffi::{c_double, c_int};

    unsafe extern "C" {
        pub unsafe fn eraTpors(xi: c_double, eta: c_double, a: c_double, b: c_double, a01: *mut c_double, b01: *mut c_double, a02: *mut c_double, b02: *mut c_double) -> c_int;
        pub unsafe fn eraTporv(xi: c_double, eta: c_double, v: *const [c_double; 3], v01: *mut [c_double; 3], v02: *mut [c_double; 3]) -> c_int;
        pub unsafe fn eraTpsts(xi: c_double, eta: c_double, a0: c_double, b0: c_double, a: *mut c_double, b: *mut c_double);
        pub unsafe fn eraTpstv(xi: c_double, eta: c_double, v0: *const [c_double; 3], v: *mut [c_double; 3]);
        pub unsafe fn eraTpxes(a: c_double, b: c_double, a0: c_double, b0: c_double, xi: *mut c_double, eta: *mut c_double) -> c_int;
        pub unsafe fn eraTpxev(v: *const [c_double; 3], v0: *const [c_double; 3], xi: *mut c_double, eta: *mut c_double) -> c_int;
    }
}

pub mod horizonequatorial {
    use std::ffi::c_double;

    unsafe extern "C" {
        pub unsafe fn eraAe2hd(az: c_double, el: c_double, phi: c_double, ha: *mut c_double, dec: *mut c_double);
        pub unsafe fn eraHd2ae(ha: c_double, dec: c_double, phi: c_double, az: *mut c_double, el: *mut c_double);
        pub unsafe fn eraHd2pa(ha: c_double, dec: c_double, phi: c_double) -> c_double;
    }
}

pub mod precnutpolar {
    use std::ffi::c_double;

    unsafe extern "C" {
        pub unsafe fn eraBi00(dpsibi: *mut c_double, depsbi: *mut c_double, dra: *mut c_double);
        pub unsafe fn eraBp00(date1: c_double, date2: c_double, rb: *mut [c_double; 9], rp: *mut [c_double; 9], rbp: *mut [c_double; 9]);
        pub unsafe fn eraBp06(date1: c_double, date2: c_double, rb: *mut [c_double; 9], rp: *mut [c_double; 9], rbp: *mut [c_double; 9]);
        pub unsafe fn eraBpn2xy(rbpn: *const [c_double; 9], x: *mut c_double, y: *mut c_double);
        pub unsafe fn eraC2i00a(date1: c_double, date2: c_double, rc2i: *mut [c_double; 9]);
        pub unsafe fn eraC2i00b(date1: c_double, date2: c_double, rc2i: *mut [c_double; 9]);
        pub unsafe fn eraC2i06a(date1: c_double, date2: c_double, rc2i: *mut [c_double; 9]);
        pub unsafe fn eraC2ibpn(date1: c_double, date2: c_double, rbpn: *const [c_double; 9], rc2i: *mut [c_double; 9]);
        pub unsafe fn eraC2ixy(date1: c_double, date2: c_double, x: c_double, y: c_double, rc2i: *mut [c_double; 9]);
        pub unsafe fn eraC2ixys(x: c_double, y: c_double, s: c_double, rc2i: *mut [c_double; 9]);
        pub unsafe fn eraC2t00a(tta: c_double, ttb: c_double, uta: c_double, utb: c_double, xp: c_double, yp: c_double, rc2t: *mut [c_double; 9]);
        pub unsafe fn eraC2t00b(tta: c_double, ttb: c_double, uta: c_double, utb: c_double, xp: c_double, yp: c_double, rc2t: *mut [c_double; 9]);
        pub unsafe fn eraC2t06a(tta: c_double, ttb: c_double, uta: c_double, utb: c_double, xp: c_double, yp: c_double, rc2t: *mut [c_double; 9]);
        pub unsafe fn eraC2tcio(rc2i: *const [c_double; 9], era: c_double, rpom: *const [c_double; 9], rc2t: *mut [c_double; 9]);
        pub unsafe fn eraC2teqx(rbpn: *const [c_double; 9], gst: c_double, rpom: *const [c_double; 9], rc2t: *mut [c_double; 9]);
        pub unsafe fn eraC2tpe(tta: c_double, ttb: c_double, uta: c_double, utb: c_double, dpsi: c_double, deps: c_double, xp: c_double, yp: c_double, rc2t: *mut [c_double; 9]);
        pub unsafe fn eraC2txy(tta: c_double, ttb: c_double, uta: c_double, utb: c_double, x: c_double, y: c_double, xp: c_double, yp: c_double, rc2t: *mut [c_double; 9]);
        pub unsafe fn eraEo06a(date1: c_double, date2: c_double) -> c_double;
        pub unsafe fn eraEors(rnpb: *const [c_double; 9], s: c_double) -> c_double;
        pub unsafe fn eraFw2m(gamb: c_double, phib: c_double, psi: c_double, eps: c_double, r: *mut [c_double; 9]);
        pub unsafe fn eraFw2xy(gamb: c_double, phib: c_double, psi: c_double, eps: c_double, x: *mut c_double, y: *mut c_double);
        pub unsafe fn eraLtp(epj: c_double, rp: *mut [c_double; 9]);
        pub unsafe fn eraLtpb(epj: c_double, rpb: *mut [c_double; 9]);
        pub unsafe fn eraLtpecl(epj: c_double, vec: *mut [c_double; 3]);
        pub unsafe fn eraLtpequ(epj: c_double, veq: *mut [c_double; 3]);
        pub unsafe fn eraNum00a(date1: c_double, date2: c_double, rmatn: *mut [c_double; 9]);
        pub unsafe fn eraNum00b(date1: c_double, date2: c_double, rmatn: *mut [c_double; 9]);
        pub unsafe fn eraNum06a(date1: c_double, date2: c_double, rmatn: *mut [c_double; 9]);
        pub unsafe fn eraNumat(epsa: c_double, dpsi: c_double, deps: c_double, rmatn: *mut [c_double; 9]);
        pub unsafe fn eraNut00a(date1: c_double, date2: c_double, dpsi: *mut c_double, deps: *mut c_double);
        pub unsafe fn eraNut00b(date1: c_double, date2: c_double, dpsi: *mut c_double, deps: *mut c_double);
        pub unsafe fn eraNut06a(date1: c_double, date2: c_double, dpsi: *mut c_double, deps: *mut c_double);
        pub unsafe fn eraNut80(date1: c_double, date2: c_double, dpsi: *mut c_double, deps: *mut c_double);
        pub unsafe fn eraNutm80(date1: c_double, date2: c_double, rmatn: *mut [c_double; 9]);
        pub unsafe fn eraObl06(date1: c_double, date2: c_double) -> c_double;
        pub unsafe fn eraObl80(date1: c_double, date2: c_double) -> c_double;
        pub unsafe fn eraP06e(date1: c_double, date2: c_double, eps0: *mut c_double, psia: *mut c_double, oma: *mut c_double, bpa: *mut c_double, bqa: *mut c_double, pia: *mut c_double, bpia: *mut c_double, epsa: *mut c_double, chia: *mut c_double, za: *mut c_double, zetaa: *mut c_double, thetaa: *mut c_double, pa: *mut c_double, gam: *mut c_double, phi: *mut c_double, psi: *mut c_double);
        pub unsafe fn eraPb06(date1: c_double, date2: c_double, bzeta: *mut c_double, bz: *mut c_double, btheta: *mut c_double);
        pub unsafe fn eraPfw06(date1: c_double, date2: c_double, gamb: *mut c_double, phib: *mut c_double, psib: *mut c_double, epsa: *mut c_double);
        pub unsafe fn eraPmat00(date1: c_double, date2: c_double, rbp: *mut [c_double; 9]);
        pub unsafe fn eraPmat06(date1: c_double, date2: c_double, rbp: *mut [c_double; 9]);
        pub unsafe fn eraPmat76(date1: c_double, date2: c_double, rmatp: *mut [c_double; 9]);
        pub unsafe fn eraPn00(date1: c_double, date2: c_double, dpsi: c_double, deps: c_double, epsa: *mut c_double, rb: *mut [c_double; 9], rp: *mut [c_double; 9], rbp: *mut [c_double; 9], rn: *mut [c_double; 9], rbpn: *mut [c_double; 9]);
        pub unsafe fn eraPn00a(date1: c_double, date2: c_double, dpsi: *mut c_double, deps: *mut c_double, epsa: *mut c_double, rb: *mut [c_double; 9], rp: *mut [c_double; 9], rbp: *mut [c_double; 9], rn: *mut [c_double; 9], rbpn: *mut [c_double; 9]);
        pub unsafe fn eraPn00b(date1: c_double, date2: c_double, dpsi: *mut c_double, deps: *mut c_double, epsa: *mut c_double, rb: *mut [c_double; 9], rp: *mut [c_double; 9], rbp: *mut [c_double; 9], rn: *mut [c_double; 9], rbpn: *mut [c_double; 9]);
        pub unsafe fn eraPn06(date1: c_double, date2: c_double, dpsi: c_double, deps: c_double, epsa: *mut c_double, rb: *mut [c_double; 9], rp: *mut [c_double; 9], rbp: *mut [c_double; 9], rn: *mut [c_double; 9], rbpn: *mut [c_double; 9]);
        pub unsafe fn eraPn06a(date1: c_double, date2: c_double, dpsi: *mut c_double, deps: *mut c_double, epsa: *mut c_double, rb: *mut [c_double; 9], rp: *mut [c_double; 9], rbp: *mut [c_double; 9], rn: *mut [c_double; 9], rbpn: *mut [c_double; 9]);
        pub unsafe fn eraPnm00a(date1: c_double, date2: c_double, rbpn: *mut [c_double; 9]);
        pub unsafe fn eraPnm00b(date1: c_double, date2: c_double, rbpn: *mut [c_double; 9]);
        pub unsafe fn eraPnm06a(date1: c_double, date2: c_double, rnpb: *mut [c_double; 9]);
        pub unsafe fn eraPnm80(date1: c_double, date2: c_double, rmatpn: *mut [c_double; 9]);
        pub unsafe fn eraPom00(xp: c_double, yp: c_double, sp: c_double, rpom: *mut [c_double; 9]);
        pub unsafe fn eraPr00(date1: c_double, date2: c_double, dpsipr: *mut c_double, depspr: *mut c_double);
        pub unsafe fn eraPrec76(date01: c_double, date02: c_double, date11: c_double, date12: c_double, zeta: *mut c_double, z: *mut c_double, theta: *mut c_double);
        pub unsafe fn eraS00(date1: c_double, date2: c_double, x: c_double, y: c_double) -> c_double;
        pub unsafe fn eraS00a(date1: c_double, date2: c_double) -> c_double;
        pub unsafe fn eraS00b(date1: c_double, date2: c_double) -> c_double;
        pub unsafe fn eraS06(date1: c_double, date2: c_double, x: c_double, y: c_double) -> c_double;
        pub unsafe fn eraS06a(date1: c_double, date2: c_double) -> c_double;
        pub unsafe fn eraSp00(date1: c_double, date2: c_double) -> c_double;
        pub unsafe fn eraXy06(date1: c_double, date2: c_double, x: *mut c_double, y: *mut c_double);
        pub unsafe fn eraXys00a(date1: c_double, date2: c_double, x: *mut c_double, y: *mut c_double, s: *mut c_double);
        pub unsafe fn eraXys00b(date1: c_double, date2: c_double, x: *mut c_double, y: *mut c_double, s: *mut c_double);
        pub unsafe fn eraXys06a(date1: c_double, date2: c_double, x: *mut c_double, y: *mut c_double, s: *mut c_double);
    }
}

pub mod rotationtime {
    use std::ffi::c_double;

    unsafe extern "C" {
        pub unsafe fn eraEe00(date1: c_double, date2: c_double, epsa: c_double, dpsi: c_double) -> c_double;
        pub unsafe fn eraEe00a(date1: c_double, date2: c_double) -> c_double;
        pub unsafe fn eraEe00b(date1: c_double, date2: c_double) -> c_double;
        pub unsafe fn eraEe06a(date1: c_double, date2: c_double) -> c_double;
        pub unsafe fn eraEect00(date1: c_double, date2: c_double) -> c_double;
        pub unsafe fn eraEqeq94(date1: c_double, date2: c_double) -> c_double;
        pub unsafe fn eraEra00(dj1: c_double, dj2: c_double) -> c_double;
        pub unsafe fn eraGmst00(uta: c_double, utb: c_double, tta: c_double, ttb: c_double) -> c_double;
        pub unsafe fn eraGmst06(uta: c_double, utb: c_double, tta: c_double, ttb: c_double) -> c_double;
        pub unsafe fn eraGmst82(dj1: c_double, dj2: c_double) -> c_double;
        pub unsafe fn eraGst00a(uta: c_double, utb: c_double, tta: c_double, ttb: c_double) -> c_double;
        pub unsafe fn eraGst00b(uta: c_double, utb: c_double) -> c_double;
        pub unsafe fn eraGst06(uta: c_double, utb: c_double, tta: c_double, ttb: c_double, rnpb: *const [c_double; 9]) -> c_double;
        pub unsafe fn eraGst06a(uta: c_double, utb: c_double, tta: c_double, ttb: c_double) -> c_double;
        pub unsafe fn eraGst94(uta: c_double, utb: c_double) -> c_double;
    }
}

pub mod spacemotion {
    use std::ffi::{c_double, c_int};

    unsafe extern "C" {
        pub unsafe fn eraPvstar(pv: *const [c_double; 6], ra: *mut c_double, dec: *mut c_double, pmr: *mut c_double, pmd: *mut c_double, px: *mut c_double, rv: *mut c_double) -> c_int;
        pub unsafe fn eraStarpv(ra: c_double, dec: c_double, pmr: c_double, pmd: c_double, px: c_double, rv: c_double, pv: *mut [c_double; 6]) -> c_int;
    }
}

pub mod starcatalogs {
    use std::ffi::{c_double, c_int};

    unsafe extern "C" {
        pub unsafe fn eraFk425(r1950: c_double, d1950: c_double, dr1950: c_double, dd1950: c_double, p1950: c_double, v1950: c_double, r2000: *mut c_double, d2000: *mut c_double, dr2000: *mut c_double, dd2000: *mut c_double, p2000: *mut c_double, v2000: *mut c_double);
        pub unsafe fn eraFk45z(r1950: c_double, d1950: c_double, bepoch: c_double, r2000: *mut c_double, d2000: *mut c_double);
        pub unsafe fn eraFk524(r2000: c_double, d2000: c_double, dr2000: c_double, dd2000: c_double, p2000: c_double, v2000: c_double, r1950: *mut c_double, d1950: *mut c_double, dr1950: *mut c_double, dd1950: *mut c_double, p1950: *mut c_double, v1950: *mut c_double);
        pub unsafe fn eraFk52h(r5: c_double, d5: c_double, dr5: c_double, dd5: c_double, px5: c_double, rv5: c_double, rh: *mut c_double, dh: *mut c_double, drh: *mut c_double, ddh: *mut c_double, pxh: *mut c_double, rvh: *mut c_double);
        pub unsafe fn eraFk54z(r2000: c_double, d2000: c_double, bepoch: c_double, r1950: *mut c_double, d1950: *mut c_double, dr1950: *mut c_double, dd1950: *mut c_double);
        pub unsafe fn eraFk5hip(r5h: *mut [c_double; 9], s5h: *mut [c_double; 3]);
        pub unsafe fn eraFk5hz(r5: c_double, d5: c_double, date1: c_double, date2: c_double, rh: *mut c_double, dh: *mut c_double);
        pub unsafe fn eraH2fk5(rh: c_double, dh: c_double, drh: c_double, ddh: c_double, pxh: c_double, rvh: c_double, r5: *mut c_double, d5: *mut c_double, dr5: *mut c_double, dd5: *mut c_double, px5: *mut c_double, rv5: *mut c_double);
        pub unsafe fn eraHfk5z(rh: c_double, dh: c_double, date1: c_double, date2: c_double, r5: *mut c_double, d5: *mut c_double, dr5: *mut c_double, dd5: *mut c_double);
        pub unsafe fn eraStarpm(ra1: c_double, dec1: c_double, pmr1: c_double, pmd1: c_double, px1: c_double, rv1: c_double, ep1a: c_double, ep1b: c_double, ep2a: c_double, ep2b: c_double, ra2: *mut c_double, dec2: *mut c_double, pmr2: *mut c_double, pmd2: *mut c_double, px2: *mut c_double, rv2: *mut c_double) -> c_int;
    }
}

pub mod timescales {
    use std::ffi::{c_char, c_double, c_int};

    unsafe extern "C" {
        pub unsafe fn eraD2dtf(scale: *const c_char, ndp: c_int, d1: c_double, d2: c_double, iy: *mut c_int, im: *mut c_int, id: *mut c_int, ihmsf: *mut [c_int; 4]) -> c_int;
        pub unsafe fn eraDat(iy: c_int, im: c_int, id: c_int, fd: c_double, deltat: *mut c_double) -> c_int;
        pub unsafe fn eraDtdb(date1: c_double, date2: c_double, ut: c_double, elong: c_double, u: c_double, v: c_double) -> c_double;
        pub unsafe fn eraDtf2d(scale: *const c_char, iy: c_int, im: c_int, id: c_int, ihr: c_int, imn: c_int, sec: c_double, d1: *mut c_double, d2: *mut c_double) -> c_int;
        pub unsafe fn eraTaitt(tai1: c_double, tai2: c_double, tt1: *mut c_double, tt2: *mut c_double) -> c_int;
        pub unsafe fn eraTaiut1(tai1: c_double, tai2: c_double, dta: c_double, ut11: *mut c_double, ut12: *mut c_double) -> c_int;
        pub unsafe fn eraTaiutc(tai1: c_double, tai2: c_double, utc1: *mut c_double, utc2: *mut c_double) -> c_int;
        pub unsafe fn eraTcbtdb(tcb1: c_double, tcb2: c_double, tdb1: *mut c_double, tdb2: *mut c_double) -> c_int;
        pub unsafe fn eraTcgtt(tcg1: c_double, tcg2: c_double, tt1: *mut c_double, tt2: *mut c_double) -> c_int;
        pub unsafe fn eraTdbtcb(tdb1: c_double, tdb2: c_double, tcb1: *mut c_double, tcb2: *mut c_double) -> c_int;
        pub unsafe fn eraTdbtt(tdb1: c_double, tdb2: c_double, dtr: c_double, tt1: *mut c_double, tt2: *mut c_double) -> c_int;
        pub unsafe fn eraTttai(tt1: c_double, tt2: c_double, tai1: *mut c_double, tai2: *mut c_double) -> c_int;
        pub unsafe fn eraTttcg(tt1: c_double, tt2: c_double, tcg1: *mut c_double, tcg2: *mut c_double) -> c_int;
        pub unsafe fn eraTttdb(tt1: c_double, tt2: c_double, dtr: c_double, tdb1: *mut c_double, tdb2: *mut c_double) -> c_int;
        pub unsafe fn eraTtut1(tt1: c_double, tt2: c_double, dt: c_double, ut11: *mut c_double, ut12: *mut c_double) -> c_int;
        pub unsafe fn eraUt1tai(ut11: c_double, ut12: c_double, dta: c_double, tai1: *mut c_double, tai2: *mut c_double) -> c_int;
        pub unsafe fn eraUt1tt(ut11: c_double, ut12: c_double, dt: c_double, tt1: *mut c_double, tt2: *mut c_double) -> c_int;
        pub unsafe fn eraUt1utc(ut11: c_double, ut12: c_double, dut1: c_double, utc1: *mut c_double, utc2: *mut c_double) -> c_int;
        pub unsafe fn eraUtctai(utc1: c_double, utc2: c_double, tai1: *mut c_double, tai2: *mut c_double) -> c_int;
        pub unsafe fn eraUtcut1(utc1: c_double, utc2: c_double, dut1: c_double, ut11: *mut c_double, ut12: *mut c_double) -> c_int;
    }
}
