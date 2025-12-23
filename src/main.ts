import { createApp } from "vue";
import { createPinia } from "pinia";

import App from "@/App.vue";
import { router, i18nModule, TauriTOML, EventBus } from "@/modules";

const app = createApp(App);

app.use(createPinia());
app.use(router);
app.use(i18nModule);

app.mount("#app");

import "@/assets/index.css";

// 异步启动钩子
(async () => {
    const config = await TauriTOML.getGlobalConfig();
    // 应用配置
    i18nModule.global.locale.value = config.customize.language;
    // 应用主题
    EventBus.emit("theme:change", config.customize.theme);
})();
