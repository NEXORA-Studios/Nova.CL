<script setup lang="ts">
    import { EventBus, TauriConfig } from "@/modules";
    import { ITauriTypes } from "@/types";
    import { watchAndSet } from "@/utils";
    import { onMounted, ref, watch } from "vue";
    import { useI18n } from "vue-i18n";

    const { t, locale } = useI18n();

    const Theme = ref<ITauriTypes.Config.CustomizeConfig["UI"]["Theme"]>("auto");
    const Language = ref<ITauriTypes.Config.CustomizeConfig["UI"]["Language"]>("zh-CN");

    onMounted(async () => {
        Theme.value = await TauriConfig.get<ITauriTypes.Config.CustomizeConfig["UI"]["Theme"]>("Customize.UI.Theme");
        Language.value = await TauriConfig.get<ITauriTypes.Config.CustomizeConfig["UI"]["Language"]>("Customize.UI.Language");
    });

    watchAndSet(Theme, "Customize.UI.Theme", (new_value) => {
        EventBus.emit("theme:change", new_value);
    });
    watchAndSet(Language, "Customize.UI.Language", (new_value) => {
        locale.value = new_value;
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
