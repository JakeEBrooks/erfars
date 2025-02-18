//! ERFA Precession, Nutation, and Polar Motion Functions

use crate::raw::precnutpolar::*;

pub fn Bi00() -> (f64, f64, f64) {
    let mut dpsibi: f64 = 0.0;
    let mut depsbi: f64 = 0.0;
    let mut dra: f64 = 0.0;
    unsafe {
        eraBi00(&mut dpsibi, &mut depsbi, &mut dra);
    }

    return (dpsibi, depsbi, dra)
}

pub fn Bp00(date1: f64, date2: f64, rb: &mut [f64; 9], rp: &mut [f64; 9], rbp: &mut [f64; 9]) {
    unsafe {
        eraBp00(date1, date2, rb, rp, rbp);
    }
}

pub fn Bp06(date1: f64, date2: f64, rb: &mut [f64; 9], rp: &mut [f64; 9], rbp: &mut [f64; 9]) {
    unsafe {
        eraBp06(date1, date2, rb, rp, rbp);
    }
}

pub fn Bpn2xy(rbpn: &[f64; 9]) -> (f64, f64) {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    unsafe {
        eraBpn2xy(rbpn, &mut x, &mut y);
    }

    return (x, y)
}

pub fn C2i00a(date1: f64, date2: f64, rc2i: &mut [f64; 9]) {
    unsafe {
        eraC2i00a(date1, date2, rc2i);
    }
}

pub fn C2i00b(date1: f64, date2: f64, rc2i: &mut [f64; 9]) {
    unsafe {
        eraC2i00b(date1, date2, rc2i);
    }
}

pub fn C2i06a(date1: f64, date2: f64, rc2i: &mut [f64; 9]) {
    unsafe {
        eraC2i06a(date1, date2, rc2i);
    }
}

pub fn C2ibpn(date1: f64, date2: f64, rbpn: &[f64; 9], rc2i: &mut [f64; 9]) {
    unsafe {
        eraC2ibpn(date1, date2, rbpn, rc2i);
    }
}

pub fn C2ixy(date1: f64, date2: f64, x: f64, y: f64, rc2i: &mut [f64; 9]) {
    unsafe {
        eraC2ixy(date1, date2, x, y, rc2i);
    }
}

pub fn C2ixys(x: f64, y: f64, s: f64, rc2i: &mut [f64; 9]) {
    unsafe {
        eraC2ixys(x, y, s, rc2i);
    }
}

pub fn C2t00a(tta: f64, ttb: f64, uta: f64, utb: f64, xp: f64, yp: f64, rc2t: &mut [f64; 9]) {
    unsafe {
        eraC2t00a(tta, ttb, uta, utb, xp, yp, rc2t);
    }
}

pub fn C2t00b(tta: f64, ttb: f64, uta: f64, utb: f64, xp: f64, yp: f64, rc2t: &mut [f64; 9]) {
    unsafe {
        eraC2t00b(tta, ttb, uta, utb, xp, yp, rc2t);
    }
}

pub fn C2t06a(tta: f64, ttb: f64, uta: f64, utb: f64, xp: f64, yp: f64, rc2t: &mut [f64; 9]) {
    unsafe {
        eraC2t06a(tta, ttb, uta, utb, xp, yp, rc2t);
    }
}

pub fn C2tcio(rc2i: &[f64; 9], era: f64, rpom: &[f64; 9], rc2t: &mut [f64; 9]) {
    unsafe {
        eraC2tcio(rc2i, era, rpom, rc2t);
    }
}

pub fn C2teqx(rbpn: &[f64; 9], gst: f64, rpom: &[f64; 9], rc2t: &mut [f64; 9]) {
    unsafe {
        eraC2teqx(rbpn, gst, rpom, rc2t);
    }
}

pub fn C2tpe(tta: f64, ttb: f64, uta: f64, utb: f64, dpsi: f64, deps: f64, xp: f64, yp: f64, rc2t: &mut [f64; 9]) {
    unsafe {
        eraC2tpe(tta, ttb, uta, utb, dpsi, deps, xp, yp, rc2t);
    }
}

