export interface DownloadSourceConfig {
    DownloadSource: "offical" | "balance" | "mirror";
    VersionSource: "offical" | "balance" | "mirror";
}

export interface DownloadInternetConfig {
    MaxConcurrent: number;
    MaxBandwidth: number;
}

export interface DownloadPostInstallConfig {
    SelectInstance: boolean;
    UpdateAuthlib: boolean;
}

export interface DownloadConfig {
    Source: DownloadSourceConfig;
    Internet: DownloadInternetConfig;
    PostInstall: DownloadPostInstallConfig;
}
