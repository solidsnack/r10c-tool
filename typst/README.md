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

Three functions are provided:

- `near(float) -> r10c` -- Take a bare Typst number, round it to the nearest
  value in the R10c series, and return a *descriptor* -- an opaque handle to
  that position in the series.
- `step(r10c, int) -> r10c` -- Move forward or backward in the series,
  producing a new descriptor.
- `resolve(r10c) -> float` -- Recover the described value as a bare Typst
  number.

For example:

```typst
#import "@local/r10c:0.1.0"

#let base = r10c.near(4)
#r10c.resolve(base)                // 4.0
#r10c.resolve(r10c.step(base, 1))  // 5.0
#r10c.resolve(r10c.step(base, -1)) // 3.2
```

## Vendoring

This is a small package and can be easily vendored: copy `lib.typ` and
`r10c.wasm` into a `r10c/` folder in your project and import by path:

```typst
#import "r10c/lib.typ" as r10c
```

## License

MIT OR Apache-2.0.
