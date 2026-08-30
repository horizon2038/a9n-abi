use core::arch::asm;

use crate::capability_call::generic;
use crate::*;

#[inline(always)]
pub fn convert(
    target: CapabilityDescriptor,
    capability_type: CapabilityType,
    specific_bits: Word,
    count: Word,
    node: CapabilityDescriptor,
    node_index: Word,
) -> CapabilityResult {
    let mut a0 = target;
    let mut a1 = generic::OperationType::Convert as Word;

    let a2: Word = capability_type as Word;
    let a3: Word = specific_bits;
    let a4: Word = count;
    let a5: Word = node;
    let a6: Word = node_index;

    unsafe {
        asm!(
        "svc #0",
        in("x8") KernelCallType::CapabilityCall as Sword,
        inout("x0") a0 => a0, // descriptor -> is_success
        inout("x1") a1 => a1, // operation  -> capability_error
        in("x2") a2, // capability type
        in("x3")  a3, // specific bits
        in("x4")  a4, // count
        in("x5") a5, // node descriptor
        in("x6") a6, // node index
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}
