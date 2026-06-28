// Predicate correctness and coverage attacks on IntCOSet.

use crate::interval::I8CO;
use crate::set::IntCOSet;
use alloc::vec::Vec;
use std::vec;

use super::super::test_support::{build, iv};

#[test]
fn contains_point_boundaries() {
    let set = build([(0, 5), (10, 15)]);
    assert!(set.contains_point(0));
    assert!(set.contains_point(4));
    assert!(!set.contains_point(5));
    assert!(!set.contains_point(7));
    assert!(set.contains_point(10));
    assert!(!set.contains_point(15));
}

#[test]
fn contains_interval_full_match() {
    let set = build([(0, 10), (20, 30)]);
    assert!(set.contains_interval(iv(0, 10)));
    assert!(set.contains_interval(iv(20, 30)));
}

#[test]
fn contains_interval_subset_of_one() {
    let set = build([(0, 10), (20, 30)]);
    assert!(set.contains_interval(iv(2, 8)));
}

#[test]
fn contains_interval_straddling_gap() {
    let set = build([(0, 10), (20, 30)]);
    assert!(!set.contains_interval(iv(5, 25)));
}

#[test]
fn intersects_interval_partial_overlap() {
    let set = build([(0, 10), (20, 30)]);
    assert!(set.intersects_interval(iv(5, 15)));
    assert!(set.intersects_interval(iv(15, 25)));
    assert!(!set.intersects_interval(iv(12, 18)));
}

#[test]
fn intersects_interval_boundary_touch() {
    let set = build([(0, 5), (10, 15)]);
    assert!(!set.intersects_interval(iv(5, 10)));
    assert!(!set.intersects_interval(iv(5, 6)));
}

#[test]
fn interval_containing_point_exact_boundaries() {
    let set = build([(0, 5), (10, 15)]);
    assert_eq!(set.interval_containing_point(0), Some(iv(0, 5)));
    assert_eq!(set.interval_containing_point(4), Some(iv(0, 5)));
    assert_eq!(set.interval_containing_point(5), None);
    assert_eq!(set.interval_containing_point(14), Some(iv(10, 15)));
}

#[test]
fn intervals_intersecting_single_interval() {
    let set = build([(0, 10), (20, 30), (40, 50)]);
    let result: Vec<_> = set.intervals_intersecting(iv(5, 25)).collect();
    assert_eq!(result, vec![iv(0, 10), iv(20, 30)]);
}

#[test]
fn intervals_intersecting_all() {
    let set = build([(0, 10), (20, 30)]);
    assert_eq!(set.intervals_intersecting(iv(-100, 100)).count(), 2);
}

#[test]
fn intervals_intersecting_none() {
    let set = build([(0, 10), (20, 30)]);
    assert_eq!(set.intervals_intersecting(iv(12, 18)).count(), 0);
}

#[test]
fn exhaustive_contains_point_vs_oracle() {
    let intervals: Vec<I8CO> = [iv(0, 10), iv(20, 30), iv(40, 50), iv(25, 45)]
        .into_iter()
        .collect();
    let set: IntCOSet<I8CO> = intervals.into_iter().collect();
    for x in i8::MIN..=i8::MAX {
        let expected = (0..10).contains(&x) || (20..50).contains(&x);
        assert_eq!(
            set.contains_point(x),
            expected,
            "contains_point({x}) mismatch"
        );
    }
}

// -- coverage --

#[test]
fn covered_plus_uncovered_equals_query_len() {
    let set = build([(10, 20), (30, 40), (50, 60)]);
    let queries = [iv(0, 100), iv(5, 45), iv(15, 35), iv(55, 65), iv(0, 15)];
    for q in &queries {
        let covered = set.covered_len_of(*q);
        let uncovered = set.uncovered_len_of(*q);
        assert_eq!(covered + uncovered, q.len());
    }
}

#[test]
fn covered_len_zero_when_no_overlap() {
    let set = build([(10, 20), (30, 40)]);
    assert_eq!(set.covered_len_of(iv(50, 60)), 0);
    assert_eq!(set.covered_len_of(iv(20, 30)), 0);
}

#[test]
fn coverage_ratio_bounds() {
    let set = build([(0, 50)]);
    let ratio = set.coverage_ratio_f64_of(iv(0, 100));
    assert!((ratio - 0.5).abs() < 0.001);
}
