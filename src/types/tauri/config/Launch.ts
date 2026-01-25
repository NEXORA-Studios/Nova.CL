export interface LaunchBasicConfig {
    VersionIndieType: "disabled" | "modded" | "snapshot" | "modded_or_snapshot" | "all";
    LauncherVisibility: "immediately_quit" | "hide_then_quit" | "hide_then_show" | "minimize" | "constant";
    PreferIpStack: 0 | 4 | 6;
}

export interface LaunchRamConfig {
    AutoRam: boolean;
    CustomRam: number;
    PreSwap: boolean;
}

export interface LaunchAdvancedConfig {
    Renderer: "default" | "llvmpipe" | "d3d12" | "zink";
    JvmArgs: string;
    GameArgs: string;
    PreLaunchCommand: string;
    DisableRetroWrapper: boolean;
    UseDiscreteGpu: boolean;
    UseJavaExe: boolean;
}

export interface LaunchConfig {
    Basic: LaunchBasicConfig;
    Ram: LaunchRamConfig;
    Advanced: LaunchAdvancedConfig;
}
