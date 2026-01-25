use async_trait::async_trait;
use chrono::Local;
use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;
use log::{debug, error, info, trace, warn};
use std::collections::HashMap;
use std::fs::{self, File};
use std::sync::Arc;
use tauri::AppHandle;
use tokio::sync::Mutex;

use crate::lifecycle::{sync_cmd, CommandError, CommandHashMap, CommandInput, CommandOutput, LifecycleService, ServiceState};

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
        let app_data_dir = dirs::data_dir().ok_or("无法获取应用数据目录")?.join("NovaCL");
        let logs_dir = app_data_dir.join("Logs");

        // 创建日志目录
        fs::create_dir_all(&logs_dir)?;

        // 生成日志文件名（格式：YYYY-MM-DD_HH-MM-SS.log）
        let log_file_name = Local::now().format("%Y-%m-%d_%H-%M-%S.log").to_string();
        let log_file_path = logs_dir.join(log_file_name);

        // ────────────────────────────────────────────────
        //              颜色配置（仅用于控制台）
        // ────────────────────────────────────────────────
        let colors = ColoredLevelConfig::new().error(Color::Red).warn(Color::Yellow).info(Color::Green).debug(Color::Blue).trace(Color::Magenta);

        // ────────────────────────────────────────────────
        //                  控制台格式化闭包
        // ────────────────────────────────────────────────
        let console_format = move |out: fern::FormatCallback<'_>, message: &std::fmt::Arguments<'_>, record: &log::Record<'_>| {
            let target = record.target();
            let (source, category) = if target.starts_with("ts::") {
                ("TS", &target[4..])
            } else {
                ("Rust", target.split("::").last().unwrap_or(target))
            };

            out.finish(format_args!(
                "[{}] [{level_colored}] [{source}] [{category}] {message}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                level_colored = colors.color(record.level()),
                source = source,
                category = category,
                message = message
            ));
        };

        // ────────────────────────────────────────────────
        //                  文件格式化闭包
        // ────────────────────────────────────────────────
        let file_format = |out: fern::FormatCallback<'_>, message: &std::fmt::Arguments<'_>, record: &log::Record<'_>| {
            let target = record.target();
            let (source, category) = if target.starts_with("ts::") {
                ("TS", &target[4..])
            } else {
                ("Rust", target.split("::").last().unwrap_or(target))
            };

            out.finish(format_args!("[{}] [{}] [{}] [{}] {}", Local::now().format("%Y-%m-%d %H:%M:%S"), record.level(), source, category, message));
        };

        // ────────────────────────────────────────────────
        //              构建 dispatch
        // ────────────────────────────────────────────────
        // 控制台 dispatch（带颜色）
        let console = fern::Dispatch::new()
            .format(console_format)
            .level(if cfg!(debug_assertions) { LevelFilter::Trace } else { LevelFilter::Info })
            .chain(std::io::stdout());

        // 文件 dispatch（纯文本）
        let file = fern::Dispatch::new()
            .format(file_format)
            .level(if cfg!(debug_assertions) { LevelFilter::Trace } else { LevelFilter::Info })
            .chain(File::create(log_file_path)?);

        // 根 dispatch 只负责合并，不设置 format
        fern::Dispatch::new().chain(console).chain(file).apply()?;

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
                            return Err(CommandError::Text("缺少 category 或 message 参数".to_string()));
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
                            return Err(CommandError::Text("缺少 category 或 message 参数".to_string()));
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
                            return Err(CommandError::Text("缺少 category 或 message 参数".to_string()));
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
                            return Err(CommandError::Text("缺少 category 或 message 参数".to_string()));
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
                            return Err(CommandError::Text("缺少 category 或 message 参数".to_string()));
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
