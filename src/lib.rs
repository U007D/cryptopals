#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

#[cfg(test)] mod unit_tests;

extern crate fluent_validator;
extern crate owned_chars;

mod hex_char_byte_string;
mod byte_buffer;
mod hex_char;
