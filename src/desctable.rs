use crate::handler;
use crate::io;
use crate::interrupt::inthandler21;

#[repr(C)]
pub struct SegmentDescriptor {
    limit_low: u16,
    base_low: u16,
    base_mid: u8,
    access_right: u8,
    limit_high: u8,
    base_high: u8,
}

#[repr(C)]
pub struct GateDescriptor {
    offset_low: u16,
    selector: u16,
    dw_count: u8,
    access_right: u8,
    offset_high: u16,
}

pub fn init_gdtidt() {
    unsafe {
        let gdt: *mut SegmentDescriptor = &mut *(0x002700000 as *mut SegmentDescriptor);
        let idt: *mut GateDescriptor = &mut *(0x0026f800 as *mut GateDescriptor);

        for i in 0..8192 {
            set_segmdesc(&mut *gdt.offset(i), 0, 0, 0);
        }
        set_segmdesc(&mut *gdt.offset(1), 0xffffffff, 0x00000000, 0x4092);
        set_segmdesc(&mut *gdt.offset(2), 0x0007ffff, 0x00280000, 0x409a);
        io::load_gdtr(0xffff, 0x00270000);

        for i in 0..256 {
            set_gatedesc(&mut *idt.offset(i), 0, 0, 0);
        }
        io::load_idtr(0x7ff, 0x0026f800);
        set_gatedesc(&mut *idt.offset(0x21), handler!(inthandler21) as i32, 2 * 8, 0x008e);
	      //set_gatedesc(&mut *idt.offset(0x27), (int) asm_inthandler27, 2 * 8, AR_INTGATE32);
	      // set_gatedesc(&mut *idt.offset(0x2c), (int) asm_inthandler2c, 2 * 8, AR_INTGATE32);
    }
}
pub fn set_segmdesc(sd: &mut SegmentDescriptor, mut limit: u32, base: i32, mut ar: i32) {
    if limit > 0xfffff {
        ar |= 0x8000; /* G_bit = 1 */
        limit /= 0x1000;
    }
    sd.limit_low = (limit & 0xffff) as u16;
    sd.base_low = (base & 0xffff) as u16;
    sd.base_mid = ((base >> 16) & 0xff) as u8;
    sd.access_right = (ar & 0xff) as u8;
    sd.limit_high = (((limit >> 16) & 0x0f) | ((ar >> 8) & 0xf0) as u32) as u8;
    sd.base_high = ((base >> 24) & 0xff) as u8;
}
pub fn set_gatedesc(gd: &mut GateDescriptor, offset: i32, selector: u16, ar: i32) {
    gd.offset_low = (offset & 0xffff) as u16;
    gd.selector = selector as u16;
    gd.dw_count = ((ar >> 8) & 0xff) as u8;
    gd.access_right = (ar & 0xff) as u8;
    gd.offset_high = ((offset >> 16) & 0xffff) as u16;
}
