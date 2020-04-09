#![no_std]
#![feature(asm)]
#![feature(start)]

use core::panic::PanicInfo;
mod io;
mod palette;


#[no_mangle]
#[start]
pub extern "C" fn haribote_os() -> ! {
    palette::set_palette();
    unsafe {
        for i in 0..=0xffff {
            let p = &mut *((0xa0000 + i) as *mut u8);
            *p = (i & 0x0f) as u8;
        }
    }
    loop {
        io::hlt();
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        io::hlt()
    }
}
