use crate::*;
use core::arch::asm;

pub fn write_char(c: char) {
    unsafe {
        asm!(
        "svc #0",
        in("x8") KernelCallType::DebugCall as isize, // kernel call number 2 : debug::put_char
        in("x0") c as Word, // debug_write_char (Unicode scalar value)
        options(nostack),
        );
    }
}
