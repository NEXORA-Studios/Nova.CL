use crate::app::host::NovaHost;

#[repr(C)]
pub struct PluginInfo {
    pub name: *const u8,
    pub name_len: usize,
}

pub type PluginInit = extern "C" fn(host: *const NovaHost) -> i32;
