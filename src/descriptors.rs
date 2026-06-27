/// A descriptor holding value `v` in the R10c series consists of:
///
/// - An `significand`, a value from the series in the range `(0.0,10.0)` or
///   `(-0.0,-10.0)`, depending on the sign of the described value;
/// - An `exponent`, the power of ten that the `significand` is multiplied by
///   to get the value `v` that the descriptor holds.
///
/// Internally, the descriptor is not unlike a floating point format.
///
/// A descriptor has an underlying value type. The significand and
/// exponent always fit within this type.
pub trait Descriptor: Sized {
    type Of: Copy;

    /// Create a descriptor. Use indexes 0, ..., 9.
    fn of(is_positive: bool, index: usize, exponent: isize) -> Option<Self>;

    /// Resolve the descriptor to a value.
    fn resolve(&self) -> Self::Of;

    /// The significand: a member of the R10c series, between 0.0 and 10.0.
    /// -10.0 and -0.0, depending on the sign of the described value.
    fn significand(&self) -> Self::Of;

    /// The exponent -- an integer indicating which power of ten the
    /// significand should be multiplied by.
    fn exponent(&self) -> isize;

    /// The index of the significand in the R10c series, an integer in `[0,9]`.
    fn index(&self) -> usize;

    /// Represent the value as a string. This is simple to do without float
    /// formatting logic. We just move the decimal point forward or back in the
    /// according to the decade.
    fn text(&self) -> String;

    /// Obtain a descriptor for the nearest value in the series that is less
    /// than the input value.
    /// For infinite values, NaN, and zero, `None` is returned.
    /// If the value is too small, `None` is returned.
    fn prev(input: Self::Of) -> Option<Self>;

    /// Obtain a descriptor for the nearest value in the series to the input
    /// value. For infinite values, NaN, and zero, `None` is returned.
    fn near(input: Self::Of) -> Option<Self>;

    /// Obtain a descriptor for the nearest value in the series that is greater
    /// than the input value.
    /// For infinite values, NaN, and zero, `None` is returned.
    /// If the value is too large, `None` is returned.
    fn next(input: Self::Of) -> Option<Self>;

    /// Produce a new descriptor that resolves to a value `distance` steps
    /// before or after this descriptor. Stepping may fail if the resulting
    /// value is out of range for the target numeric type.
    ///
    /// Stepping can never change the sign of the described value. A negative
    /// step produces a value of lesser magnitude; a positive step produces a
    /// value of greater magnitude.
    fn step(&self, distance: isize) -> Option<Self>;
}

pub(crate) mod sealed {
    pub trait Marker {}
}

/// A convenient trait, intended to support the free functions of this module.
pub trait Describable: Copy + sealed::Marker {
    type D: Descriptor<Of = Self>;
    fn prev(self) -> Option<Self::D>;

    fn near(self) -> Option<Self::D>;

    fn next(self) -> Option<Self::D>;
}
