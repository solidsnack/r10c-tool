use std::ops::{Range, RangeInclusive};
use std::str::FromStr;

use assert2::{assert, check};
use proptest::prelude::*;
use r10c::{self, descriptors::Descriptor, float64};
use regex::Regex;

const TEN: f64 = 10.0;
// NB: These numbers are all exactly representable in floating point, whereas
//     many of those in earlier decades (for example, 0.8) are not.
const PREFERRED: [f64; 12] =
    // [0.8, 1.0, 1.25, 1.6, 2.0, 2.5, 3.2, 4.0, 5.0, 6.4, 8.0, 10.0];
    [
        8.0, 10.0, 12.5, 16.0, 20.0, 25.0, 32.0, 40.0, 50.0, 64.0, 80.0, 100.0,
    ];

fn centered() -> Range<usize> {
    let len = PREFERRED.len() - 1;
    1..len
}

#[test]
fn near_preferred_is_self() {
    for i in centered() {
        let n = PREFERRED[i];

        assert!(
            let Some(d) = r10c::near(n),
            "The R10c value {n} should resolve."
        );

        check!(
            d.resolve() == n,
            "The R10c value {n} should resolve to itself: {n} == near({n})"
        );

        let text = d.text();

        assert!(
            let Ok(parsed) = f64::from_str(&text),
            "The descriptor's text display ({text}) should parse to a number"
        );

        check!(
            parsed == n,
            // parsed == n && false,
            "The descriptor's text display ({text}) should parse to: {n}"
        );
    }
}

#[test]
fn prev_preferred() {
    for i in centered() {
        let other = PREFERRED[i - 1];
        let n = PREFERRED[i];

        assert!(
            let Some(d) = r10c::prev(n),
            "The R10c value {n} should have a previous value."
        );

        check!(
            d.resolve() == other,
            "The previous value in the series should be resolved with \
             `prev`: {other} == prev({n})"
        );
    }
}

#[test]
fn next_preferred() {
    for i in centered() {
        let other = PREFERRED[i + 1];
        let n = PREFERRED[i];

        assert!(
            let Some(d) = r10c::next(n),
            "The R10c value {n} should have a next value."
        );

        check!(
            d.resolve() == other,
            "The next value in the series should be resolved with \
             `next`: {other} == next({n})"
        );
    }
}

#[test]
fn inner_decades_text_roundtrip() {
    for exponent in -2..=2 {
        for index in 0..=9 {
            assert!(
                let Some(d) = float64::Descriptor::of(true, index, exponent),
                "It should be possible to create a descriptor for: \
                 {exponent}:{index}",
            );

            let float = d.resolve();
            let text = d.text();

            assert!(
                let Ok(parsed) = f64::from_str(&text),
                "The text display ({text}) of the descriptor for \
                 {exponent}:{index} should parse to a number but failed."
            );

            check!(
                float == parsed,
                "The text display ({text}) of the descriptor for \
                 {exponent}:{index} should parse to the float form: \
                 {float} == {parsed}"
            );
        }
    }
}

#[test]
fn inner_decades_leading_decimal_point() {
    assert!(
        let Ok(leading_decimal_point) = Regex::new("^[.]"),
        "Can't parse test regex!",
    );

    for exponent in -2..=2 {
        for index in 0..=9 {
            assert!(
                let Some(d) = float64::Descriptor::of(true, index, exponent),
                "It should be possible to create a descriptor for: \
                 {exponent}:{index}",
            );

            let text = d.text();

            check!(
                !leading_decimal_point.is_match(&text),
                "The text display ({text}) of the descriptor for \
                 {exponent}:{index} should not start with a decimal point.",
            );
        }
    }
}

#[test]
fn inner_decades_trailing_zero() {
    assert!(
        let Ok(trailing_zeros) = Regex::new("[.].*0+$"),
        "Can't parse test regex!",
    );

    for exponent in -2..=2 {
        for index in 0..=9 {
            assert!(
                let Some(d) = float64::Descriptor::of(true, index, exponent),
                "It should be possible to create a descriptor for: \
                 {exponent}:{index}",
            );

            let text = d.text();

            check!(
                !trailing_zeros.is_match(&text),
                "The text display ({text}) of the descriptor for \
                 {exponent}:{index} should omit trailing zeros.",
            );
        }
    }
}

#[test]
fn inner_decades_trailing_decimal_point() {
    assert!(
        let Ok(trailing_decimal) = Regex::new("[.]$"),
        "Can't parse test regex!",
    );

    for exponent in -2..=2 {
        for index in 0..=9 {
            assert!(
                let Some(d) = float64::Descriptor::of(true, index, exponent),
                "It should be possible to create a descriptor for: \
                 {exponent}:{index}",
            );

            let text = d.text();

            check!(
                !trailing_decimal.is_match(&text),
                "The text display ({text}) of the descriptor for \
                 {exponent}:{index} should omit a trailing decimal point.",
            );
        }
    }
}

