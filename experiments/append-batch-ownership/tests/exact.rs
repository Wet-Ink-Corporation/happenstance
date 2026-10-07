//! The one numeric conversion the replica arms make themselves.

use append_batch_ownership::exact::safe_integer;

/// Rejects: a conversion that rounds, truncates a large position, or accepts a
/// value Workers SQL would have handed back as a `Real`.
#[test]
fn safe_integers_convert_exactly_and_nothing_else_does() {
    let max_safe: i64 = (1_i64 << 53) - 1;
    for value in [1_i64, 2, 3, 7, 1023, 1024, 1025, 65_537, 1 << 40, max_safe] {
        let number = f64::from(u32::try_from(value).unwrap_or(0));
        if number != 0.0 {
            assert_eq!(safe_integer(number), Some(value), "{value}");
        }
        let exact = value.to_string().parse::<f64>().expect("parses");
        assert_eq!(safe_integer(exact), Some(value), "{value}");
    }
    assert_eq!(safe_integer(0.0), Some(0));
    assert_eq!(safe_integer(-5.0), Some(-5));
    assert_eq!(safe_integer(1.5), None, "a fraction is not an integer");
    assert_eq!(
        safe_integer(9_007_199_254_740_992.0),
        None,
        "2^53 is outside the safe range"
    );
    assert_eq!(safe_integer(f64::NAN), None);
    assert_eq!(safe_integer(f64::INFINITY), None);
}
