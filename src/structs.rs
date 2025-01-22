/// Star-independent astrometry parameters
#[repr(C)]
#[derive(Debug, Default)]
pub struct Astrom {
    /// PM time interval (SSB, Julian years)
    pub pmt: f64,
    /// SSB to observer (vector, au)
    pub eb: [f64; 3],
    /// Sun to observer (unit vector)
    pub eh: [f64; 3],
    /// Distance from Sun to observer (au)
    pub em: f64,
    /// Barycentric observer velocity (vector, c)
    pub v: [f64; 3],
    /// sqrt(1 - |v|^2): reciprocal of Lorenz factor
    pub bm1: f64,
    /// Bias-precession-nutation matrix
    pub bpn: [f64; 9],
    /// Longitude + s' + dERA(DUT) (radians)
    pub along: f64,
    /// Geodetic latitude (radians)
    pub phi: f64,
    /// Polar motion xp wrt local meridian (radians)
    pub xpl: f64,
    /// Polar motion yp wrt local meridian (radians)
    pub ypl: f64,
    /// Sine of geodetic latitude
    pub sphi: f64,
    /// Cosine of geodetic latitude
    pub cphi: f64,
    /// Magnitude of diurnal aberration vector
    pub diurab: f64,
    /// "local" Earth rotation angle (radians)
    pub eral: f64,
    /// Refraction constant A (radians)
    pub refa: f64,
    /// Refraction constant B (radians)
    pub refb: f64,
}

/// Body parameters for light deflection
#[repr(C)]
#[derive(Debug, Default)]
pub struct LDBody {
    /// Mass of the body (solar masses)
    pub bm: f64,
    /// Deflection limiter (radians^2/2)
    pub d1: f64,
    /// Barycentric PV of the body (au, au/day)
    pub pv: [f64; 6],
}
