// r10c -- round numbers to R10c preferred numbers.
//
// Numbers in the series are handled as opaque *descriptors*. `near` maps a
// number to the nearest descriptor, `step` moves along the series, and
// `resolve` turns a descriptor back into a number. `near` and `step` return
// `none` when there is no representable result (e.g. zero, NaN, infinity, or
// a value beyond the series bounds).

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

// Nearest descriptor to `n`, or `none` if `n` has no R10c representation.
#let near(n) = _wrap(_call((Near: float(n))))

// Descriptor `distance` steps along the series (negative = smaller
// magnitude), or `none` if the result is out of range.
#let step(descriptor, distance) = _wrap(
    _call((Step: (_bits(descriptor), distance))),
)

// Resolve a descriptor to its number.
#let resolve(descriptor) = _call((Resolve: _bits(descriptor)))
