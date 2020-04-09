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
    let p = 0xa0000 as *mut u8;
    palette::boxfill8(p, 320, 1,  20,  20, 120, 120);
	  palette::boxfill8(p, 320, 2,  70,  50, 170, 150);
	  palette::boxfill8(p, 320, 4, 120,  80, 220, 180);
    
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
