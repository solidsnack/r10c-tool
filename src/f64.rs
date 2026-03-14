const BASES: [f64; 11] =
    [1.0, 1.25, 1.6, 2.0, 2.5, 3.2, 4.0, 5.0, 6.4, 8.0, 10.0];
const BOUNDARIES: [f64; 10] = [
    0.048452, 0.150502, 0.252552, 0.349450, 0.451475, 0.553600, 0.650550,
    0.752600, 0.854650, 0.951550,
];
const TEN: f64 = 10.0;

struct Syndrome {
    /// 0, -1 or 1.
    sign: f64,
    /// Value between 0 and 1.
    locator: f64,
    /// Exponent.
    decade: i32,
}

impl Syndrome {
    fn new(n: f64) -> Self {
        let abs = n.abs();
        let sign = n.signum();
        let log10 = abs.log10();
        let decade  = log10.floor() as i32;
        let locator = log10 - (decade as f64);

        return Syndrome {
            sign,
            locator,
            decade,
        }
    }
}

pub fn near(n: f64) -> f64 {
    if (!n.is_finite()) || n == 0.0 {
        return n;
    }

    let syndrome = Syndrome::new(n);

    let mut left = 0 as usize;
    let mut right = BOUNDARIES.len();

    while left < right {
        let mid = usize::midpoint(left, right);

        if BOUNDARIES[mid] < syndrome.locator {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    BASES[left] * (TEN).powi(syndrome.decade) * syndrome.sign
}

pub fn prev(n: f64) -> f64 {
    let nearest = near(n);

    if n > nearest {
        nearest
    } else {
        near(nearest * 0.9)
    }
}

pub fn next(n: f64) -> f64 {
    let nearest = near(n);

    if n < nearest {
        nearest
    } else {
        near(nearest / 0.9)
    }
}
