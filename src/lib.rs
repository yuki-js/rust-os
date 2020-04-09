#![no_std]
#![feature(asm)]
#![feature(start)]

use core::panic::PanicInfo;
mod utils;

#[no_mangle]
fn draw(i: u32) {
    let a: u8 = i as u8 & 0x0f;
    let ptr = unsafe { &mut *(i as *mut u8) };
    *ptr = a 
}

#[no_mangle]
#[start]
pub extern "C" fn haribote_os() -> ! {
    // 本にある通り、0xa0000から0xaffffまで描画
    for i in 0xa0000..0xaffff {
      draw(i);
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
