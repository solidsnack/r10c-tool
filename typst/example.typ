#import "lib.typ": near, step, resolve

// Numbers: near returns a descriptor, resolve reads the value back.
#let d = near(26)
near(26) resolves to #resolve(d) \
one step up: #resolve(step(d, 1)) \
one step down: #resolve(step(d, -1))

// Lengths: length in, length out -- a scale of sizes.
#let base = near(4mm)
#text(size: step(base, -1))[small ]
#text(size: base)[base ]
#text(size: step(base, 1))[large]
