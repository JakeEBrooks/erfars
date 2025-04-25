//! ERFA Fundamental Arguments for Nutation

use crate::raw::fundamentalargs::*;

///  Fundamental argument, IERS Conventions (2003):
///  mean elongation of the Moon from the Sun.
///
///  Given:
///     t     double    TDB, Julian centuries since J2000.0 (Note 1)
///
///  Returned (function value):
///           double    D, radians (Note 2)
///
///  Notes:
///
///  1) Though t is strictly TDB, it is usually more convenient to use
///     TT, which makes no significant difference.
///
///  2) The expression used is as adopted in IERS Conventions (2003) and
///     is from Simon et al. (1994).
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M.,
///     Francou, G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
pub fn Fad03(t: f64) -> f64 {
    return unsafe { eraFad03(t) };
}

///  Fundamental argument, IERS Conventions (2003):
///  mean longitude of Earth.
///
///  Given:
///     t     double    TDB, Julian centuries since J2000.0 (Note 1)
///
///  Returned (function value):
///           double    mean longitude of Earth, radians (Note 2)
///
///  Notes:
///
///  1) Though t is strictly TDB, it is usually more convenient to use
///     TT, which makes no significant difference.
///
///  2) The expression used is as adopted in IERS Conventions (2003) and
///     comes from Souchay et al. (1999) after Simon et al. (1994).
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M.,
///     Francou, G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
///
///     Souchay, J., Loysel, B., Kinoshita, H., Folgueira, M. 1999,
///     Astron.Astrophys.Supp.Ser. 135, 111
pub fn Fae03(t: f64) -> f64 {
    return unsafe { eraFae03(t) };
}

///  Fundamental argument, IERS Conventions (2003):
///  mean longitude of the Moon minus mean longitude of the ascending
///  node.
///
///  Given:
///     t     double    TDB, Julian centuries since J2000.0 (Note 1)
///
///  Returned (function value):
///           double    F, radians (Note 2)
///
///  Notes:
///
///  1) Though t is strictly TDB, it is usually more convenient to use
///     TT, which makes no significant difference.
///
///  2) The expression used is as adopted in IERS Conventions (2003) and
///     is from Simon et al. (1994).
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M.,
///     Francou, G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
pub fn Faf03(t: f64) -> f64 {
    return unsafe { eraFaf03(t) };
}

///  Fundamental argument, IERS Conventions (2003):
///  mean longitude of Jupiter.
///
///  Given:
///     t     double    TDB, Julian centuries since J2000.0 (Note 1)
///
///  Returned (function value):
///           double    mean longitude of Jupiter, radians (Note 2)
///
///  Notes:
///
///  1) Though t is strictly TDB, it is usually more convenient to use
///     TT, which makes no significant difference.
///
///  2) The expression used is as adopted in IERS Conventions (2003) and
///     comes from Souchay et al. (1999) after Simon et al. (1994).
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M.,
///     Francou, G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
///
///     Souchay, J., Loysel, B., Kinoshita, H., Folgueira, M. 1999,
///     Astron.Astrophys.Supp.Ser. 135, 111
pub fn Faju03(t: f64) -> f64 {
    return unsafe { eraFaju03(t) };
}

///  Fundamental argument, IERS Conventions (2003):
///  mean anomaly of the Moon.
///
///  Given:
///     t     double    TDB, Julian centuries since J2000.0 (Note 1)
///
///  Returned (function value):
///           double    l, radians (Note 2)
///
///  Notes:
///
///  1) Though t is strictly TDB, it is usually more convenient to use
///     TT, which makes no significant difference.
///
///  2) The expression used is as adopted in IERS Conventions (2003) and
///     is from Simon et al. (1994).
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M.,
///     Francou, G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
pub fn Fal03(t: f64) -> f64 {
    return unsafe { eraFal03(t) };
}

///  Fundamental argument, IERS Conventions (2003):
///  mean anomaly of the Sun.
///
///  Given:
///     t     double    TDB, Julian centuries since J2000.0 (Note 1)
///
///  Returned (function value):
///           double    l', radians (Note 2)
///
///  Notes:
///
///  1) Though t is strictly TDB, it is usually more convenient to use
///     TT, which makes no significant difference.
///
///  2) The expression used is as adopted in IERS Conventions (2003) and
///     is from Simon et al. (1994).
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M.,
///     Francou, G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
pub fn Falp03(t: f64) -> f64 {
    return unsafe { eraFalp03(t) };
}

///  Fundamental argument, IERS Conventions (2003):
///  mean longitude of Mars.
///
///  Given:
///     t     double    TDB, Julian centuries since J2000.0 (Note 1)
///
///  Returned (function value):
///           double    mean longitude of Mars, radians (Note 2)
///
///  Notes:
///
///  1) Though t is strictly TDB, it is usually more convenient to use
///     TT, which makes no significant difference.
///
///  2) The expression used is as adopted in IERS Conventions (2003) and
///     comes from Souchay et al. (1999) after Simon et al. (1994).
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M.,
///     Francou, G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
///
///     Souchay, J., Loysel, B., Kinoshita, H., Folgueira, M. 1999,
///     Astron.Astrophys.Supp.Ser. 135, 111
pub fn Fama03(t: f64) -> f64 {
    return unsafe { eraFama03(t) };
}

