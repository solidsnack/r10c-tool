use std::num::NonZeroU16;

use delegate::delegate;

use crate::bits;
use crate::descriptors;
use crate::sign::sign;

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

mod bounds {
    /// Coordinates of least R10c value that is greater than or equal
    /// to: 4.941e-324
    pub mod smallest {
        /// The index into `PREFERRED` (so 1 is 1.0, 2 is 1.25, &c).
        pub const INDEX: usize = 8;
        /// The smallest floating point number is subnormal -- the exponent is
        /// smaller than the greatest exponent is large.
        pub const EXPONENT: isize = -324;
    }
    /// Greatest R10c value that is less than or equal to: 1.798e+308
    pub mod largest {
        /// The index into `PREFERRED` (so 1 is 1.0, 2 is 1.25, &c).
        pub const INDEX: usize = 3;
        pub const EXPONENT: isize = 308;
    }
}

#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct Descriptor(NonZeroU16);

impl Descriptor {
    fn lens(&self) -> bits::Lens16 {
        self.0.get().into()
    }

    delegate! {
        to self {
            /// Resolve the descriptor to a value.
            #[through(descriptors::Descriptor)]
            pub fn resolve(&self) -> <Self as descriptors::Descriptor>::Of;

            /// The significand: a member of the R10c series, between 0.0 and
            /// 10.0.  -10.0 and -0.0, depending on the sign of the described
            /// value.
            #[through(descriptors::Descriptor)]
            pub fn significand(&self) -> <Self as descriptors::Descriptor>::Of;

            /// The exponent -- an integer indicating which power of ten the
            /// significand should be multiplied by.
            #[through(descriptors::Descriptor)]
            pub fn exponent(&self) -> isize;

            /// The index of the significand in the R10c series, an integer in
            /// `[0,9]`.
            #[through(descriptors::Descriptor)]
            pub fn index(&self) -> usize;

            /// Produce a new descriptor that resolves to a value `distance`
            /// steps before or after this descriptor. Stepping may fail if the
            /// resulting value is out of range for the target numeric type.
            ///
            /// Stepping can never change the sign of the described value. A
            /// negative step produces a value of lesser magnitude; a positive
            /// step produces a value of greater magnitude.
            #[through(descriptors::Descriptor)]
            pub fn step(&self, distance: isize) -> Option<Self>;
        }
    }
}

impl descriptors::Descriptor for Descriptor {
    type Of = f64;

    fn significand(&self) -> Self::Of {
        let sign: Self::Of = sign!(self.lens().positive()).into();
        sign * PREFERRED[self.lens().index()]
    }

    fn index(&self) -> usize {
        self.lens().index() - 1
    }

    fn exponent(&self) -> isize {
        self.lens().exponent()
    }

    fn prev(input: Self::Of) -> Option<Self> {
        let near = Self::near(input)?;

        if near.resolve().abs() < input.abs() {
            Some(near)
        } else {
            near.step(-1)
        }
    }

    fn near(input: Self::Of) -> Option<Self> {
        use bounds::*;

        let (index, exponent) =
            match (input == 0.0, input.is_infinite(), input.is_nan()) {
                (true, _, _) => return None,
                (_, true, _) => return None,
                (_, _, true) => return None,
                (_, _, _) => {
                    let abs = input.abs();
                    let log10 = abs.log10();
                    let decade = log10.floor() as isize;
                    let locator = log10 - (decade as f64);
                    let found = search(locator, &MID_BOUNDARIES);

                    match found {
                        0 => (10, decade - 1),
                        11 => (1, decade + 1),
                        _ => (found, decade),
                    }
                }
            };

        let bounded = match exponent {
            ..smallest::EXPONENT => (smallest::INDEX, smallest::EXPONENT),
            smallest::EXPONENT if index < smallest::INDEX => {
                (smallest::INDEX, smallest::EXPONENT)
            }
            largest::EXPONENT if index > largest::INDEX => {
                (largest::INDEX, largest::EXPONENT)
            }
            largest::EXPONENT.. if exponent > largest::EXPONENT => {
                (largest::INDEX, largest::EXPONENT)
            }
            _ => (index, exponent),
        };

        let bits = bits::Lens16::new()
            .with_positive(input.is_sign_positive())
            .with_index(bounded.0)
            .with_exponent(bounded.1)
            .into();

        let non_zero = NonZeroU16::new(bits)?;

        Some(Self(non_zero))
    }

    fn next(input: Self::Of) -> Option<Self> {
        let near = Self::near(input)?;

        if near.resolve().abs() > input.abs() {
            Some(near)
        } else {
            near.step(1)
        }
    }

    fn resolve(&self) -> Self::Of {
        // Split up the exponent so we don't form out of range values as
        // intermediates.
        let lo = self.exponent() / 2;
        let hi = self.exponent() - lo;

        self.significand() * TEN.powi(lo as i32) * TEN.powi(hi as i32)
    }

    fn step(&self, distance: isize) -> Option<Self> {
        use bounds::*;

        let inner = self.lens();

        // Logarithm space position.
        let position =
            (inner.exponent() * 10) + ((inner.index() - 1) as isize);
        let stepped = position + distance;

        // Seperate index and exponent again; translate index back in [1..10].
        let (index, exponent) = (
            (stepped.rem_euclid(10) as usize) + 1,
            stepped.div_euclid(10),
        );

        match exponent {
            ..smallest::EXPONENT => None,
            smallest::EXPONENT if index < smallest::INDEX => None,
            largest::EXPONENT if index > largest::INDEX => None,
            largest::EXPONENT.. if exponent > largest::EXPONENT => None,
            _ => {
                let bits: u16 = bits::Lens16::new()
                    .with_positive(inner.positive())
                    .with_index(index)
                    .with_exponent(exponent)
                    .into();

                let non_zero = NonZeroU16::new(bits)?;

                Some(Self(non_zero))
            }
        }
    }
}

impl descriptors::sealed::Marker for f64 {}

impl descriptors::Describable for f64 {
    type D = Descriptor;

    fn prev(self) -> Option<Self::D> {
        use descriptors::Descriptor;

        Self::D::prev(self)
    }

    fn near(self) -> Option<Self::D> {
        use descriptors::Descriptor;

        Self::D::near(self)
    }

    fn next(self) -> Option<Self::D> {
        use descriptors::Descriptor;

        Self::D::next(self)
    }
}

impl From<Descriptor> for f64 {
    fn from(value: Descriptor) -> Self {
        value.resolve()
    }
}

const fn search(locator: f64, boundaries: &[f64]) -> usize {
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
