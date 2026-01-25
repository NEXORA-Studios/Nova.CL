export interface CustomizeUIConfig {
    Theme: "light" | "auto" | "dark";
    Language: "zh-CN" | "en-US";
}

export interface CustomizeConfig {
    UI: CustomizeUIConfig;
}
