use core::arch::asm;

use crate::*;

#[inline(always)]
pub fn cycle_counter() -> Word {
    let counter: Word;
    unsafe {
        asm!("mrs {counter}, cntvct_el0", counter = out(reg) counter, options(nomem, nostack));
    }
    counter
}
