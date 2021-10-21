#![doc = include_str!("../Readme.md")]
#![warn(missing_docs)]
#![forbid(unsafe_code)]
#![allow(dead_code)] // XXX
#![allow(unused_imports)] // XXX

#[path = "read/_read.rs"]   mod read;   pub use read::*;
