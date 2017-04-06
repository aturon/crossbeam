//! Epoch-based memory management.
//!
//! This module provides fast, easy to use memory management for lock free data structures. It's
//! inspired by [Keir Fraser's *epoch-based
//! reclamation*](https://www.cl.cam.ac.uk/techreports/UCAM-CL-TR-579.pdf).
//!
//! The basic problem this is solving is the fact that when one thread has removed a node from a
//! data structure, other threads may still have pointers to that node (in the form of snapshots
//! that will be validated through things like compare-and-swap), so the memory cannot be
//! immediately freed. Put differently:
//!
//! 1. There are two sources of reachability at play -- the data structure, and the snapshots in
//!    threads accessing it. Before we delete a node, we need to know that it cannot be reached in
//!    either of these ways.
//!
//! 2. Once a node has been unlinked from the data structure, no *new* snapshots reaching it will
//!    be created.
//!
//! Using the epoch scheme is fairly straightforward, and does not require understanding any of the
//! implementation details:
//!
//! - When operating on a shared data structure, a thread must "pin the current epoch", which is
//!   done by calling `pin()`. This function returns a `Guard` which unpins the epoch when
//!   destroyed.
//!
//! - When the thread subsequently reads from a lock-free data structure, the pointers it extracts
//!   act like references with lifetime tied to the `Guard`. This allows threads to safely read
//!   from snapshotted data, being guaranteed that the data will remain allocated until they exit
//!   the epoch.
//!
//! To put the `Guard` to use, Crossbeam provides a set of three pointer types meant to work together:
//!
//! - `Shared<'a, T>`, akin to `&'a T`, which points to shared data that may or may not be
//!   reachable from a data structure, but it guaranteed not to be freed during lifetime `'a`.
//!
//! - `Atomic<T>`, akin to `std::sync::atomic::AtomicPtr`, which provides atomic updates to a
//!   pointer using the `Shared` types, and connects them to a `Guard`.
//!
//! Each of these types provides further documentation on usage.
//!
//! # Example
//!
//! ```
//! use std::sync::atomic::Ordering::{Acquire, Release, Relaxed};
//! use std::ptr;
//!
//! use crossbeam::epoch::{self, Atomic};
//!
//! struct TreiberStack<T> {
//!     head: Atomic<Node<T>>,
//! }
//!
//! struct Node<T> {
//!     data: T,
//!     next: Atomic<Node<T>>,
//! }
//!
//! impl<T> TreiberStack<T> {
//!     fn new() -> TreiberStack<T> {
//!         TreiberStack {
//!             head: Atomic::null()
//!         }
//!     }
//!
//!     fn push(&self, t: T) {
//!         // allocate the node via Box.
//!         let mut n = Box::new(Node {
//!             data: t,
//!             next: Atomic::null(),
//!         });
//!
//!         // become active
//!         let guard = epoch::pin();
//!
//!         loop {
//!             // snapshot current head
//!             let head = self.head.load(Relaxed, &guard);
//!
//!             // update `next` pointer with snapshot
//!             n.next.store_shared(head, Relaxed);
//!
//!             // if snapshot is still good, link in the new node
//!             match self.head.compare_and_set_ref(head, n, Release, &guard) {
//!                 Ok(_) => return,
//!                 Err(owned) => n = owned,
//!             }
//!         }
//!     }
//!
//!     fn pop(&self) -> Option<T> {
//!         // become active
//!         let guard = epoch::pin();
//!
//!         loop {
//!             // take a snapshot
//!             match self.head.load(Acquire, &guard) {
//!                 // the stack is non-empty
//!                 Some(head) => {
//!                     // read through the snapshot, *safely*!
//!                     let next = head.next.load(Relaxed, &guard);
//!
//!                     // if snapshot is still good, update from `head` to `next`
//!                     if self.head.compare_and_set_shared(Some(head), next, Release) {
//!                         unsafe {
//!                             // mark the node as unlinked
//!                             guard.unlinked(head);
//!
//!                             // extract out the data from the now-unlinked node
//!                             return Some(ptr::read(&(*head).data))
//!                         }
//!                     }
//!                 }
//!
//!                 // we observed the stack empty
//!                 None => return None
//!             }
//!         }
//!     }
//! }
//! ```

