#![no_std]
#![no_main]

mod vga_buffer;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("PANIC! {info:?}");
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world!");
    loop {}
}
