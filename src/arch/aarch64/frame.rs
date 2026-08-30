use core::arch::asm;

use crate::capability_call::frame;
use crate::*;

#[inline(always)]
pub fn get_address(descriptor: CapabilityDescriptor) -> Result<PhysicalAddress, CapabilityError> {
    let mut a0 = descriptor;
    let mut a1 = frame::OperationType::GetAddress as Word;
    let mut a2 = 0; // address

    unsafe {
        asm!(
        "svc #0",
        in("x8") KernelCallType::CapabilityCall as Sword,
        inout("x0") a0 => a0, // descriptor -> is_success
        inout("x1") a1 => a1, // oepration  -> capablity_error
        out("x2") a2,
        out("x3") _,
        out("x4") _,
        out("x5") _,
        out("x6") _,
        out("x7") _,
        out("x9") _,
        out("x10") _,
        options(nostack),
        );
    }

    convert_capability_result(a0, a1).map(|_| a2)
}
