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
    let vram = 0xa0000 as *mut u8;

    let xsize = 320;
    let ysize = 200;

    palette::boxfill8(vram, xsize, palette::Color::COL8_008484, 0, 0, xsize - 1, ysize - 29);
    palette::boxfill8(
        vram,
        xsize,
        palette::Color::COL8_C6C6C6,
        0,
        ysize - 28,
        xsize - 1,
        ysize - 28,
    );
    palette::boxfill8(
        vram,
        xsize,
        palette::Color::COL8_FFFFFF,
        0,
        ysize - 27,
        xsize - 1,
        ysize - 27,
    );
    palette::boxfill8(
        vram,
        xsize,
        palette::Color::COL8_C6C6C6,
        0,
        ysize - 26,
        xsize - 1,
        ysize - 1,
    );

    palette::boxfill8(vram, xsize, palette::Color::COL8_FFFFFF, 3, ysize - 24, 59, ysize - 24);
    palette::boxfill8(vram, xsize, palette::Color::COL8_FFFFFF, 2, ysize - 24, 2, ysize - 4);
    palette::boxfill8(vram, xsize, palette::Color::COL8_848484, 3, ysize - 4, 59, ysize - 4);
    palette::boxfill8(vram, xsize, palette::Color::COL8_848484, 59, ysize - 23, 59, ysize - 5);
    palette::boxfill8(vram, xsize, palette::Color::COL8_000000, 2, ysize - 3, 59, ysize - 3);
    palette::boxfill8(vram, xsize, palette::Color::COL8_000000, 60, ysize - 24, 60, ysize - 3);

    palette::boxfill8(
        vram,
        xsize,
        palette::Color::COL8_848484,
        xsize - 47,
        ysize - 24,
        xsize - 4,
        ysize - 24,
    );
    palette::boxfill8(
        vram,
        xsize,
        palette::Color::COL8_848484,
        xsize - 47,
        ysize - 23,
        xsize - 47,
        ysize - 4,
    );
    palette::boxfill8(
        vram,
        xsize,
        palette::Color::COL8_FFFFFF,
        xsize - 47,
        ysize - 3,
        xsize - 4,
        ysize - 3,
    );
    palette::boxfill8(
        vram,
        xsize,
        palette::Color::COL8_FFFFFF,
        xsize - 3,
        ysize - 24,
        xsize - 3,
        ysize - 3,
    );

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
