#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

#[macro_use]
pub(crate) mod vga_buffer;
pub(crate) mod keyboard;

pub mod interrupts;
