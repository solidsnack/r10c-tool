use crate::{R10c, sealed};

const PREFERRED: [f128; 12] =
    [0.8, 1.0, 1.25, 1.6, 2.0, 2.5, 3.2, 4.0, 5.0, 6.4, 8.0, 10.0];
const MID_BOUNDARIES: [f128; 11] = [
    -0.048455006504028,
    0.048455006504028,
    0.150514997831991,
    0.252574989159953,
    0.349485002168009,
    0.451544993495972,
    0.553604984823934,
    0.650514997831991,
    0.752574989159953,
    0.854634980487915,
    0.951544993495972,
];
const TEN: f128 = 10.0;

struct Syndrome {
    /// 0, -1 or 1.
    sign: f128,
    /// Value between 0 and 1.
    locator: f128,
    /// Exponent.
    decade: i32,
}

impl Syndrome {
    fn new(n: f128) -> Self {
        let abs = n.abs();
        let sign = n.signum();
        let log10 = abs.log10();
        let decade = log10.floor() as i32;
        let locator = log10 - (decade as f128);

        Syndrome {
            sign,
            locator,
            decade,
        }
    }
}

impl sealed::Marker for f128 {}

impl R10c for f128 {
    fn prev(n: Self) -> Self {
        if (!n.is_finite()) || n == 0.0 {
            return n;
        }

        if n < 0.0 {
            -Self::next(n.abs())
        } else {
            let m = Self::near(n);

            if m < n { m } else { Self::near(n * 0.8) }
        }
    }

    fn near(n: Self) -> Self {
        if (!n.is_finite()) || n == 0.0 {
            return n;
        }

        let syndrome = Syndrome::new(n);
        let idx = search(syndrome.locator, &MID_BOUNDARIES);

        PREFERRED[idx] * (TEN).powi(syndrome.decade) * syndrome.sign
    }

    fn next(n: Self) -> Self {
        if (!n.is_finite()) || n == 0.0 {
            return n;
        }

        if n < 0.0 {
            -Self::prev(n.abs())
        } else {
            let m = Self::near(n);

            if m > n { m } else { Self::near(n / 0.8) }
        }
    }
}

fn search(locator: f128, boundaries: &[f128]) -> usize {
    let mut left = 0 as usize;
    let mut right = boundaries.len();

    while left < right {
        let mid = usize::midpoint(left, right);

        if boundaries[mid] < locator {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    left
}
