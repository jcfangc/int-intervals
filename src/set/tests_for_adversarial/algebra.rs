// Self-operation, complex algebra, extreme-value, and proptest attacks on IntCOSet.

use crate::interval::I8CO;
use crate::set::IntCOSet;
use alloc::vec::Vec;

use super::super::test_support::{arb_iv, build, iv};
use super::canonical::is_canonical;
use proptest::prelude::*;

// -- self operations --

#[test]
fn self_intersection_is_self() {
    let a = build([(0, 5), (10, 15), (20, 25)]);
    assert_eq!(a.intersection_with_set(&a), a);
}

#[test]
fn self_union_is_self() {
    let a = build([(0, 5), (10, 15), (20, 25)]);
    assert_eq!(a.union_with_set(&a), a);
}

#[test]
fn self_difference_is_empty() {
    let a = build([(0, 5), (10, 15), (20, 25)]);
    assert!(a.difference_with_set(&a).is_empty());
}

#[test]
fn self_symmetric_difference_is_empty() {
    let a = build([(0, 5), (10, 15), (20, 25)]);
    assert!(a.symmetric_difference_with_set(&a).is_empty());
}

#[test]
fn clone_is_equal() {
    let a = build([(0, 5), (10, 15)]);
    let b = a.clone();
    assert_eq!(a, b);
    assert_eq!(a.as_slice(), b.as_slice());
}

// -- extreme values --

#[test]
fn set_at_domain_boundaries() {
    let set = build([(i8::MIN, i8::MIN + 5)]);
    assert!(set.contains_point(i8::MIN));
    assert!(set.contains_point(i8::MIN + 4));
    assert!(!set.contains_point(i8::MIN + 5));

    let set = build([(i8::MAX - 5, i8::MAX)]);
    assert!(set.contains_point(i8::MAX - 5));
    assert!(!set.contains_point(i8::MAX));
}

#[test]
fn full_domain_interval() {
    let set = build([(i8::MIN, i8::MAX)]);
    for x in i8::MIN..i8::MAX {
        assert!(set.contains_point(x));
    }
    assert!(!set.contains_point(i8::MAX));
}

#[test]
fn duplicate_intervals_in_input() {
    let ivs: Vec<I8CO> = (0..100).map(|_| iv(10, 20)).collect();
    let set: IntCOSet<I8CO> = ivs.into_iter().collect();
    assert_eq!(set.interval_count(), 1);
}

#[test]
fn reversed_order_input() {
    let set: IntCOSet<I8CO> = [iv(30, 40), iv(20, 30), iv(0, 10)].into_iter().collect();
    // [0,10) separate; [20,30)+[30,40) merged → 2 intervals
    assert_eq!(set.interval_count(), 2);
    assert_eq!(set.as_slice()[0], iv(0, 10));
    assert_eq!(set.as_slice()[1], iv(20, 40));
}

#[test]
fn single_point_interval_extremes() {
    for x in [i8::MIN, -1, 0, 1, i8::MAX - 1] {
        let set: IntCOSet<I8CO> = [iv(x, x + 1)].into_iter().collect();
        assert!(set.contains_point(x));
        assert!(!set.contains_point(x + 1));
    }
}

// -- complex algebra --

#[test]
fn union_then_intersection_with_original_is_original() {
    let a = build([(0, 10), (20, 30)]);
    let b = build([(5, 25)]);
    let union = a.union_with_set(&b);
    assert_eq!(union.intersection_with_set(&a), a);
}

#[test]
fn difference_then_union_restores_superset() {
    let a = build([(0, 10), (20, 30)]);
    let b = build([(5, 25)]);
    let diff = a.difference_with_set(&b);
    let restored = diff.union_with_set(&a.intersection_with_set(&b));
    assert_eq!(restored, a);
}

#[test]
fn de_morgan_law_sym_diff() {
    let a = build([(0, 10), (20, 30)]);
    let b = build([(5, 25), (40, 50)]);
    assert_eq!(
        a.symmetric_difference_with_set(&b)
            .symmetric_difference_with_set(&b),
        a
    );
}

#[test]
fn distributive_intersection_over_union() {
    let a = build([(0, 20), (30, 50)]);
    let b = build([(5, 15)]);
    let c = build([(35, 45)]);
    let left = a.intersection_with_set(&b.union_with_set(&c));
    let right = a
        .intersection_with_set(&b)
        .union_with_set(&a.intersection_with_set(&c));
    assert_eq!(left, right);
}

