extern crate alloc;

use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::c_char;
use vmprotect_sys::VMProtectGetCurrentHWID;

#[inline(always)]
pub fn get_hwid() -> String {
    let size = unsafe { VMProtectGetCurrentHWID(core::ptr::null_mut(), 0) };

    let mut buf: Vec<u8> = Vec::with_capacity(size as usize);
    unsafe { VMProtectGetCurrentHWID(buf.as_mut_ptr() as *mut c_char, size) };
    unsafe { buf.set_len(size as usize - 1) };

    unsafe { String::from_utf8_unchecked(buf) }
}
