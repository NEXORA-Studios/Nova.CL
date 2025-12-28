<script setup lang="ts">
    import { NovaAside, NovaFooter, NovaHeader, WindowFrame } from "@/layout";
    import { channelWrapper } from "@/utils";
    import { useI18n } from "vue-i18n";
    import { ThemeController } from "@/components";
    import { useTimer } from "./composables";

    const $meta = import.meta.env;
    const { t } = useI18n();
    const { onShortInterval } = useTimer();

    onShortInterval(() => {
        document.querySelectorAll(".btn:not(.has-tabindex)").forEach((button) => {
            button.setAttribute("tabindex", "-1");
        });
    });
</script>

<template>
    <WindowFrame>
        <NovaHeader />
        <NovaAside />
        <main style="grid-area: main" class="border-y border-base-content/25">
            <div class="w-full h-full ease-in-out overflow-hidden relative">
                <RouterView v-slot="{ Component }">
                    <Transition name="fade-slide" mode="out-in">
                        <component :is="Component" />
                    </Transition>
                    <div class="text-right absolute bottom-4 right-4 opacity-50 text-xs z-999">
                        <p>
                            {{
                                t("InsiderWarning.Line1", {
                                    Channel: t(channelWrapper($meta.NOVA_CHANNEL)) + t("Main.More/About.UpdateChannel.Suffix"),
                                    Version: t("Main.More/About.Version.Title") + " " + $meta.NOVA_VERSION,
                                })
                            }}
                        </p>
                        <p>{{ t("InsiderWarning.Line2") }}</p>
                    </div>
                </RouterView>
            </div>
        </main>
        <NovaFooter />
    </WindowFrame>
    <ThemeController />
</template>

<style scoped>
    .fade-slide-enter-active,
    .fade-slide-leave-active {
        transition: all 0.3s ease;
    }

    .fade-slide-enter-from {
        opacity: 0;
        transform: translateX(-20px);
    }

    .fade-slide-leave-to {
        opacity: 0;
        transform: translateX(20px);
    }
</style>

