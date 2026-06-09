//! Bitfield structs for representing numbers in R10c.  R10c is kind of like a
//! low-precision floating point format (small significand), and a large range
//! of values can be represented in a small number of bits. The R10c
//! significand can be stored as an index in the range [0,9] so it only ever
//! requires 4 bits. With the sign as 1 bit, that leaves 11 bits for the
//! decimal exponent in a `u16` (more than are need for `f32`) and 27 in a
//! `u32` (more than are needed for IEEE `f256`).
use bitfield_struct::bitfield;

/* IEEE floating point formats.

      sign exponent significand
f16:  1     5        10
f32:  1     8        23
f64:  1    11        52
f128: 1    15       112
f256: 1    19       236

As mentioned earlier, if we store a sign with the significand, we need 5 bits.

- A `u16` has 11 bits left, so we can accommodate the exponent of `f16`, `f32`
  and `f64`.
- A `u32` has 27 bits left, se we can accommodate values up to `f256`.

The IEEE exponent is a binary exponent -- the significand is multiplied by
`2^exponent` -- but for R10c, we store a decimal exponent. We need somewhat
less space. For example, the largest / smallest decimal exponent value for
`f64` is ±308, so we need 9.3 bits, not 11.

A fairly compact way to encode an R10c value is as a decimal number, with the
least signficant digit representing the index into the series and the remaining
digits representing the decimal exponent. However, it isn't really needed.

*/

#[bitfield(u16)]
pub struct Lens16 {
    #[bits(1)]
    pub positive: bool,
    /// The index into the R10c series, in the range `[1,10]`.
    #[bits(4)]
    pub index: usize,
    #[bits(11)]
    pub exponent: isize,
}

#[bitfield(u32)]
pub struct Lens32 {
    #[bits(1)]
    pub positive: bool,
    /// The index into the R10c series, in the range `[1,10]`.
    #[bits(4)]
    pub index: usize,
    #[bits(27)]
    pub exponent: isize,
}
