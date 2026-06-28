// Algebra attacks on interval operations: convex_hull, intersection, between,
// union, difference, symmetric_difference. Includes accessor and predicate attacks.

use crate::interval::I32CO;
use crate::interval::U32CO;
use crate::interval::res::{OneTwo, ZeroOneTwo};

fn iv(start: i32, end_excl: i32) -> I32CO {
    I32CO::try_new(start, end_excl).unwrap()
}

// Simple deterministic PRNG for randomized tests.
fn xorshift(state: &mut u64) -> u64 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 7;
    x ^= x << 17;
    *state = x;
    x
}

fn rand_i32(state: &mut u64) -> i32 {
    xorshift(state) as i32
}

fn random_iv(state: &mut u64) -> I32CO {
    loop {
        let a = rand_i32(state);
        let b = rand_i32(state);
        if let Some(iv) = I32CO::try_new(a.min(b), a.max(b)) {
            return iv;
        }
    }
}

#[cfg(test)]
mod accessors {
    use super::*;

    #[test]
    fn end_incl_at_min() {
        let a = iv(i32::MIN, i32::MIN.saturating_add(1));
        assert_eq!(a.end_incl(), i32::MIN);
    }

    #[test]
    fn end_incl_at_max() {
        let a = iv(i32::MAX - 1, i32::MAX);
        assert_eq!(a.end_incl(), i32::MAX - 1);
    }

    #[test]
    fn end_incl_never_less_than_start() {
        let tests = [
            iv(i32::MIN, i32::MAX),
            iv(0, 1),
            iv(-5, 5),
            iv(i32::MAX - 1, i32::MAX),
            iv(i32::MIN, i32::MIN + 1),
        ];
        for t in &tests {
            assert!(t.end_incl() >= t.start(), "end_incl >= start for {t:?}");
        }
    }

    #[test]
    fn len_full_range() {
        let a = iv(i32::MIN, i32::MAX);
        assert_eq!(a.len(), u32::MAX);
    }

    #[test]
    fn len_minimal() {
        let a = iv(0, 1);
        assert_eq!(a.len(), 1);
    }

    #[test]
    fn len_cross_zero() {
        let a = iv(-3, 2);
        assert_eq!(a.len(), 5);
    }

    #[test]
    fn len_never_zero() {
        let tests = [
            iv(i32::MIN, i32::MAX),
            iv(0, 1),
            iv(-5, 5),
            iv(i32::MAX - 1, i32::MAX),
        ];
        for t in &tests {
            assert!(t.len() > 0, "len > 0 for {t:?}");
        }
    }

    #[test]
    fn midpoint_always_in_interval() {
        let mut state = 12345u64;
        for _ in 0..1000 {
            let start = rand_i32(&mut state);
            let end_excl = rand_i32(&mut state);
            let Some(a) = I32CO::try_new(start.min(end_excl), start.max(end_excl)) else {
                continue;
            };
            let mid = a.midpoint();
            assert!(
                a.contains(mid),
                "midpoint {mid} not in [{}, {})",
                a.start(),
                a.end_excl()
            );
        }
    }

    #[test]
    fn midpoint_full_range() {
        let a = iv(i32::MIN, i32::MAX);
        let mid = a.midpoint();
        assert!(a.contains(mid));
    }

    #[test]
    fn midpoint_odd_length() {
        assert_eq!(iv(0, 3).midpoint(), 1);
    }

    #[test]
    fn midpoint_even_length() {
        assert_eq!(iv(0, 4).midpoint(), 2);
    }

    #[test]
    fn u32_end_incl_at_max() {
        let a = U32CO::try_new(u32::MAX - 1, u32::MAX).unwrap();
        assert_eq!(a.end_incl(), u32::MAX - 1);
    }

    #[test]
    fn u32_len_full_range() {
        let a = U32CO::try_new(0, u32::MAX).unwrap();
        assert_eq!(a.len(), u32::MAX);
    }
}

#[cfg(test)]
mod predicates {
    use super::*;

    #[test]
    fn contains_at_boundaries() {
        let a = iv(0, 5);
        assert!(a.contains(0));
        assert!(a.contains(4));
        assert!(!a.contains(5));
        assert!(!a.contains(-1));
    }

    #[test]
    fn contains_min_max() {
        let a = iv(i32::MIN, i32::MAX);
        assert!(a.contains(i32::MIN));
        assert!(a.contains(0));
        assert!(a.contains(i32::MAX - 1));
        assert!(!a.contains(i32::MAX));
    }

    #[test]
    fn contains_interval_self() {
        let a = iv(0, 5);
        assert!(a.contains_interval(a));
    }

