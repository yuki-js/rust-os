#![no_std]
#![feature(asm)]
#![feature(start)]

use core::panic::PanicInfo;
mod font;
mod io;
mod screen;
mod desctable;
mod interrupt;

#[repr(C)]
pub struct BootInfo {
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
    screen::put_char(vram, xsize, screen::Color::COL8_FFFFFF, 10, 10, 0x31);
    screen::put_char(vram, xsize, screen::Color::COL8_FF00FF, 10, 26, 'X' as u8);
    screen::put_string(
        vram,
        xsize,
        screen::Color::COL8_00FFFF,
        26,
        10,
        "Hello world!",
    );
    desctable::init_gdtidt();
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

