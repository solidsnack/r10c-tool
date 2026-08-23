class R10c {
  static MIN_NORMAL = 2.2250738585072014e-308

  static near(n) {
    const abs = Math.abs(n)

    // Check that number is not subnormal, NaN, zero, &c.
    if (!(abs >= R10c.MIN_NORMAL && abs <= Number.MAX_VALUE)) return null

    const preferred = [
      0.8, 1.0, 1.25, 1.6, 2.0, 2.5,
      3.2, 4.0, 5.0, 6.4, 8.0, 10.0,
    ]
    // const midBoundaries = preferred
    //   .slice(0, -1)
    //   .map((_, i) =>
    //     (Math.log10(preferred[i]) + Math.log10(preferred[i + 1])) / 2
    //   )
    const midBoundaries = [
     -0.048455006504028,
      0.048455006504028,
      0.150514997831991,
      0.252574989159953,
      0.349485002168009,
      0.451544993495972,
      0.553604984823934,
      0.650514997831991,
      0.752574989159953,
      0.854634980487915,
      0.951544993495972,
    ]

    const sign = Math.sign(n)
    const log10 = Math.log10(abs)
    const decade = Math.floor(log10)
    const locator = log10 - decade

    let left = 0
    let right = midBoundaries.length
    while (left < right) {
      const mid = Math.floor((left + right) / 2)
      if (midBoundaries[mid] < locator) {
        left = mid + 1
      } else {
        right = mid
      }
    }

    return preferred[left] * Math.pow(10, decade) * sign
  }

  static prev(n) {
    const abs = Math.abs(n)

    // Check that number is not subnormal, NaN, zero, &c.
    if (!(abs >= R10c.MIN_NORMAL && abs <= Number.MAX_VALUE)) return null

    if (n < 0) return -R10c.prev(abs)

    const m = R10c.near(n)

    if (m === null) return m

    return m < n ? m : R10c.near(n * 0.8)
  }

  static next(n) {
    const abs = Math.abs(n)

    // Check that number is not subnormal, NaN, zero, &c.
    if (!(abs >= R10c.MIN_NORMAL && abs <= Number.MAX_VALUE)) return null

    if (n < 0) return -R10c.next(abs)

    const m = R10c.near(n)

    if (m === null) return m

    return m > n ? m : R10c.near(n / 0.8)
  }
}
