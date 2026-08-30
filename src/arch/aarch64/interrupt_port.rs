use core::arch::asm;

use crate::capability_call::interrupt_port;
use crate::*;

#[inline(always)]
pub fn bind(
    descriptor: CapabilityDescriptor,
    target_notification_port: CapabilityDescriptor,
) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = interrupt_port::OperationType::Bind as Word;
    let mut a2 = target_notification_port as Word;

    unsafe {
        asm!(
        "svc #0",
        in("x8") KernelCallType::CapabilityCall as Sword,
        inout("x0") a0 => a0, // descriptor -> is_success
        inout("x1") a1 => a1, // operation  -> capability_error
        in("x2")     a2,       // target_notification_port
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn unbind(descriptor: CapabilityDescriptor) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = interrupt_port::OperationType::Unbind as Word;

    unsafe {
        asm!(
        "svc #0",
        in("x8") KernelCallType::CapabilityCall as Sword,
        inout("x0") a0 => a0, // descriptor -> is_success
        inout("x1") a1 => a1, // operation  -> capability_error
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn ack(descriptor: CapabilityDescriptor) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = interrupt_port::OperationType::Ack as Word;

    unsafe {
        asm!(
        "svc #0",
        in("x8") KernelCallType::CapabilityCall as Sword,
        inout("x0") a0 => a0, // descriptor -> is_success
        inout("x1") a1 => a1, // operation  -> capability_error
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}

pub fn get_irq_number(descriptor: CapabilityDescriptor) -> Result<Word, CapabilityError> {
    let mut a0 = descriptor;
    let mut a1 = interrupt_port::OperationType::GetIrqNumber as Word;
    let mut a2 = 0usize; // irq_number (return value)

    unsafe {
        asm!(
        "svc #0",
        in("x8") KernelCallType::CapabilityCall as Sword,
        inout("x0") a0 => a0, // descriptor -> is_success
        inout("x1") a1 => a1, // operation  -> capability_error
        out("x2")     a2,      // irq_number
        options(nostack),
        );
    }

    match convert_capability_result(a0, a1) {
        Ok(()) => Ok(a2),
        Err(e) => Err(e),
    }
}
