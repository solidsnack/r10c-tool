/// Display decimal digits at a certain point place.
pub fn digit_display<S: AsRef<str> + ?Sized>(
    // Digits to display. Decimal points and trailing zeroes will be removed.
    digits: &S,
    // Position in the decimal space. Position -1 means the digits start
    // behind the decimal place (after `0.``); position -2 means they start
    // after `0.0` and so on; the position can be thought of as the exponent.
    position: isize,
    // Whether the number is positive or negative.
    is_positive: bool,
) -> String {
    let trimmed: String = digits
        .as_ref()
        .replace(".", "")
        .trim_end_matches('0')
        .into();
    let start = isize::min(position, 0);
    let end = isize::max(position + 1, trimmed.len() as isize);
    let nonzero = 0..(trimmed.len() as isize);

    let mut accum = String::new();

    if !is_positive {
        accum.push_str("-");
    }

    for i in start..end {
        let digit = if nonzero.contains(&i) {
            &trimmed[(i as usize)..((i + 1) as usize)]
        } else {
            "0"
        };

        if i == position + 1 {
            accum.push_str(".");
        }

        accum.push_str(digit);
    }

    accum
}
