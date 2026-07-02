use wasm_bindgen::prelude::*;

use crate::descriptors::Descriptor;
use crate::float64;

#[wasm_bindgen]
#[derive(Clone, Copy, Debug)]
#[repr(transparent)]
pub struct R10c(float64::Descriptor);

#[wasm_bindgen]
impl R10c {
    /// Create a descriptor. Use indexes 0, ..., 9.
    pub fn of(
        is_positive: bool,
        index: usize,
        exponent: isize,
    ) -> Option<Self> {
        float64::Descriptor::of(is_positive, index, exponent).map(|d| Self(d))
    }

    /// Obtain a descriptor for the nearest value in the series that is less
    /// than the input value.
    /// For infinite values, NaN, and zero, `None` is returned.
    /// If the value is too small, `None` is returned.
    pub fn prev(n: f64) -> Option<Self> {
        float64::Descriptor::prev(n).map(|d| Self(d))
    }

    /// Obtain a descriptor for the nearest value in the series to the input
    /// value. For infinite values, NaN, and zero, `None` is returned.
    pub fn near(n: f64) -> Option<Self> {
        float64::Descriptor::near(n).map(|d| Self(d))
    }

    /// Obtain a descriptor for the nearest value in the series that is greater
    /// than the input value.
    /// For infinite values, NaN, and zero, `None` is returned.
    /// If the value is too large, `None` is returned.
    pub fn next(n: f64) -> Option<Self> {
        float64::Descriptor::next(n).map(|d| Self(d))
    }

    /// Resolve the descriptor to a value.
    pub fn resolve(&self) -> f64 {
        self.0.resolve()
    }

    #[wasm_bindgen(getter)]
    /// The significand: a member of the R10c series, between 0.0 and 10.0.
    /// -10.0 and -0.0, depending on the sign of the described value.
    pub fn significand(&self) -> f64 {
        self.0.significand()
    }

    #[wasm_bindgen(getter)]
    /// The exponent -- an integer indicating which power of ten the
    /// significand should be multiplied by.
    pub fn exponent(&self) -> isize {
        self.0.exponent()
    }

    #[wasm_bindgen(getter)]
    /// The index of the significand in the R10c series, an integer in `[0,9]`.
    pub fn index(&self) -> usize {
        self.0.index()
    }

    #[wasm_bindgen(getter)]
    /// Represent the value as a string. This is simple to do without float
    /// formatting logic. We just move the decimal point forward or back in the
    /// according to the decade.
    pub fn text(&self) -> String {
        self.0.text()
    }

    /// Produce a new descriptor that resolves to a value `distance` steps
    /// before or after this descriptor. Stepping may fail if the resulting
    /// value is out of range for the target numeric type.
    ///
    /// Stepping can never change the sign of the described value. A negative
    /// step produces a value of lesser magnitude; a positive step produces a
    /// value of greater magnitude.
    pub fn step(&self, distance: isize) -> Option<R10c> {
        self.0.step(distance).map(|d| Self(d))
    }
}
