//! Support for concurrent and parallel programming.
//!
//! This crate is an early work in progress. The focus for the moment is concurrency:
//!
//! - **Non-blocking data structures**. These data structures allow for high performance,
//! highly-concurrent access, much superior to wrapping with a `Mutex`. Ultimately the goal is to
//! include stacks, queues, deques, bags, sets and maps. These live in the `sync` module.
//!
//! - **Memory management**. Because non-blocking data structures avoid global synchronization, it
//! is not easy to tell when internal data can be safely freed. The `mem` module provides generic,
//! easy to use, and high-performance APIs for managing memory in these cases. These live in the
//! `mem` module.
//!
//! - **Synchronization**. The standard library provides a few synchronization primitives (locks,
//! semaphores, barriers, etc) but this crate seeks to expand that set to include more
//! advanced/niche primitives, as well as userspace alternatives. These live in the `sync` module.
//!
//! - **Scoped thread API**. Finally, the crate provides a "scoped" thread API, making it possible
//! to spawn threads that share stack data with their parents. This functionality is exported at
//! the top-level.

#![deny(missing_docs)]
#![cfg_attr(feature = "nightly",
            feature(const_fn, repr_simd, optin_builtin_traits))]

#[macro_use]
extern crate lazy_static;

pub mod epoch;
pub mod sync;

mod cache_padded;
pub use self::cache_padded::{CachePadded, ZerosValid};
