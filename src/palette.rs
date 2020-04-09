use crate::io;
static TABLE_RGB: [[u8; 3]; 16] = [
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
pub fn set_palette() {
    let eflags = io::load_eflags();
    io::cli();
    io::out8(0x03c8, 0);
    for i in 0..16 {
        io::out8(0x03c9, TABLE_RGB[i as usize][0] / 4);
        io::out8(0x03c9, TABLE_RGB[i as usize][1] / 4);
        io::out8(0x03c9, TABLE_RGB[i as usize][2] / 4);
    }
    io::store_eflags(eflags);
}

pub fn boxfill8(vram: *mut u8, xsize: u32, c: Color, x0: u32, y0: u32, x1: u32, y1: u32) {
    for i in y0..=y1 {
        for j in x0..=x1 {
            let p = unsafe { &mut *(vram.offset((i * xsize + j) as isize)) };
            *p = c as u8;
        }
    }
}
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
