use std::ffi::CString;

use crate::{raw::astrometry::*, unexpected_val_err, Astrom, ERFAError, LDBody};

pub fn Ab(pnat: &[f64; 3], v: &[f64; 3], s: f64, bm1: f64) -> [f64; 3] {
    let mut ppr: [f64; 3] = [0.0; 3];
    unsafe{ eraAb(pnat, v, s, bm1, &mut ppr) }
    return ppr
}

pub fn Apcg(date1: f64, date2: f64, ebpv: &[f64; 6], ehp: &[f64; 3], astrom: &mut Astrom) {
    unsafe{ eraApcg(date1, date2, ebpv, ehp, astrom) }
}

pub fn Apcg13(date1: f64, date2: f64, astrom: &mut Astrom) {
    unsafe{ eraApcg13(date1, date2, astrom) }
}

pub fn Apci(date1: f64, date2: f64, ebpv: &[f64; 6], ehp: &[f64; 3], x: f64, y: f64, s: f64, astrom: &mut Astrom) {
    unsafe{ eraApci(date1, date2, ebpv, ehp, x, y, s, astrom) }
}

pub fn Apci13(date1: f64, date2: f64, astrom: &mut Astrom) -> f64 {
    let mut eo: f64 = 0.0;
    unsafe{ eraApci13(date1, date2, astrom, &mut eo) }
    return eo
}

pub fn Apco(date1: f64, date2: f64, ebpv: &[f64; 6], ehp: &[f64; 3], x: f64, y: f64, s: f64, theta: f64, elong: f64, phi: f64, hm: f64, xp: f64, yp: f64, sp: f64, refa: f64, refb: f64, astrom: &mut Astrom) {
    unsafe{ eraApco(date1, date2, ebpv, ehp, x, y, s, theta, elong, phi, hm, xp, yp, sp, refa, refb, astrom) }
}

pub fn Apco13(utc1: f64, utc2: f64, dut1: f64, elong: f64, phi: f64, hm: f64, xp: f64, yp: f64, phpa: f64, tc: f64, rh: f64, w1: f64, astrom: &mut Astrom) -> Result<f64, ERFAError> {
    let mut eo: f64 = 0.0;
    let err: i32;
    unsafe{ err = eraApco13(utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, w1, astrom, &mut eo) }
    
    match err {
        1 => Ok(eo),
        0 => Ok(eo),
        -1 => Err(ERFAError::ERFABadDate),
        _ => unexpected_val_err!(eraApco13)
    }
}

pub fn Apcs(date1: f64, date2: f64, pv: &[f64; 6], ebpv: &[f64; 6], ehp: &[f64; 3], astrom: &mut Astrom) {
    unsafe{ eraApcs(date1, date2, pv, ebpv, ehp, astrom) }
}

pub fn Apcs13(date1: f64, date2: f64, pv: &[f64; 6], astrom: &mut Astrom) {
    unsafe{ eraApcs13(date1, date2, pv, astrom) }
}

pub fn Aper(theta: f64, astrom: &mut Astrom) {
    unsafe{ eraAper(theta, astrom) }
}

pub fn Aper13(ut11: f64, ut12: f64, astrom: &mut Astrom) {
    unsafe{ eraAper13(ut11, ut12, astrom) }
}

pub fn Apio(sp: f64, theta: f64, elong: f64, phi: f64, hm: f64, xp: f64, yp: f64, refa: f64, refb: f64, astrom: &mut Astrom) {
    unsafe{ eraApio(sp, theta, elong, phi, hm, xp, yp, refa, refb, astrom) }
}

pub fn Apio13(utc1: f64, utc2: f64, dut1: f64, elong: f64, phi: f64, hm: f64, xp: f64, yp: f64, phpa: f64, tc: f64, rh: f64, w1: f64, astrom: &mut Astrom) -> Option<ERFAError> {
    let err: i32;
    unsafe{ err = eraApio13(utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, w1, astrom) }

    match err {
        1 => None,
        0 => None,
        -1 => Some(ERFAError::ERFABadDate),
        _ => unexpected_val_err!(eraApio13)
    }
}

pub fn Atcc13(rc: f64, dc: f64, pr: f64, pd: f64, px: f64, rv: f64, date1: f64, date2: f64) -> (f64, f64) {
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    unsafe{ eraAtcc13(rc, dc, pr, pd, px, rv, date1, date2, &mut ra, &mut dec) }
    return (ra, dec)
}

pub fn Atccq(rc: f64, dc: f64, pr: f64, pd: f64, px: f64, rv: f64, astrom: &Astrom) -> (f64, f64) {
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    unsafe{ eraAtccq(rc, dc, pr, pd, px, rv, astrom, &mut ra, &mut dec) }
    return (ra, dec)
}

pub fn Atci13(rc: f64, dc: f64, pr: f64, pd: f64, px: f64, rv: f64, date1: f64, date2: f64) -> (f64, f64, f64) {
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    let mut eo: f64 = 0.0;
    unsafe{ eraAtci13(rc, dc, pr, pd, px, rv, date1, date2, &mut ra, &mut dec, &mut eo) }
    return (ra, dec, eo)
}

