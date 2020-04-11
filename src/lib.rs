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
    desctable::init_gdtidt();
    screen::put_char(vram, xsize, screen::Color::COL8_FFFFFF, 10, 10, 0x31);
    screen::put_char(vram, xsize, screen::Color::COL8_FF00FF, 10, 26, 'X' as u8);
    let mut buf = [0u8; 64];
    let msg: &str = io::write_to::show(
        &mut buf,
        format_args!("{:?}: {:?}", "Foo", "Bar"),
    ).unwrap();
    io::print(msg);
    
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

