use crate::io;
use crate::screen;
const PIC0_ICW1: u32 = 0x0020;
const PIC0_OCW2: u32 = 0x0020;
const PIC0_IMR: u32 = 0x0021;
const PIC0_ICW2: u32 = 0x0021;
const PIC0_ICW3: u32 = 0x0021;
const PIC0_ICW4: u32 = 0x0021;
const PIC1_ICW1: u32 = 0x00a0;
const PIC1_OCW2: u32 = 0x00a0;
const PIC1_IMR: u32 = 0x00a1;
const PIC1_ICW2: u32 = 0x00a1;
const PIC1_ICW3: u32 = 0x00a1;
const PIC1_ICW4: u32 = 0x00a1;

pub fn init_pic() {
    io::out8(PIC0_IMR, 0xff); /* 全ての割り込みを受け付けない */
    io::out8(PIC1_IMR, 0xff); /* 全ての割り込みを受け付けない */

    io::out8(PIC0_ICW1, 0x11); /* エッジトリガモード */
    io::out8(PIC0_ICW2, 0x20); /* IRQ0-7は、INT20-27で受ける */
    io::out8(PIC0_ICW3, 1 << 2); /* PIC1はIRQ2にて接続 */
    io::out8(PIC0_ICW4, 0x01); /* ノンバッファモード */

    io::out8(PIC1_ICW1, 0x11); /* エッジトリガモード */
    io::out8(PIC1_ICW2, 0x28); /* IRQ8-15は、INT28-2fで受ける */
    io::out8(PIC1_ICW3, 2); /* PIC1はIRQ2にて接続 */
    io::out8(PIC1_ICW4, 0x01); /* ノンバッファモード */

    io::out8(PIC0_IMR, 0xfb); /* 11111011 PIC1以外は全て禁止 */
    io::out8(PIC1_IMR, 0xff); /* 11111111 全ての割り込みを受け付けない */
}
pub fn enable_pic(){
    io::out8(PIC0_IMR, 0xf9); /* PIC1とキーボードを許可(11111001) */
	  io::out8(PIC1_IMR, 0xef); /* マウスを許可(11101111) */
}

pub extern "C" fn inthandler21() {
    io::print("Keyboard");
    loop {
        io::hlt();
    }
}

pub extern "C" fn inthandler2c() {
    io::print("Mouse");
    loop {
        io::hlt();
    }
}
