// don't link the Rust standard library
#![no_std]
// disable all Rust-level entry points
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";
static MEM_VIDEO_ADDRESS: u32 = 0xB8000;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = MEM_VIDEO_ADDRESS as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