pub fn Atciq(rc: f64, dc: f64, pr: f64, pd: f64, px: f64, rv: f64, astrom: &Astrom) -> (f64, f64) {
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    unsafe{ eraAtciq(rc, dc, pr, pd, px, rv, astrom, &mut ra, &mut dec) }
    return (ra, dec)
}

pub fn Atciqn(rc: f64, dc: f64, pr: f64, pd: f64, px: f64, rv: f64, astrom: &Astrom, n: i32, b: &LDBody) -> (f64, f64) {
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    unsafe{ eraAtciqn(rc, dc, pr, pd, px, rv, astrom, n, b, &mut ra, &mut dec) }
    return (ra, dec)
}

pub fn Atciqz(rc: f64, dc: f64, astrom: &Astrom) -> (f64, f64) {
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    unsafe{ eraAtciqz(rc, dc, astrom, &mut ra, &mut dec) }
    return (ra, dec)
}

pub fn Atco13(rc: f64, dc: f64, pr: f64, pd: f64, px: f64, rv: f64, utc1: f64, utc2: f64, dut1: f64, elong: f64, phi: f64, hm: f64, xp: f64, yp: f64, phpa: f64, tc: f64, rh: f64, w1: f64) -> Result<(f64, f64, f64, f64, f64, f64), ERFAError> {
    let mut aob: f64 = 0.0;
    let mut zob: f64 = 0.0;
    let mut hob: f64 = 0.0;
    let mut dob: f64 = 0.0;
    let mut rob: f64 = 0.0;
    let mut eo: f64 = 0.0;
    let err: i32;
    unsafe{ err = eraAtco13(rc, dc, pr, pd, px, rv, utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, w1, &mut aob, &mut zob, &mut hob, &mut dob, &mut rob, &mut eo) }
    
    match err {
        1 => Ok((aob, zob, hob, dob, rob, eo)),
        0 => Ok((aob, zob, hob, dob, rob, eo)),
        -1 => Err(ERFAError::ERFABadDate),
        _ => unexpected_val_err!(eraAtco13)
    }
}

pub fn Atic13(ri: f64, di: f64, date1: f64, date2: f64) -> (f64, f64, f64) {
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    let mut eo: f64 = 0.0;
    unsafe{ eraAtic13(ri, di, date1, date2, &mut ra, &mut dec, &mut eo) }
    return (ra, dec, eo)
}

pub fn Aticq(ri: f64, di: f64, astrom: &Astrom) -> (f64, f64) {
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    unsafe{ eraAticq(ri, di, astrom, &mut ra, &mut dec) }
    return (ra, dec)
}

pub fn Aticqn(ri: f64, di: f64, astrom: &Astrom, b: &[LDBody]) -> (f64, f64) {
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    unsafe{ eraAticqn(ri, di, astrom, b.len().try_into().unwrap(), b.as_ptr(), &mut ra, &mut dec) }
    return (ra, dec)
}

pub fn Atio13(ri: f64, di: f64, utc1: f64, utc2: f64, dut1: f64, elong: f64, phi: f64, hm: f64, xp: f64, yp: f64, phpa: f64, tc: f64, rh: f64, w1: f64) -> Result<(f64, f64, f64, f64, f64), ERFAError> {
    let mut aob: f64 = 0.0;
    let mut zob: f64 = 0.0;
    let mut hob: f64 = 0.0;
    let mut dob: f64 = 0.0;
    let mut rob: f64 = 0.0;
    let err: i32;
    unsafe{ err = eraAtio13(ri, di, utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, w1, &mut aob, &mut zob, &mut hob, &mut dob, &mut rob) }
    match err {
        1 => Ok((aob, zob, hob, dob, rob)),
        0 => Ok((aob, zob, hob, dob, rob)),
        -1 => Err(ERFAError::ERFABadDate),
        _ => unexpected_val_err!(eraAtio13)
    }
}

pub fn Atioq(ri: f64, di: f64, astrom: &Astrom) -> (f64, f64, f64, f64, f64) {
    let mut aob: f64 = 0.0;
    let mut zob: f64 = 0.0;
    let mut hob: f64 = 0.0;
    let mut dob: f64 = 0.0;
    let mut rob: f64 = 0.0;
    unsafe{ eraAtioq(ri, di, astrom, &mut aob, &mut zob, &mut hob, &mut dob, &mut rob) }
    return (aob, zob, hob, dob, rob)
}

pub fn Atoc13(ctype: char, ob1: f64, ob2: f64, utc1: f64, utc2: f64, dut1: f64, elong: f64, phi: f64, hm: f64, xp: f64, yp: f64, phpa: f64, tc: f64, rh: f64, w1: f64) -> Result<(f64, f64), ERFAError> {
    let charin = match ctype {
        'R' => CString::new("R"),
        'r' => CString::new("r"),
        'H' => CString::new("H"),
        'h' => CString::new("h"),
        'A' => CString::new("A"),
        'a' => CString::new("a"),
        _ => return Err(ERFAError::ERFABadInputValue)
    };
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    let err: i32;
    unsafe{ err = eraAtoc13(charin.unwrap().as_ptr(), ob1, ob2, utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, w1, &mut ra, &mut dec) }
    match err {
        1 => Ok((ra, dec)),
        0 => Ok((ra, dec)),
        -1 => Err(ERFAError::ERFABadDate),
        _ => unexpected_val_err!(eraAtoc13)
    }
}

