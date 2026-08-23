// r10c -- round numbers to R10c preferred numbers.
//
// Numbers in the series are handled as opaque *descriptors*: `near`, `prev`
// and `next` map a number to a descriptor -- the nearest position in the
// series, or the nearest one strictly below or above it in magnitude --
// `step` moves along the series, and `resolve` turns a descriptor back into a
// number. Each returns `none` when there is no representable result (e.g.
// zero, NaN, infinity, or a value beyond the series bounds).
//
// All five functions also accept a `length`. A length is rounded via its
// millimetre magnitude (`length.mm()`) and the result is handed back as a
// length in millimetres (`... * 1mm`) -- never a descriptor -- so length in,
// length out. (Lengths carrying `em` units are rejected by Typst's `.mm()`.)

#let _plugin = plugin("r10c.wasm")

// The plugin speaks one CBOR `request` endpoint whose payload is an
// externally-tagged enum. Build the matching dictionary, hand it over as
// CBOR, and decode the CBOR reply.
#let _call(request) = cbor(_plugin.request(cbor.encode(request)))

// Wrap plugin output (descriptor bits, or `none`) as an opaque descriptor.
#let _wrap(bits) = if bits == none { none } else { (descriptor: bits) }

// Unwrap a descriptor, asserting the shape for a clear error on misuse.
#let _bits(d) = {
    assert(
        type(d) == dictionary and "descriptor" in d,
        message: "expected an r10c descriptor from `near` or `step`",
    )
    d.descriptor
}

// Descriptor-domain primitives (the raw plugin calls).
#let _near(n) = _wrap(_call((Near: float(n))))
#let _prev(n) = _wrap(_call((Prev: float(n))))
#let _next(n) = _wrap(_call((Next: float(n))))
#let _step(descriptor, distance) = _wrap(
    _call((Step: (_bits(descriptor), distance))),
)
#let _resolve(descriptor) = _call((Resolve: _bits(descriptor)))

// Resolve a descriptor, or pass `none` through.
#let _value(descriptor) = if descriptor == none {
    none
} else {
    _resolve(descriptor)
}

// Float-domain composites, propagating `none`: the R10c value at, below and
// above `x`, and the value `distance` steps from the position nearest `x`.
#let _near_value(x) = _value(_near(x))
#let _prev_value(x) = _value(_prev(x))
#let _next_value(x) = _value(_next(x))
#let _step_value(x, distance) = {
    let d = _near(x)
    if d == none { none } else { _value(_step(d, distance)) }
}

// Re-express a float value (or `none`) as a length in millimetres.
#let _mm(value) = if value == none { none } else { value * 1mm }

// Nearest descriptor to the number `n` -- or, given a length, the nearest
// R10c value as a length in millimetres. `none` if not representable.
#let near(n) = if type(n) == length {
    _mm(_near_value(n.mm()))
} else {
    _near(n)
}

// Descriptor for the nearest position strictly below `n` in magnitude -- or,
// given a length, that value as a length in millimetres. `none` if there is
// none, or if `n` is not representable.
#let prev(n) = if type(n) == length {
    _mm(_prev_value(n.mm()))
} else {
    _prev(n)
}

// Descriptor for the nearest position strictly above `n` in magnitude -- or,
// given a length, that value as a length in millimetres. `none` if there is
// none, or if `n` is not representable.
#let next(n) = if type(n) == length {
    _mm(_next_value(n.mm()))
} else {
    _next(n)
}

// Descriptor `distance` steps along the series from `x` -- or, given a
// length, that value as a length in millimetres. Negative is toward smaller
// magnitude; `none` if out of range.
#let step(x, distance) = if type(x) == length {
    _mm(_step_value(x.mm(), distance))
} else {
    _step(x, distance)
}

// Resolve a descriptor to its number -- or, given a length, snap it to the
// nearest R10c value and return that as a length in millimetres.
#let resolve(x) = if type(x) == length {
    _mm(_near_value(x.mm()))
} else {
    _resolve(x)
}
