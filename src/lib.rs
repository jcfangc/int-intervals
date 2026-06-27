#![no_std]
#[cfg(any(feature = "set", feature = "stack"))]
extern crate alloc;
#[cfg(test)]
extern crate std;

pub mod interval;

#[cfg(feature = "set")]
pub mod set;

#[cfg(feature = "stack")]
pub mod stack;

// Re-export interval types at crate root for convenience.
pub use interval::traits::{IntCO, IntPrimitive, UnsignedPrimitive};
pub use interval::{EmptyRangeError, OneTwo, ZeroOneTwo};
pub use interval::{
    I8CO, I16CO, I32CO, I64CO, I128CO, IsizeCO, U8CO, U16CO, U32CO, U64CO, U128CO, UsizeCO,
};

#[cfg(feature = "set")]
pub use set::IntCOSet;

#[cfg(feature = "set")]
pub type I8COSet = IntCOSet<I8CO>;
#[cfg(feature = "set")]
pub type I16COSet = IntCOSet<I16CO>;
#[cfg(feature = "set")]
pub type I32COSet = IntCOSet<I32CO>;
#[cfg(feature = "set")]
pub type I64COSet = IntCOSet<I64CO>;
#[cfg(feature = "set")]
pub type I128COSet = IntCOSet<I128CO>;
#[cfg(feature = "set")]
pub type IsizeCOSet = IntCOSet<IsizeCO>;
#[cfg(feature = "set")]
pub type U8COSet = IntCOSet<U8CO>;
#[cfg(feature = "set")]
pub type U16COSet = IntCOSet<U16CO>;
#[cfg(feature = "set")]
pub type U32COSet = IntCOSet<U32CO>;
#[cfg(feature = "set")]
pub type U64COSet = IntCOSet<U64CO>;
#[cfg(feature = "set")]
pub type U128COSet = IntCOSet<U128CO>;
#[cfg(feature = "set")]
pub type UsizeCOSet = IntCOSet<UsizeCO>;

#[cfg(feature = "stack")]
pub use stack::{ChangePoint, HeightRun, HeightSegment, HeightStats, IntCOStack, StackWindow};

#[cfg(feature = "stack")]
pub type I8COStack = IntCOStack<I8CO>;
#[cfg(feature = "stack")]
pub type I16COStack = IntCOStack<I16CO>;
#[cfg(feature = "stack")]
pub type I32COStack = IntCOStack<I32CO>;
#[cfg(feature = "stack")]
pub type I64COStack = IntCOStack<I64CO>;
#[cfg(feature = "stack")]
pub type I128COStack = IntCOStack<I128CO>;
#[cfg(feature = "stack")]
pub type IsizeCOStack = IntCOStack<IsizeCO>;
#[cfg(feature = "stack")]
pub type U8COStack = IntCOStack<U8CO>;
#[cfg(feature = "stack")]
pub type U16COStack = IntCOStack<U16CO>;
#[cfg(feature = "stack")]
pub type U32COStack = IntCOStack<U32CO>;
#[cfg(feature = "stack")]
pub type U64COStack = IntCOStack<U64CO>;
#[cfg(feature = "stack")]
pub type U128COStack = IntCOStack<U128CO>;
#[cfg(feature = "stack")]
pub type UsizeCOStack = IntCOStack<UsizeCO>;
