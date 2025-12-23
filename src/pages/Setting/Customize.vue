<script setup lang="ts">
    import { EventBus, TauriTOML } from "@/modules";
    import { ITauriTypes } from "@/types";
    import { onMounted, ref, watch } from "vue";
    import { useI18n } from "vue-i18n";

    const { t, locale } = useI18n();

    const Theme = ref<ITauriTypes.TOML.CustomizeConfig["theme"]>("auto");
    const Language = ref<ITauriTypes.TOML.CustomizeConfig["language"]>("zh-CN");

    onMounted(async () => {
        const config = await TauriTOML.getGlobalConfig();
        const theme_config = config.customize;
        Theme.value = theme_config.theme;
        Language.value = theme_config.language;
    });

    watch([Theme, Language], async ([Theme, Language]) => {
        const config = await TauriTOML.getGlobalConfig();
        const theme_config = config.customize;
        theme_config.theme = Theme;
        theme_config.language = Language;
        await TauriTOML.saveGlobalConfig(config);
        // Post-Updates
        EventBus.emit("theme:change", Theme);
        locale.value = Language;
    });
</script>

<template>
    <main class="p-6 pr-8 max-h-[calc(100vh-128px-var(--spacing)*4)] rounded-box overflow-auto beautiful-scrollbar">
        <div class="card bg-base-100 outline outline-base-content/25 w-full">
            <div class="card-body">
                <h2 class="card-title">{{ t("Main.Setting/Customize.Basic.__Title__") }}</h2>

                <section class="grid grid-cols-[64px_4fr_64px_4fr] gap-x-8 gap-y-2 items-center">
                    <span class="text-sm ml-4">{{ t("Main.Setting/Customize.Theme.__Name__") }}</span>
                    <select class="select select-sm w-full outline-none" v-model="Theme">
                        <option value="auto">{{ t("Main.Setting/Customize.Theme.Auto") }}</option>
                        <option value="light">{{ t("Main.Setting/Customize.Theme.Light") }}</option>
                        <option value="dark">{{ t("Main.Setting/Customize.Theme.Dark") }}</option>
                    </select>

                    <span class="text-sm ml-4">
                        {{ t("Main.Setting/Customize.Language.__Name__") }}
                    </span>
                    <select class="select select-sm w-full outline-none" v-model="Language">
                        <option value="zh-CN">中文（简体）</option>
                        <option value="en-US">English (US)</option>
                    </select>
                </section>
            </div>
        </div>
    </main>
</template>