    #[test]
    fn contains_interval_boundary_match() {
        let a = iv(0, 5);
        let b = iv(0, 5);
        assert!(a.contains_interval(b));
    }

    #[test]
    fn contains_interval_not_contained() {
        assert!(!iv(0, 5).contains_interval(iv(3, 7)));
    }

    #[test]
    fn intersects_yes() {
        assert!(iv(0, 5).intersects(iv(3, 8)));
        assert!(iv(0, 5).intersects(iv(0, 5)));
        assert!(iv(0, 5).intersects(iv(-2, 1)));
        assert!(iv(0, 5).intersects(iv(3, 5)));
    }

    #[test]
    fn intersects_no() {
        assert!(!iv(0, 5).intersects(iv(5, 10)));
        assert!(!iv(0, 5).intersects(iv(6, 10)));
        assert!(!iv(0, 5).intersects(iv(-5, 0)));
    }

    #[test]
    fn is_adjacent_yes() {
        assert!(iv(0, 5).is_adjacent(iv(5, 10)));
        assert!(iv(5, 10).is_adjacent(iv(0, 5)));
    }

    #[test]
    fn is_adjacent_no() {
        assert!(!iv(0, 5).is_adjacent(iv(6, 10)));
        assert!(!iv(0, 5).is_adjacent(iv(3, 8)));
    }

    #[test]
    fn is_contiguous_with_adjacent() {
        assert!(iv(0, 5).is_contiguous_with(iv(5, 10)));
    }

    #[test]
    fn is_contiguous_with_overlapping() {
        assert!(iv(0, 5).is_contiguous_with(iv(3, 8)));
    }

    #[test]
    fn is_contiguous_with_separated() {
        assert!(!iv(0, 5).is_contiguous_with(iv(6, 10)));
    }

    #[test]
    fn u32_intersects_adjacent_boundary() {
        let a = U32CO::try_new(0, 5).unwrap();
        let b = U32CO::try_new(5, 10).unwrap();
        assert!(!a.intersects(b));
        assert!(a.is_adjacent(b));
    }
}

#[cfg(test)]
mod algebra_unit {
    use super::*;

    // --- convex_hull ---
    #[test]
    fn convex_hull_full_span() {
        let hull = iv(i32::MIN, 0).convex_hull(iv(0, i32::MAX));
        assert_eq!(hull.start(), i32::MIN);
        assert_eq!(hull.end_excl(), i32::MAX);
        assert!(hull.start() < hull.end_excl());
    }

    #[test]
    fn convex_hull_identical() {
        let a = iv(0, 5);
        let hull = a.convex_hull(a);
        assert_eq!(hull, a);
        assert!(hull.start() < hull.end_excl());
    }

    #[test]
    fn convex_hull_adjacent() {
        let hull = iv(0, 5).convex_hull(iv(5, 10));
        assert_eq!(hull.start(), 0);
        assert_eq!(hull.end_excl(), 10);
        assert_eq!(I32CO::try_new(hull.start(), hull.end_excl()), Some(hull));
    }

    // --- intersection ---
    #[test]
    fn intersection_disjoint() {
        assert_eq!(iv(0, 5).intersection(iv(10, 15)), None);
    }

    #[test]
    fn intersection_overlapping() {
        let inter = iv(0, 10).intersection(iv(5, 15)).unwrap();
        assert_eq!(inter.start(), 5);
        assert_eq!(inter.end_excl(), 10);
    }

    #[test]
    fn intersection_contained() {
        let inter = iv(2, 8).intersection(iv(0, 10)).unwrap();
        assert_eq!(inter.start(), 2);
        assert_eq!(inter.end_excl(), 8);
    }

    #[test]
    fn intersection_self_is_self() {
        let a = iv(0, 5);
        assert_eq!(a.intersection(a), Some(a));
    }

    #[test]
    fn intersection_full_range_with_self() {
        let a = iv(i32::MIN, i32::MAX);
        assert_eq!(a.intersection(a), Some(a));
    }

    #[test]
    fn intersection_adjacent() {
        assert_eq!(iv(0, 5).intersection(iv(5, 10)), None);
    }

    // --- between ---
    #[test]
    fn between_separated() {
        let b = iv(0, 5).between(iv(10, 15)).unwrap();
        assert_eq!(b.start(), 5);
        assert_eq!(b.end_excl(), 10);
    }

    #[test]
    fn between_adjacent_returns_none() {
        assert_eq!(iv(0, 5).between(iv(5, 10)), None);
    }

    #[test]
    fn between_overlapping_returns_none() {
        assert_eq!(iv(0, 10).between(iv(5, 15)), None);
    }

    #[test]
    fn between_reversed_order() {
        let b = iv(10, 15).between(iv(0, 5)).unwrap();
        assert_eq!(b.start(), 5);
        assert_eq!(b.end_excl(), 10);
    }

