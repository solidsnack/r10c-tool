// Compile-only test: `prev` and `next` in both the number and length domains.
// The assertions are the test -- there is nothing to compare against a
// reference page, so a failure surfaces as a compile error.

#import "../../lib.typ": near, next, prev, resolve, step

#let close(a, b) = calc.abs(a - b) < 1e-9

#let eq(actual, expected, what) = assert(
    if expected == none { actual == none } else { close(actual, expected) },
    message: what + ": expected " + repr(expected) + ", got " + repr(actual),
)

// Bracketing a number that is not in the series. 26 rounds down to 25.
#eq(resolve(near(26)), 25.0, "near(26)")
#eq(resolve(prev(26)), 25.0, "prev(26)")
#eq(resolve(next(26)), 32.0, "next(26)")

// Bracketing a number that IS in the series: strictly below and above.
#eq(resolve(near(4)), 4.0, "near(4)")
#eq(resolve(prev(4)), 3.2, "prev(4)")
#eq(resolve(next(4)), 5.0, "next(4)")

// Decade crossings.
#eq(resolve(prev(1)), 0.8, "prev(1)")
#eq(resolve(next(8)), 10.0, "next(8)")

// Negatives move by magnitude, as `step` does.
#eq(resolve(prev(-26)), -25.0, "prev(-26)")
#eq(resolve(next(-26)), -32.0, "next(-26)")
#eq(resolve(step(near(-26), -1)), -20.0, "step(near(-26), -1)")

// On the series, prev/next agree with a single step.
#eq(resolve(prev(4)), resolve(step(near(4), -1)), "prev(4) vs step(-1)")
#eq(resolve(next(4)), resolve(step(near(4), 1)), "next(4) vs step(+1)")

// Non-representable input yields `none`, as with `near`.
#eq(near(0), none, "near(0)")
#eq(prev(0), none, "prev(0)")
#eq(next(0), none, "next(0)")
#eq(prev(float.nan), none, "prev(nan)")
#eq(next(float.inf), none, "next(inf)")

// Lengths: length in, length out, rounded via millimetres.
#eq(prev(4.3mm).mm(), 4.0, "prev(4.3mm)")
#eq(next(4.3mm).mm(), 5.0, "next(4.3mm)")
#eq(prev(4mm).mm(), 3.2, "prev(4mm)")
#eq(next(4mm).mm(), 5.0, "next(4mm)")
#eq(prev(0mm), none, "prev(0mm)")
#eq(next(1cm).mm(), 12.5, "next(1cm)")

// Descriptors stay opaque: a number in, a handle out.
#assert.eq(
    type(prev(26)),
    dictionary,
    message: "prev should return an opaque descriptor",
)
