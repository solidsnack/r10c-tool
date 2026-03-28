use crate::{R10c, sealed};

const PREFERRED: [f32; 12] =
    [0.8, 1.0, 1.25, 1.6, 2.0, 2.5, 3.2, 4.0, 5.0, 6.4, 8.0, 10.0];
const MID_BOUNDARIES: [f32; 11] = [
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
const TEN: f32 = 10.0;

struct Syndrome {
    /// 0, -1 or 1.
    sign: f32,
    /// Value between 0 and 1.
    locator: f32,
    /// Exponent.
    decade: i32,
}

impl Syndrome {
    fn new(n: f32) -> Self {
        let abs = n.abs();
        let sign = n.signum();
        let log10 = abs.log10();
        let decade = log10.floor() as i32;
        let locator = log10 - (decade as f32);

        Syndrome {
            sign,
            locator,
            decade,
        }
    }
}

impl sealed::Marker for f32 {}

impl R10c for f32 {
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

fn search(locator: f32, boundaries: &[f32]) -> usize {
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