    // --- union ---
    #[test]
    fn union_disjoint() {
        match iv(0, 5).union(iv(10, 15)) {
            OneTwo::Two(l, r) => {
                assert_eq!(l, iv(0, 5));
                assert_eq!(r, iv(10, 15));
            }
            other => panic!("expected Two, got {other:?}"),
        }
    }

    #[test]
    fn union_overlapping_merged() {
        match iv(0, 7).union(iv(3, 10)) {
            OneTwo::One(z) => assert_eq!(z, iv(0, 10)),
            other => panic!("expected One, got {other:?}"),
        }
    }

    #[test]
    fn union_adjacent_merged() {
        match iv(0, 5).union(iv(5, 10)) {
            OneTwo::One(z) => {
                assert_eq!(z.start(), 0);
                assert_eq!(z.end_excl(), 10);
            }
            other => panic!("expected One, got {other:?}"),
        }
    }

    #[test]
    fn union_self_is_self() {
        let a = iv(0, 5);
        match a.union(a) {
            OneTwo::One(z) => assert_eq!(z, a),
            other => panic!("expected One, got {other:?}"),
        }
    }

    #[test]
    fn union_containment_law() {
        let mut state = 12345u64;
        for _ in 0..1000 {
            let a = random_iv(&mut state);
            let b = random_iv(&mut state);
            let p = rand_i32(&mut state);
            let in_union = match a.union(b) {
                OneTwo::One(z) => z.contains(p),
                OneTwo::Two(l, r) => l.contains(p) || r.contains(p),
            };
            assert_eq!(in_union, a.contains(p) || b.contains(p));
        }
    }

    // --- difference ---
    #[test]
    fn difference_self_is_zero() {
        match iv(0, 5).difference(iv(0, 5)) {
            ZeroOneTwo::Zero => {}
            other => panic!("expected Zero, got {other:?}"),
        }
    }

    #[test]
    fn difference_no_overlap_is_one() {
        match iv(0, 5).difference(iv(10, 15)) {
            ZeroOneTwo::One(z) => assert_eq!(z, iv(0, 5)),
            other => panic!("expected One, got {other:?}"),
        }
    }

    #[test]
    fn difference_fully_contained_is_zero() {
        match iv(2, 8).difference(iv(0, 10)) {
            ZeroOneTwo::Zero => {}
            other => panic!("expected Zero, got {other:?}"),
        }
    }

    #[test]
    fn difference_middle_cut() {
        match iv(0, 10).difference(iv(3, 7)) {
            ZeroOneTwo::Two(l, r) => {
                assert_eq!(l, iv(0, 3));
                assert_eq!(r, iv(7, 10));
                assert!(!l.intersects(r));
            }
            other => panic!("expected Two, got {other:?}"),
        }
    }

    #[test]
    fn difference_points_are_in_x_not_in_y() {
        let mut state = 12345u64;
        for _ in 0..1000 {
            let x = random_iv(&mut state);
            let y = random_iv(&mut state);
            let p = rand_i32(&mut state);
            let in_diff = match x.difference(y) {
                ZeroOneTwo::Zero => false,
                ZeroOneTwo::One(z) => z.contains(p),
                ZeroOneTwo::Two(l, r) => l.contains(p) || r.contains(p),
            };
            assert_eq!(in_diff, x.contains(p) && !y.contains(p));
        }
    }

    // --- symmetric_difference ---
    #[test]
    fn symmetric_difference_self_is_zero() {
        match iv(0, 5).symmetric_difference(iv(0, 5)) {
            ZeroOneTwo::Zero => {}
            other => panic!("expected Zero, got {other:?}"),
        }
    }

    #[test]
    fn symmetric_difference_disjoint() {
        match iv(0, 5).symmetric_difference(iv(10, 15)) {
            ZeroOneTwo::Two(l, r) => {
                assert_eq!(l, iv(0, 5));
                assert_eq!(r, iv(10, 15));
            }
            other => panic!("expected Two, got {other:?}"),
        }
    }

    #[test]
    fn symmetric_difference_points_in_exactly_one() {
        let mut state = 12345u64;
        for _ in 0..1000 {
            let x = random_iv(&mut state);
            let y = random_iv(&mut state);
            let p = rand_i32(&mut state);
            let in_sd = match x.symmetric_difference(y) {
                ZeroOneTwo::Zero => false,
                ZeroOneTwo::One(z) => z.contains(p),
                ZeroOneTwo::Two(l, r) => l.contains(p) || r.contains(p),
            };
            assert_eq!(in_sd, x.contains(p) ^ y.contains(p));
        }
    }
}
