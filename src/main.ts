import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "@/App.vue";
import { router, i18nModule, TauriTOML, EventBus, useAccountStore, TauriLogging } from "@/modules";

const app = createApp(App);

app.use(createPinia());
app.use(router);
app.use(i18nModule);

import "@/assets/index.css";

// 异步启动钩子
(async () => {
    const GlobalConfig = await TauriTOML.getGlobalConfig();
    const ProfileConfig = await TauriTOML.getProfileConfig();
    // 应用配置
    i18nModule.global.locale.value = GlobalConfig.customize.language;
    // 应用主题
    EventBus.emit("theme:change", GlobalConfig.customize.theme);
    // 应用账户信息
    const CurrentProfile = ProfileConfig?.profile?.find((item) => item.picked)!;
    if (CurrentProfile) {
        useAccountStore().setAccountState(CurrentProfile.name, CurrentProfile.type);
    }
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

