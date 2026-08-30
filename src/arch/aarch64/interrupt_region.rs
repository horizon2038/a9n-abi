use core::arch::asm;

use crate::capability_call::interrupt_region;
use crate::*;

#[inline(always)]
pub fn make_port(
    descriptor: CapabilityDescriptor,
    irq_number: Word,
    target_node: CapabilityDescriptor,
    target_node_index: Word,
) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = interrupt_region::OperationType::MakePort as Word;
    let mut a2 = irq_number;
    let mut a3 = target_node as Word;
    let mut a4 = target_node_index;

    unsafe {
        asm!(
        "svc #0",
        in("x8") KernelCallType::CapabilityCall as Sword,
        inout("x0") a0 => a0, // descriptor -> is_success
        inout("x1") a1 => a1, // operation  -> capability_error
        in("x2")     a2,       // irq_number
        in("x3")     a3,       // target_node
        in("x4")    a4,       // target_node_index
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}