///  Fundamental argument, IERS Conventions (2003):
///  mean longitude of Mercury.
///
///  Given:
///     t     double    TDB, Julian centuries since J2000.0 (Note 1)
///
///  Returned (function value):
///           double    mean longitude of Mercury, radians (Note 2)
///
///  Notes:
///
///  1) Though t is strictly TDB, it is usually more convenient to use
///     TT, which makes no significant difference.
///
///  2) The expression used is as adopted in IERS Conventions (2003) and
///     comes from Souchay et al. (1999) after Simon et al. (1994).
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M.,
///     Francou, G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
///
///     Souchay, J., Loysel, B., Kinoshita, H., Folgueira, M. 1999,
///     Astron.Astrophys.Supp.Ser. 135, 111
pub fn Fame03(t: f64) -> f64 {
    return unsafe { eraFame03(t) };
}

///  Fundamental argument, IERS Conventions (2003):
///  mean longitude of Neptune.
///
///  Given:
///     t     double    TDB, Julian centuries since J2000.0 (Note 1)
///
///  Returned (function value):
///           double    mean longitude of Neptune, radians (Note 2)
///
///  Notes:
///
///  1) Though t is strictly TDB, it is usually more convenient to use
///     TT, which makes no significant difference.
///
///  2) The expression used is as adopted in IERS Conventions (2003) and
///     is adapted from Simon et al. (1994).
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M.,
///     Francou, G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
pub fn Fane03(t: f64) -> f64 {
    return unsafe { eraFane03(t) };
}

///  Fundamental argument, IERS Conventions (2003):
///  mean longitude of the Moon's ascending node.
///
///  Given:
///     t     double    TDB, Julian centuries since J2000.0 (Note 1)
///
///  Returned (function value):
///           double    Omega, radians (Note 2)
///
///  Notes:
///
///  1) Though t is strictly TDB, it is usually more convenient to use
///     TT, which makes no significant difference.
///
///  2) The expression used is as adopted in IERS Conventions (2003) and
///     is from Simon et al. (1994).
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M.,
///     Francou, G., Laskar, J., 1994, Astron.Astrophys. 282, 663-683.
pub fn Faom03(t: f64) -> f64 {
    return unsafe { eraFaom03(t) };
}

///  Fundamental argument, IERS Conventions (2003):
///  general accumulated precession in longitude.
///
///  Given:
///     t     double    TDB, Julian centuries since J2000.0 (Note 1)
///
///  Returned (function value):
///           double    general precession in longitude, radians (Note 2)
///
///  Notes:
///
///  1) Though t is strictly TDB, it is usually more convenient to use
///     TT, which makes no significant difference.
///
///  2) The expression used is as adopted in IERS Conventions (2003).  It
///     is taken from Kinoshita & Souchay (1990) and comes originally
///     from Lieske et al. (1977).
///
///  References:
///
///     Kinoshita, H. and Souchay J. 1990, Celest.Mech. and Dyn.Astron.
///     48, 187
///
///     Lieske, J.H., Lederle, T., Fricke, W. & Morando, B. 1977,
///     Astron.Astrophys. 58, 1-16
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
pub fn Fapa03(t: f64) -> f64 {
    return unsafe { eraFapa03(t) };
}

///  Fundamental argument, IERS Conventions (2003):
///  mean longitude of Saturn.
///
///  Given:
///     t     double    TDB, Julian centuries since J2000.0 (Note 1)
///
///  Returned (function value):
///           double    mean longitude of Saturn, radians (Note 2)
///
///  Notes:
///
///  1) Though t is strictly TDB, it is usually more convenient to use
///     TT, which makes no significant difference.
///
///  2) The expression used is as adopted in IERS Conventions (2003) and
///     comes from Souchay et al. (1999) after Simon et al. (1994).
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M.,
///     Francou, G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
///
///     Souchay, J., Loysel, B., Kinoshita, H., Folgueira, M. 1999,
///     Astron.Astrophys.Supp.Ser. 135, 111
pub fn Fasa03(t: f64) -> f64 {
    return unsafe { eraFasa03(t) };
}

///  Fundamental argument, IERS Conventions (2003):
///  mean longitude of Uranus.
///
///  Given:
///     t     double    TDB, Julian centuries since J2000.0 (Note 1)
///
///  Returned  (function value):
///           double    mean longitude of Uranus, radians (Note 2)
///
///  Notes:
///
///  1) Though t is strictly TDB, it is usually more convenient to use
///     TT, which makes no significant difference.
///
///  2) The expression used is as adopted in IERS Conventions (2003) and
///     is adapted from Simon et al. (1994).
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M.,
///     Francou, G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
pub fn Faur03(t: f64) -> f64 {
    return unsafe { eraFaur03(t) };
}

///  Fundamental argument, IERS Conventions (2003):
///  mean longitude of Venus.
///
///  Given:
///     t     double    TDB, Julian centuries since J2000.0 (Note 1)
///
///  Returned (function value):
///           double    mean longitude of Venus, radians (Note 2)
///
///  Notes:
///
///  1) Though t is strictly TDB, it is usually more convenient to use
///     TT, which makes no significant difference.
///
///  2) The expression used is as adopted in IERS Conventions (2003) and
///     comes from Souchay et al. (1999) after Simon et al. (1994).
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     Simon, J.-L., Bretagnon, P., Chapront, J., Chapront-Touze, M.,
///     Francou, G., Laskar, J. 1994, Astron.Astrophys. 282, 663-683
///
///     Souchay, J., Loysel, B., Kinoshita, H., Folgueira, M. 1999,
///     Astron.Astrophys.Supp.Ser. 135, 111
pub fn Fave03(t: f64) -> f64 {
    return unsafe { eraFave03(t) };
}
