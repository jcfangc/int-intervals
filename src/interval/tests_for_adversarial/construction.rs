// Construction boundary attacks for signed (i32) and unsigned (u32) intervals.

use crate::interval::I32CO;
use crate::interval::U32CO;

#[cfg(test)]
mod i32_construction {
    use super::*;

    #[test]
    fn try_new_full_range_succeeds() {
        let iv = I32CO::try_new(i32::MIN, i32::MAX);
        assert!(iv.is_some());
    }

    #[test]
    fn try_new_reversed_fails() {
        assert!(I32CO::try_new(i32::MAX, i32::MIN).is_none());
        assert!(I32CO::try_new(5, 5).is_none());
        assert!(I32CO::try_new(0, -1).is_none());
    }

    #[test]
    fn try_new_empty_fails() {
        for x in [i32::MIN, -1, 0, 1, i32::MAX] {
            assert!(
                I32CO::try_new(x, x).is_none(),
                "try_new({x},{x}) should be None"
            );
        }
    }

    #[test]
    fn checked_from_start_len_min_with_max_length() {
        let iv = I32CO::checked_from_start_len(i32::MIN, u32::MAX).unwrap();
        assert_eq!(iv.start(), i32::MIN);
        assert_eq!(iv.end_excl(), i32::MAX);
    }

    #[test]
    fn checked_from_start_len_max_plus_one_fails() {
        assert!(I32CO::checked_from_start_len(i32::MAX, 1).is_none());
    }

    #[test]
    fn checked_from_start_len_zero_len_fails() {
        assert!(I32CO::checked_from_start_len(0, 0).is_none());
    }

    #[test]
    fn checked_from_start_len_negative_start_cross_zero() {
        let iv = I32CO::checked_from_start_len(-3, 5).unwrap();
        assert_eq!(iv.start(), -3);
        assert_eq!(iv.end_excl(), 2);
        assert_eq!(iv.len(), 5);
    }

    #[test]
    fn checked_from_start_len_min_plus_one() {
        let iv = I32CO::checked_from_start_len(i32::MIN, 1).unwrap();
        assert_eq!(iv.start(), i32::MIN);
        assert_eq!(iv.end_excl(), i32::MIN.wrapping_add(1));
    }

    #[test]
    fn saturating_from_start_len_max_fails() {
        assert!(I32CO::saturating_from_start_len(i32::MAX, 1).is_none());
    }

    #[test]
    fn saturating_from_start_len_max_minus_one_succeeds() {
        let iv = I32CO::saturating_from_start_len(i32::MAX - 1, 100).unwrap();
        assert_eq!(iv.start(), i32::MAX - 1);
        assert_eq!(iv.end_excl(), i32::MAX);
    }

    #[test]
    fn saturating_from_start_len_zero_len_fails() {
        assert!(I32CO::saturating_from_start_len(0, 0).is_none());
    }

    #[test]
    fn checked_from_midpoint_len_min_with_max_len() {
        assert!(I32CO::checked_from_midpoint_len(i32::MIN, u32::MAX).is_none());
    }

    #[test]
    fn checked_from_midpoint_len_max_with_max_len() {
        assert!(I32CO::checked_from_midpoint_len(i32::MAX, u32::MAX).is_none());
    }

    #[test]
    fn checked_from_midpoint_len_len_one() {
        let iv = I32CO::checked_from_midpoint_len(42, 1).unwrap();
        assert_eq!(iv.start(), 42);
        assert_eq!(iv.end_excl(), 43);
    }

    #[test]
    fn checked_from_midpoint_len_cross_zero() {
        let iv = I32CO::checked_from_midpoint_len(-1, 5).unwrap();
        assert_eq!(iv.start(), -3);
        assert_eq!(iv.end_excl(), 2);
    }

    #[test]
    fn saturating_from_midpoint_len_min_clamps() {
        let iv = I32CO::saturating_from_midpoint_len(i32::MIN, 100).unwrap();
        assert_eq!(iv.start(), i32::MIN);
        assert!(iv.start() < iv.end_excl());
    }

    #[test]
    fn saturating_from_midpoint_len_max_clamps() {
        let iv = I32CO::saturating_from_midpoint_len(i32::MAX, 100).unwrap();
        assert_eq!(iv.end_excl(), i32::MAX);
        assert!(iv.start() < iv.end_excl());
    }

    #[test]
    fn try_from_range_valid() {
        use core::convert::TryFrom;
        let iv = I32CO::try_from(0..5).unwrap();
        assert_eq!(iv.start(), 0);
        assert_eq!(iv.end_excl(), 5);
    }

    #[test]
    fn try_from_range_empty() {
        use core::convert::TryFrom;
        assert!(I32CO::try_from(5..5).is_err());
        assert!(I32CO::try_from(10..0).is_err());
    }
}

#[cfg(test)]
mod u32_construction {
    use super::*;

    #[test]
    fn try_new_full_range_succeeds() {
        let iv = U32CO::try_new(0, u32::MAX);
        assert!(iv.is_some());
    }

    #[test]
    fn try_new_reversed_fails() {
        assert!(U32CO::try_new(5, 0).is_none());
        assert!(U32CO::try_new(5, 5).is_none());
    }

    #[test]
    fn checked_from_start_len_max_fails() {
        assert!(U32CO::checked_from_start_len(u32::MAX, 1).is_none());
    }

    #[test]
    fn checked_from_start_len_zero_len_fails() {
        assert!(U32CO::checked_from_start_len(0, 0).is_none());
    }

    #[test]
    fn checked_from_start_len_max_minus_one_ok() {
        let iv = U32CO::checked_from_start_len(u32::MAX - 1, 1).unwrap();
        assert_eq!(iv.start(), u32::MAX - 1);
        assert_eq!(iv.end_excl(), u32::MAX);
    }

    #[test]
    fn saturating_from_start_len_max_fails() {
        assert!(U32CO::saturating_from_start_len(u32::MAX, 1).is_none());
    }

    #[test]
    fn checked_from_midpoint_len_zero() {
        assert!(U32CO::checked_from_midpoint_len(0, 5).is_none());
    }

    #[test]
    fn checked_from_midpoint_len_max_overflow() {
        assert!(U32CO::checked_from_midpoint_len(u32::MAX, u32::MAX).is_none());
    }
}
