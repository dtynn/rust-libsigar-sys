//! wrapper of libsigar

#![warn(missing_docs)]

extern crate libc;

#[link(name = "sigar")]
mod sigar;

pub use sigar::*;
