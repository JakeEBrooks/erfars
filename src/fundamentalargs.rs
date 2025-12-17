//! ERFA Fundamental Arguments for Nutation

use crate::raw::fundamentalargs::*;

/// Fundamental argument, IERS Conventions (2003): mean elongation of the Moon
/// from the Sun.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/fad03.c)
pub fn Fad03(t: f64) -> f64 {
    return unsafe { eraFad03(t) };
}

/// Fundamental argument, IERS Conventions (2003): mean longitude of Earth.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/fae03.c)
pub fn Fae03(t: f64) -> f64 {
    return unsafe { eraFae03(t) };
}

/// Fundamental argument, IERS Conventions (2003): mean longitude of the Moon
/// minus mean longitude of the ascending node.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/faf03.c)
pub fn Faf03(t: f64) -> f64 {
    return unsafe { eraFaf03(t) };
}

/// Fundamental argument, IERS Conventions (2003): mean longitude of Jupiter.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/faju03.c)
pub fn Faju03(t: f64) -> f64 {
    return unsafe { eraFaju03(t) };
}

/// Fundamental argument, IERS Conventions (2003): mean anomaly of the Moon.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/fal03.c)
pub fn Fal03(t: f64) -> f64 {
    return unsafe { eraFal03(t) };
}

/// Fundamental argument, IERS Conventions (2003): mean anomaly of the Sun.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/falp03.c)
pub fn Falp03(t: f64) -> f64 {
    return unsafe { eraFalp03(t) };
}

/// Fundamental argument, IERS Conventions (2003): mean longitude of Mars.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/fama03.c)
pub fn Fama03(t: f64) -> f64 {
    return unsafe { eraFama03(t) };
}

/// Fundamental argument, IERS Conventions (2003): mean longitude of Mercury.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/fame03.c)
pub fn Fame03(t: f64) -> f64 {
    return unsafe { eraFame03(t) };
}

/// Fundamental argument, IERS Conventions (2003): mean longitude of Neptune.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/fane03.c)
pub fn Fane03(t: f64) -> f64 {
    return unsafe { eraFane03(t) };
}

/// Fundamental argument, IERS Conventions (2003): mean longitude of the Moon's
/// ascending node.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/faom03.c)
pub fn Faom03(t: f64) -> f64 {
    return unsafe { eraFaom03(t) };
}

/// Fundamental argument, IERS Conventions (2003): general accumulated
/// precession in longitude.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/fapa03.c)
pub fn Fapa03(t: f64) -> f64 {
    return unsafe { eraFapa03(t) };
}

/// Fundamental argument, IERS Conventions (2003): mean longitude of Saturn.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/fasa03.c)
pub fn Fasa03(t: f64) -> f64 {
    return unsafe { eraFasa03(t) };
}

/// Fundamental argument, IERS Conventions (2003): mean longitude of Uranus.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/faur03.c)
pub fn Faur03(t: f64) -> f64 {
    return unsafe { eraFaur03(t) };
}

/// Fundamental argument, IERS Conventions (2003): mean longitude of Venus.
///
/// Please see the full ERFA docs for this function [here](https://github.com/liberfa/erfa/blob/master/src/fave03.c)
pub fn Fave03(t: f64) -> f64 {
    return unsafe { eraFave03(t) };
}
