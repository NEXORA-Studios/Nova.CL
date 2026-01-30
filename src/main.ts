import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "@/App.vue";
import { router, i18nModule, EventBus, useProfileStore, TauriLogging, TauriConfig } from "@/modules";

const app = createApp(App);

app.use(createPinia());
app.use(router);
app.use(i18nModule);

import "@/assets/index.css";
import { Profile } from "./types/tauri/config/Profiles";

// 异步启动钩子
(async () => {
    // 应用配置
    // i18nModule.global.locale.value = await TauriConfig.get("UI.Language");
    // 应用主题
    // EventBus.emit("theme:change", await TauriConfig.get("UI.Theme"));
    // 应用账户信息
    const [listStr, current] = await Promise.all([TauriConfig.get<string>("Profiles.Profile"), TauriConfig.get<string>("Profiles.Current")]);
    const list = JSON.parse(listStr) as Profile[];
    const currentProfile = list.find((i) => i.Guid == current);
    useProfileStore().setProfile(currentProfile);
})();

// 注入日志系统
function cleanOutput(output: any) {
    if (typeof output !== "string") return output;
    if (output.startsWith("[")) {
        return output.slice(1);
    }
    if (output.endsWith("]")) {
        return output.slice(0, -1);
    }
    return output;
}
function getTimeStr() {
    return `[${new Date().toLocaleString()}]`;
}
console.tLog = (params) => {
    TauriLogging.info(params);
    console.log(getTimeStr(), "[INFO]", "[TS]", `[${params.category ?? "-"}]`, cleanOutput(params.message));
};
console.tTrace = (params) => {
    TauriLogging.trace(params);
    console.trace(getTimeStr(), "[TRACE]", "[TS]", `[${params.category ?? "-"}]`, cleanOutput(params.message));
};
console.tDebug = (params) => {
    TauriLogging.debug(params);
    console.debug(getTimeStr(), "[DEBUG]", "[TS]", `[${params.category ?? "-"}]`, cleanOutput(params.message));
};
console.tInfo = (params) => {
    TauriLogging.info(params);
    console.info(getTimeStr(), "[INFO]", "[TS]", `[${params.category ?? "-"}]`, cleanOutput(params.message));
};
console.tWarn = (params) => {
    TauriLogging.warn(params);
    console.warn(getTimeStr(), "[WARN]", "[TS]", `[${params.category ?? "-"}]`, cleanOutput(params.message));
};
console.tError = (params) => {
    TauriLogging.error(params);
    console.error(getTimeStr(), "[ERROR]", "[TS]", `[${params.category ?? "-"}]`, cleanOutput(params.message));
};

app.mount("#app");
