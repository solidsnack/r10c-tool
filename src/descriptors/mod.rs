use bitfield_struct::bitfield;

pub mod f64;
pub use f64::*;

/// A descriptor holding value `v` in the R10c series consists of:
///
/// - An `significand`, a value from the series in the range `(0.0,10.0)` or
///   `(-0.0,-10.0)`, depending on the sign of the described value;
/// - An `exponent`, the power of ten that the `significand` is multiplied by
///   to get the value `v` that the descriptor holds.
///
/// The descriptor is not unlike a floating point format.
///
/// A descriptor has an underlying value type. The significand and
/// exponent always fit within this type.
pub trait Descriptor: Sized {
    type Of;

    /// The significand: a member of the R10c series, between 0.0 and 10.0.
    /// -10.0 and -0.0, depending on the sign of the described value.
    fn significand(&self) -> Self::Of;

    /// The exponent as an integer power of ten.
    fn exponent(&self) -> usize;

    /// The index of the significand.
    fn index(&self) -> isize;

    /// Obtain a descriptor for the nearest value in the series that is less
    /// than the input value.
    fn prev(input: Self::Of) -> Option<Self>;

    /// Obtain a descriptor for the nearest value in the series to the input
    /// value.
    fn near(input: Self::Of) -> Self;

    /// Obtain a descriptor for the nearest value in the series that is greater
    /// than the input value.
    fn next(input: Self::Of) -> Option<Self>;

    /// Resolve the descriptor to a value.
    fn resolve(&self) -> Self::Of;

    /// Produce a new descriptor that resolves to a value `distance` steps
    /// before or after this descriptor. Stepping may fail if we have reached
    /// the maximum or minimum value of the descriptor type.
    ///
    /// Stepping can never change the sign of the described value.
    fn step(&self, distance: isize) -> Option<Self>;
}

/* IEEE floating point formats.

      sign exponent significand
f16:  1     5        10
f32:  1     8        23
f64:  1    11        52
f128: 1    15       112
f256: 1    19       236

The R10c significand is an index in the range [0,9] so it only ever requires
4 bits.

If we store a sign with the significand, we need 5 bits.

- A `u16` has 11 bits left, so we can accommodate the exponent of `f16`, `f32`
  and `f64`.
- A `u32` has 27 bits left, se we can accommodate values up to `f256`.

*/

#[bitfield(u16)]
struct U16 {
    #[bits(1)]
    sign: bool,
    #[bits(4)]
    index: usize,
    #[bits(11)]
    exponent: usize,
}

#[bitfield(u32)]
struct U32 {
    #[bits(1)]
    sign: bool,
    #[bits(4)]
    index: usize,
    #[bits(27)]
    exponent: usize,
}
