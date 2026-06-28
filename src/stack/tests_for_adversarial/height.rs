// Height correctness, segment iteration, stats, and covered set attacks.

use crate::interval::I32CO;
use crate::stack::IntCOStack;
use alloc::vec::Vec;
use proptest::prelude::*;

use super::super::int_co_stack::test_support::{
    collect_segments, intervals_strategy, naive_height_at, oracle_segments, stack_from_intervals,
};

// -- height_at correctness --

#[test]
fn height_at_inside_interval() {
    let stack = stack_from_intervals(&[(0, 10)]);
    assert_eq!(stack.height_at(0), 1);
    assert_eq!(stack.height_at(5), 1);
    assert_eq!(stack.height_at(9), 1);
}

#[test]
fn height_at_outside_interval() {
    let stack = stack_from_intervals(&[(0, 10)]);
    assert_eq!(stack.height_at(-1), 0);
    assert_eq!(stack.height_at(10), 0);
    assert_eq!(stack.height_at(100), 0);
}

#[test]
fn height_at_gap_between_intervals() {
    let stack = stack_from_intervals(&[(0, 5), (10, 15)]);
    assert_eq!(stack.height_at(0), 1);
    assert_eq!(stack.height_at(7), 0);
    assert_eq!(stack.height_at(12), 1);
}

#[test]
fn height_at_overlapping_region() {
    let stack = stack_from_intervals(&[(0, 10), (5, 15), (7, 12)]);
    assert_eq!(stack.height_at(0), 1);
    assert_eq!(stack.height_at(6), 2);
    assert_eq!(stack.height_at(8), 3);
    assert_eq!(stack.height_at(11), 2);
    assert_eq!(stack.height_at(13), 1);
}

#[test]
fn height_at_boundary_coordinates() {
    let stack = stack_from_intervals(&[(0, 5), (5, 10)]);
    assert_eq!(stack.height_at(4), 1);
    assert_eq!(stack.height_at(5), 1);
    assert_eq!(stack.height_at(6), 1);
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 512,
        .. ProptestConfig::default()
    })]

    #[test]
    fn prop_height_at_matches_naive_oracle(intervals in intervals_strategy(0..16)) {
        let stack = stack_from_intervals(&intervals);
        let mut test_points = Vec::new();
        for &(s, e) in &intervals {
            test_points.push(s);
            test_points.push(e - 1);
            test_points.push(s + (e - s) / 2);
        }
        test_points.push(-100);
        test_points.push(0);
        test_points.push(100);

        for x in &test_points {
            let actual = stack.height_at(*x);
            let expected = naive_height_at(&intervals, *x);
            prop_assert_eq!(actual, expected);
        }
    }
}

// -- height segments --

#[test]
fn iter_height_segments_empty_stack() {
    let stack: IntCOStack<I32CO> = IntCOStack::default();
    assert_eq!(stack.iter_height_segments().count(), 0);
}

#[test]
fn iter_height_segments_matches_oracle() {
    let intervals = [(0, 10), (5, 15), (20, 30)];
    let stack = stack_from_intervals(&intervals);
    let actual = collect_segments(stack.iter_height_segments());
    let expected = oracle_segments(&intervals);
    assert_eq!(actual, expected);
}

#[test]
fn iter_height_segments_all_filters() {
    let intervals = [(0, 30), (5, 25), (10, 20)];
    let stack = stack_from_intervals(&intervals);

    let at_least_2: Vec<_> = collect_segments(stack.iter_height_segments_at_least(2));
    for (_, h) in &at_least_2 {
        assert!(*h >= 2);
    }

    let at_most_1: Vec<_> = collect_segments(stack.iter_height_segments_at_most(1));
    for (_, h) in &at_most_1 {
        assert!(*h <= 1);
    }

    let exactly_3: Vec<_> = collect_segments(stack.iter_height_segments_exactly(3));
    for (_, h) in &exactly_3 {
        assert_eq!(*h, 3);
    }

    let peak: Vec<_> = collect_segments(stack.iter_peak_height_segments());
    let max_h = stack.height_stats().max_height();
    for (_, h) in &peak {
        assert_eq!(*h, max_h);
    }
}

