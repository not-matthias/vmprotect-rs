#![no_std]

use core::ffi::c_void;
pub use real_c_string;

/// The marker that marks the start of the code that should be protected. You
/// can pass a unique name for this marker that will be shown in the gui.
#[macro_export]
macro_rules! vmp_start {
    ($name:expr) => {
        unsafe { $crate::VMProtectBegin($name.as_ptr() as _) };
    };
    (Virtualize, $name:expr) => {
        unsafe { $crate::VMProtectBeginVirtualization($name.as_ptr() as _) };
    };
    (Mutation, $name:expr) => {
        unsafe { $crate::VMProtectBeginMutation($name.as_ptr() as _) };
    };
    (Ultra, $name:expr) => {
        unsafe { $crate::VMProtectBeginUltra($name.as_ptr() as _) };
    };
}

/// Marks the end of a code section that should be protected.
#[macro_export]
macro_rules! vmp_end {
    () => {
        unsafe { $crate::VMProtectEnd() };
    };
}

#[cfg_attr(all(user, target_pointer_width = "64"), link(name = "VMProtectSDK64"))]
#[cfg_attr(
    all(kernel, target_pointer_width = "64"),
    link(name = "VMProtectDDK64")
)]
extern "system" {
    // Markers
    //
    pub fn VMProtectBegin(name: *const i8);
    pub fn VMProtectBeginVirtualization(name: *const i8);
    pub fn VMProtectBeginMutation(name: *const i8);
    pub fn VMProtectBeginUltra(name: *const i8);
    pub fn VMProtectBeginVirtualizationLockByKey(name: *const i8);
    pub fn VMProtectBeginUltraLockByKey(name: *const i8);
    pub fn VMProtectEnd();

    // Service
    //
    pub fn VMProtectIsProtected() -> u8;
    pub fn VMProtectIsDebuggerPresent(kernel: u8) -> u8;
    pub fn VMProtectIsVirtualMachinePresent() -> u8;
    pub fn VMProtectIsValidImageCRC() -> u8;

    // Also service by vmprotect docs, but here located under strings feature
    //
    pub fn VMProtectDecryptStringA(value: *const i8) -> *const i8;
    pub fn VMProtectDecryptStringW(value: *const i16) -> *const i16;
    pub fn VMProtectFreeString(value: *const c_void) -> u8;

    // Licensing
    //
    pub fn VMProtectSetSerialNumber(serial: *const i8) -> u32;
    pub fn VMProtectGetSerialNumberState() -> u32;
    pub fn VMProtectGetSerialNumberData(data: *mut c_void, size: u32) -> u8;
    pub fn VMProtectGetCurrentHWID(hwid: *mut i8, size: u32) -> u32;

    // Activation
    //
    pub fn VMProtectActivateLicense(code: *const i8, serial: *mut i8, size: u32) -> u32;
    pub fn VMProtectDeactivateLicense(serial: *const i8) -> u32;
    pub fn VMProtectGetOfflineActivationString(code: *const i8, buf: *const i8, size: u32) -> u32;
    pub fn VMProtectGetOfflineDeactivationString(
        serial: *const i8,
        buf: *const i8,
        size: u32,
    ) -> u32;
}
