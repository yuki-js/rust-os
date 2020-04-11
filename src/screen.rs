use crate::io;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
pub enum Color {
    COL8_000000 = 0,
    COL8_FF0000 = 1,
    COL8_00FF00 = 2,
    COL8_FFFF00 = 3,
    COL8_0000FF = 4,
    COL8_FF00FF = 5,
    COL8_00FFFF = 6,
    COL8_FFFFFF = 7,
    COL8_C6C6C6 = 8,
    COL8_840000 = 9,
    COL8_008400 = 10,
    COL8_848400 = 11,
    COL8_000084 = 12,
    COL8_840084 = 13,
    COL8_008484 = 14,
    COL8_848484 = 15,
}

const TABLE_RGB: [[u8; 3]; 16] = [
    [0x00, 0x00, 0x00], /*  0:黒 */
    [0xff, 0x00, 0x00], /*  1:明るい赤 */
    [0x00, 0xff, 0x00], /*  2:明るい緑 */
    [0xff, 0xff, 0x00], /*  3:明るい黄色 */
    [0x00, 0x00, 0xff], /*  4:明るい青 */
    [0xff, 0x00, 0xff], /*  5:明るい紫 */
    [0x00, 0xff, 0xff], /*  6:明るい水色 */
    [0xff, 0xff, 0xff], /*  7:白 */
    [0xc6, 0xc6, 0xc6], /*  8:明るい灰色 */
    [0x84, 0x00, 0x00], /*  9:暗い赤 */
    [0x00, 0x84, 0x00], /* 10:暗い緑 */
    [0x84, 0x84, 0x00], /* 11:暗い黄色 */
    [0x00, 0x00, 0x84], /* 12:暗い青 */
    [0x84, 0x00, 0x84], /* 13:暗い紫 */
    [0x00, 0x84, 0x84], /* 14:暗い水色 */
    [0x84, 0x84, 0x84], /* 15:暗い灰色 */
];
pub const FONT_A: [u8; 16] = [
    0x00, 0x18, 0x18, 0x18, 0x18, 0x24, 0x24, 0x24, 0x24, 0x7e, 0x42, 0x42, 0x42, 0xe7, 0x00, 0x00,
];
pub fn set_palette() {
    let eflags = io::load_eflags();
    io::cli();
    io::out8(0x03c8, 0);
    for i in 0usize..16 {
        io::out8(0x03c9, TABLE_RGB[i][0] / 4);
        io::out8(0x03c9, TABLE_RGB[i][1] / 4);
        io::out8(0x03c9, TABLE_RGB[i][2] / 4);
    }
    io::store_eflags(eflags);
}

pub fn boxfill8(vram: *mut u8, xsize: u16, c: Color, x0: u16, y0: u16, x1: u16, y1: u16) {
    for i in y0..=y1 {
        for j in x0..=x1 {
            let p = unsafe { &mut *(vram.offset((i * xsize + j) as isize)) };
            *p = c as u8;
        }
    }
}

pub fn put_font(vram: *mut u8, xsize: u16, c: Color, x: u16, y: u16, fontdata: &[u8]) {
    for (i, line) in fontdata.iter().enumerate() {
        let mut l = line.clone();
        for j in 0..8 {
            let p = unsafe { &mut *(vram.offset(((y + i as u16) * xsize + x + (7 - j) ) as isize)) };
            if (l & 1u8) == 1 {
                *p = c as u8;
            }
            l>>=1;
        }
    }
}

pub fn init(vram: *mut u8, xsize: u16, ysize: u16) {
    boxfill8(vram, xsize, Color::COL8_008484, 0, 0, xsize - 1, ysize - 29);
    boxfill8(
        vram,
        xsize,
        Color::COL8_C6C6C6,
        0,
        ysize - 28,
        xsize - 1,
        ysize - 28,
    );
    boxfill8(
        vram,
        xsize,
        Color::COL8_FFFFFF,
        0,
        ysize - 27,
        xsize - 1,
        ysize - 27,
    );
    boxfill8(
        vram,
        xsize,
        Color::COL8_C6C6C6,
        0,
        ysize - 26,
        xsize - 1,
        ysize - 1,
    );

    boxfill8(
        vram,
        xsize,
        Color::COL8_FFFFFF,
        3,
        ysize - 24,
        59,
        ysize - 24,
    );
    boxfill8(vram, xsize, Color::COL8_FFFFFF, 2, ysize - 24, 2, ysize - 4);
    boxfill8(vram, xsize, Color::COL8_848484, 3, ysize - 4, 59, ysize - 4);
    boxfill8(
        vram,
        xsize,
        Color::COL8_848484,
        59,
        ysize - 23,
        59,
        ysize - 5,
    );
    boxfill8(vram, xsize, Color::COL8_000000, 2, ysize - 3, 59, ysize - 3);
    boxfill8(
        vram,
        xsize,
        Color::COL8_000000,
        60,
        ysize - 24,
        60,
        ysize - 3,
    );

    boxfill8(
        vram,
        xsize,
        Color::COL8_848484,
        xsize - 47,
        ysize - 24,
        xsize - 4,
        ysize - 24,
    );
    boxfill8(
        vram,
        xsize,
        Color::COL8_848484,
        xsize - 47,
        ysize - 23,
        xsize - 47,
        ysize - 4,
    );
    boxfill8(
        vram,
        xsize,
        Color::COL8_FFFFFF,
        xsize - 47,
        ysize - 3,
        xsize - 4,
        ysize - 3,
    );
    boxfill8(
        vram,
        xsize,
        Color::COL8_FFFFFF,
        xsize - 3,
        ysize - 24,
        xsize - 3,
        ysize - 3,
    );
}
