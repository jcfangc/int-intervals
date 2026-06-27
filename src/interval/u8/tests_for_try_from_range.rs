use super::*;

#[test]
fn try_from_ops_range_converts_half_open_range() {
    let r = 1..10u8;
    let iv = U8CO::try_from(r).unwrap();
    assert_eq!(iv.start(), 1);
    assert_eq!(iv.end_excl(), 10);
}

#[test]
fn try_from_ops_range_rejects_empty_range() {
    assert!(U8CO::try_from(5u8..5).is_err());
}

#[test]
fn try_from_ops_range_rejects_reversed_range() {
    assert!(U8CO::try_from(10u8..5).is_err());
}
