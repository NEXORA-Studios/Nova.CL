use serde::Serialize;
use sysinfo::System;

#[derive(Debug, Clone, Serialize)]
pub struct RamInfo {
    /// bytes
    pub total: u64,
    pub used: u64,
    pub available: u64,
    pub free: u64,
    /// 0.0 ~ 100.0
    pub usage_percent: f64,
}

pub fn get_ram_info() -> RamInfo {
    let mut sys = System::new();
    sys.refresh_memory();

    let total = sys.total_memory() * 1024;
    let used = sys.used_memory() * 1024;
    let available = sys.available_memory() * 1024;
    let free = sys.free_memory() * 1024;

    let usage_percent = if total == 0 {
        0.0
    } else {
        used as f64 / total as f64 * 100.0
    };

    RamInfo {
        total,
        used,
        available,
        free,
        usage_percent,
    }
}
