<script setup lang="ts">
    import { reactive } from "vue";
    import { channelWrapper } from "@/utils";
    import { useI18n } from "vue-i18n";

    const $meta = import.meta.env;
    const { t } = useI18n();

    const METADATA = reactive<{
        version: string;
        channel: string;
        time: string;
        hash: string;
    }>({
        version: $meta.NOVA_VERSION,
        channel: channelWrapper($meta.NOVA_CHANNEL),
        time: $meta.NOVA_BUILD_TIME ?? "Main.More/About.BuildInfo.LocalBuild",
        hash: $meta.NOVA_GIT_HASH ?? "native",
    });

    function checkUpdate() {}
</script>

<template>
    <div class="w-full h-full p-6">
        <div class="w-full card stats bg-base-100 outline outline-base-content/25">
            <div class="stat">
                <div class="stat-title">{{ t("Main.More/About.UpdateChannel.Title") }}</div>
                <div class="stat-value">{{ t(METADATA.channel) }}{{ t("Main.More/About.UpdateChannel.Suffix") }}</div>
                <div class="stat-actions h-6.5">
                    <button class="btn btn-xs btn-primary btn-outline">
                        {{ t("Main.More/About.UpdateChannel.Button") }}
                    </button>
                </div>
            </div>
            <div class="stat">
                <div class="stat-title">{{ t("Main.More/About.Version.Title") }}</div>
                <div class="stat-value">
                    <span>{{ METADATA.version }}</span>
                    <span class="text-base text-base-content/50">{{ t("ExtendVersionDescription") }}</span>
                </div>
                <div class="stat-actions h-6.5">
                    <button class="btn btn-xs btn-primary" @click="checkUpdate()">
                        {{ t("Main.More/About.Version.Button") }}
                    </button>
                </div>
            </div>
            <div class="stat">
                <div class="stat-title">{{ t("Main.More/About.BuildInfo.Title") }}</div>
                <div class="stat-value">{{ t(METADATA.time) }}</div>
                <div class="stat-desc h-6.5 flex items-center">
                    <span v-if="METADATA.hash !== 'native'">
                        {{ t("Main.More/About.BuildInfo.FromCommit") }}{{ METADATA.hash }}
                    </span>
                </div>
            </div>
        </div>
    </div>
</template>
