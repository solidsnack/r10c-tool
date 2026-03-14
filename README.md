# R10c

Implements a kind of rounding, where a number is rounded to the nearest
in a series of preferred numbers that grows geometrically, completing one
cycle for every power of ten:

    ...1, 1.25, 1.6, 2, 2.5, 3.2, 4, 5, 6.4, 8, 10, 12.5, 16, 20...

For example, 26 would be rounded to 25 while 30 would be rounded to 32.
