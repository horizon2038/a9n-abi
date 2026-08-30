use core::arch::asm;

use crate::*;

#[inline(always)]
pub fn yield_call() {
    unsafe {
        asm!(
        "svc #0",
        in("x8") KernelCallType::Yield as Sword,
        options(nostack),
        );
    }
}
