#![cfg_attr(not(target_arch = "x86_64"), no_std)]
#![cfg_attr(not(target_arch = "x86_64"), no_main)]
#![cfg_attr(not(target_arch = "x86_64"), feature(custom_test_frameworks, lang_items, start))]

extern crate libc;
extern crate rand;
extern crate rand_core;
extern crate rand_isaac;

pub mod chapter_2;
