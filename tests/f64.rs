use std::ops::Range;

use proptest::prelude::*;
use r10c;

const TEN: f64 = 10.0;
const PREFERRED: [f64; 12] =
    [0.8, 1.0, 1.25, 1.6, 2.0, 2.5, 3.2, 4.0, 5.0, 6.4, 8.0, 10.0];

fn centered() -> Range<usize> {
    let len = PREFERRED.len() - 1;
    1..len
}

#[test]
fn zero_is_zero() {
    let zero: f64 = 0.0;

    assert_eq!(zero, r10c::prev(zero), "Testing: {zero} == prev({zero})");

    assert_eq!(zero, r10c::near(zero), "Testing: {zero} == near({zero})");

    assert_eq!(zero, r10c::next(zero), "Testing: {zero} == next({zero})");
}

#[test]
fn near_preferred_is_self() {
    for i in centered() {
        let n = PREFERRED[i];
        assert_eq!(n, r10c::near(n), "Testing: {n} == near({n})");
    }
}

#[test]
fn prev_preferred() {
    for i in centered() {
        let other = PREFERRED[i - 1];
        let n = PREFERRED[i];
        assert_eq!(other, r10c::prev(n), "Testing: {other} == prev({n})");
    }
}

#[test]
fn next_preferred() {
    for i in centered() {
        let other = PREFERRED[i + 1];
        let n = PREFERRED[i];
        assert_eq!(other, r10c::next(n), "Testing: {other} == next({n})");
    }
}

fn logspace_sampling(
    min_magnitude: f64,
    max_magnitude: f64,
) -> impl Strategy<Value = f64> {
    let logspace_min = min_magnitude.abs().log10();
    let logspace_max = max_magnitude.abs().log10();
    let is_negative: prop::bool::Any = any::<bool>();

    (logspace_min..logspace_max, is_negative).prop_map(|(e, is_negative)| {
        let sign = if is_negative { -1.0 } else { 1.0 };
        TEN.powf(e) * sign
    })
}

proptest! {
    #[test]
    fn near_between_prev_and_next(n in logspace_sampling(0.01, 100.0)) {
        let prev = r10c::prev(n);
        let near = r10c::near(n);
        let next = r10c::next(n);

        prop_assert!(
            prev <= near && near <= next,
            "The rounded value of {n} must be between {prev} and {next} \
             but is {near}."
        );
    }

    #[test]
    fn prev_not_greater(n in logspace_sampling(0.01, 100.0)) {
        let prev = r10c::prev(n);

        prop_assert!(
            prev <= n,
            "The value previous to {n} must be less than {n} but is {prev}."
        );
    }

    #[test]
    fn next_not_less_than(n in logspace_sampling(0.01, 100.0)) {
        let next = r10c::prev(n);

        prop_assert!(
            next <= n,
            "The value after {n} must be greater than {n} but is {next}."
        );
    }
}
