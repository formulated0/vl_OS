use core::arch::asm;

pub fn outb(port: u16, value: u8) {
    unsafe {
        asm!(
            "out dx, al",
            in("dx") port,
            in("al") value,
            options(nomem, nostack, preserves_flags)
        );
    }
}

pub fn inb(port: u16) -> u8 {
    let ret: u8;
    unsafe {
        asm!(
            "in al, dx",
            lateout("al") ret,
            in("dx") port,
            options(nomem, nostack, preserves_flags)
        );
    }
    ret
}