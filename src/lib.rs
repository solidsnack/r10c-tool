#![cfg_attr(feature = "f16", feature(f16))]
#![cfg_attr(feature = "f128", feature(f128))]

pub mod bits;
#[cfg(feature = "const_calculator")]
pub mod const_calculator;
pub mod descriptors;
pub mod digit_display;
#[cfg(feature = "f128")]
pub mod float128;
#[cfg(feature = "f16")]
pub mod float16;
#[cfg(feature = "f32")]
pub mod float32;
#[cfg(feature = "f64")]
pub mod float64;
#[macro_use]
mod sign;
#[cfg(feature = "typst")]
mod typst;
#[cfg(feature = "wasm")]
mod wasm;

/// Obtain a descriptor for the nearest value in the series that is less
/// than the input value.
/// For infinite values, NaN, and zero, `None` is returned.
/// If the value is too small, `None` is returned.
pub fn prev<N: descriptors::Describable>(n: N) -> Option<N::D> {
    <N as descriptors::Describable>::prev(n)
}

/// Obtain a descriptor for the nearest value in the series to the input
/// value. For infinite values, NaN, and zero, `None` is returned.
pub fn near<N: descriptors::Describable>(n: N) -> Option<N::D> {
    <N as descriptors::Describable>::near(n)
}

/// Obtain a descriptor for the nearest value in the series that is greater
/// than the input value.
/// For infinite values, NaN, and zero, `None` is returned.
/// If the value is too large, `None` is returned.
pub fn next<N: descriptors::Describable>(n: N) -> Option<N::D> {
    <N as descriptors::Describable>::next(n)
}
