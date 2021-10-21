#![doc = include_str!("../Readme.md")]
#![forbid(unsafe_code)]
#![cfg_attr(target_os = "wasi", allow(unused_imports))]

#[path = "read/_read.rs"]   mod read;   pub use read::*;

#[doc(no_inline)] pub use read_write_at::{ReadAt};
