// Minkowski arithmetic attacks. Tests checked and saturating operations for
// both signed (i32) and unsigned (u32) types, plus regression tests for the
// saturating_div_hull panic bugs that were fixed.

use crate::interval::I32CO;
use crate::interval::U32CO;

fn iv(start: i32, end_excl: i32) -> I32CO {
    I32CO::try_new(start, end_excl).unwrap()
}

fn uiv(start: u32, end_excl: u32) -> U32CO {
    U32CO::try_new(start, end_excl).unwrap()
}

// Simple PRNG for randomized tests.
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

fn rand_u32(state: &mut u64) -> u32 {
    xorshift(state) as u32
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

// =========================================================================
// Signed checked minkowski
// =========================================================================

#[cfg(test)]
mod signed_checked {
    use super::*;

    #[test]
    fn add_min_plus_min_overflows() {
        let a = iv(i32::MIN, i32::MIN + 1);
        assert!(a.checked_minkowski_add(a).is_none());
    }

    #[test]
    fn add_max_overflow() {
        let a = iv(i32::MAX - 1, i32::MAX);
        assert!(a.checked_minkowski_add(iv(1, 2)).is_none());
    }

    #[test]
    fn add_contains_corners() {
        let mut state = 12345u64;
        for _ in 0..500 {
            let a = random_iv(&mut state);
            let b = random_iv(&mut state);
            if let Some(c) = a.checked_minkowski_add(b) {
                for &x in &[a.start(), a.end_incl()] {
                    for &y in &[b.start(), b.end_incl()] {
                        let sum = x.wrapping_add(y);
                        assert!(c.start() <= sum && sum < c.end_excl());
                    }
                }
            }
        }
    }

    #[test]
    fn add_commutative() {
        let mut state = 12345u64;
        for _ in 0..500 {
            let a = random_iv(&mut state);
            let b = random_iv(&mut state);
            assert_eq!(a.checked_minkowski_add(b), b.checked_minkowski_add(a));
        }
    }

    #[test]
    fn sub_underflow() {
        let a = iv(i32::MIN, i32::MIN + 1);
        assert!(a.checked_minkowski_sub(iv(1, 2)).is_none());
    }

    #[test]
    fn sub_contains_corners() {
        let mut state = 12345u64;
        for _ in 0..500 {
            let a = random_iv(&mut state);
            let b = random_iv(&mut state);
            if let Some(c) = a.checked_minkowski_sub(b) {
                for &x in &[a.start(), a.end_incl()] {
                    for &y in &[b.start(), b.end_incl()] {
                        let diff = x.wrapping_sub(y);
                        assert!(c.start() <= diff && diff < c.end_excl());
                    }
                }
            }
        }
    }

    #[test]
    fn mul_overflow() {
        let a = iv(i32::MIN, i32::MIN + 1);
        assert!(a.checked_minkowski_mul_hull(iv(2, 3)).is_none());
    }

    #[test]
    fn mul_negative_times_negative() {
        let c = iv(-3, 0).checked_minkowski_mul_hull(iv(-2, 0)).unwrap();
        assert_eq!(c.start(), 1);
        assert_eq!(c.end_excl(), 7);
    }

    #[test]
    fn mul_mixed_signs() {
        let c = iv(-2, 3).checked_minkowski_mul_hull(iv(-1, 2)).unwrap();
        assert_eq!(c.start(), -2);
        assert_eq!(c.end_excl(), 3);
    }

    #[test]
    fn mul_zero_times_anything() {
        let c = iv(0, 1)
            .checked_minkowski_mul_hull(iv(i32::MIN, i32::MAX))
            .unwrap();
        assert_eq!(c.start(), 0);
        assert_eq!(c.end_excl(), 1);
    }

    #[test]
    fn mul_contains_corners() {
        let mut state = 12345u64;
        for _ in 0..500 {
            let a = random_iv(&mut state);
            let b = random_iv(&mut state);
            if let Some(c) = a.checked_minkowski_mul_hull(b) {
                for &x in &[a.start(), a.end_incl()] {
                    for &y in &[b.start(), b.end_incl()] {
                        let prod = x.wrapping_mul(y);
                        assert!(c.start() <= prod && prod < c.end_excl());
                    }
                }
            }
        }
    }

    #[test]
    fn mul_commutative() {
        let mut state = 12345u64;
        for _ in 0..500 {
            let a = random_iv(&mut state);
            let b = random_iv(&mut state);
            assert_eq!(
                a.checked_minkowski_mul_hull(b),
                b.checked_minkowski_mul_hull(a)
            );
        }
    }

    #[test]
    fn div_zero_containing_divisor() {
        assert!(iv(1, 5).checked_minkowski_div_hull(iv(-2, 3)).is_none());
    }

    #[test]
    fn div_min_divided_by_neg1() {
        // checked_div returns None for MIN/-1, so overall returns None
        let a = iv(i32::MIN, i32::MIN + 1);
        assert!(a.checked_minkowski_div_hull(iv(-1, 0)).is_none());
    }

    #[test]
    fn div_basic() {
        let c = iv(-4, 10).checked_minkowski_div_hull(iv(2, 5)).unwrap();
        assert_eq!(c.start(), -2);
        assert_eq!(c.end_excl(), 5);
    }

    #[test]
    fn div_contains_corners() {
        let mut state = 12345u64;
        for _ in 0..500 {
            let a = random_iv(&mut state);
            let b = random_iv(&mut state);
            if b.start() <= 0 && b.end_incl() >= 0 {
                continue;
            }
            if let Some(c) = a.checked_minkowski_div_hull(b) {
                for &x in &[a.start(), a.end_incl()] {
                    for &y in &[b.start(), b.end_incl()] {
                        let div = x.checked_div(y).unwrap();
                        assert!(c.start() <= div && div < c.end_excl());
                    }
                }
            }
        }
    }
}

// =========================================================================
// Signed saturating minkowski (incl. regression tests for panic bugs)
// =========================================================================

#[cfg(test)]
mod signed_saturating {
    use super::*;

    #[test]
    fn add_extreme() {
        let a = iv(i32::MAX - 1, i32::MAX);
        assert!(a.saturating_minkowski_add(iv(1, 2)).is_none());
    }

    #[test]
    fn add_min_plus_min() {
        let a = iv(i32::MIN, i32::MIN + 1);
        assert!(a.saturating_minkowski_add(a).is_none());
    }

    #[test]
    fn sub_underflow() {
        let a = iv(i32::MIN, i32::MIN + 1);
        assert!(a.saturating_minkowski_sub(iv(1, 2)).is_none());
    }

    #[test]
    fn mul_extreme() {
        let a = iv(i32::MAX - 1, i32::MAX);
        assert!(a.saturating_minkowski_mul_hull(iv(2, 3)).is_none());
    }

    #[test]
    fn mul_with_small_values_still_valid() {
        let c = iv(-10, 10)
            .saturating_minkowski_mul_hull(iv(-2, 3))
            .unwrap();
        assert!(c.start() < c.end_excl());
    }

    // -- saturating_minkowski_div_hull: regression tests for MIN/-1 panic --

    #[test]
    fn div_hull_divisor_contains_neg1_v1() {
        // Divisor [-3, 0): end_incl == -1, does NOT contain 0.
        // MIN/-1 saturates to MAX; result should be valid and non-panicking.
        let a = iv(i32::MIN, i32::MIN + 1);
        let c = a.saturating_minkowski_div_hull(iv(-3, 0)).unwrap();
        assert!(c.start() < c.end_excl());
        assert_eq!(c.end_excl(), i32::MAX);
    }

    #[test]
    fn div_hull_divisor_contains_neg1_v2() {
        // Divisor [-1, 0): all four corners = MIN/-1 → MAX.
        // start == end_excl == MAX → empty → None.
        let a = iv(i32::MIN, i32::MIN + 1);
        assert!(a.saturating_minkowski_div_hull(iv(-1, 0)).is_none());
    }

    #[test]
    fn div_hull_divisor_contains_neg1_v3() {
        // Divisor [-5, 0): end_incl == -1, does NOT contain 0.
        let a = iv(i32::MIN, i32::MIN + 1);
        let c = a.saturating_minkowski_div_hull(iv(-5, 0)).unwrap();
        assert!(c.start() < c.end_excl());
    }

    // -- saturating_minkowski_div_scalar_hull: regression test for n == -1 ---

    #[test]
    fn div_scalar_hull_min_div_neg1() {
        // Both endpoints are MIN → both saturate to MAX → collapsed to empty.
        let a = iv(i32::MIN, i32::MIN + 1);
        assert!(a.saturating_minkowski_div_scalar_hull(-1).is_none());
    }

    #[test]
    fn div_scalar_hull_normal_interval_neg1() {
        // [-5, 10) / -1 = [-9, 6)
        let c = iv(-5, 10).saturating_minkowski_div_scalar_hull(-1).unwrap();
        assert_eq!(c.start(), -9);
        assert_eq!(c.end_excl(), 6);
    }

    // -- scalar operations --
    #[test]
    fn add_scalar_max() {
        let a = iv(i32::MAX - 1, i32::MAX);
        assert!(a.saturating_minkowski_add_scalar(5).is_none());
    }

    #[test]
    fn sub_scalar_min() {
        let a = iv(i32::MIN, i32::MIN + 1);
        assert!(a.saturating_minkowski_sub_scalar(5).is_none());
    }

    #[test]
    fn mul_scalar_hull_valid() {
        let mut state = 12345u64;
        for _ in 0..500 {
            let a = random_iv(&mut state);
            let n = rand_i32(&mut state);
            if let Some(c) = a.saturating_minkowski_mul_scalar_hull(n) {
                assert!(c.start() < c.end_excl());
            }
        }
    }
}

// =========================================================================
// Unsigned minkowski
// =========================================================================

#[cfg(test)]
mod unsigned_minkowski {
    use super::*;

    #[test]
    fn checked_add_overflow() {
        let a = uiv(u32::MAX - 1, u32::MAX);
        assert!(a.checked_minkowski_add(uiv(1, 2)).is_none());
    }

    #[test]
    fn checked_sub_underflow() {
        assert!(uiv(0, 1).checked_minkowski_sub(uiv(1, 2)).is_none());
    }

    #[test]
    fn checked_div_by_zero_start() {
        assert!(uiv(1, 5).checked_minkowski_div_hull(uiv(0, 3)).is_none());
    }

    #[test]
    fn checked_div_basic() {
        let c = uiv(10, 100).checked_minkowski_div_hull(uiv(2, 5)).unwrap();
        assert_eq!(c.start(), 2);
        assert_eq!(c.end_excl(), 50);
    }

    #[test]
    fn checked_mul_overflow() {
        let a = uiv(u32::MAX / 2, u32::MAX);
        assert!(a.checked_minkowski_mul_hull(uiv(2, 3)).is_none());
    }

    #[test]
    fn saturating_add_clamps() {
        let a = uiv(u32::MAX - 1, u32::MAX);
        assert!(a.saturating_minkowski_add(uiv(5, 6)).is_none());
    }

    #[test]
    fn saturating_sub_clamps() {
        assert!(uiv(0, 1).saturating_minkowski_sub(uiv(5, 6)).is_none());
    }

    #[test]
    fn saturating_div_by_zero_start() {
        assert!(uiv(1, 5).saturating_minkowski_div_hull(uiv(0, 3)).is_none());
    }

    #[test]
    fn saturating_div_scalar_zero() {
        assert!(uiv(1, 5).saturating_minkowski_div_scalar_hull(0).is_none());
    }

    #[test]
    fn all_checked_ops_verify_invariant() {
        let mut state = 12345u64;
        for _ in 0..500 {
            let Some(a) = random_uiv(&mut state) else {
                continue;
            };
            let Some(b) = random_uiv(&mut state) else {
                continue;
            };

            if let Some(c) = a.checked_minkowski_add(b) {
                assert!(c.start() < c.end_excl());
            }
            if let Some(c) = a.checked_minkowski_sub(b) {
                assert!(c.start() < c.end_excl());
            }
            if let Some(c) = a.checked_minkowski_mul_hull(b) {
                assert!(c.start() < c.end_excl());
            }
            if b.start() != 0 {
                if let Some(c) = a.checked_minkowski_div_hull(b) {
                    assert!(c.start() < c.end_excl());
                }
            }
        }
    }

    #[test]
    fn all_saturating_ops_verify_invariant() {
        let mut state = 12345u64;
        for _ in 0..500 {
            let Some(a) = random_uiv(&mut state) else {
                continue;
            };
            let Some(b) = random_uiv(&mut state) else {
                continue;
            };

            if let Some(c) = a.saturating_minkowski_add(b) {
                assert!(c.start() < c.end_excl());
            }
            if let Some(c) = a.saturating_minkowski_sub(b) {
                assert!(c.start() < c.end_excl());
            }
            if let Some(c) = a.saturating_minkowski_mul_hull(b) {
                assert!(c.start() < c.end_excl());
            }
            if b.start() != 0 {
                if let Some(c) = a.saturating_minkowski_div_hull(b) {
                    assert!(c.start() < c.end_excl());
                }
            }
        }
    }

    fn random_uiv(state: &mut u64) -> Option<U32CO> {
        let a = rand_u32(state);
        let b = rand_u32(state);
        U32CO::try_new(a.min(b), a.max(b))
    }
}
