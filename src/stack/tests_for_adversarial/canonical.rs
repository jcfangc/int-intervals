// Canonical change-point invariant and construction attacks on IntCOStack.

use crate::interval::I32CO;
use crate::stack::IntCOStack;
use alloc::vec::Vec;
use proptest::prelude::*;

use super::super::int_co_stack::test_support::{
    intervals_strategy, iv_i32, prop_assert_canonical, stack_from_intervals,
};

#[test]
fn empty_stack_is_canonical() {
    let stack: IntCOStack<I32CO> = IntCOStack::default();
    assert!(stack.change_points().is_empty());
}

#[test]
fn single_interval_stack_is_canonical() {
    let stack = stack_from_intervals(&[(0, 5)]);
    prop_assert_canonical(stack.change_points()).unwrap();
}

#[test]
fn two_disjoint_intervals_are_canonical() {
    let stack = stack_from_intervals(&[(0, 5), (10, 15)]);
    prop_assert_canonical(stack.change_points()).unwrap();
}

#[test]
fn two_adjacent_intervals_are_canonical() {
    let stack = stack_from_intervals(&[(0, 5), (5, 10)]);
    let pts = stack.change_points();
    prop_assert_canonical(pts).unwrap();
    assert_eq!(pts.len(), 2);
    assert_eq!(pts[0].at, 0);
    assert_eq!(pts[0].height_after, 1);
    assert_eq!(pts[1].at, 10);
    assert_eq!(pts[1].height_after, 0);
}

#[test]
fn two_overlapping_intervals_are_canonical() {
    let stack = stack_from_intervals(&[(0, 10), (5, 15)]);
    let pts = stack.change_points();
    prop_assert_canonical(pts).unwrap();
    assert_eq!(pts.len(), 4);
    assert_eq!(pts[0].height_after, 1);
    assert_eq!(pts[1].height_after, 2);
    assert_eq!(pts[2].height_after, 1);
    assert_eq!(pts[3].height_after, 0);
}

#[test]
fn three_nested_intervals_are_canonical() {
    let stack = stack_from_intervals(&[(0, 20), (2, 18), (4, 16)]);
    let pts = stack.change_points();
    prop_assert_canonical(pts).unwrap();
    assert_eq!(pts.len(), 6);
}

#[test]
fn identical_intervals_are_canonical() {
    let stack = stack_from_intervals(&[(0, 10), (0, 10), (0, 10)]);
    let pts = stack.change_points();
    prop_assert_canonical(pts).unwrap();
    assert_eq!(pts.len(), 2);
    assert_eq!(pts[0].height_after, 3);
    assert_eq!(pts[1].height_after, 0);
}

#[test]
fn many_intervals_still_canonical() {
    let mut intervals = Vec::new();
    for i in 0..200 {
        intervals.push(iv_i32(i * 2, i * 2 + 1));
    }
    let stack: IntCOStack<I32CO> = intervals.into_iter().collect();
    prop_assert_canonical(stack.change_points()).unwrap();
}

#[test]
fn intervals_at_same_start() {
    let stack = stack_from_intervals(&[(0, 10), (0, 10), (0, 10)]);
    prop_assert_canonical(stack.change_points()).unwrap();
    assert_eq!(stack.change_points()[0].height_after, 3);
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 512,
        .. ProptestConfig::default()
    })]

    #[test]
    fn prop_stack_from_intervals_is_canonical(intervals in intervals_strategy(0..32)) {
        let stack = stack_from_intervals(&intervals);
        prop_assert_canonical(stack.change_points())?;
    }

    #[test]
    fn prop_iterator_collect_is_canonical(intervals in intervals_strategy(0..64)) {
        let stack = stack_from_intervals(&intervals);
        prop_assert_canonical(stack.change_points())?;
    }
}
