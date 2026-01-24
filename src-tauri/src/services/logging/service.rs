use async_trait::async_trait;
use chrono::Local;
use log::LevelFilter;
use log::{debug, error, info, trace, warn};
use std::collections::HashMap;
use std::fs::{self, File};
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::Mutex;

use crate::lifecycle::{
    sync_cmd, CommandError, CommandHashMap, CommandInput, CommandOutput, LifecycleService,
    ServiceState,
};

#[derive(Clone)]
pub struct LogService {
    state: Arc<Mutex<ServiceState>>,
}

impl LogService {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(ServiceState::Created)),
        }
    }

    /// 初始化日志系统
    fn init_logging() -> Result<(), Box<dyn std::error::Error>> {
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
            // 设置默认日志级别为 Trace
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
}

#[async_trait]
impl LifecycleService for LogService {
    fn name(&self) -> &'static str {
        "LogService"
    }

    fn priority(&self) -> i32 {
        // 日志服务优先级很高，确保最先启动
        200
    }

    async fn on_start(&self, _app: &AppHandle) {
        let mut state_guard = self.state.lock().await;
        *state_guard = ServiceState::Starting;

        match Self::init_logging() {
            Ok(_) => {
                info!("日志服务组件启动成功");
                *state_guard = ServiceState::Running;
            }
            Err(e) => {
                eprintln!("日志服务组件启动失败: {}", e);
                *state_guard = ServiceState::Stopped;
            }
        }
    }

    async fn on_stop(&self, _app: &AppHandle) {
        let mut state_guard = self.state.lock().await;
        *state_guard = ServiceState::Stopping;
        // 日志服务无需特殊清理
        *state_guard = ServiceState::Stopped;
        info!("日志服务组件已停止");
    }

    fn state(&self) -> ServiceState {
        *tokio::task::block_in_place(|| futures::executor::block_on(self.state.lock()))
    }

    async fn commands(&self) -> CommandHashMap {
        let mut map: CommandHashMap = HashMap::new();

        // 添加日志命令
        map.insert(
            "log_trace".to_string(),
            sync_cmd({
                |args: CommandInput| -> Result<CommandOutput, CommandError> {
                    if let CommandInput::Args(args) = args {
                        if args.len() < 2 {
                            return Err(CommandError::Text(
                                "缺少 category 或 message 参数".to_string(),
                            ));
                        }
                        let category = args[0].clone();
                        let message = args[1].clone();
                        let target = format!("ts::{}", category);
                        trace!(target: &target, "{}", message);
                    } else {
                        return Err(CommandError::Text("参数格式错误".to_string()));
                    }
                    Ok(CommandOutput::Text("成功".to_string()))
                }
            }),
        );

        map.insert(
            "log_debug".to_string(),
            sync_cmd({
                |args: CommandInput| -> Result<CommandOutput, CommandError> {
                    if let CommandInput::Args(args) = args {
                        if args.len() < 2 {
                            return Err(CommandError::Text(
                                "缺少 category 或 message 参数".to_string(),
                            ));
                        }
                        let category = args[0].clone();
                        let message = args[1].clone();
                        let target = format!("ts::{}", category);
                        debug!(target: &target, "{}", message);
                    } else {
                        return Err(CommandError::Text("参数格式错误".to_string()));
                    }
                    Ok(CommandOutput::Text("成功".to_string()))
                }
            }),
        );

        map.insert(
            "log_info".to_string(),
            sync_cmd({
                |args: CommandInput| -> Result<CommandOutput, CommandError> {
                    if let CommandInput::Args(args) = args {
                        if args.len() < 2 {
                            return Err(CommandError::Text(
                                "缺少 category 或 message 参数".to_string(),
                            ));
                        }
                        let category = args[0].clone();
                        let message = args[1].clone();
                        let target = format!("ts::{}", category);
                        info!(target: &target, "{}", message);
                    } else {
                        return Err(CommandError::Text("参数格式错误".to_string()));
                    }
                    Ok(CommandOutput::Text("成功".to_string()))
                }
            }),
        );

        map.insert(
            "log_warn".to_string(),
            sync_cmd({
                |args: CommandInput| -> Result<CommandOutput, CommandError> {
                    if let CommandInput::Args(args) = args {
                        if args.len() < 2 {
                            return Err(CommandError::Text(
                                "缺少 category 或 message 参数".to_string(),
                            ));
                        }
                        let category = args[0].clone();
                        let message = args[1].clone();
                        let target = format!("ts::{}", category);
                        warn!(target: &target, "{}", message);
                    } else {
                        return Err(CommandError::Text("参数格式错误".to_string()));
                    }
                    Ok(CommandOutput::Text("成功".to_string()))
                }
            }),
        );

        map.insert(
            "log_error".to_string(),
            sync_cmd({
                |args: CommandInput| -> Result<CommandOutput, CommandError> {
                    if let CommandInput::Args(args) = args {
                        if args.len() < 2 {
                            return Err(CommandError::Text(
                                "缺少 category 或 message 参数".to_string(),
                            ));
                        }
                        let category = args[0].clone();
                        let message = args[1].clone();
                        let target = format!("ts::{}", category);
                        error!(target: &target, "{}", message);
                    } else {
                        return Err(CommandError::Text("参数格式错误".to_string()));
                    }
                    Ok(CommandOutput::Text("成功".to_string()))
                }
            }),
        );

        map
    }
}