// FIXME: document implementation details

mod atomic;
mod garbage;
mod global;
mod guard;
mod local;
mod participant;
mod participants;

pub use self::atomic::Atomic;
pub use self::guard::{pin, Guard, Pinned};

use std::ops;

/// Like `&'a T`: a shared reference valid for lifetime `'a`.
///
/// This implicitly asserts that an epoch is active in this thread for its lifetime, which is what
/// makes this different from `&'a T`.
#[derive(Debug, PartialEq, Eq)]
pub struct Shared<'a, T: 'a> {
    /// The inner reference.
    data: &'a T,
}

impl<'a, T> Shared<'a, T> {
    /// Create a `Shared` pointer from a reference.
    ///
    /// # Safety
    ///
    /// This is unsafe as it assumes the call side assure an epoch is valid for `'a`.
    unsafe fn from_ptr(r: *const T) -> Shared<'a, T> {
        Shared {
            data: &*r,
        }
    }

    /// Map the pointer to some other pointer.
    ///
    /// This applies `f` to the reference and returns the result wrapped in `Shared`.
    ///
    /// # Safety
    ///
    /// This is safe as it preserves the invariant that `'a` spans an epoch pin.
    pub fn map<U: 'a, F>(self, f: F) -> Shared<'a, U>
        where F: FnOnce(&'a T) -> &'a U {
        unsafe { Shared::from_ptr(f(&self)) }
    }

    /// Obtain the raw pointer to the inner data.
    ///
    /// # Safety
    ///
    /// Creating a raw pointer is not unsafe, but using it is.
    pub fn as_raw(&self) -> *mut T {
        self.data as *const _ as *mut _
    }

    /// Convert it to a owned guard/pinned reference.
    ///
    /// This converts `self` into a value holding both `guard` and the reference, getting rid of
    /// the bound epoch guard by instead storing it (owned) in the return value.
    pub fn into_pinned(self, guard: Guard) -> Pinned<&'a T> {
        Pinned::new(self.data, guard)
    }
}

impl<'a, T> Copy for Shared<'a, T> {}
impl<'a, T> Clone for Shared<'a, T> {
    fn clone(&self) -> Shared<'a, T> {
        Shared { data: self.data }
    }
}

impl<'a, T> ops::Deref for Shared<'a, T> {
    type Target = &'a T;
    fn deref(&self) -> &&'a T {
        &self.data
    }
}

#[cfg(test)]
mod test {
    use std::sync::atomic::Ordering;
    use super::*;
    use epoch;

    #[test]
    fn test_no_drop() {
        static mut DROPS: i32 = 0;
        struct Test;
        impl Drop for Test {
            fn drop(&mut self) {
                unsafe {
                    DROPS += 1;
                }
            }
        }
        let g = pin();

        let x = Atomic::null();
        x.store(Some(Box::new(Test)), Ordering::Relaxed);
        x.store_and_ref(Box::new(Test), Ordering::Relaxed, &g);
        let y = x.load(Ordering::Relaxed, &g);
        let z = x.compare_and_set_ref(y, Box::new(Test), Ordering::Relaxed, &g).ok();
        let _ = x.compare_and_set(z, Some(Box::new(Test)), Ordering::Relaxed);
        x.swap(Some(Box::new(Test)), Ordering::Relaxed, &g);

        unsafe {
            assert_eq!(DROPS, 0);
        }
    }

    #[test]
    fn test_new() {
        let guard = epoch::pin();
        let my_atomic = Atomic::new(42);

        assert_eq!(**my_atomic.load(Ordering::Relaxed, &guard).unwrap(), 42);
    }
}
