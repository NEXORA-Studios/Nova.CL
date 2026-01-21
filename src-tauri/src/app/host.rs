// src/app/host.rs
use std::ffi::c_void;

#[repr(C)]
pub struct NovaHost {
    pub version: u32,

    pub log: extern "C" fn(level: u32, msg: *const u8, len: usize),

    pub http_get: extern "C" fn(
        url: *const u8,
        len: usize,
        out_ptr: *mut *mut u8,
        out_len: *mut usize,
    ) -> i32,
}