pub fn C2txy(tta: f64, ttb: f64, uta: f64, utb: f64, x: f64, y: f64, xp: f64, yp: f64, rc2t: &mut [f64; 9]) {
    unsafe {
        eraC2txy(tta, ttb, uta, utb, x, y, xp, yp, rc2t);
    }
}

pub fn Eo06a(date1: f64, date2: f64) -> f64 {
    return unsafe { eraEo06a(date1, date2) }
}

pub fn Eors(rnpb: &[f64; 9], s: f64) -> f64 {
    return unsafe { eraEors(rnpb, s) }
}

pub fn Fw2m(gamb: f64, phib: f64, psi: f64, eps: f64, r: &mut [f64; 9]) {
    unsafe {
        eraFw2m(gamb, phib, psi, eps, r);
    }
}

pub fn Fw2xy(gamb: f64, phib: f64, psi: f64, eps: f64) -> (f64, f64) {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    unsafe {
        eraFw2xy(gamb, phib, psi, eps, &mut x, &mut y);
    }

    return (x, y)
}

pub fn Ltp(epj: f64, rp: &mut [f64; 9]) {
    unsafe {
        eraLtp(epj, rp);
    }
}

pub fn Ltpb(epj: f64, rpb: &mut [f64; 9]) {
    unsafe {
        eraLtpb(epj, rpb);
    }
}

pub fn Ltpecl(epj: f64) -> [f64; 3] {
    let mut vec: [f64; 3] = [0.0; 3];
    unsafe {
        eraLtpecl(epj, &mut vec);
    }

    return vec
}

pub fn Ltpequ(epj: f64) -> [f64; 3] {
    let mut veq: [f64; 3] = [0.0; 3];
    unsafe {
        eraLtpequ(epj, &mut veq);
    }

    return veq
}

pub fn Num00a(date1: f64, date2: f64, rmatn: &mut [f64; 9]) {
    unsafe {
        eraNum00a(date1, date2, rmatn);
    }
}

pub fn Num00b(date1: f64, date2: f64, rmatn: &mut [f64; 9]) {
    unsafe {
        eraNum00b(date1, date2, rmatn);
    }
}

pub fn Num06a(date1: f64, date2: f64, rmatn: &mut [f64; 9]) {
    unsafe {
        eraNum06a(date1, date2, rmatn);
    }
}

pub fn Numat(epsa: f64, dpsi: f64, deps: f64, rmatn: &mut [f64; 9]) {
    unsafe {
        eraNumat(epsa, dpsi, deps, rmatn);
    }
}

pub fn Nut00a(date1: f64, date2: f64) -> (f64, f64) {
    let mut dpsi: f64 = 0.0;
    let mut deps: f64 = 0.0;
    unsafe {
        eraNut00a(date1, date2, &mut dpsi, &mut deps);
    }

    return (dpsi, deps)
}

pub fn Nut00b(date1: f64, date2: f64) -> (f64, f64) {
    let mut dpsi: f64 = 0.0;
    let mut deps: f64 = 0.0;
    unsafe {
        eraNut00b(date1, date2, &mut dpsi, &mut deps);
    }

    return (dpsi, deps)
}

pub fn Nut06a(date1: f64, date2: f64) -> (f64, f64) {
    let mut dpsi: f64 = 0.0;
    let mut deps: f64 = 0.0;
    unsafe {
        eraNut06a(date1, date2, &mut dpsi, &mut deps);
    }

    return (dpsi, deps)
}

pub fn Nut80(date1: f64, date2: f64) -> (f64, f64) {
    let mut dpsi: f64 = 0.0;
    let mut deps: f64 = 0.0;
    unsafe {
        eraNut80(date1, date2, &mut dpsi, &mut deps);
    }

    return (dpsi, deps)
}

