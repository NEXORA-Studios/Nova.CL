use chrono::Local;
use log::LevelFilter;
use std::fs::{self, File};
use tauri::Config;

/// 初始化日志系统
///
/// # 参数
/// - `config`: Tauri 配置对象，用于获取应用数据目录
///
/// # 返回值
/// - `Result<(), Box<dyn std::error::Error>>`: 初始化结果
pub fn init_logging(_config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    // 使用 dirs 库获取应用数据目录
    let app_data_dir = dirs::data_dir()
        .ok_or("Failed to get data directory")?
        .join("NovaCL");
    let logs_dir = app_data_dir.join("Logs");

    // 创建日志目录
    fs::create_dir_all(&logs_dir)?;

    // 生成日志文件名（格式：YYYY-MM-DD_HH-MM-SS.log）
    let log_file_name = Local::now().format("%Y-%m-%d_%H-%M-%S.log").to_string();
    let log_file_path = logs_dir.join(log_file_name);

    // 配置 fern 日志器
    #[cfg(debug_assertions)]
    fern::Dispatch::new()
        // 设置日志格式：[时间] [等级] [TS/Rust] [分类] 内容
        .format(|out, message, record| {
            // 解析目标，获取来源（TS/Rust）和分类
            let target = record.target();
            let (source, category) = if target.starts_with("ts::") {
                // 前端调用的日志，格式：ts::category
                ("TS", &target[4..])
            } else {
                // Rust 内部日志，格式：crate::module 或 module
                ("Rust", target.split("::").last().unwrap_or(target))
            };

            out.finish(format_args!(
                "[{}] [{}] [{}] [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                source,
                category,
                message
            ))
        })
        // 设置默认日志级别为 Info
        .level(LevelFilter::Trace)
        // 输出到文件
        .chain(File::create(log_file_path)?)
        // 同时输出到控制台
        .chain(std::io::stdout())
        // 应用配置
        .apply()?;

    #[cfg(not(debug_assertions))]
    fern::Dispatch::new()
        // 设置日志格式：[时间] [等级] [TS/Rust] [分类] 内容
        .format(|out, message, record| {
            // 解析目标，获取来源（TS/Rust）和分类
            let target = record.target();
            let (source, category) = if target.starts_with("ts::") {
                // 前端调用的日志，格式：ts::category
                ("TS", &target[4..])
            } else {
                // Rust 内部日志，格式：crate::module 或 module
                ("Rust", target.split("::").last().unwrap_or(target))
            };

            out.finish(format_args!(
                "[{}] [{}] [{}] [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                source,
                category,
                message
            ))
        })
        // 设置默认日志级别为 Info
        .level(LevelFilter::Info)
        // 输出到文件
        .chain(File::create(log_file_path)?)
        // 同时输出到控制台
        .chain(std::io::stdout())
        // 应用配置
        .apply()?;

    Ok(())
}
