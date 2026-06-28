// Attacks on ChangePoint, HeightSegment, HeightRun types.

use crate::interval::I32CO;
use crate::stack::ChangePoint;
use crate::stack::{HeightRun, HeightSegment};
use core::num::NonZeroUsize;
use std::format;

use super::super::int_co_stack::test_support::iv_i32;

#[test]
fn change_point_at_values() {
    let cp = ChangePoint {
        at: 42i32,
        height_after: 0,
    };
    assert_eq!(cp.at, 42);
    assert_eq!(cp.height_after, 0);
}

#[test]
fn change_point_debug() {
    let cp = ChangePoint {
        at: 0i32,
        height_after: 5,
    };
    let s = format!("{cp:?}");
    assert!(s.contains("0"));
    assert!(s.contains("5"));
}

#[test]
fn change_point_clone_eq() {
    let cp1 = ChangePoint {
        at: 0i32,
        height_after: 5,
    };
    let cp2 = cp1.clone();
    assert_eq!(cp1, cp2);
}

#[test]
fn height_run_from_height_segment() {
    let seg = HeightSegment {
        interval: iv_i32(0, 10),
        height: NonZeroUsize::new(3).unwrap(),
    };
    let run: HeightRun<I32CO> = seg.into();
    assert_eq!(run.interval, iv_i32(0, 10));
    assert_eq!(run.height, 3);
}

#[test]
fn height_segment_fields() {
    let seg = HeightSegment {
        interval: iv_i32(0, 5),
        height: NonZeroUsize::new(2).unwrap(),
    };
    assert_eq!(seg.interval.start(), 0);
    assert_eq!(seg.height.get(), 2);
}

#[test]
fn height_run_fields() {
    let run = HeightRun {
        interval: iv_i32(10, 20),
        height: 0,
    };
    assert_eq!(run.interval.start(), 10);
    assert_eq!(run.height, 0);
}

#[test]
fn height_segment_clone_eq() {
    let seg = HeightSegment {
        interval: iv_i32(0, 5),
        height: NonZeroUsize::new(2).unwrap(),
    };
    assert_eq!(seg, seg.clone());
}

#[test]
fn height_run_clone_eq() {
    let run = HeightRun {
        interval: iv_i32(0, 5),
        height: 1,
    };
    assert_eq!(run, run.clone());
}
