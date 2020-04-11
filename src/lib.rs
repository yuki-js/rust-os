#![no_std]
#![feature(asm)]
#![feature(start)]

use core::panic::PanicInfo;
mod io;
mod screen;

#[repr(C)]
struct BootInfo {
    cyls: u8,
    leds: u8,
    vmode: u8,
    reserve: u8,
    scrnx: u16,
    scrny: u16,
    vram: *mut u8,
}
#[no_mangle]
#[start]
pub extern "C" fn haribote_os() -> ! {
    screen::set_palette();

    let binfo = unsafe { &*(0x0ff0 as *const BootInfo) };

    let vram = binfo.vram;

    let xsize = binfo.scrnx;
    let ysize = binfo.scrny;

    screen::init(vram, xsize, ysize);
    screen::put_font(vram, xsize, screen::Color::COL8_FFFFFF, 10, 10, &screen::FONT_A);
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