pub fn Nutm80(date1: f64, date2: f64, rmatn: &mut [f64; 9]) {
    unsafe {
        eraNutm80(date1, date2, rmatn);
    }
}

pub fn Obl06(date1: f64, date2: f64) -> f64 {
    return unsafe { eraObl06(date1, date2) }
}

pub fn Obl80(date1: f64, date2: f64) -> f64 {
    return unsafe { eraObl80(date1, date2) }
}

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
        eraP06e(date1, date2, &mut eps0, &mut psia, &mut oma, &mut bpa, &mut bqa, &mut pia, &mut bpia, &mut epsa, &mut chia, &mut za, &mut zetaa, &mut thetaa, &mut pa, &mut gam, &mut phi, &mut psi);
    }

    return (eps0, psia, oma, bpa, bqa, pia, bpia, epsa, chia, za, zetaa, thetaa, pa, gam, phi, psi)
}

pub fn Pb06(date1: f64, date2: f64) -> (f64, f64, f64) {
    let mut bzeta: f64 = 0.0;
    let mut bz: f64 = 0.0;
    let mut btheta: f64 = 0.0;

    unsafe {
        eraPb06(date1, date2, &mut bzeta, &mut bz, &mut btheta);
    }

    return (bzeta, bz, btheta)
}

pub fn Pfw06(date1: f64, date2: f64) -> (f64, f64, f64, f64) {
    let mut gamb: f64 = 0.0;
    let mut phib: f64 = 0.0;
    let mut psib: f64 = 0.0;
    let mut epsa: f64 = 0.0;

    unsafe {
        eraPfw06(date1, date2, &mut gamb, &mut phib, &mut psib, &mut epsa);
    }

    return (gamb, phib, psib, epsa)
}

pub fn Pmat00(date1: f64, date2: f64, rbp: &mut [f64; 9]) {
    unsafe {
        eraPmat00(date1, date2, rbp);
    }
}

pub fn Pmat06(date1: f64, date2: f64, rbp: &mut [f64; 9]) {
    unsafe {
        eraPmat06(date1, date2, rbp);
    }
}

pub fn Pmat76(date1: f64, date2: f64, rmatp: &mut [f64; 9]) {
    unsafe {
        eraPmat76(date1, date2, rmatp);
    }
}

pub fn Pn00(date1: f64, date2: f64, dpsi: f64, deps: f64, epsa: &mut f64, rb: &mut [f64; 9], rp: &mut [f64; 9], rbp: &mut [f64; 9], rn: &mut [f64; 9], rbpn: &mut [f64; 9]) {
    unsafe {
        eraPn00(date1, date2, dpsi, deps, epsa, rb, rp, rbp, rn, rbpn);
    }
}

pub fn Pn00a(date1: f64, date2: f64, dpsi: &mut f64, deps: &mut f64, epsa: &mut f64, rb: &mut [f64; 9], rp: &mut [f64; 9], rbp: &mut [f64; 9], rn: &mut [f64; 9], rbpn: &mut [f64; 9]) {
    unsafe {
        eraPn00a(date1, date2, dpsi, deps, epsa, rb, rp, rbp, rn, rbpn);
    }
}

pub fn Pn00b(date1: f64, date2: f64, dpsi: &mut f64, deps: &mut f64, epsa: &mut f64, rb: &mut [f64; 9], rp: &mut [f64; 9], rbp: &mut [f64; 9], rn: &mut [f64; 9], rbpn: &mut [f64; 9]) {
    unsafe {
        eraPn00b(date1, date2, dpsi, deps, epsa, rb, rp, rbp, rn, rbpn);
    }
}

pub fn Pn06(date1: f64, date2: f64, dpsi: f64, deps: f64, epsa: &mut f64, rb: &mut [f64; 9], rp: &mut [f64; 9], rbp: &mut [f64; 9], rn: &mut [f64; 9], rbpn: &mut [f64; 9]) {
    unsafe {
        eraPn06(date1, date2, dpsi, deps, epsa, rb, rp, rbp, rn, rbpn);
    }
}

