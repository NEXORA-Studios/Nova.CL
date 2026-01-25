export interface NetworkDNSConfig {
    UseDoh: boolean;
}

export interface NetworkProxyConfig {
    UseSystemProxy: boolean;
    UseCustomProxy: boolean;
    CustomProxyUri: string;
    CustomProxyAccount: string;
    CustomProxyPassword: string;
}

export interface NetworkConfig {
    DNS: NetworkDNSConfig;
    Proxy: NetworkProxyConfig;
}
