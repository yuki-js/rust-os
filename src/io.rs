pub fn hlt() {
    unsafe {
        asm!("hlt");
    }
}
pub fn cli() {
    unsafe {
        asm!("cli");
    }
}
pub fn sti() {
    unsafe {
        asm!("sti");
    }
}
pub fn stihlt() {
    sti();
    hlt();
}
pub fn in8(port: u32) -> u8 {
    let result: u8;
    unsafe {
        asm!("IN AL,DX": "={AL}"(result) : "{DX}"(port) : : "intel");
    }
    result
}
pub fn in16(port: u32) -> u16 {
    let result: u16;
    unsafe {
        asm!("IN AX,DX": "={AX}"(result): "{DX}"(port) : : "intel");
    }
    result
}

pub fn in32(port: u32) -> u32 {
    let result: u32;
    unsafe {
        asm!("IN EAX,DX": "={EAX}"(result): "{DX}"(port) : : "intel");
    }
    result
}

pub fn out8(port: u32, data: u8) {
    unsafe {
        asm!("OUT DX,AL": : "{DX}"(port), "{AL}"(data): : "intel");
    }
}
pub fn out16(port: u32, data: u16) {
    unsafe {
        asm!("OUT DX,AX": : "{DX}"(port), "{AX}"(data): : "intel");
    }
}
pub fn out32(port: u32, data: u32) {
    unsafe {
        asm!("OUT DX,EAX": : "{DX}"(port), "{EAX}"(data): : "intel");
    }
}
pub fn load_eflags() -> u32 {
    let result: u32;
    unsafe{
        asm!("PUSHFD"::::"intel");
        asm!("POP EAX":"={EAX}"(result):::"intel");
    }
    result
}
pub fn store_eflags(eflags: u32) {
    unsafe{
        asm!("PUSH EAX": : "{EAX}"(eflags) :: "intel");
        asm!("POPFD"::::"intel");
    }
}
