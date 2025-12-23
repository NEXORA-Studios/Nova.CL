use serde::{Deserialize, Serialize};

use super::crypto::encrypted_field;

// ---------------------- Global Config ----------------------

/// 全局配置
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GlobalConfig {
    /// 启动配置
    pub launch: LaunchConfig,
    /// 自定义配置
    pub customize: CustomizeConfig,
    /// 其他配置
    pub other: OtherConfig,
}

/// 启动配置
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LaunchConfig {
    /// 基本启动配置
    pub basic: BasicLaunchConfig,
    /// RAM 配置
    pub rams: RamConfig,
    /// 高级启动配置
    pub advanced: AdvancedLaunchConfig,
}

/// 基本启动配置
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct BasicLaunchConfig {
    /// 版本类型限制
    pub version_indie_type: String, // disabled | modded | snapshot | modded_or_snapshot | all
    /// 启动器可见性
    pub launcher_visibility: String, // immediately_quit | hide_then_quit | hide_then_show | minimize | constant
    /// 首选 IP 栈
    pub prefer_ip_stack: u8, // 4=ipv4 | 0=default | 6=ipv6
    /// 选中的 Java
    pub selected_java: String,
    /// Java 列表
    pub java: Vec<JavaConfig>,
}

/// Java 配置
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct JavaConfig {
    /// Java ID
    pub id: String,
    /// Java 路径
    pub path: String,
    /// Java 类型
    pub r#type: String, // jre | jdk
    /// Java 版本
    pub version: u8,
}

/// RAM 配置
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct RamConfig {
    /// 自动 RAM
    pub auto_ram: bool,
    /// 自定义 RAM
    pub custom_ram: u32, // MB
    /// 预交换
    pub pre_swap: bool,
}

/// 高级启动配置
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AdvancedLaunchConfig {
    /// 渲染器
    pub renderer: String, // default | llvmpipe | d3d12 | zink
    /// JVM 参数
    pub jvm_args: String,
    /// 游戏参数
    pub game_args: String,
    /// 预命令
    pub pre_command: String,
    /// 禁用 retrowrapper
    pub disable_retrowrapper: bool,
    /// 使用独立 GPU
    pub use_discrete_gpu: bool,
    /// 使用 java.exe
    pub use_java_exe: bool,
}

/// 自定义配置
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CustomizeConfig {
    /// 主题
    pub theme: String, // light | auto | dark
    /// 语言
    pub language: String, // zh-CN | en-US
}

/// 其他配置
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct OtherConfig {
    /// 下载配置
    pub download: DownloadConfig,
    /// 组件配置
    pub comp: ComponentConfig,
    /// 无障碍配置
    pub accessibility: AccessibilityConfig,
    /// 启动器配置
    pub launcher: LauncherConfig,
    /// 网络配置
    pub network: NetworkConfig,
    /// 调试配置
    pub debug: DebugConfig,
}

/// 下载配置
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DownloadConfig {
    /// 下载源
    pub download_source: String, // offical | balance | mirror
    /// 版本源
    pub version_source: String, // offical | balance | mirror
    /// 最大并发数
    pub max_concurrent: u8, // 1 <= max_concurrent <= 256
    /// 最大带宽
    pub max_bandwidth: f64, // -1 无限制，0.1 <= max_bandwidth <= 20
    /// 后选择实例
    pub postselect_instance: bool,
    /// 更新 authlib
    pub update_authlib: bool,
}

/// 组件配置
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ComponentConfig {
    /// 源
    pub source: String, // offical | balance | mirror
    /// 忽略 quilt
    pub ignore_quilt: bool,
    /// 检测剪贴板
    pub detect_clipboard: bool,
}

/// 无障碍配置
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AccessibilityConfig {
    /// 发布说明
    pub release_note: bool,
    /// 快照说明
    pub snapshot_note: bool,
    /// 自动中文
    pub auto_chinese: bool,
}

/// 启动器配置
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LauncherConfig {
    /// 更新方法
    pub update_method: String, // auto | notice | major_notice | disable
    /// 频道
    pub channel: String, // overworld | nether | ender
    /// 通知
    pub notification: String, // all | major | disable
    /// 缓存目录
    pub cache_dir: String, // empty=default, or a valid path
}

/// 网络配置
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NetworkConfig {
    /// 使用 DoH
    pub use_doh: bool,
    /// 使用系统代理
    pub use_system_proxy: bool,
    /// 使用自定义代理
    pub use_custom_proxy: bool,
    /// 自定义代理 URI
    pub custom_proxy_uri: String,
    /// 自定义代理账户
    #[serde(with = "encrypted_field")]
    pub custom_proxy_account: String,
    /// 自定义代理密码
    #[serde(with = "encrypted_field")]
    pub custom_proxy_password: String,
}

