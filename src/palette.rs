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
