use crate::R10c;
use crate::sign::sign;
use super::*;

const PREFERRED: [f64; 12] =
    [0.8, 1.0, 1.25, 1.6, 2.0, 2.5, 3.2, 4.0, 5.0, 6.4, 8.0, 10.0];
const MID_BOUNDARIES: [f64; 11] = [
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
const TEN: f64 = 10.0;

struct F64Descriptor(U32);

impl Descriptor for F64Descriptor {
    type Of = f64;

    fn significand(&self) -> Self::Of {
        sign!(self.0.sign) * PREFERRED[self.0.index()]
    }

    fn exponent(&self) -> usize {
        self.0.exponent()
    }

    fn prev(input: Self::Of) -> Self {
        todo!()
    }

    fn near(input: Self::Of) -> Self {
        let abs = input.abs();
        let log10 = abs.log10();
        let decade = log10.floor() as usize;
        let locator = log10 - (decade as f64);
        let index= search(locator, &MID_BOUNDARIES);

        let bits = U32::new()
            .with_sign(input.is_sign_positive())
            .with_index(index) // Not really correct.
            .with_exponent(decade);

        Self(bits)
    }

    fn next(input: Self::Of) -> Self {
        todo!()
    }

    fn resolve(&self) -> Option<Self::Of> {
        todo!()
    }

    fn step(&self, distance: isize) -> Option<Self> {
        todo!()
    }
}

fn search(locator: f64, boundaries: &[f64]) -> usize {
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