#[test]
fn height_segments_no_zero_height() {
    let stack = stack_from_intervals(&[(0, 10), (20, 30)]);
    for segment in stack.iter_height_segments() {
        assert!(segment.height.get() > 0);
    }
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 256,
        .. ProptestConfig::default()
    })]

    #[test]
    fn prop_height_segments_match_oracle(intervals in intervals_strategy(0..16)) {
        let stack = stack_from_intervals(&intervals);
        let actual = collect_segments(stack.iter_height_segments());
        let expected = oracle_segments(&intervals);
        prop_assert_eq!(actual, expected);
    }

    #[test]
    fn prop_peak_segments_at_max_height(intervals in intervals_strategy(0..16)) {
        let stack = stack_from_intervals(&intervals);
        let max_h = stack.height_stats().max_height();
        if max_h == 0 {
            prop_assert_eq!(stack.iter_peak_height_segments().count(), 0);
        } else {
            for seg in stack.iter_peak_height_segments() {
                prop_assert_eq!(seg.height.get(), max_h);
            }
        }
    }
}

// -- height stats --

#[test]
fn empty_stack_stats() {
    let stack: IntCOStack<I32CO> = IntCOStack::default();
    let stats = stack.height_stats();
    assert_eq!(stats.min_positive_height_or_zero(), 0);
    assert_eq!(stats.max_height(), 0);
    assert!(!stats.has_positive_height());
    assert!(!stats.has_overlap());
}

#[test]
fn single_interval_stats() {
    let stack = stack_from_intervals(&[(0, 10)]);
    let stats = stack.height_stats();
    assert_eq!(stats.max_height(), 1);
    assert!(stats.has_positive_height());
    assert!(!stats.has_overlap());
    assert!(stats.is_uniform_positive_height());
    assert_eq!(stats.uniform_positive_height().map(|nz| nz.get()), Some(1));
}

#[test]
fn overlapping_stats() {
    let stack = stack_from_intervals(&[(0, 10), (5, 15)]);
    let stats = stack.height_stats();
    assert_eq!(stats.max_height(), 2);
    assert!(stats.has_overlap());
    assert!(!stats.is_uniform_positive_height());
    assert_eq!(stats.uniform_positive_height(), None);
}

#[test]
fn uniform_non_one_height() {
    let stack = stack_from_intervals(&[(0, 10), (0, 10)]);
    let stats = stack.height_stats();
    assert_eq!(stats.max_height(), 2);
    assert_eq!(stats.min_positive_height_or_zero(), 2);
    assert!(stats.is_uniform_positive_height());
}

// -- covered set --

#[test]
fn covered_empty_stack() {
    let stack: IntCOStack<I32CO> = IntCOStack::default();
    assert!(stack.covered().is_empty());
}

#[test]
fn covered_single_interval() {
    let stack = stack_from_intervals(&[(0, 10)]);
    let covered = stack.covered();
    assert_eq!(covered.interval_count(), 1);
    assert!(covered.contains_point(0));
    assert!(!covered.contains_point(10));
}

#[test]
fn covered_two_overlapping() {
    let stack = stack_from_intervals(&[(0, 10), (5, 15)]);
    let covered = stack.covered();
    assert_eq!(covered.interval_count(), 1);
    assert_eq!(covered.as_slice()[0].start(), 0);
    assert_eq!(covered.as_slice()[0].end_excl(), 15);
}

#[test]
fn covered_disjoint() {
    let stack = stack_from_intervals(&[(0, 5), (10, 15)]);
    assert_eq!(stack.covered().interval_count(), 2);
}

#[test]
fn covered_matches_union_of_inputs() {
    let stack = stack_from_intervals(&[(0, 10), (5, 15), (20, 30), (25, 35)]);
    let covered = stack.covered();
    assert_eq!(covered.interval_count(), 2);
    assert_eq!(covered.as_slice()[0].start(), 0);
    assert_eq!(covered.as_slice()[0].end_excl(), 15);
    assert_eq!(covered.as_slice()[1].start(), 20);
    assert_eq!(covered.as_slice()[1].end_excl(), 35);
}
