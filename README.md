# int-intervals

[![Crates.io](https://img.shields.io/crates/v/int-intervals.svg)](https://crates.io/crates/int-intervals)
[![Documentation](https://docs.rs/int-intervals/badge.svg)](https://docs.rs/int-intervals)
[![License](https://img.shields.io/crates/l/int-intervals.svg)](https://crates.io/crates/int-intervals)
[![CI](https://github.com/jcfangc/int-intervals/actions/workflows/gate.yml/badge.svg)](https://github.com/jcfangc/int-intervals/actions/workflows/gate.yml)
[![CodSpeed](https://github.com/jcfangc/int-intervals/actions/workflows/codspeed.yml/badge.svg)](https://github.com/jcfangc/int-intervals/actions/workflows/codspeed.yml)
[![Coverage](https://codecov.io/gh/jcfangc/int-intervals/branch/main/graph/badge.svg)](https://codecov.io/gh/jcfangc/int-intervals)

A `no_std`-friendly closed-open integer interval algebra, canonical interval-set
containers, and overlap-stack structures.

This crate consolidates and supersedes [`int-interval`][], [`int-interval-set`][],
and [`int-interval-stack`][].

[`int-interval`]: https://crates.io/crates/int-interval
[`int-interval-set`]: https://crates.io/crates/int-interval-set
[`int-interval-stack`]: https://crates.io/crates/int-interval-stack

---

## Quick start

```rust
use int_intervals::I32CO;

// Core interval algebra — always available, zero deps.
let a = I32CO::try_new(0, 10).unwrap();
let b = I32CO::try_new(5, 15).unwrap();
assert_eq!(a.intersection(b), Some(I32CO::try_new(5, 10).unwrap()));
assert_eq!(a.convex_hull(b), I32CO::try_new(0, 15).unwrap());

// With `set` feature: canonical interval sets.
#[cfg(feature = "set")]
{
    use int_intervals::I32COSet;
    let set: I32COSet = [a, b].into_iter().collect();
    assert!(set.contains_point(7));
}

// With `stack` feature: overlap-multiplicity stacks.
#[cfg(feature = "stack")]
{
    use int_intervals::I32COStack;
    let stack: I32COStack = [a, b].into_iter().collect();
    assert_eq!(stack.height_at(7), 2); // covered by both a and b
}
```

### Migration from the old crates

| Old crate | New Cargo.toml | New `use` |
|-----------|----------------|-----------|
| `int-interval = "0.9"` | `int-intervals = "0.1"` | `use int_intervals::*;` |
| `int-interval-set = "0.3"` | `int-intervals = { version = "0.1", features = ["set"] }` | `use int_intervals::IntCOSet;` |
| `int-interval-stack = "0.3"` | `int-intervals = { version = "0.1", features = ["stack"] }` | `use int_intervals::IntCOStack;` |

All type aliases (`I32CO`, `U8COSet`, `I16COStack`, etc.) are re-exported at the crate root.

---

## Modules

| Module | Availability | Description |
|--------|-------------|-------------|
| `interval` | Always | Half-open `[start, end_excl)` algebra for 12 primitive integer types |
| `set` | `features = ["set"]` | Immutable canonical interval sets (`Arc<[I]>`-backed) |
| `stack` | `features = ["stack"]` | Overlap-multiplicity stack, change-point analysis, window iteration |

---

## Interval model

Intervals are `[start, end_excl)` — closed at the start, open at the end:

```
start < end_excl
len = end_excl - start
```

Empty or reversed intervals are not representable. Construction returns `Option`:

```rust
let x = I32CO::try_new(2, 8).unwrap();   // [2, 8)
let y = I32CO::try_new(5, 3);            // None
```

12 concrete types are provided:

```
U8CO/I8CO, U16CO/I16CO, U32CO/I32CO, U64CO/I64CO,
U128CO/I128CO, UsizeCO/IsizeCO
```

---

## Interval algebra

Core operations return fixed-capacity containers — no heap allocation:

| Operation | Result |
|-----------|--------|
| `intersection` | `Option<T>` |
| `convex_hull` | `T` |
| `between` | `Option<T>` |
| `union` | `OneTwo<T>` → iterates 1–2 values |
| `difference` | `ZeroOneTwo<T>` → iterates 0–2 values |
| `symmetric_difference` | `ZeroOneTwo<T>` → iterates 0–2 values |

```rust
let a = I32CO::try_new(2, 10).unwrap();
let b = I32CO::try_new(4, 6).unwrap();

let residues: Vec<_> = a.difference(b).into_iter().collect();
assert_eq!(residues.len(), 2);
```

All result containers implement `Iterator`, `DoubleEndedIterator`, `ExactSizeIterator`, `FusedIterator`.

### Minkowski arithmetic

Checked (exact) and saturating (clamped) Minkowski operations are provided for both linear images and non-linear hulls:

```rust
a.checked_minkowski_add(b);               // exact sum
a.checked_minkowski_mul_hull(b);          // containing hull after multiplication
a.saturating_minkowski_add(b);            // clamped to representable domain
a.saturating_minkowski_mul_scalar_hull(3); // clamped scalar hull
```

### Generic programming

The `IntCO` trait bundles the core capabilities needed by downstream algorithms:

```rust
fn residual_measure<I>(lhs: I, rhs: I) -> I::MeasureType
where
    I: IntCO,
{
    lhs.difference(rhs).into_iter().map(|iv| iv.len()).sum()
}
```

For a full trait catalogue, see the [docs](https://docs.rs/int-intervals).

---

## Interval sets (`set` feature)

`IntCOSet<I>` is an immutable canonical set of half-open intervals, stored as `Arc<[I]>`.

### Construction

```rust
use int_intervals::I32CO;
use int_intervals::I32COSet;

let set: I32COSet = [
    I32CO::try_new(10, 20).unwrap(),
    I32CO::try_new(15, 30).unwrap(),
    I32CO::try_new(40, 50).unwrap(),
]
.into_iter()
.collect();

// Automatically canonicalized: overlapping [10,20) + [15,30) → [10,30)
assert_eq!(set.as_slice(), &[
    I32CO::try_new(10, 30).unwrap(),
    I32CO::try_new(40, 50).unwrap(),
]);
```

Parallel construction via Rayon is available with the `parallel` feature.

### Queries

```rust
// Predicates
set.contains_point(18);
set.contains_interval(query);
set.intersects_interval(query);

// Search
set.interval_containing_point(18);          // O(log n)
set.intervals_intersecting(query);           // O(log n + k)

// Algebra (set vs interval)
set.intersection_with_interval(query);
set.union_with_interval(query);
set.difference_with_interval(query);

// Algebra (set vs set)
left.intersection_with_set(&right);          // O(n + m)
left.union_with_set(&right);
left.difference_with_set(&right);
left.symmetric_difference_with_set(&right);

// Coverage
set.covered_len_of(query);                  // exact length
set.coverage_ratio_f32_of(query);           // 0.0 .. 1.0
```

---

## Interval stacks (`stack` feature)

`IntCOStack<I>` builds a canonical piecewise-constant height function from interval endpoints.

```
input:  [0, 10), [3, 7), [5, 12)
height:   1        2        3        1     0
          ├────────┼────────┼────────┼─────┤
          0        3    5    7   10   12
```

### Construction and queries

```rust
use int_intervals::{I32CO, IntCOStack};

fn iv(s: i32, e: i32) -> I32CO { I32CO::try_new(s, e).unwrap() }

let stack: IntCOStack = [iv(0, 10), iv(3, 7), iv(5, 12)]
    .into_iter().collect();

assert_eq!(stack.height_at(2), 1);
assert_eq!(stack.height_at(4), 2);
assert_eq!(stack.height_at(6), 3);
assert_eq!(stack.height_stats().max_height(), 3);
```

### Height segments

```rust
let segments: Vec<_> = stack.iter_height_segments()
    .map(|seg| ((seg.interval.start(), seg.interval.end_excl()), seg.height.get()))
    .collect();

assert_eq!(segments, vec![
    ((0, 3), 1), ((3, 7), 2), ((7, 10), 1), ((10, 12), 0),
]);
```

Filtered iterators: `at_least(n)`, `at_most(n)`, `exactly(n)`, `between(lo, hi)`, `peak`.

### Covered set

`stack.covered()` lazily projects the stack to an `IntCOSet` containing all coordinates with positive height. The result is cached after the first call.

### Window iteration

Slide a fixed-length window across `[from, to)` and decompose each position into constant-height runs:

```rust
let windows: Vec<_> = stack.iter_windows(0, 10, 4)
    .map(|w| {
        let runs: Vec<_> = w.iter_height_runs()
            .map(|r| ((r.interval.start(), r.interval.end_excl()), r.height))
            .collect();
        ((w.interval().start(), w.interval().end_excl()), runs)
    })
    .collect();
```

Parallel window iteration is available with the `parallel` feature.

---

## Features

| Feature | What you get | Extra deps |
|---------|-------------|-----------|
| *(none)* | Core interval algebra (`no_std`, zero deps) | None |
| `set` | `IntCOSet<I>`, canonical set algebra | `alloc` |
| `stack` | `IntCOStack<I>`, height functions, window iterators | `alloc`, `once_cell`, `either` |
| `parallel` | Rayon parallel construction and iteration | `rayon`, `std` |

---

## License

MIT OR Apache-2.0
