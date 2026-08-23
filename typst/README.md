# r10c

Typst package for scaling sizes according to the R10c series of preferred
numbers.

The R10c series, a variation of the Renard 10 Series, takes ten logarithmic
steps for every power of ten, with intermediate steps that are pleasing ratios:
powers of two, approximations of the golden section, &c. The first decade is 1,
1.25, 1.6, 2, 2.5, 3.2, 4, 5, 6.4, 8 and then we hit 10.

This allows a document to declare a "base" size and then operate according to
a "scale" of sizes by stepping up and down in the series.

## API

Five functions are provided:

- `near(float) -> r10c` -- Take a bare Typst number, round it to the nearest
  value in the R10c series, and return a *descriptor* -- an opaque handle to
  that position in the series.
- `prev(float) -> r10c` -- As `near`, but for the nearest position *strictly
  below* the number in magnitude.
- `next(float) -> r10c` -- As `near`, but for the nearest position *strictly
  above* the number in magnitude.
- `step(r10c, int) -> r10c` -- Move forward or backward in the series,
  producing a new descriptor.
- `resolve(r10c) -> float` -- Recover the described value as a bare Typst
  number.

Any of them may return `none`, when there is no value to give back: for zero,
NaN and infinity, and at the ends of the series.

For example:

```typst
#import "@local/r10c:0.1.0"

#let base = r10c.near(4)
#r10c.resolve(base)                // 4.0
#r10c.resolve(r10c.step(base, 1))  // 5.0
#r10c.resolve(r10c.step(base, -1)) // 3.2
```

Where `near` rounds to whichever side is closer, `prev` and `next` bracket the
number, so a value between two members of the series is caught either way:

```typst
#r10c.resolve(r10c.near(26))  // 25.0 -- 26 is nearer 25 than 32
#r10c.resolve(r10c.prev(26))  // 25.0
#r10c.resolve(r10c.next(26))  // 32.0
```

They are strict, so a number already in the series moves off it, just as a
single `step` would:

```typst
#r10c.resolve(r10c.prev(4))   // 3.2
#r10c.resolve(r10c.next(4))   // 5.0
```

For negative numbers, "below" and "above" are read as magnitudes -- `prev`
moves toward zero and `next` away from it, the same convention `step` follows.

### Lengths

Each function also accepts a `length` in place of a number or descriptor. The
length is rounded by its millimetre magnitude and the result is handed back as
a length in millimetres. For example:

```typst
#import "@local/r10c:0.1.0"

#let base = r10c.near(4mm)   // 4mm
#r10c.step(base, 1)          // 5mm
#r10c.step(base, -1)         // 3.2mm
#r10c.prev(4.3mm)            // 4mm
#r10c.next(4.3mm)            // 5mm
```

`resolve` accepts a length too, snapping it to the nearest series value. Note
that a length carrying `em` units cannot be rounded (Typst's `.mm()` rejects
it).

## Vendoring

This is a small package and can be easily vendored: copy `lib.typ` and
`r10c.wasm` into a `r10c/` folder in your project and import by path:

```typst
#import "r10c/lib.typ" as r10c
```

## License

MIT OR Apache-2.0.