pub fn Atoi13(ctype: char, ob1: f64, ob2: f64, utc1: f64, utc2: f64, dut1: f64, elong: f64, phi: f64, hm: f64, xp: f64, yp: f64, phpa: f64, tc: f64, rh: f64, w1: f64) -> Result<(f64, f64), ERFAError> {
    let charin = match ctype {
        'R' => CString::new("R"),
        'r' => CString::new("r"),
        'H' => CString::new("H"),
        'h' => CString::new("h"),
        'A' => CString::new("A"),
        'a' => CString::new("a"),
        _ => return Err(ERFAError::ERFABadInputValue)
    };
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    let err: i32;
    unsafe{ err = eraAtoi13(charin.unwrap().as_ptr(), ob1, ob2, utc1, utc2, dut1, elong, phi, hm, xp, yp, phpa, tc, rh, w1, &mut ra, &mut dec) }
    match err {
        1 => Ok((ra, dec)),
        0 => Ok((ra, dec)),
        -1 => Err(ERFAError::ERFABadDate),
        _ => unexpected_val_err!(eraAtoi13)
    }
}

pub fn Atoiq(ctype: char, ob1: f64, ob2: f64, astrom: &Astrom) -> Result<(f64, f64), ERFAError> {
    let charin = match ctype {
        'R' => CString::new("R"),
        'r' => CString::new("r"),
        'H' => CString::new("H"),
        'h' => CString::new("h"),
        'A' => CString::new("A"),
        'a' => CString::new("a"),
        _ => return Err(ERFAError::ERFABadInputValue)
    };
    let mut ra: f64 = 0.0;
    let mut dec: f64 = 0.0;
    unsafe{ eraAtoiq(charin.unwrap().as_ptr(), ob1, ob2, astrom, &mut ra, &mut dec) }
    return Ok((ra, dec))
}

pub fn Ld(bm: f64, p: &[f64; 3], q: &[f64; 3], e: &[f64; 3], em: f64, dlim: f64) -> [f64; 3] {
    let mut p1: [f64; 3] = [0.0; 3];
    unsafe{ eraLd(bm, p, q, e, em, dlim, &mut p1) }
    return p1
}

pub fn Ldn(b: &[LDBody], ob: &[f64; 3], sc: &[f64; 3]) -> [f64; 3] {
    let mut sn: [f64; 3] = [0.0; 3];
    unsafe{ eraLdn(b.len().try_into().unwrap(), b.as_ptr(), ob, sc, &mut sn) }
    return sn
}

pub fn Ldsun(p: &[f64; 3], e: &[f64; 3], em: f64) -> [f64; 3] {
    let mut p1: [f64; 3] = [0.0; 3];
    unsafe{ eraLdsun(p, e, em, &mut p1) }
    return p1
}

pub fn Pmpx(rc: f64, dc: f64, pr: f64, pd: f64, px: f64, rv: f64, pmt: f64, pob: &[f64; 3]) -> [f64; 3] {
    let mut pco: [f64; 3] = [0.0; 3];
    unsafe{ eraPmpx(rc, dc, pr, pd, px, rv, pmt, pob, &mut pco) }
    return pco
}

pub fn Pmsafe(ra: f64, dec: f64, pmr: f64, pmd: f64, px: f64, rv: f64, ep1a: f64, ep1b: f64, ep2a: f64, ep2b: f64) -> (f64, f64, f64, f64, f64, f64) {
    let mut ra2: f64 = 0.0;
    let mut dec2: f64 = 0.0;
    let mut pmr2: f64 = 0.0;
    let mut pmd2: f64 = 0.0;
    let mut px2: f64 = 0.0;
    let mut rv2: f64 = 0.0;
    let err: i32;
    unsafe{ err = eraPmsafe(ra, dec, pmr, pmd, px, rv, ep1a, ep1b, ep2a, ep2b, &mut ra2, &mut dec2, &mut pmr2, &mut pmd2, &mut px2, &mut rv2) }

    if err < 0 {
        panic!("ERFA encountered a system error in call to eraPmsafe")
    }

    return (ra2, dec2, pmr2, pmd2, px2, rv2)
}

pub fn Pvtob(elong: f64, phi: f64, hm: f64, xp: f64, yp: f64, sp: f64, theta: f64) -> [f64; 6] {
    let mut pv: [f64; 6] = [0.0; 6];
    unsafe{ eraPvtob(elong, phi, hm, xp, yp, sp, theta, &mut pv) }
    return pv
}

pub fn Refco(phpa: f64, tc: f64, rh: f64, w1: f64) -> (f64, f64) {
    let mut refa: f64 = 0.0;
    let mut refb: f64 = 0.0;
    unsafe{ eraRefco(phpa, tc, rh, w1, &mut refa, &mut refb) }
    return (refa, refb)
}

