use core::arch::asm;

use crate::capability_call::io_port;
use crate::*;

#[inline(always)]
pub fn read(
    target: CapabilityDescriptor,
    address: Word,
    byte_width: Word,
    data: &mut Word,
) -> CapabilityResult {
    let mut a0 = target;
    let mut a1 = io_port::OperationType::Read as Word;
    let mut a2 = address;
    let mut a3 = byte_width;

    unsafe {
        asm!(
        "svc #0",
        in("x8") KernelCallType::CapabilityCall as Sword,
        inout("x0") a0 => a0, // descriptor -> is_success
        inout("x1") a1 => a1, // oepration  -> capablity_error
        inout("x2")  a2 => a2, // address
        inout("x3")  a3 => a3, // byte_width
        options(nostack),
        );
    }

    *data = a2;

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn write(
    target: CapabilityDescriptor,
    address: Word,
    byte_width: Word,
    data: Word,
) -> CapabilityResult {
    let mut a0 = target;
    let mut a1 = io_port::OperationType::Write as Word;
    let mut a2 = address;
    let mut a3 = byte_width;
    let mut a4 = data;

    unsafe {
        asm!(
        "svc #0",
        in("x8") KernelCallType::CapabilityCall as Sword,
        inout("x0") a0 => a0, // descriptor -> is_success
        inout("x1") a1 => a1, // oepration  -> capablity_error
        in("x2")  a2,          // address
        in("x3")  a3,        // byte_width
        in("x4")  a4,        // data
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn mint(
    target: CapabilityDescriptor,
    range_min: Word,
    range_max: Word,
    destination_node: CapabilityDescriptor,
    destination_index: Word,
) -> Result<(), CapabilityError> {
    let mut a0 = target;
    let mut a1 = io_port::OperationType::Mint as Word;
    let a2 = range_min;
    let a3 = range_max;
    let a4 = destination_node as Word;
    let a5 = destination_index;

    unsafe {
        asm!(
            "svc #0",
            in("x8") KernelCallType::CapabilityCall as Sword,
            inout("x0") a0 => a0,
            inout("x1") a1 => a1,
            in("x2") a2,
            in("x3") a3,
            in("x4") a4,
            in("x5") a5,
            options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}
