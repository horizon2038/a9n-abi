use crate::capability_call::virtual_cpu;
use crate::*;
use core::arch::asm;

#[inline(always)]
pub fn configure(
    descriptor: CapabilityDescriptor,
    address_space_descriptor: CapabilityDescriptor,
    vcpu_configuration: virtual_cpu::VcpuConfiguration,
) -> CapabilityResult {
    let mut a0 = descriptor;
    let mut a1 = virtual_cpu::OperationType::Configure as Word;

    let mut a2 = address_space_descriptor as Word;
    let mut a3 = vcpu_configuration.data;

    unsafe {
        asm!(
        "svc #0",
        in("x8") KernelCallType::CapabilityCall as Sword,
        inout("x0") a0 => a0, // descriptor -> is_success
        inout("x1") a1 => a1, // operation -> capability_error
        in("x2") a2,          // address_space_descriptor
        in("x3") a3,          // vcpu_configuration
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}
