// Proptest-driven invariant stress tests and trait forwarding attacks.

use crate::interval::I32CO;
use crate::interval::res::{OneTwo, ZeroOneTwo};
use crate::interval::traits::*;
use proptest::prelude::*;
use std::vec;
use std::vec::Vec;

fn edge_values() -> Vec<i32> {
    let mut v = vec![i32::MIN, i32::MAX, 0, 1, -1];
    v.push(i32::MIN.saturating_add(1));
    v.push(i32::MAX.saturating_sub(1));
    v.sort_unstable();
    v.dedup();
    v
}

fn mixed_scalar() -> impl Strategy<Value = i32> {
    let edges = edge_values();
    prop_oneof![
        3 => prop::sample::select(edges),
        7 => any::<i32>(),
    ]
}

fn arb_iv() -> impl Strategy<Value = I32CO> {
    (mixed_scalar(), mixed_scalar()).prop_filter_map("non-empty interval", |(a, b)| {
        let (lo, hi) = if a < b { (a, b) } else { (b, a) };
        I32CO::try_new(lo, hi)
    })
}

proptest! {
    #![proptest_config(ProptestConfig {
        cases: 2048,
        .. ProptestConfig::default()
    })]

    #[test]
    fn prop_invariant_start_less_than_end_excl(iv in arb_iv()) {
        prop_assert!(iv.start() < iv.end_excl());
    }

    #[test]
    fn prop_invariant_end_incl_ge_start(iv in arb_iv()) {
        prop_assert!(iv.end_incl() >= iv.start());
    }

    #[test]
    fn prop_invariant_len_positive(iv in arb_iv()) {
        prop_assert!(iv.len() > 0);
    }

    #[test]
    fn prop_invariant_midpoint_contained(iv in arb_iv()) {
        let mid = iv.midpoint();
        prop_assert!(iv.contains(mid));
    }

    #[test]
    fn prop_convex_hull_valid(a in arb_iv(), b in arb_iv()) {
        let hull = a.convex_hull(b);
        prop_assert!(hull.start() < hull.end_excl());
    }

    #[test]
    fn prop_intersection_subset(a in arb_iv(), b in arb_iv(), p in any::<i32>()) {
        if let Some(inter) = a.intersection(b) {
            if inter.contains(p) {
                prop_assert!(a.contains(p));
                prop_assert!(b.contains(p));
            }
        }
    }

    #[test]
    fn prop_intersection_idempotent(a in arb_iv()) {
        prop_assert_eq!(a.intersection(a), Some(a));
    }

    #[test]
    fn prop_difference_self_zero(a in arb_iv()) {
        prop_assert!(matches!(a.difference(a), ZeroOneTwo::Zero));
    }

    #[test]
    fn prop_symmetric_difference_self_zero(a in arb_iv()) {
        prop_assert!(matches!(a.symmetric_difference(a), ZeroOneTwo::Zero));
    }

    #[test]
    fn prop_union_self_one(a in arb_iv()) {
        prop_assert!(matches!(a.union(a), OneTwo::One(x) if x == a));
    }

    #[test]
    fn prop_len_matches_range_count(iv in arb_iv()) {
        let range_len = (iv.end_excl() as i64 - iv.start() as i64).unsigned_abs() as u32;
        if range_len <= 1000 {
            let count: u32 = (iv.start()..iv.end_excl()).count().try_into().unwrap();
            prop_assert_eq!(count, iv.len());
        }
    }

    #[test]
    fn prop_range_iteration(iv in arb_iv()) {
        let len = iv.len();
        if len > 1000 {
            return Ok(());
        }
        for x in iv.to_range() {
            prop_assert!(iv.contains(x));
        }
    }
}

// =========================================================================
// Trait forwarding attacks
// =========================================================================

#[test]
fn trait_construct_try_new_matches_direct() {
    let direct = I32CO::try_new(-5i32, 10i32);
    let via_trait: Option<I32CO> = COConstruct::try_new(-5i32, 10i32);
    assert_eq!(direct, via_trait);
}

#[test]
fn trait_accessors_match_direct() {
    let a = I32CO::try_new(-3, 7).unwrap();
    assert_eq!(COBounds::start(a), a.start());
    assert_eq!(COBounds::end_excl(a), a.end_excl());
    assert_eq!(COBounds::end_incl(a), a.end_incl());
}

#[test]
fn trait_predicates_match_direct() {
    let a = I32CO::try_new(0, 10).unwrap();
    let b = I32CO::try_new(5, 15).unwrap();
    assert_eq!(COPredicates::contains(a, 3), a.contains(3));
    assert_eq!(
        COPredicates::contains_interval(a, b),
        a.contains_interval(b)
    );
    assert_eq!(COPredicates::intersects(a, b), a.intersects(b));
    assert_eq!(COPredicates::is_adjacent(a, b), a.is_adjacent(b));
}

#[test]
fn trait_algebra_matches_direct() {
    let a = I32CO::try_new(0, 10).unwrap();
    let b = I32CO::try_new(5, 15).unwrap();
    assert_eq!(COAlgebra::intersection(a, b), a.intersection(b));
    assert_eq!(COAlgebra::convex_hull(a, b), a.convex_hull(b));
    assert_eq!(COAlgebra::between(a, b), a.between(b));
}

#[test]
fn trait_len_matches_direct() {
    let a = I32CO::try_new(0, 10).unwrap();
    assert_eq!(COMeasure::len(a), a.len());
}

#[test]
fn trait_midpoint_matches_direct() {
    let a = I32CO::try_new(0, 10).unwrap();
    assert_eq!(COMidpoint::midpoint(a), a.midpoint());
}

#[test]
fn trait_start_len_construct_matches_direct() {
    let iv1 = I32CO::checked_from_start_len(5, 3);
    let iv2: Option<I32CO> = COStartLenConstruct::checked_from_start_len(5, 3);
    assert_eq!(iv1, iv2);
}
