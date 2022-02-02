#![cfg_attr(not(target_arch = "x86_64"), no_std)]
#![cfg_attr(not(target_arch = "x86_64"), no_main)]
#![cfg_attr(not(target_arch = "x86_64"), feature(custom_test_frameworks, lang_items, start))]

extern crate libc;
extern crate rand;
extern crate rand_core;
extern crate rand_isaac;

pub mod chapter_2;


#[cfg(target_arch = "x86_64")]
use std::{ops::{Add, BitAnd, BitOr, BitOrAssign, BitXor, Not, Shl, ShlAssign, Shr, ShrAssign, Sub}, hash::{Hash, Hasher}, cmp::{Ord, Ordering, PartialOrd}, fmt::{Binary, Display, Formatter, LowerHex, Octal, UpperHex}};

#[cfg(target_arch = "riscv64")]
use core::{ops::{Add, BitAnd, BitOr, BitOrAssign, BitXor, Not, Shl, ShlAssign, Shr, ShrAssign, Sub}, hash::{Hash, Hasher}, cmp::{Ord, Ordering, PartialOrd}, fmt::{Binary, Display, Formatter, LowerHex, Octal, UpperHex}};

#[cfg(target_arch = "x86_64")]
pub use std::fmt::Error;
#[cfg(target_arch = "riscv64")]
pub use core::fmt::Error;

macro_rules! implement_from {
    {[($name:ident),*], [$($from:ident),*] } => {$(implement_from!($name, $from);)*};
    {$name:ident, [$($from:ident),*] } => {$(implement_from!($name, $from);)*};
    {[$($name:ident),*], $from:ident } => {$(implement_from!($name, $from);)*};
    {$name:ident, $from: ty} => {
        impl From<$from> for $name {
            fn from(x: $from) -> $name {
                $name(x.into())
            }
        }
    };
}
