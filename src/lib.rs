#![cfg_attr(test, deny(warnings))]
#![deny(missing_docs)]

//! # unreachable
//!
//! An unreachable code optimization hint in stable rust, and some useful
//! extension traits for `Option` and `Result`.
//!

extern crate void;

/// Hint to the optimizer that any code path which calls this function is
/// statically unreachable and can be removed.
///
/// Calling this function in reachable code invokes undefined behavior. Be
/// very, very sure this is what you want; often, a simple `panic!` is more
/// suitable.
pub unsafe fn unreachable() -> ! {
    let x: &void::Void = mem::transmute(1usize);
    void::unreachable(*x)
}

