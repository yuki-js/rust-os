#![no_std]
#![feature(asm)]
#![feature(start)]

use core::panic::PanicInfo;
mod utils;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
#[start]
pub extern "C" fn haribote_os() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {
        utils::hlt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        utils::hlt()
    }
}
