// Attacks on OneTwo / ZeroOneTwo containers.

use crate::interval::res::{OneTwo, ZeroOneTwo};
use std::vec;
use std::vec::Vec;

#[test]
fn one_two_into_iter_one() {
    let ot: OneTwo<i32> = OneTwo::One(42);
    let v: Vec<i32> = ot.into_iter().collect();
    assert_eq!(v, vec![42]);
}

#[test]
fn one_two_into_iter_two() {
    let ot: OneTwo<i32> = OneTwo::Two(10, 20);
    let v: Vec<i32> = ot.into_iter().collect();
    assert_eq!(v, vec![10, 20]);
}

#[test]
fn one_two_double_ended() {
    let ot: OneTwo<i32> = OneTwo::Two(10, 20);
    let mut iter = ot.into_iter();
    assert_eq!(iter.next(), Some(10));
    assert_eq!(iter.next_back(), Some(20));
    assert_eq!(iter.next(), None);
}

#[test]
fn one_two_exact_size() {
    let ot: OneTwo<i32> = OneTwo::Two(10, 20);
    let iter = ot.into_iter();
    assert_eq!(iter.len(), 2);
}

#[test]
fn zero_one_two_into_iter() {
    let z: ZeroOneTwo<i32> = ZeroOneTwo::Zero;
    assert_eq!(z.into_iter().count(), 0);

    let o: ZeroOneTwo<i32> = ZeroOneTwo::One(42);
    assert_eq!(o.into_iter().collect::<Vec<_>>(), vec![42]);

    let t: ZeroOneTwo<i32> = ZeroOneTwo::Two(10, 20);
    assert_eq!(t.into_iter().collect::<Vec<_>>(), vec![10, 20]);
}
