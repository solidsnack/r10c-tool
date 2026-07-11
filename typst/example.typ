#import "lib.typ": near, step, resolve

#let d = near(26)
near(26) resolves to #resolve(d) \
one step up: #resolve(step(d, 1)) \
one step down: #resolve(step(d, -1))
