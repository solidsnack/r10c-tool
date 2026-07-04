// Cross-runtime tests for r10c.js -- runs under Deno or Node, no dependencies.
//
// r10c.js is intentionally export-free (browser-console friendly), so we read
// its source and evaluate it rather than importing it. Expected values below
// were cross-checked against the Rust reference implementation.

const isDeno = typeof Deno !== "undefined"

async function loadR10c() {
  const path = `${import.meta.dirname}/r10c.js`
  let source
  if (isDeno) {
    source = Deno.readTextFileSync(path)
  } else {
    const { readFileSync } = await import("node:fs")
    source = readFileSync(path, "utf8")
  }
  return new Function(`${source}\nreturn R10c`)()
}

const R10c = await loadR10c()

let failures = 0

// Exact for null; relative tolerance for floats (near/prev/next accumulate a
// little rounding once a decade multiplier is involved, e.g. 0.64).
function eq(a, b) {
  if (a === null || b === null) return a === b
  if (Number.isNaN(a) && Number.isNaN(b)) return true
  return Math.abs(a - b) <= 1e-9 * Math.max(1, Math.abs(a), Math.abs(b))
}

function check(op, input, expected) {
  let actual
  try {
    actual = R10c[op](input)
  } catch (e) {
    console.log(`FAIL  ${op}(${input}): threw ${e}`)
    failures++
    return
  }
  const ok = eq(actual, expected)
  console.log(`${ok ? "ok  " : "FAIL"}  ${op}(${input}): got ${actual}, want ${expected}`)
  if (!ok) failures++
}

// [op, input, expected]
const cases = [
  // positives
  ["near", 26, 25], ["prev", 26, 25], ["next", 26, 32],
  ["near", 3.7, 4], ["prev", 3.7, 3.2], ["next", 3.7, 4],
  ["near", 100, 100], ["prev", 100, 80], ["next", 100, 125],
  ["near", 0.9, 1], ["prev", 0.5, 0.4], ["next", 0.5, 0.64],
  ["near", 1, 1], ["prev", 1, 0.8], ["next", 1, 1.25],
  // negatives are magnitude-based, matching the Rust reference
  ["near", -26, -25], ["prev", -26, -25], ["next", -26, -32],
  ["near", -3.7, -4], ["prev", -3.7, -3.2], ["next", -3.7, -4],
  ["prev", -100, -80], ["next", -100, -125],
  // non-resolvable inputs -> null (zero, NaN, +/-Infinity, subnormal)
  ["near", 0, null], ["prev", 0, null], ["next", 0, null],
  ["near", NaN, null], ["near", Infinity, null], ["near", -Infinity, null],
  ["next", Infinity, null], ["prev", -Infinity, null],
  ["near", 1e-320, null],
]

for (const [op, input, expected] of cases) check(op, input, expected)

console.log(`\n${cases.length - failures}/${cases.length} passed`)
if (failures > 0) {
  console.error(`${failures} test(s) failed`)
  if (isDeno) Deno.exit(1)
  else process.exit(1)
}
