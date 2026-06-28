// Empty set attacks.

use crate::interval::I8CO;
use crate::set::IntCOSet;

use super::super::test_support::{build, iv};

fn empty() -> IntCOSet<I8CO> {
    IntCOSet::default()
}

fn non_empty() -> IntCOSet<I8CO> {
    build([(0, 5), (10, 15)])
}

#[test]
fn empty_default_is_empty() {
    let e = empty();
    assert!(e.is_empty());
    assert_eq!(e.interval_count(), 0);
    assert!(e.as_slice().is_empty());
    assert_eq!(e.iter_intervals().count(), 0);
}

#[test]
fn empty_union_with_empty() {
    assert!(empty().union_with_set(&empty()).is_empty());
}

#[test]
fn empty_union_with_nonempty() {
    assert_eq!(empty().union_with_set(&non_empty()), non_empty());
}

#[test]
fn nonempty_union_with_empty() {
    assert_eq!(non_empty().union_with_set(&empty()), non_empty());
}

#[test]
fn empty_intersection_with_empty() {
    assert!(empty().intersection_with_set(&empty()).is_empty());
}

#[test]
fn empty_difference_with_empty() {
    assert!(empty().difference_with_set(&empty()).is_empty());
}

#[test]
fn empty_difference_with_nonempty() {
    assert!(empty().difference_with_set(&non_empty()).is_empty());
}

#[test]
fn nonempty_difference_with_empty() {
    assert_eq!(non_empty().difference_with_set(&empty()), non_empty());
}

#[test]
fn empty_symmetric_difference_with_empty() {
    assert!(empty().symmetric_difference_with_set(&empty()).is_empty());
}

#[test]
fn empty_symmetric_difference_with_nonempty() {
    assert_eq!(
        empty().symmetric_difference_with_set(&non_empty()),
        non_empty()
    );
}

#[test]
fn empty_contains_nothing() {
    let e = empty();
    for x in i8::MIN..=i8::MAX {
        assert!(!e.contains_point(x));
    }
}

#[test]
fn empty_does_not_contain_interval() {
    assert!(!empty().contains_interval(iv(0, 5)));
}

#[test]
fn empty_does_not_intersect() {
    assert!(!empty().intersects_interval(iv(0, 5)));
}

#[test]
fn empty_interval_containing_point_returns_none() {
    assert_eq!(empty().interval_containing_point(0), None);
}

#[test]
fn empty_covered_len_is_zero() {
    let e = empty();
    assert_eq!(e.covered_len_of(iv(0, 100)), 0);
    assert_eq!(e.uncovered_len_of(iv(0, 100)), 100);
}

#[test]
fn empty_coverage_ratio_is_zero() {
    let e = empty();
    assert_eq!(e.coverage_ratio_f32_of(iv(0, 10)), 0.0);
    assert_eq!(e.coverage_ratio_f64_of(iv(0, 10)), 0.0);
}

#[test]
fn empty_interval_ops() {
    let e = empty();
    let q = iv(0, 5);
    assert_eq!(e.intersection_with_interval(q), e);
    assert_eq!(e.union_with_interval(q), build([(0, 5)]));
    assert_eq!(e.difference_with_interval(q), e);
    assert_eq!(e.symmetric_difference_with_interval(q), build([(0, 5)]));
}
