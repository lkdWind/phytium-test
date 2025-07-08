#![no_std]

use core::ptr::NonNull;

extern crate alloc;

mod pl011;
pub use pl011::*;