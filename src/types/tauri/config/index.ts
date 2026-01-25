import type { LaunchConfig } from "./Launch";
import type { CustomizeConfig } from "./Customize";
import type { DownloadConfig } from "./Download";
import type { NetworkConfig } from "./Network";
import type { DebugConfig } from "./Debug";
import type { ProfileConfig } from "./Profiles";
import type { JavaConfig } from "./Java";

export interface AllConfig {
    Launch: LaunchConfig;
    Customize: CustomizeConfig;
    Download: DownloadConfig;
    Network: NetworkConfig;
    Debug: DebugConfig;
    Profiles: ProfileConfig;
    Java: JavaConfig;
}

export type { LaunchConfig, CustomizeConfig, DownloadConfig, NetworkConfig, DebugConfig, ProfileConfig, JavaConfig };
