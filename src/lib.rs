#![no_std]
#![feature(asm)]
#![feature(start)]

use core::panic::PanicInfo;
mod io;

fn init_palette() {
    static TABLE_RGB: &[u8] = &[
        0x00, 0x00, 0x00, /*  0:黒 */
        0xff, 0x00, 0x00, /*  1:明るい赤 */
        0x00, 0xff, 0x00, /*  2:明るい緑 */
        0xff, 0xff, 0x00, /*  3:明るい黄色 */
        0x00, 0x00, 0xff, /*  4:明るい青 */
        0xff, 0x00, 0xff, /*  5:明るい紫 */
        0x00, 0xff, 0xff, /*  6:明るい水色 */
        0xff, 0xff, 0xff, /*  7:白 */
        0xc6, 0xc6, 0xc6, /*  8:明るい灰色 */
        0x84, 0x00, 0x00, /*  9:暗い赤 */
        0x00, 0x84, 0x00, /* 10:暗い緑 */
        0x84, 0x84, 0x00, /* 11:暗い黄色 */
        0x00, 0x00, 0x84, /* 12:暗い青 */
        0x84, 0x00, 0x84, /* 13:暗い紫 */
        0x00, 0x84, 0x84, /* 14:暗い水色 */
        0x84, 0x84, 0x84, /* 15:暗い灰色 */
    ];
    set_palette(0, 15, TABLE_RGB);
}
fn set_palette(start: u8, end: u8, rgb: &[u8]) {
    let eflags = io::load_eflags();
    io::cli();
    io::out8(0x03c8, start);
    for i in start..=end {
        io::out8(0x03c9, rgb[(i*3 + 0) as usize] / 4);
        io::out8(0x03c9, rgb[(i*3 + 1) as usize] / 4);
        io::out8(0x03c9, rgb[(i*3 + 2) as usize] / 4);
    }
    io::store_eflags(eflags);
}
#[no_mangle]
#[start]
pub extern "C" fn haribote_os() -> ! {
    init_palette();
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
