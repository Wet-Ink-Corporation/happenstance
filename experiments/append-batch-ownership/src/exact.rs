//! An exact `f64` → `i64` for the integers a JS number can hold.
//!
//! Workers SQL hands every integer column back as a JS number. The adapter
//! narrows it with a scoped `#[allow]` and an `as` cast
//! (`sql_storage.rs`, `safe_integer`); this crate may do neither, so it reads
//! the IEEE-754 fields instead. No allocation, so the replica arms' counts are
//! unaffected by the difference in spelling.

/// The integer `number` holds, if it holds one in the JS safe range
/// (`|x| < 2^53`); `None` for a fraction, a value at or above `2^53`, `NaN` or
/// an infinity.
#[must_use]
pub fn safe_integer(number: f64) -> Option<i64> {
    const FRACTION_BITS: u32 = 52;
    const BIAS: u64 = 1023;
    let bits = number.to_bits();
    let negative = bits >> 63 == 1;
    let exponent = (bits >> FRACTION_BITS) & 0x7ff;
    let fraction = bits & ((1_u64 << FRACTION_BITS) - 1);

    if exponent == 0 && fraction == 0 {
        return Some(0);
    }
    // Below 2^0 a non-zero value is a fraction; at 2^53 and above it is outside
    // the safe range; 0x7ff is NaN or an infinity, which the second test covers.
    let power = exponent.checked_sub(BIAS)?;
    let shift = u32::try_from(u64::from(FRACTION_BITS).checked_sub(power)?).ok()?;
    let mantissa = fraction | (1_u64 << FRACTION_BITS);
    if mantissa & ((1_u64 << shift) - 1) != 0 {
        return None;
    }
    let magnitude = i64::try_from(mantissa >> shift).ok()?;
    Some(if negative { -magnitude } else { magnitude })
}
