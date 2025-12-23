use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::toml::error::ConfigError;
use crate::toml::manager::ConfigManager;

/// 验证 Java 路径是否有效
pub fn validate_java_path(path: &Path) -> bool {
    path.exists() && path.is_file()
}

/// 生成配置文件备份
pub fn backup_config_file(path: &Path) -> Result<PathBuf, ConfigError> {
    if !path.exists() {
        return Err(ConfigError::PathError(format!("File not found: {:?}", path)));
    }
    
    // 获取文件扩展名
    let file_stem = path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("config");
    
    let extension = path.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("toml");
    
    // 生成时间戳
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    
    // 生成备份文件路径
    let backup_dir = path.parent().unwrap_or_else(|| Path::new("."));
    let backup_path = backup_dir.join(format!("{}-backup-{}.{}", file_stem, timestamp, extension));
    
    // 复制文件
    fs::copy(path, &backup_path).map_err(ConfigError::from)?;
    
    Ok(backup_path)
}

/// 恢复配置文件备份
pub fn restore_config_backup(backup_path: &Path, target_path: &Path) -> Result<(), ConfigError> {
    if !backup_path.exists() {
        return Err(ConfigError::PathError(format!("Backup file not found: {:?}", backup_path)));
    }
    
    // 复制备份文件到目标路径
    fs::copy(backup_path, target_path).map_err(ConfigError::from)?;
    
    Ok(())
}

/// 获取全局配置的备份路径
pub fn get_global_config_backup_path() -> PathBuf {
    let config_path = ConfigManager::instance().global_config_path();
    let parent = config_path.parent().unwrap_or_else(|| Path::new("."));
    parent.join("Config-backup.toml")
}

/// 获取账户配置的备份路径
pub fn get_profile_config_backup_path() -> PathBuf {
    let config_path = ConfigManager::instance().profile_config_path();
    let parent = config_path.parent().unwrap_or_else(|| Path::new("."));
    parent.join("Profile-backup.toml")
}

/// 验证下载源是否有效
pub fn validate_download_source(source: &str) -> bool {
    matches!(source, "offical" | "balance" | "mirror")
}

/// 验证更新方法是否有效
pub fn validate_update_method(method: &str) -> bool {
    matches!(method, "auto" | "notice" | "major_notice" | "disable")
}

/// 验证频道是否有效
pub fn validate_channel(channel: &str) -> bool {
    matches!(channel, "overworld" | "nether" | "ender")
}

/// 验证主题是否有效
pub fn validate_theme(theme: &str) -> bool {
    matches!(theme, "light" | "auto" | "dark")
}

/// 验证语言是否有效
pub fn validate_language(language: &str) -> bool {
    matches!(language, "auto" | "zh-CN" | "en-US")
}

/// 验证加载器类型是否有效
pub fn validate_loader_type(loader_type: &str) -> bool {
    matches!(loader_type, "forge" | "fabric" | "quilt" | "vanilla")
}

/// 清理过期的配置备份
pub fn cleanup_old_backups(directory: &Path, max_backups: usize) -> Result<(), ConfigError> {
    if !directory.exists() || !directory.is_dir() {
        return Ok(());
    }
    
    // 获取所有备份文件
    let mut backup_files = Vec::new();
    
    for entry in fs::read_dir(directory).map_err(ConfigError::from)? {
        let entry = entry.map_err(ConfigError::from)?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(file_name) = path.file_name().and_then(|s| s.to_str()) {
                if file_name.contains("-backup-") {
                    if let Ok(metadata) = path.metadata() {
                        if let Ok(modified) = metadata.modified() {
                            backup_files.push((modified, path));
                        }
                    }
                }
            }
        }
    }
    
    // 按修改时间排序（旧的在前）
    backup_files.sort_by_key(|(modified, _)| *modified);
    
    // 删除超过最大数量的旧备份
    while backup_files.len() > max_backups {
        let (_, path) = backup_files.remove(0);
        fs::remove_file(&path).map_err(ConfigError::from)?;
    }
    
    Ok(())
}

/// 生成随机的实例 ID
pub fn generate_instance_id() -> String {
    use rand::Rng;
    
    let mut rng = rand::thread_rng();
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
    
    (0..16)
        .map(|_| chars[rng.gen_range(0..chars.len())])
        .collect()
}
