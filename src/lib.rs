#![no_std]

use core::ptr::NonNull;

extern crate alloc;

pub struct Pl011{
   pub  base_addr: NonNull<u8>,
}



impl Pl011 {
    pub fn new(base_addr: NonNull<u8>) -> Self {
        Self { base_addr }
    }
}