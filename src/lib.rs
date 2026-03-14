#![cfg_attr(feature = "f16", feature(f16))]
#![cfg_attr(feature = "f128", feature(f128))]

#[cfg(feature = "const_calculator")]
pub mod const_calculator;
#[cfg(feature = "f16")]
mod f16;
#[cfg(feature = "f32")]
mod f32;
#[cfg(feature = "f64")]
mod f64;
#[cfg(feature = "f128")]
mod f128;

mod sealed {
    pub trait Marker {}
}

pub trait R10c: sealed::Marker {
    fn prev(n: Self) -> Self;
    fn near(n: Self) -> Self;
    fn next(n: Self) -> Self;
}

pub fn next<N: R10c>(n: N) -> N {
    <N as R10c>::next(n)
}

pub fn near<N: R10c>(n: N) -> N {
    <N as R10c>::near(n)
}

pub fn prev<N: R10c>(n: N) -> N {
    <N as R10c>::prev(n)
}
