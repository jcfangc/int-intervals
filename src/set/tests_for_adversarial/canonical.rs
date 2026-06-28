// Canonical invariant attacks on IntCOSet.

use crate::interval::I8CO;
use crate::set::IntCOSet;

use super::super::test_support::{build, intervals, iv};

pub(super) fn is_canonical(set: &IntCOSet<I8CO>) -> bool {
    let s = intervals(set);
    s.windows(2).all(|w| w[0].end_excl() < w[1].start())
}

#[test]
fn empty_set_is_canonical() {
    let set: IntCOSet<I8CO> = IntCOSet::default();
    assert!(is_canonical(&set));
}

#[test]
fn single_interval_is_canonical() {
    let set = build([(0, 5)]);
    assert!(is_canonical(&set));
}

#[test]
fn adjacent_intervals_are_merged_by_from_iter() {
    let set: IntCOSet<I8CO> = [iv(0, 5), iv(5, 10)].into_iter().collect();
    assert!(is_canonical(&set));
    assert_eq!(set.interval_count(), 1);
}

#[test]
fn overlapping_intervals_are_merged_by_from_iter() {
    let set: IntCOSet<I8CO> = [iv(0, 7), iv(3, 10)].into_iter().collect();
    assert!(is_canonical(&set));
    assert_eq!(set.interval_count(), 1);
}

#[test]
fn union_preserves_canonical() {
    let a = build([(0, 5), (10, 15)]);
    let b = build([(3, 12)]);
    let c = a.union_with_set(&b);
    assert!(is_canonical(&c));
    assert_eq!(c.interval_count(), 1);
}

#[test]
fn intersection_preserves_canonical() {
    let a = build([(0, 10), (20, 30), (40, 50)]);
    let b = build([(5, 25), (45, 60)]);
    let c = a.intersection_with_set(&b);
    assert!(is_canonical(&c));
}

#[test]
fn difference_preserves_canonical() {
    let a = build([(0, 10), (20, 30), (40, 50)]);
    let b = build([(5, 25), (45, 60)]);
    assert!(is_canonical(&a.difference_with_set(&b)));
}

#[test]
fn symmetric_difference_preserves_canonical() {
    let a = build([(0, 10), (20, 30), (40, 50)]);
    let b = build([(5, 25), (45, 60)]);
    assert!(is_canonical(&a.symmetric_difference_with_set(&b)));
}

#[test]
fn interval_algebra_ops_preserve_canonical() {
    let set = build([(10, 20), (30, 40)]);
    assert!(is_canonical(&set.intersection_with_interval(iv(5, 15))));
    assert!(is_canonical(&set.union_with_interval(iv(5, 15))));
    assert!(is_canonical(&set.difference_with_interval(iv(5, 15))));
    assert!(is_canonical(
        &set.symmetric_difference_with_interval(iv(5, 15))
    ));
}
