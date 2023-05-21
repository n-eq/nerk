#![feature(abi_x86_interrupt)]
#![no_std]
#![no_main]

mod interrupts;
mod vga_buffer;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("PANIC! {info:?}");
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    interrupts::init();

    println!("Hello world!");
    loop {}
}
