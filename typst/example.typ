#import "lib.typ": near, next, prev, resolve, step

// Numbers: near returns a descriptor, resolve reads the value back.
#let d = near(26)
near(26) resolves to #resolve(d) \
one step up: #resolve(step(d, 1)) \
one step down: #resolve(step(d, -1))

// prev and next bracket a number instead of rounding to the nearest, so 26
// -- which rounds down to 25 -- is bracketed by 25 and 32.
26 lies between #resolve(prev(26)) and #resolve(next(26))

// Lengths: length in, length out -- a scale of sizes.
#let base = near(4mm)
#text(size: step(base, -1))[small ]
#text(size: base)[base ]
#text(size: step(base, 1))[large]

// Bracketing works on lengths too: 4.3mm lies between 4mm and 5mm.
#text(size: prev(4.3mm))[under ]
#text(size: next(4.3mm))[over]
