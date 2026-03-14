use crate::{R10c, sealed};

const PREFERRED: [f128; 12] =
    [0.8, 1.0, 1.25, 1.6, 2.0, 2.5, 3.2, 4.0, 5.0, 6.4, 8.0, 10.0];
const LOWER_BOUNDARIES: [f128; 11] = [
    -0.09691001300805639,
    0.0,
    0.09691001300805642,
    0.2041199826559248,
    0.3010299956639812,
    0.3979400086720376,
    0.505149978319906,
    0.6020599913279624,
    0.6989700043360189,
    0.8061799739838872,
    0.9030899869919435,
];
const MID_BOUNDARIES: [f128; 11] = [
    -0.048455006504028196,
    0.04845500650402821,
    0.1505149978319906,
    0.252574989159953,
    0.3494850021680094,
    0.4515449934959718,
    0.5536049848239342,
    0.6505149978319906,
    0.752574989159953,
    0.8546349804879154,
    0.9515449934959718,
];
const UPPER_BOUNDARIES: [f128; 11] = [
    0.0,
    0.09691001300805642,
    0.2041199826559248,
    0.3010299956639812,
    0.3979400086720376,
    0.505149978319906,
    0.6020599913279624,
    0.6989700043360189,
    0.8061799739838872,
    0.9030899869919435,
    1.0,
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

        return Syndrome {
            sign,
            locator,
            decade,
        };
    }
}

impl sealed::Marker for f128 {}

impl R10c for f128 {
    fn prev(n: Self) -> Self {
        if (!n.is_finite()) || n == 0.0 {
            return n;
        }

        let syndrome = Syndrome::new(n);
        let idx = search(syndrome.locator, &UPPER_BOUNDARIES);

        PREFERRED[idx] * (TEN).powi(syndrome.decade) * syndrome.sign
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

        let syndrome = Syndrome::new(n);
        let idx = search(syndrome.locator, &LOWER_BOUNDARIES);

        PREFERRED[idx] * (TEN).powi(syndrome.decade) * syndrome.sign
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