/// 调试配置
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct DebugConfig {
    /// 调试模式
    pub debug_mode: bool,
}

// ---------------------- Profile Config ----------------------

/// 账户配置
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ProfileConfig {
    /// 账户列表
    pub profile: Vec<Profile>,
}

/// 单个账户
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Profile {
    /// 账户类型
    pub r#type: String, // msa | legacy
    /// 刷新令牌（加密存储）
    #[serde(with = "encrypted_field")]
    pub refresh_token: String,
    /// 用户 UUID
    pub uuid: String,
    /// 账户名称（仅 legacy 类型）
    pub id: Option<String>,
}

// ---------------------- Instance Config ----------------------

/// 实例配置
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InstanceConfig {
    /// 实例名称
    pub name: String,
    /// 实例描述
    pub description: String,
    /// Minecraft 版本
    pub mc_version: String,
    /// 加载器类型
    pub loader_type: String, // forge | fabric | quilt | vanilla
    /// 加载器版本
    pub loader_version: String,
    /// 启动次数
    pub launch_count: u32,
    /// 分类
    pub category: u8,
    /// 实例路径
    pub path: Option<String>,
    /// 图标路径
    pub icon: Option<String>,
    /// 上次启动时间
    pub last_launch: Option<String>,
}

// ---------------------- Default Implementations ----------------------

impl Default for GlobalConfig {
    fn default() -> Self {
        Self {
            launch: LaunchConfig {
                basic: BasicLaunchConfig {
                    version_indie_type: "disabled".to_string(),
                    launcher_visibility: "immediately_quit".to_string(),
                    prefer_ip_stack: 4,
                    selected_java: "".to_string(),
                    java: Vec::new(),
                },
                rams: RamConfig {
                    auto_ram: true,
                    custom_ram: 8192,
                    pre_swap: false,
                },
                advanced: AdvancedLaunchConfig {
                    renderer: "default".to_string(),
                    jvm_args: "-XX:+UseG1GC -XX:-UseAdaptiveSizePolicy -XX:-OmitStackTraceInFastThrow -Djdk.lang.Process.allowAmbiguousCommands=true -Dfml.ignoreInvalidMinecraftCertificates=True -Dfml.ignorePatchDiscrepancies=True -Dlog4j2.formatMsgNoLookups=true -DproxySet=false -Djava.net.useSystemProxies=false".to_string(),
                    game_args: "".to_string(),
                    pre_command: "".to_string(),
                    disable_retrowrapper: false,
                    use_discrete_gpu: true,
                    use_java_exe: false,
                },
            },
            customize: CustomizeConfig {
                theme: "auto".to_string(),
                language: "zh-CN".to_string(),
            },
            other: OtherConfig {
                download: DownloadConfig {
                    download_source: "offical".to_string(),
                    version_source: "offical".to_string(),
                    max_concurrent: 64,
                    max_bandwidth: -1.0,
                    postselect_instance: true,
                    update_authlib: true,
                },
                comp: ComponentConfig {
                    source: "offical".to_string(),
                    ignore_quilt: false,
                    detect_clipboard: false,
                },
                accessibility: AccessibilityConfig {
                    release_note: false,
                    snapshot_note: false,
                    auto_chinese: true,
                },
                launcher: LauncherConfig {
                    update_method: "auto".to_string(),
                    channel: "ender".to_string(),
                    notification: "all".to_string(),
                    cache_dir: "".to_string(),
                },
                network: NetworkConfig {
                    use_doh: false,
                    use_system_proxy: false,
                    use_custom_proxy: false,
                    custom_proxy_uri: "".to_string(),
                    custom_proxy_account: "".to_string(),
                    custom_proxy_password: "".to_string(),
                },
                debug: DebugConfig {
                    debug_mode: false,
                },
            },
        }
    }
}

impl Default for ProfileConfig {
    fn default() -> Self {
        Self {
            profile: Vec::new(),
        }
    }
}

impl Default for InstanceConfig {
    fn default() -> Self {
        Self {
            name: "New Instance".to_string(),
            description: "".to_string(),
            mc_version: "1.20.1".to_string(),
            loader_type: "vanilla".to_string(),
            loader_version: "".to_string(),
            launch_count: 0,
            category: 0,
            path: None,
            icon: None,
            last_launch: None,
        }
    }
}
