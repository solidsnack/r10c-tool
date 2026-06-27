pub use std::num::NonZeroU16 as BackingType;

pub use f16 as FloatingType;

pub use crate::bits::Lens16 as LensType;

// NB: No negative powers of ten are exact.
pub const MAX_TEN_POS_POW: isize = 4;

pub mod bounds {
    /// Coordinates of least R10c value that is greater than or equal to the
    /// least positive value of this floating point type.
    pub mod smallest {
        /// The index into `PREFERRED` (so 1 is 1.0, 2 is 1.25, &c).
        pub const INDEX: usize = 8;
        /// The smallest floating point number is subnormal -- the exponent is
        /// smaller than the greatest exponent is large.
        pub const EXPONENT: isize = -5;
    }
    /// Greatest R10c value that is less than or equal to the greatest value of
    /// this floating point type.
    pub mod largest {
        /// The index into `PREFERRED` (so 1 is 1.0, 2 is 1.25, &c).
        pub const INDEX: usize = 9;
        pub const EXPONENT: isize = 4;
    }
}

// NB: These numbers are all exactly representable in floating point, whereas
//     many of those in earlier decades (for example, 0.8) are not.
pub const PREFERRED: [FloatingType; 12] =
    // [0.8, 1.0, 1.25, 1.6, 2.0, 2.5, 3.2, 4.0, 5.0, 6.4, 8.0, 10.0];
    [
        8.0, 10.0, 12.5, 16.0, 20.0, 25.0, 32.0, 40.0, 50.0, 64.0, 80.0, 100.0,
    ];

pub const PREFERRED_DIGITS: [&str; 12] =
    ["8", "1", "125", "16", "20", "25", "32", "40", "50", "64", "80", "100"];

pub const MID_BOUNDARIES: [FloatingType; 11] = [
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

pub const TEN: FloatingType = 10.0;
