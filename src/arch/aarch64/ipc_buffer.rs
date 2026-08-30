use core::arch::asm;

use crate::capability_call::process_control_block;
use crate::*;
use crate::{CapabilityDescriptor, CapabilityResult, IpcBuffer};

// The initial process must configure TPIDR_EL0 before get_ipc_buffer() can be used.
// Put the IPC buffer address in MR10 and issue the configure call directly.
pub fn early_configure_to_tls(
    pcb_descriptor: CapabilityDescriptor,
    ipc_buffer: &mut IpcBuffer,
) -> CapabilityResult {
    // println!("Early configuring IPC buffer to TLS base...");

    let mut a0 = pcb_descriptor;
    let mut a1 = process_control_block::OperationType::Configure as Word;
    let mut a2 = process_control_block::ConfigurationInfo::new(
        false, false, false, false, false, false, false, true, false, false,
    )
    .data;

    let ipc_buffer_ptr = ipc_buffer as *mut IpcBuffer;
    let ipc_buffer_raw = ipc_buffer_ptr as usize;
    ipc_buffer.configure_message(10, ipc_buffer_raw);

    // UNSAFE: use raw pointer to configure IPC buffer and thread local storage base
    unsafe {
        asm!(
        "svc #0",
        in("x8") KernelCallType::CapabilityCall as Sword,
        inout("x0") a0 => a0, // descriptor -> is_success
        inout("x1") a1 => a1, // oepration  -> capablity_error
        in("x2")    a2,       // info
        options(nostack),
        );
    }

    convert_capability_result(a0, a1)
}

#[inline(always)]
pub fn configure_to_tls(
    pcb_descriptor: CapabilityDescriptor,
    ipc_buffer: &mut IpcBuffer,
) -> CapabilityResult {
    let configuration_info = process_control_block::ConfigurationInfo::new(
        false, false, false, false, false, false, false, true, false, false,
    );

    let ipc_buffer_ptr = ipc_buffer as *mut IpcBuffer;
    let ipc_buffer_raw = ipc_buffer_ptr as usize;

    crate::arch::process_control_block::configure(
        pcb_descriptor,
        configuration_info,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        ipc_buffer_raw,
        0,
        0,
    )
}

#[inline(always)]
pub unsafe fn unsafe_get_ipc_buffer() -> *mut IpcBuffer {
    let ipc_buffer_ptr: *mut IpcBuffer;
    unsafe {
        asm!(
            "mrs {ipc_buffer}, tpidr_el0",
            ipc_buffer = lateout(reg) ipc_buffer_ptr,
            options(nostack, readonly)
        );
    }

    ipc_buffer_ptr
}

#[inline(always)]
pub fn get_ipc_buffer() -> &'static mut IpcBuffer {
    unsafe { &mut *unsafe_get_ipc_buffer() }
}
