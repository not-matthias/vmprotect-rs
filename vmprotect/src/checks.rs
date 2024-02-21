use vmprotect_sys::{VMProtectIsDebuggerPresent, VMProtectIsProtected, VMProtectIsValidImageCRC, VMProtectIsVirtualMachinePresent};

#[inline(always)]
pub fn is_protected() -> bool {
    unsafe { VMProtectIsProtected() == 1 }
}

#[inline(always)]
pub fn is_debugger_present(check_kernel_mode: bool) -> bool {
    unsafe { VMProtectIsDebuggerPresent(if check_kernel_mode { 1 } else { 0 }) == 1 }
}

#[inline(always)]
pub fn is_virtual_machine() -> bool {
    unsafe { VMProtectIsVirtualMachinePresent() == 1 }
}

#[inline(always)]
pub fn is_valid_image_crc() -> bool {
    unsafe { VMProtectIsValidImageCRC() == 1 }
}