/*
   The idea behind this test:
   * We can always format an R10c descriptor as text correctly without making
     any use of floating point. We use the decimal exponent to decide how many
     places before or after to put the digits of the value and then zero fill
     as needed.
   * This text string is a parseable float.
   * We parse the float.
   * The floating point calculations in `.resolve()` should result in this
     float.
   * The range in which this test passes is governed by the nature of the
     algorithm in `.resolve()`. If the algorithm involves more than one
     rounding step, eventually we won't be able to get the parsed float and
     the calculated float to match up.
*/
fn test_range_roundtrip(range: RangeInclusive<isize>) {
    for i in range {
        let (index, exponent) = ((i.abs() as usize) % 10, i / 10);

        assert!(
            let Some(d) = float64::Descriptor::of(true, index, exponent),
            "It should be possible to create a descriptor for: \
                {exponent}:{index}",
        );

        let float = d.resolve();
        let text = d.text();

        assert!(
            let Ok(parsed) = f64::from_str(&text),
            "The text display ({text}) of the descriptor for \
                {exponent}:{index} should parse to a number but failed."
        );

        check!(
            float == parsed,
            "The text display ({text}) of the descriptor for \
                {exponent}:{index} should parse to the float form: \
                {float} == {parsed}"
        );
    }
}

#[test]
fn mid_portion_text_roundtrip() {
    use float64::constants::bounds::*;

    let range = (RANGE.start() / 16)..=(RANGE.end() / 16);

    test_range_roundtrip(range);
}

// #[test]
// fn mid_half_text_roundtrip() {
//     use float64::constants::bounds::*;

//     let range = (RANGE.start() / 2)..=(RANGE.end() / 2);

//     test_range_roundtrip(range);
// }

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
        match (r10c::prev(n), r10c::near(n), r10c::next(n)) {
            (Some(prev), Some(near), Some(next)) => {
                let mid = near.resolve();
                let (lesser, greater) = if n.is_sign_negative() {
                    (next.resolve(), prev.resolve())
                } else {
                    (prev.resolve(), next.resolve())
                };

                prop_assert!(
                    lesser <= mid && mid <= greater,
                    "The rounded value of {n} must be between {lesser} and \
                     {greater} but is {mid}."
                );
            }
            _ => { /* Do nothing. */ }
        }
    }

    #[test]
    fn prev_not_greater(n in logspace_sampling(0.01, 100.0)) {
        if let Some(prev) = r10c::prev(n).map(|d| d.resolve()) {
            if n.is_sign_negative() {
                prop_assert!(
                    n <= prev,
                    "The value before {n} must be greater than {n} but is \
                     {prev}."
                );
            } else {
                prop_assert!(
                    prev <= n,
                    "The value before {n} must be less than {n} but is {prev}."
                );
            }
        }
    }

    #[test]
    fn next_not_less_than(n in logspace_sampling(0.01, 100.0)) {
        if let Some(next) = r10c::next(n).map(|d| d.resolve()) {
            if n.is_sign_negative() {
                prop_assert!(
                    next <= n,
                    "The value after {n} must be less than {n} but is {next}."
                );
            } else {
                prop_assert!(
                    n <= next,
                    "The value after {n} must be greater than {n} but is \
                     {next}."
                );
            }
        }
    }

    #[test]
    fn near_between_prev_and_next_full_range(
        n in logspace_sampling(f64::MIN_POSITIVE, f64::MAX)
    ) {
        match (r10c::prev(n), r10c::near(n), r10c::next(n)) {
            (Some(prev), Some(near), Some(next)) => {
                let mid = near.resolve();
                let (lesser, greater) = if n.is_sign_negative() {
                    (next.resolve(), prev.resolve())
                } else {
                    (prev.resolve(), next.resolve())
                };

                prop_assert!(
                    lesser <= mid && mid <= greater,
                    "The rounded value of {n} must be between {lesser} and \
                     {greater} but is {mid}."
                );
            }
            _ => { /* Do nothing. */ }
        }
    }

    #[test]
    fn prev_not_greater_full_range(
        n in logspace_sampling(f64::MIN_POSITIVE, f64::MAX)
    ) {
        if let Some(prev) = r10c::prev(n).map(|d| d.resolve()) {
            if n.is_sign_negative() {
                prop_assert!(
                    n <= prev,
                    "The value before {n} must be greater than {n} but is \
                     {prev}."
                );
            } else {
                prop_assert!(
                    prev <= n,
                    "The value before {n} must be less than {n} but is {prev}."
                );
            }
        }
    }

    #[test]
    fn next_not_less_than_full_range(
        n in logspace_sampling(f64::MIN_POSITIVE, f64::MAX)
    ) {
        if let Some(next) = r10c::next(n).map(|d| d.resolve()) {
            if n.is_sign_negative() {
                prop_assert!(
                    next <= n,
                    "The value after {n} must be less than {n} but is {next}."
                );
            } else {
                prop_assert!(
                    n <= next,
                    "The value after {n} must be greater than {n} but is \
                     {next}."
                );
            }
        }
    }
}
