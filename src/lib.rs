#![no_std]
#![feature(asm)]
#![feature(start)]

use core::panic::PanicInfo;

#[no_mangle]
fn hlt() {
    unsafe {
        // assembly で "HLT" したのと同じ効果がある。
        asm!("hlt");
    }
}

#[no_mangle]
fn show_gray(i: u32) {
    let a: u8 = 8;
    let ptr = unsafe { &mut *(i as *mut u8) };
    *ptr = a 
}

#[no_mangle]
#[start]
pub extern "C" fn haribote_os() -> ! {
    // 本にある通り、0xa0000から0xaffffまで描画
    for i in 0xa0000..0xaffff {
      show_gray(i);
    }
    loop {
        hlt()
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        hlt()
    }
}
