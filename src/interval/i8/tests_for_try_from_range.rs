use super::*;

#[test]
fn try_from_ops_range_converts_half_open_range() {
    let r = -5..10i8;
    let iv = I8CO::try_from(r).unwrap();
    assert_eq!(iv.start(), -5);
    assert_eq!(iv.end_excl(), 10);
}

#[test]
fn try_from_ops_range_rejects_empty_range() {
    assert!(I8CO::try_from(3i8..3).is_err());
}

#[test]
fn try_from_ops_range_rejects_reversed_range() {
    assert!(I8CO::try_from(10i8..-5).is_err());
}
