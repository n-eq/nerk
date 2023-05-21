#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod interrupts;
mod keyboard;
mod vga_buffer;

use x86_64::instructions::hlt;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("PANIC! {info:?}");
    loop {
        hlt();
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    interrupts::init();

    if !x86_64::instructions::interrupts::are_enabled() {
        x86_64::instructions::interrupts::enable();
    }

    println!("Hello world!");
    loop {
        hlt();
    }
}
