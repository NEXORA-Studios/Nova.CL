use crate::system::ram;
use tauri::command;

#[command]
pub fn get_ram_info() -> ram::RamInfo {
    ram::get_ram_info()
}
