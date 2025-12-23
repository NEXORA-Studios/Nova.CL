// 配置相关类型定义

// ---------------------- Global Config ----------------------

/** Java 配置 */
export interface JavaConfig {
    id: string;
    path: string;
    type: "jre" | "jdk";
    version: number;
}

/** 基本启动配置 */
export interface BasicLaunchConfig {
    version_indie_type: "disabled" | "modded" | "snapshot" | "modded_or_snapshot" | "all";
    launcher_visibility: "immediately_quit" | "hide_then_quit" | "hide_then_show" | "minimize" | "constant";
    prefer_ip_stack: 0 | 4 | 6;
    selected_java: string;
    java: JavaConfig[];
}

/** RAM 配置 */
export interface RamConfig {
    auto_ram: boolean;
    custom_ram: number;
    pre_swap: boolean;
}

/** 高级启动配置 */
export interface AdvancedLaunchConfig {
    renderer: "default" | "llvmpipe" | "d3d12" | "zink";
    jvm_args: string;
    game_args: string;
    pre_command: string;
    disable_retrowrapper: boolean;
    use_discrete_gpu: boolean;
    use_java_exe: boolean;
}

/** 启动配置 */
export interface LaunchConfig {
    basic: BasicLaunchConfig;
    rams: RamConfig;
    advanced: AdvancedLaunchConfig;
}

/** 自定义配置 */
export interface CustomizeConfig {
    theme: "light" | "auto" | "dark";
    language: "zh-CN" | "en-US";
}

/** 下载配置 */
export interface DownloadConfig {
    download_source: "offical" | "balance" | "mirror";
    version_source: "offical" | "balance" | "mirror";
    max_concurrent: number;
    max_bandwidth: number;
    postselect_instance: boolean;
    update_authlib: boolean;
}

/** 组件配置 */
export interface ComponentConfig {
    source: "offical" | "balance" | "mirror";
    ignore_quilt: boolean;
    detect_clipboard: boolean;
}

/** 无障碍配置 */
export interface AccessibilityConfig {
    release_note: boolean;
    snapshot_note: boolean;
    auto_chinese: boolean;
}

/** 启动器配置 */
export interface LauncherConfig {
    update_method: "auto" | "notice" | "major_notice" | "disable";
    channel: "overworld" | "nether" | "ender";
    notification: "all" | "major" | "disable";
    cache_dir: string;
}

/** 网络配置 */
export interface NetworkConfig {
    use_doh: boolean;
    use_system_proxy: boolean;
    use_custom_proxy: boolean;
    custom_proxy_uri: string;
    custom_proxy_account: string;
    custom_proxy_password: string;
}

/** 调试配置 */
export interface DebugConfig {
    debug_mode: boolean;
}

/** 其他配置 */
export interface OtherConfig {
    download: DownloadConfig;
    comp: ComponentConfig;
    accessibility: AccessibilityConfig;
    launcher: LauncherConfig;
    network: NetworkConfig;
    debug: DebugConfig;
}

/** 全局配置 */
export interface GlobalConfig {
    launch: LaunchConfig;
    customize: CustomizeConfig;
    other: OtherConfig;
}

// ---------------------- Profile Config ----------------------

/** 单个账户 */
export interface Profile {
    type: "msa" | "legacy";
    refresh_token: string;
    uuid: string;
    id?: string;
}

/** 账户配置 */
export interface ProfileConfig {
    profile: Profile[];
}

// ---------------------- Instance Config ----------------------

/** 实例配置 */
export interface InstanceConfig {
    name: string;
    description: string;
    mc_version: string;
    loader_type: "forge" | "fabric" | "quilt" | "vanilla";
    loader_version: string;
    launch_count: number;
    category: number;
    path?: string;
    icon?: string;
    last_launch?: string;
}

/** 配置错误 */
export interface ConfigError {
    message: string;
}
