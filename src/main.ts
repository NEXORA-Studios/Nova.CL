import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "@/App.vue";
import { router, i18nModule, TauriTOML, EventBus, useAccountStore } from "@/modules";

const app = createApp(App);

app.use(createPinia());
app.use(router);
app.use(i18nModule);

app.mount("#app");

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
    useAccountStore().setAccountState(CurrentProfile.name, CurrentProfile.type);
})();