#[test]
fn chained_operations_remain_canonical() {
    let a = build([(0, 10), (20, 30), (40, 50), (60, 70)]);
    let b = build([(5, 25), (35, 45), (55, 75)]);
    let c = build([(15, 20), (30, 35), (50, 55), (70, 80)]);
    let r1 = a.intersection_with_set(&b);
    let r2 = r1.union_with_set(&c);
    let r3 = r2.difference_with_set(&a);
    let r4 = r3.symmetric_difference_with_set(&b);
    assert!(is_canonical(&r4));
}

// -- accessors --

#[test]
fn iter_intervals_matches_as_slice() {
    let set = build([(0, 5), (10, 15), (20, 25)]);
    let iterated: Vec<_> = set.iter_intervals().collect();
    assert_eq!(iterated, set.as_slice().to_vec());
}

#[test]
fn as_slice_reflects_canonical_order() {
    let set: IntCOSet<I8CO> = [iv(30, 40), iv(0, 10), iv(20, 25), iv(15, 20)]
        .into_iter()
        .collect();
    let s = set.as_slice();
    assert_eq!(s.len(), 3);
    assert_eq!(s[0].start(), 0);
    assert_eq!(s[1].start(), 15);
    assert_eq!(s[1].end_excl(), 25);
    assert_eq!(s[2].start(), 30);
}

// -- proptest --

fn arb_set() -> impl Strategy<Value = IntCOSet<I8CO>> {
    prop::collection::vec(arb_iv(), 0..32).prop_map(|ivs| ivs.into_iter().collect())
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 512,
        .. ProptestConfig::default()
    })]

    #[test]
    fn prop_from_iter_produces_canonical(ivs in prop::collection::vec(arb_iv(), 0..64)) {
        let set: IntCOSet<I8CO> = ivs.into_iter().collect();
        prop_assert!(is_canonical(&set));
    }

    #[test]
    fn prop_union_produces_canonical(a in arb_set(), b in arb_set()) {
        prop_assert!(is_canonical(&a.union_with_set(&b)));
    }

    #[test]
    fn prop_intersection_produces_canonical(a in arb_set(), b in arb_set()) {
        prop_assert!(is_canonical(&a.intersection_with_set(&b)));
    }

    #[test]
    fn prop_difference_produces_canonical(a in arb_set(), b in arb_set()) {
        prop_assert!(is_canonical(&a.difference_with_set(&b)));
    }

    #[test]
    fn prop_symmetric_difference_produces_canonical(a in arb_set(), b in arb_set()) {
        prop_assert!(is_canonical(&a.symmetric_difference_with_set(&b)));
    }

    #[test]
    fn prop_union_contains_both_inputs(a in arb_set(), b in arb_set()) {
        let c = a.union_with_set(&b);
        for x in i8::MIN..=i8::MAX {
            prop_assert_eq!(c.contains_point(x), a.contains_point(x) || b.contains_point(x));
        }
    }

    #[test]
    fn prop_intersection_subset_of_both(a in arb_set(), b in arb_set()) {
        let c = a.intersection_with_set(&b);
        for x in i8::MIN..=i8::MAX {
            if c.contains_point(x) {
                prop_assert!(a.contains_point(x));
                prop_assert!(b.contains_point(x));
            }
        }
    }

    #[test]
    fn prop_difference_disjoint_from_b(a in arb_set(), b in arb_set()) {
        let c = a.difference_with_set(&b);
        prop_assert!(c.intersection_with_set(&b).is_empty());
    }

    #[test]
    fn prop_symmetric_difference_xor_semantics(a in arb_set(), b in arb_set()) {
        let c = a.symmetric_difference_with_set(&b);
        for x in i8::MIN..=i8::MAX {
            prop_assert_eq!(c.contains_point(x), a.contains_point(x) ^ b.contains_point(x));
        }
    }

    #[test]
    fn prop_covered_plus_uncovered_equals_query_len(set in arb_set()) {
        let full_q = iv(i8::MIN, i8::MAX);
        let covered = set.covered_len_of(full_q);
        let uncovered = set.uncovered_len_of(full_q);
        prop_assert_eq!(covered + uncovered, full_q.len());
    }

    #[test]
    fn prop_interval_algebra_ops_canonical(set in arb_set()) {
        let q = iv(-50, 50);
        prop_assert!(is_canonical(&set.intersection_with_interval(q)));
        prop_assert!(is_canonical(&set.union_with_interval(q)));
        prop_assert!(is_canonical(&set.difference_with_interval(q)));
        prop_assert!(is_canonical(&set.symmetric_difference_with_interval(q)));
    }

    #[test]
    fn prop_clone_equals_original(set in arb_set()) {
        let cloned = set.clone();
        let orig_slice = set.as_slice().to_vec();
        let cloned_slice = cloned.as_slice().to_vec();
        prop_assert_eq!(set, cloned);
        prop_assert_eq!(orig_slice, cloned_slice);
    }
}
