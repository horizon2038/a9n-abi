use core::arch::asm;

use crate::capability_call::address_space;
use crate::capability_call::address_space::Attribute;
use crate::*;

#[inline(always)]
pub fn map(
    descriptor: CapabilityDescriptor,
    map_descriptor: CapabilityDescriptor,
    virtual_address: VirtualAddress,
    attribute: Attribute,
) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = address_space::OperationType::Map as Word;
    let mut a2 = map_descriptor as Word;
    let mut a3 = virtual_address as Word;
    let mut a4 = attribute.bits() as Word;

    unsafe {
        asm!(
        "svc #0",
        in("x8") KernelCallType::CapabilityCall as Sword,
        inout("x0") a0 => a0, // descriptor -> is_success
        inout("x1") a1 => a1, // oepration  -> capablity_error
        in("x2")    a2,       // map_descriptor
        in("x3")     a3,       // virtual_address
        in("x4")     a4,       // attribute
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn unmap(
    descriptor: CapabilityDescriptor,
    memory_descriptor: CapabilityDescriptor,
    virtual_address: VirtualAddress,
) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = address_space::OperationType::Unmap as Word;
    let mut a2 = memory_descriptor as Word;
    let mut a3 = virtual_address as Word;

    unsafe {
        asm!(
        "svc #0",
        in("x8") KernelCallType::CapabilityCall as Sword,
        inout("x0") a0 => a0, // descriptor -> is_success
        inout("x1") a1 => a1, // oepration  -> capablity_error
        in("x2")    a2,       // memory_descriptor
        in("x3")     a3,       // virtual_address
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn get_unset_depth(
    descriptor: CapabilityDescriptor,
    address: VirtualAddress,
    size_bits: usize,
) -> Result<usize, CapabilityError> {
    let mut a0 = descriptor;
    let mut a1 = address_space::OperationType::GetUnsetDepth as Word;
    let mut a2 = address as Word; // address (r8)
    let mut a3 = size_bits as Word; // FIXME: 2^12 = 4KiB

    unsafe {
        asm!(
        "svc #0",
        in("x8") KernelCallType::CapabilityCall as Sword,
        inout("x0") a0 => a0, // descriptor -> is_success
        inout("x1") a1 => a1, // oepration  -> capablity_error
        inout("x2") a2 => a2, // address -> depth
        in("x3")     a3,       // leaf size bits
        options(nostack),
        );
    }

    match convert_capability_result(a0, a1) {
        Ok(()) => {
            let depth = a2;
            Ok(depth)
        }
        Err(e) => Err(e),
    }
}
