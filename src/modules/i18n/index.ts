import { createI18n } from "vue-i18n";
// locales
import enUS from "./locales/en-US.json";
import zhCN from "./locales/zh-CN.json";

export const i18nModule = createI18n({
    legacy: false,

    locale: "zh-CN",
    fallbackLocale: "en-US",

    messages: { "en-US": enUS, "zh-CN": zhCN },
});