pub fn Pn06a(date1: f64, date2: f64, dpsi: &mut f64, deps: &mut f64, epsa: &mut f64, rb: &mut [f64; 9], rp: &mut [f64; 9], rbp: &mut [f64; 9], rn: &mut [f64; 9], rbpn: &mut [f64; 9]) {
    unsafe {
        eraPn06a(date1, date2, dpsi, deps, epsa, rb, rp, rbp, rn, rbpn);
    }
}

pub fn Pnm00a(date1: f64, date2: f64, rbpn: &mut [f64; 9]) {
    unsafe {
        eraPnm00a(date1, date2, rbpn);
    }
}

pub fn Pnm00b(date1: f64, date2: f64, rbpn: &mut [f64; 9]) {
    unsafe {
        eraPnm00b(date1, date2, rbpn);
    }
}

pub fn Pnm06a(date1: f64, date2: f64, rbpn: &mut [f64; 9]) {
    unsafe {
        eraPnm06a(date1, date2, rbpn);
    }
}

pub fn Pnm80(date1: f64, date2: f64, rmatpn: &mut [f64; 9]) {
    unsafe {
        eraPnm80(date1, date2, rmatpn);
    }
}

pub fn Pom00(xp: f64, yp: f64, sp: f64, rpom: &mut [f64; 9]) {
    unsafe {
        eraPom00(xp, yp, sp, rpom);
    }
}

pub fn Pr00(date1: f64, date2: f64) -> (f64, f64) {
    let mut dpsipr: f64 = 0.0;
    let mut depspr: f64 = 0.0;

    unsafe {
        eraPr00(date1, date2, &mut dpsipr, &mut depspr);
    }

    return (dpsipr, depspr)
}

pub fn Prec76(date01: f64, date02: f64, date11: f64, date12: f64) -> (f64, f64, f64) {
    let mut zeta: f64 = 0.0;
    let mut z: f64 = 0.0;
    let mut theta: f64 = 0.0;

    unsafe {
        eraPrec76(date01, date02, date11, date12, &mut zeta, &mut z, &mut theta);
    }

    return (zeta, z, theta)
}

pub fn S00(date1: f64, date2: f64, x: f64, y: f64) -> f64 {
    return unsafe { eraS00(date1, date2, x, y) }
}

pub fn S00a(date1: f64, date2: f64) -> f64 {
    return unsafe { eraS00a(date1, date2) }
}

pub fn S00b(date1: f64, date2: f64) -> f64 {
    return unsafe { eraS00b(date1, date2) }
}

pub fn S06(date1: f64, date2: f64, x: f64, y: f64) -> f64 {
    return unsafe { eraS06(date1, date2, x, y) }
}

pub fn S06a(date1: f64, date2: f64) -> f64 {
    return unsafe { eraS06a(date1, date2) }
}

pub fn Sp00(date1: f64, date2: f64) -> f64 {
    return unsafe { eraSp00(date1, date2) }
}

pub fn Xy06(date1: f64, date2: f64) -> (f64, f64) {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;

    unsafe {
        eraXy06(date1, date2, &mut x, &mut y);
    }

    return (x, y)
}

pub fn Xys00a(date1: f64, date2: f64) -> (f64, f64, f64) {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut s: f64 = 0.0;

    unsafe {
        eraXys00a(date1, date2, &mut x, &mut y, &mut s);
    }

    return (x, y, s)
}

pub fn Xys00b(date1: f64, date2: f64) -> (f64, f64, f64) {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut s: f64 = 0.0;

    unsafe {
        eraXys00b(date1, date2, &mut x, &mut y, &mut s);
    }

    return (x, y, s)
}

pub fn Xys06a(date1: f64, date2: f64) -> (f64, f64, f64) {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut s: f64 = 0.0;

    unsafe {
        eraXys06a(date1, date2, &mut x, &mut y, &mut s);
    }

    return (x, y, s)
}
