const BASES: [f64; 11] =
    [1.0, 1.25, 1.6, 2.0, 2.5, 3.2, 4.0, 5.0, 6.4, 8.0, 10.0];
const BOUNDARIES: [f64; 10] = [
    0.048452, 0.150502, 0.252552, 0.349450, 0.451475, 0.553600, 0.650550,
    0.752600, 0.854650, 0.951550,
];

pub fn near(n: f64) -> f64 {
    if (!n.is_finite()) || n == 0.0 {
        return n;
    }

    let ten: f64 = 10.0;
    let abs: f64 = n.abs();
    let sign: f64 = n.signum();
    let log10: f64 = abs.log10();
    let decade: f64 = log10.floor();
    let locator: f64 = log10 - decade;

    let mut left = 0 as usize;
    let mut right = BOUNDARIES.len();

    while left < right {
        let mid = usize::midpoint(left, right);

        if BOUNDARIES[mid] < locator {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    BASES[left] * (ten).powf(decade) * sign
}

// TODO: Next, previous, nth next, nth previous
