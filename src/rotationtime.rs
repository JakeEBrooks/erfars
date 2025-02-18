//! ERFA Earth Rotation Angle and Sidereal Time Functions

use crate::raw::rotationtime::*;

pub fn Ee00(date1: f64, date2: f64, epsa: f64, dpsi: f64) -> f64 {
    return unsafe {
        eraEe00(date1, date2, epsa, dpsi)
    }
}

pub fn Ee00a(date1: f64, date2: f64) -> f64 {
    return unsafe {
        eraEe00a(date1, date2)
    }
}

pub fn Ee00b(date1: f64, date2: f64) -> f64 {
    return unsafe {
        eraEe00b(date1, date2)
    }
}

pub fn Ee06a(date1: f64, date2: f64) -> f64 {
    return unsafe {
        eraEe06a(date1, date2)
    }
}

pub fn Eect00(date1: f64, date2: f64) -> f64 {
    return unsafe {
        eraEect00(date1, date2)
    }
}

pub fn Eqeq94(date1: f64, date2: f64) -> f64 {
    return unsafe {
        eraEqeq94(date1, date2)
    }
}

pub fn Era00(dj1: f64, dj2: f64) -> f64 {
    return unsafe {
        eraEra00(dj1, dj2)
    }
}

pub fn Gmst00(uta: f64, utb: f64, tta: f64, ttb: f64) -> f64 {
    return unsafe {
        eraGmst00(uta, utb, tta, ttb)
    }
}

pub fn Gmst06(uta: f64, utb: f64, tta: f64, ttb: f64) -> f64 {
    return unsafe {
        eraGmst06(uta, utb, tta, ttb)
    }
}

pub fn Gmst82(dj1: f64, dj2: f64) -> f64 {
    return unsafe {
        eraGmst82(dj1, dj2)
    }
}

pub fn Gst00a(uta: f64, utb: f64, tta: f64, ttb: f64) -> f64 {
    return unsafe {
        eraGst00a(uta, utb, tta, ttb)
    }
}

pub fn Gst00b(uta: f64, utb: f64) -> f64 {
    return unsafe {
        eraGst00b(uta, utb)
    }
}

pub fn Gst06(uta: f64, utb: f64, tta: f64, ttb: f64, rnpb: &[f64; 9]) -> f64 {
    return unsafe {
        eraGst06(uta, utb, tta, ttb, rnpb)
    }
}

pub fn Gst06a(uta: f64, utb: f64, tta: f64, ttb: f64) -> f64 {
    return unsafe {
        eraGst06a(uta, utb, tta, ttb)
    }
}

pub fn Gst94(uta: f64, utb: f64) -> f64 {
    return unsafe {
        eraGst94(uta, utb)
    }
}
