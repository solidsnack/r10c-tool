#![cfg(feature = "f16")]
#![cfg_attr(feature = "f16", feature(f16))]

use std::assert_matches;
use std::ops::Range;

use r10c;

const TEN: f16 = 10.0;
// NB: These numbers are all exactly representable in floating point, whereas
//     many of those in earlier decades (for example, 0.8) are not.
const PREFERRED: [f16; 12] =
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

        assert_matches!(
            r10c::near(n),
            Some(d) if d.resolve() == n,
            "Testing: {n} == near({n})"
        );
    }
}

#[test]
fn prev_preferred() {
    for i in centered() {
        let other = PREFERRED[i - 1];
        let n = PREFERRED[i];

        assert_matches!(
            r10c::prev(n),
            Some(d) if d.resolve() == other,
            "Testing: {other} == prev({n})"
        );
    }
}

#[test]
fn next_preferred() {
    for i in centered() {
        let other = PREFERRED[i + 1];
        let n = PREFERRED[i];

        assert_matches!(
            r10c::next(n),
            Some(d) if d.resolve() == other,
            "Testing: {other} == next({n})"
        );
    }
}

#[test]
fn inner_decades_text_roundtrip() {
    for exponent in -2..=2 {
        for index in 0..=9 {
            assert!(
                let Some(d) = float32::Descriptor::of(true, index, exponent),
                "It should be possible to create a descriptor for: \
                 {exponent}:{index}",
            );

            let float = d.resolve();
            let text = d.text();

            assert!(
                let Ok(parsed) = f32::from_str(&text),
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
                let Some(d) = float32::Descriptor::of(true, index, exponent),
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
                let Some(d) = float32::Descriptor::of(true, index, exponent),
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
                let Some(d) = float32::Descriptor::of(true, index, exponent),
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

#[cfg(feature = "exhaustive")]
mod exhaustive {
    use assert2::{assert, check};

    // let band: f16 = f16::sqrt(1.25);
    const BAND: f16 = 1.118033988749895;

    #[test]
    fn complete_coverage() {
        for i in 0..=u16::MAX {
            let n = f16::from_bits(i);
            let in_r10c = r10c::near(n);

            if n.is_infinite() || n.is_nan() || n == 0.0 {
                assert!(
                    let None = in_r10c,
                    "This should be unresolvable: {n}"
                );
            }

            if n.is_normal() {
                assert!(
                    let Some(_) = in_r10c,
                    "Normal number failure: {n}"
                );
            }

            if n.is_subnormal() {
                assert!(
                    let Some(_) = in_r10c,
                    "Subnormal number failure: {n}"
                );
            }
        }
    }

    #[test]
    fn normal_nearness() {
        for i in 0..=u16::MAX {
            let n = f16::from_bits(i);

            if !n.is_normal() {
                continue;
            }

            assert!(
                let Some(d) = r10c::near(n),
                "Nearest value should be resolvable for: {n}"
            );

            let r = d.resolve();
            let (lower, upper) = (r / BAND, r * BAND);
            let neighbors = (r10c::prev(n), r10c::next(n));

            let (index, exponent) = (d.index(), d.exponent());

            let desc = if let (Some(prev), Some(next)) = neighbors {
                let (pn, nn) = (prev.resolve(), next.resolve());
                let (pnp, pnn, nnp, nnn) = (
                    pn.next_down(),
                    pn.next_up(),
                    nn.next_down(),
                    nn.next_up(),
                );
                format!(
                    "The value {n} should be within a band centered on \
                         {r}: {lower} < {r} (index: {index}, exponent: {exponent}) < {upper}\n\
                         The near value {r} should be geometrically closer to \
                         the input {n} than previous or next: \
                         {pnp} {pn} {pnn} < {r} < {nnp} {nn} {nnn}"
                )
            } else {
                format!(
                    "The value {n} should be within a band centered on \
                         {r}: {lower} < {r} < {upper}"
                )
            };

            check!(n >= lower, "{desc}");
            check!(n <= upper, "{desc}");
        }
    }
}
