pub mod command;

use std::path::PathBuf;

/// 初始化环境变量模块
pub fn init_env() -> Result<PathBuf, dotenvy::Error> {
    dotenvy::from_filename("../.env")
}
