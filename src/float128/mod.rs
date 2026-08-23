pub mod constants;

use std::num::NonZero;

use constants::*;
use delegate::delegate;

use crate::descriptors;
use crate::digit_display::digit_display;
use crate::sign::sign;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Descriptor(NonZero<BackingType>);

impl Descriptor {
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

            /// Represent the value as a string. This is simple to do without
            /// float formatting logic. We just move the decimal point forward
            /// or back in the according to the decade.
            #[through(descriptors::Descriptor)]
            pub fn text(&self) -> String;

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

    fn lens(&self) -> LensType {
        self.0.get().into()
    }
}

impl From<Descriptor> for BackingType {
    fn from(value: Descriptor) -> Self {
        value.0.into()
    }
}

impl TryFrom<BackingType> for Descriptor {
    type Error = ();

    fn try_from(value: BackingType) -> Result<Self, Self::Error> {
        let lens = LensType::from(value);
        let positive = lens.positive();
        let index = lens.index().checked_sub(1).ok_or(())?;
        let exponent = lens.exponent();

        <Self as descriptors::Descriptor>::of(positive, index, exponent)
            .ok_or(())
    }
}

impl descriptors::Descriptor for Descriptor {
    type Of = FloatingType;

    /// Create a descriptor. Use indexes 0, ..., 9.
    fn of(is_positive: bool, index: usize, exponent: isize) -> Option<Self> {
        use bounds::*;

        let corrected = index + 1;

        if corrected < 1 || corrected > 10 {
            return None;
        }

        match exponent {
            ..smallest::EXPONENT => None,
            smallest::EXPONENT if corrected < smallest::INDEX => None,
            largest::EXPONENT if corrected > largest::INDEX => None,
            largest::EXPONENT.. if exponent > largest::EXPONENT => None,
            _ => {
                let bits = LensType::new()
                    .with_positive(is_positive)
                    .with_index(corrected)
                    .with_exponent(exponent)
                    .into();

                let non_zero = NonZero::<BackingType>::new(bits)?;

                Some(Self(non_zero))
            }
        }
    }

    fn significand(&self) -> Self::Of {
        let sign: Self::Of = sign!(self.lens().positive());
        sign * PREFERRED[self.lens().index()]
    }

    fn exponent(&self) -> isize {
        self.lens().exponent()
    }

    fn index(&self) -> usize {
        self.lens().index() - 1
    }

    fn text(&self) -> String {
        // The digits will be "10", "125", "16", "20", "25", &c.
        let base = PREFERRED_DIGITS[self.lens().index()];

        digit_display(&base, self.lens().exponent(), self.lens().positive())
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

        let (index, exponent) = if input.is_normal() {
            let abs = input.abs();
            let log10 = abs.log10();
            let decade = log10.floor() as isize;
            let locator = log10 - (decade as FloatingType);
            let found = search(locator, &MID_BOUNDARIES);

            let pair = match found {
                0 => (10, decade - 1),
                11 => (1, decade + 1),
                _ => (found, decade),
            };

            Some(pair)
        } else {
            None
        }?;

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

        Self::of(input.is_sign_positive(), bounded.0 - 1, bounded.1)
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
        // NB: Base is biased upwards by one power of ten.
        let mut accum = PREFERRED[self.lens().index()];
        let mut exponent = (self.exponent() - 1).abs();
        let positive_exp = self.exponent().is_positive();

        while exponent > 0 {
            let step = MAX_TEN_POS_POW.min(exponent);

            if positive_exp {
                accum *= TEN.powi(step as i32);
            } else {
                accum /= TEN.powi(step as i32);
            }

            exponent -= step;
        }

        if self.lens().positive() {
            accum
        } else {
            -accum
        }
    }

    fn step(&self, distance: isize) -> Option<Self> {
        let inner = self.lens();

        // Logarithm space position.
        let position =
            (inner.exponent() * 10) + ((inner.index() - 1) as isize);
        let stepped = position + distance;

        // Seperate index and exponent again.
        let (index, exponent) =
            ((stepped.rem_euclid(10) as usize), stepped.div_euclid(10));

        Self::of(inner.positive(), index, exponent)
    }
}

impl descriptors::sealed::Marker for FloatingType {}

impl descriptors::Describable for FloatingType {
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

impl From<Descriptor> for FloatingType {
    fn from(value: Descriptor) -> Self {
        value.resolve()
    }
}

const fn search(locator: FloatingType, boundaries: &[FloatingType]) -> usize {
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
