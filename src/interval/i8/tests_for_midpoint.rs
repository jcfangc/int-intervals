use super::*;

mod unit_tests {
    use super::*;

    #[test]
    fn midpoint_basic() {
        let iv = I8CO::try_new(0, 5).unwrap();
        assert_eq!(iv.midpoint(), 2);

        let iv = I8CO::try_new(-5, 5).unwrap();
        assert_eq!(iv.midpoint(), 0);

        let iv = I8CO::try_new(i8::MIN, i8::MAX).unwrap();
        assert_eq!(iv.midpoint(), i8::MIN + ((iv.len() / 2) as i8));
    }

    #[test]
    fn checked_from_midpoint_len_basic() {
        let mid = 0;
        let len = 5;
        let iv = I8CO::checked_from_midpoint_len(mid, len).unwrap();
        assert_eq!(iv.len(), len);
        assert_eq!(iv.midpoint(), mid);

        let mid = 10;
        let len = 4;
        let iv = I8CO::checked_from_midpoint_len(mid, len).unwrap();
        assert_eq!(iv.len(), len);
        assert_eq!(iv.midpoint(), mid);

        assert!(I8CO::checked_from_midpoint_len(mid, 0).is_none());
        assert!(I8CO::checked_from_midpoint_len(i8::MIN, 10).is_none());
        assert!(I8CO::checked_from_midpoint_len(i8::MAX, 10).is_none());
    }

    #[test]
    fn saturating_from_midpoint_len_basic() {
        let mid = 0;
        let len = 5;
        let iv = I8CO::saturating_from_midpoint_len(mid, len).unwrap();
        assert_eq!(iv.len(), len);
        assert_eq!(iv.midpoint(), mid);

        assert!(I8CO::saturating_from_midpoint_len(mid, 0).is_none());

        let iv = I8CO::saturating_from_midpoint_len(i8::MIN, 10).unwrap();
        assert!(iv.start() >= i8::MIN);

        let iv = I8CO::saturating_from_midpoint_len(i8::MAX, 10).unwrap();
        assert!(iv.end_excl() <= i8::MAX);
    }
}

mod prop_tests {
    use proptest::prelude::*;

    use super::*;

    fn mixed_scalar() -> impl Strategy<Value = i8> {
        prop_oneof![3 => prop::sample::select(&[i8::MIN, i8::MAX, 0, -1, 1]), 7 => any::<i8>()]
    }

    proptest! {
        #[test]
        fn prop_midpoint_in_bounds(start in any::<i8>(), end in any::<i8>()) {
            if let Some(iv) = I8CO::try_new(start, end) {
                let mp = iv.midpoint();
                prop_assert!(mp >= iv.start());
                prop_assert!(mp <= iv.end_incl());
            }
        }

        #[test]
        fn prop_checked_from_midpoint_len_inverse(mid in mixed_scalar(), len in 1u8..=255) {
            if let Some(iv) = I8CO::checked_from_midpoint_len(mid, len) {
                prop_assert_eq!(iv.len(), len);
                prop_assert_eq!(iv.midpoint(), mid);
            }
        }

        #[test]
        fn prop_saturating_from_midpoint_len_safety(mid in mixed_scalar(), len in 1u8..=255) {
            if let Some(iv) = I8CO::saturating_from_midpoint_len(mid, len) {
                prop_assert!(iv.start() < iv.end_excl());
                prop_assert_eq!(iv.midpoint(), iv.start() + (iv.len()/2) as i8);
            }
        }
    }
}
