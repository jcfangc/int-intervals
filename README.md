# int-intervals

Closed-open integer interval algebra, canonical interval sets, and overlap-stack structures.

## Modules

- **`interval`** — Half-open interval algebra for primitive integer types. `no_std` by default.
- **`set`** (feature `set`) — Immutable canonical interval sets (`IntCOSet`), `Arc`-backed.
- **`stack`** (feature `stack`) — Height/overlap stacks (`IntCOStack`), window iterators, and change-point analysis.

## Usage

```rust
use int_intervals::I32CO;

let a = I32CO::try_new(0, 10).unwrap();
let b = I32CO::try_new(5, 15).unwrap();
assert_eq!(a.intersection(b), Some(I32CO::try_new(5, 10).unwrap()));
```

With features:

```rust,ignore
#[cfg(feature = "set")]
use int_intervals::I32COSet;

#[cfg(feature = "stack")]
use int_intervals::I32COStack;
```

## Features

| Feature | Description | Extra deps |
|---------|-------------|-----------|
| (none)  | Core interval algebra | None |
| `set`   | Canonical interval sets | `rayon` |
| `stack` | Overlap stacks & windows | `rayon`, `either` |

## License

MIT OR Apache-2.0
