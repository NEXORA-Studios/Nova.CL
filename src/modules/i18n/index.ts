import { createI18n } from "vue-i18n";
// locales
import en from "./locales/en-US.json";
import zh from "./locales/zh-CN.json";

export const i18nModule = createI18n({
    legacy: false,

    // locale: "en",
    // fallbackLocale: "zh",

    locale: "zh",
    fallbackLocale: "en",

    messages: { en, zh },
});
