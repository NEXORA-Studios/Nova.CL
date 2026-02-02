<script setup lang="ts">
    import { onMounted, ref, watch } from "vue";
    import { useI18n } from "vue-i18n";
    import { TauriConfig } from "@/modules";
    import { ITauriTypes } from "@/types";
    import { useRoute } from "vue-router";
    import { watchAndSet } from "@/utils";

    const { t } = useI18n();
    const route = useRoute();

    const refUpdateMethodName = ref<HTMLSpanElement | null>(null);

    // 下载
    const DownloadSource = ref<ITauriTypes.Config.DownloadConfig["Source"]["DownloadSource"]>("offical");
    const VersionSource = ref<ITauriTypes.Config.DownloadConfig["Source"]["VersionSource"]>("offical");
    const MaxConcurrent = ref<ITauriTypes.Config.DownloadConfig["Internet"]["MaxConcurrent"]>(1);
    const MaxBandwidth = ref<ITauriTypes.Config.DownloadConfig["Internet"]["MaxBandwidth"]>(0);
    const PostSelectInstance = ref<ITauriTypes.Config.DownloadConfig["PostInstall"]["SelectInstance"]>(false);
    const UpdateAuthlib = ref<ITauriTypes.Config.DownloadConfig["PostInstall"]["UpdateAuthlib"]>(false);
    const _MaxBandwidth = ref<number>(0);
    function mapMaxBandwidthInputToOutput(input: number): number {
        if (input < 1) return -1; // 输入小于 1 返回 -1

        let index = input - 1; // 输入从 1 开始计数

        // 输出 0.1 - 1.5，每 0.1 对应 1 输入
        const firstSegmentLength = 15;
        if (index < firstSegmentLength) {
            return 0.1 + 0.1 * index;
        }

        // 输出 1.5 - 10，每 0.5 对应 1 输入
        const secondSegmentLength = 18;
        if (index < firstSegmentLength + secondSegmentLength) {
            const secondIndex = index - firstSegmentLength;
            return 1.5 + 0.5 * secondIndex;
        }

        // 输出 10 - 20，每 1 对应 1 输入
        const thirdSegmentLength = 11;
        if (index < firstSegmentLength + secondSegmentLength + thirdSegmentLength) {
            const thirdIndex = index - firstSegmentLength - secondSegmentLength;
            return 10 + 1 * thirdIndex;
        }

        // 多输入返回 -1
        return -1;
    }
    function mapMaxBandwidthOutputToInput(output: number): number {
        if (output === -1) {
            return 45; // 多输入对应 -1
        } else if (output >= 0.1 && output <= 1.5) {
            return Math.round((output - 0.1) / 0.1) + 1;
        } else if (output > 1.5 && output <= 10) {
            return 15 + Math.round((output - 1.5) / 0.5) + 1;
        } else if (output > 10 && output <= 20) {
            return 15 + 18 + Math.round(output - 10) + 1;
        } else {
            return -1; // 输出不在范围内
        }
    }
    watch(_MaxBandwidth, (v) => {
        MaxBandwidth.value = mapMaxBandwidthInputToOutput(v);
        MaxBandwidth.value = Math.round(MaxBandwidth.value * 100) / 100;
    });

    // 社区资源
    // const Source = ref<ITauriTypes.Config.CompConfig["Source"]>("offical");
    // const IgnoreQuilt = ref<ITauriTypes.Config.CompConfig["IgnoreQuilt"]>(false);
    // const DetectClipboard = ref<ITauriTypes.Config.CompConfig["DetectClipboard"]>(false);

    // 辅助功能
    // const ReleaseNote = ref<ITauriTypes.TOML.AccessibilityConfig["release_note"]>(false);
    // const SnapshotNote = ref<ITauriTypes.TOML.AccessibilityConfig["snapshot_note"]>(false);
    // const AutoChinese = ref<ITauriTypes.TOML.AccessibilityConfig["auto_chinese"]>(true);

    // 启动器
    // const UpdateMethod = ref<ITauriTypes.TOML.LauncherConfig["update_method"]>("auto");
    // const Channel = ref<ITauriTypes.TOML.LauncherConfig["channel"]>("overworld");
    // const Notification = ref<ITauriTypes.TOML.LauncherConfig["notification"]>("all");
    // const CacheDir = ref<ITauriTypes.TOML.LauncherConfig["cache_dir"]>("");

    // 网络
    const UseDoH = ref<ITauriTypes.Config.NetworkConfig["DNS"]["UseDoh"]>(false);
    const _ProxyType = ref<"disable" | "system" | "custom">("disable");
    watch(_ProxyType, async (v) => {
        UseSystemProxy.value = v === "system";
        UseCustomProxy.value = v === "custom";
        await TauriConfig.set("Proxy.UseSystemProxy", UseSystemProxy.value);
        await TauriConfig.set("Proxy.UseCustomProxy", UseCustomProxy.value);
    });
    const UseSystemProxy = ref<ITauriTypes.Config.NetworkConfig["Proxy"]["UseSystemProxy"]>(false);
    const UseCustomProxy = ref<ITauriTypes.Config.NetworkConfig["Proxy"]["UseCustomProxy"]>(false);
    const CustomProxyUri = ref<ITauriTypes.Config.NetworkConfig["Proxy"]["CustomProxyUri"]>("");
    const CustomProxyUsername = ref<ITauriTypes.Config.NetworkConfig["Proxy"]["CustomProxyAccount"]>("");
    const CustomProxyPassword = ref<ITauriTypes.Config.NetworkConfig["Proxy"]["CustomProxyPassword"]>("");

    // 调试
    const DebugMode = ref<ITauriTypes.Config.DebugConfig["Root"]["Enabled"]>(false);

    onMounted(async () => {
        DownloadSource.value = await TauriConfig.get("Download.Source.DownloadSource");
        VersionSource.value = await TauriConfig.get("Download.Source.VersionSource");
        MaxConcurrent.value = await TauriConfig.get("Download.Internet.MaxConcurrent");
        MaxBandwidth.value = await TauriConfig.get("Download.Internet.MaxBandwidth");
        _MaxBandwidth.value = mapMaxBandwidthOutputToInput(MaxBandwidth.value);
        PostSelectInstance.value = await TauriConfig.get("Download.PostInstall.SelectInstance");
        UpdateAuthlib.value = await TauriConfig.get("Download.PostInstall.UpdateAuthlib");
        // Source.value = other_config.comp.source;
        // IgnoreQuilt.value = other_config.comp.ignore_quilt;
        // DetectClipboard.value = other_config.comp.detect_clipboard;
        // ReleaseNote.value = other_config.accessibility.release_note;
        // SnapshotNote.value = other_config.accessibility.snapshot_note;
        // AutoChinese.value = other_config.accessibility.auto_chinese;
        // UpdateMethod.value = other_config.launcher.update_method;
        // Channel.value = other_config.launcher.channel;
        // Notification.value = other_config.launcher.notification;
        // CacheDir.value = other_config.launcher.cache_dir;
        UseDoH.value = await TauriConfig.get("Network.UseDoh");
        UseSystemProxy.value = await TauriConfig.get("Network.Proxy.UseSystemProxy");
        UseCustomProxy.value = await TauriConfig.get("Network.Proxy.UseCustomProxy");
        _ProxyType.value = UseSystemProxy.value ? "system" : UseCustomProxy.value ? "custom" : "disable";
        CustomProxyUri.value = await TauriConfig.get("Network.Proxy.CustomProxyUri");
        CustomProxyUsername.value = await TauriConfig.get("Network.Proxy.CustomProxyAccount");
        CustomProxyPassword.value = await TauriConfig.get("Network.Proxy.CustomProxyPassword");
        DebugMode.value = await TauriConfig.get("Debug.Root.Enabled");

        // 强制更新 [更新通道] 以避免回溯更新
        // if ($meta.NOVA_CHANNEL === "Ender" && !["overworld", "nether"].includes(Channel.value)) {
        //     Channel.value = "ender";
        // }
        // if ($meta.NOVA_CHANNEL === "Nether" && Channel.value === "overworld") {
        //     Channel.value = "nether";
        // }

        // focus 指示
        setTimeout(() => {
            if (refUpdateMethodName.value && route.query && route.query["focus"] === "updateMethod") {
                refUpdateMethodName.value.focus();
            }
        }, 50);
    });

    watchAndSet(DownloadSource, "Download.Source.DownloadSource");
    watchAndSet(VersionSource, "Download.Source.VersionSource");
    watchAndSet(MaxConcurrent, "Download.Internet.MaxConcurrent");
    watchAndSet(MaxBandwidth, "Download.Internet.MaxBandwidth");
    watchAndSet(PostSelectInstance, "Download.PostInstall.SelectInstance");
    watchAndSet(UpdateAuthlib, "Download.PostInstall.UpdateAuthlib");
    watchAndSet(UseDoH, "Network.UseDoh");
    watchAndSet(UseSystemProxy, "Network.Proxy.UseSystemProxy");
    watchAndSet(UseCustomProxy, "Network.Proxy.UseCustomProxy");
    watchAndSet(CustomProxyUri, "Network.Proxy.CustomProxyUri");
    watchAndSet(CustomProxyUsername, "Network.Proxy.CustomProxyAccount");
    watchAndSet(CustomProxyPassword, "Network.Proxy.CustomProxyPassword");
    watchAndSet(DebugMode, "Debug.Root.Enabled");
</script>

<template>
    <div class="overflow-hidden">
        <main class="p-6 pr-5 w-full max-h-[calc(100vh-128px-var(--spacing)*1)] rounded-box overflow-y-auto beautiful-scrollbar">
            <div class="card bg-base-100 outline outline-base-content/25 w-full">
                <div class="card-body px-4 py-3 pb-4">
                    <h1 class="card-title">{{ t("Main.Setting/Other.Download.__Title__") }}</h1>

                    <section class="grid grid-cols-[144px_4fr] grid-rows-4 gap-x-8 gap-y-2 items-center">
                        <span class="text-sm ml-4">{{ t("Main.Setting/Other.Download.DownloadSource.__Name__") }}</span>
                        <select class="select select-bordered select-sm outline-none w-full" v-model="DownloadSource">
                            <option value="mirror">{{ t("Main.Setting/Other.Download.DownloadSource.Mirror") }}</option>
                            <option value="balance">
                                {{ t("Main.Setting/Other.Download.DownloadSource.Balance") }}
                            </option>
                            <option value="offical">
                                {{ t("Main.Setting/Other.Download.DownloadSource.Offical") }}
                            </option>
                        </select>

                        <span class="text-sm ml-4">{{ t("Main.Setting/Other.Download.VersionSource.__Name__") }}</span>
                        <select class="select select-bordered select-sm outline-none w-full" v-model="VersionSource">
                            <option value="mirror">{{ t("Main.Setting/Other.Download.VersionSource.Mirror") }}</option>
                            <option value="balance">
                                {{ t("Main.Setting/Other.Download.VersionSource.Balance") }}
                            </option>
                            <option value="offical">
                                {{ t("Main.Setting/Other.Download.VersionSource.Offical") }}
                            </option>
                        </select>

                        <span class="text-sm ml-4">{{ t("Main.Setting/Other.Download.MaxConcurrent.__Name__") }}</span>
                        <div class="flex items-center gap-4 translate-y-px">
                            <span
                                class="w-22 badge badge-soft"
                                :class="{
                                    'badge-success': MaxConcurrent <= 128,
                                    'badge-warning': MaxConcurrent > 128 && MaxConcurrent <= 196,
                                    'badge-error': MaxConcurrent > 196,
                                }">
                                {{ MaxConcurrent }}x
                            </span>
                            <input
                                type="range"
                                min="8"
                                max="256"
                                step="8"
                                class="range range-xs range-primary w-full"
                                v-model.number="MaxConcurrent" />
                        </div>

                        <span class="text-sm ml-4">{{ t("Main.Setting/Other.Download.MaxBandwidth.__Name__") }}</span>
                        <div class="flex items-center gap-4">
                            <span class="w-22 badge badge-soft badge-info" v-if="MaxBandwidth !== -1"> {{ MaxBandwidth }} MB/s </span>
                            <span class="w-22 badge badge-soft badge-success" v-else>
                                {{ t("Main.Setting/Other.Download.MaxBandwidth.Unlimited") }}
                            </span>
                            <input
                                type="range"
                                min="1"
                                max="45"
                                step="1"
                                class="range range-xs range-primary w-full translate-y-px"
                                v-model.number="_MaxBandwidth" />
                        </div>

                        <span class="text-sm ml-4">
                            {{ t("Main.Setting/Other.Download.InstallBehavior.__Name__") }}
                        </span>
                        <div class="flex gap-4">
                            <label class="label text-base-content">
                                <input type="checkbox" class="checkbox checkbox-primary checkbox-sm" v-model="PostSelectInstance" />
                                {{ t("Main.Setting/Other.Download.InstallBehavior.PostSelectInstance") }}
                            </label>
                            <label class="label text-base-content">
                                <input type="checkbox" class="checkbox checkbox-primary checkbox-sm" v-model="UpdateAuthlib" />
                                {{ t("Main.Setting/Other.Download.InstallBehavior.UpdateAuthlib") }}
                            </label>
                        </div>
                    </section>
                </div>
            </div>
            <!--
            <div class="card bg-base-100 outline outline-base-content/25 w-full mt-4">
                <div class="card-body px-4 py-3 pb-4">
                    <h1 class="card-title">{{ t("Main.Setting/Other.Download.__Title__") }}</h1>

                    <section class="grid grid-cols-[144px_4fr] grid-rows-2 gap-x-8 gap-y-2 items-center">
                        <span class="text-sm ml-4">{{ t("Main.Setting/Other.Comp.DownloadSource.__Name__") }}</span>
                        <select class="select select-bordered select-sm outline-none w-full" v-model="Source">
                            <option value="mirror">{{ t("Main.Setting/Other.Comp.DownloadSource.Mirror") }}</option>
                            <option value="balance">{{ t("Main.Setting/Other.Comp.DownloadSource.Balance") }}</option>
                            <option value="offical">{{ t("Main.Setting/Other.Comp.DownloadSource.Offical") }}</option>
                        </select>

                        <div class="flex gap-4 col-span-2 ml-4">
                            <label class="label text-base-content">
                                <input type="checkbox" class="checkbox checkbox-primary checkbox-sm" v-model="IgnoreQuilt" />
                                {{ t("Main.Setting/Other.Comp.IgnoreQuilt") }}
                            </label>
                            <label class="label text-base-content">
                                <input type="checkbox" class="checkbox checkbox-primary checkbox-sm" v-model="DetectClipboard" />
                                {{ t("Main.Setting/Other.Comp.DetectClipboard") }}
                            </label>
                        </div>
                    </section>
                </div>
            </div>
            <div class="card bg-base-100 outline outline-base-content/25 w-full mt-4">
                <div class="card-body px-4 py-3 pb-4">
                    <h1 class="card-title">{{ t("Main.Setting/Other.Accessibility.__Title__") }}</h1>

                    <section class="grid grid-cols-[144px_4fr] grid-rows-2 gap-x-8 gap-y-3 mt-1 items-center">
                        <span class="text-sm ml-4">
                            {{ t("Main.Setting/Other.Accessibility.GameUpdateNote.__Name__") }}
                        </span>
                        <div class="flex gap-4">
                            <label class="label text-base-content">
                                <input type="checkbox" class="checkbox checkbox-primary checkbox-sm" v-model="ReleaseNote" />
                                {{ t("Main.Setting/Other.Accessibility.GameUpdateNote.Release") }}
                            </label>
                            <label class="label text-base-content">
                                <input type="checkbox" class="checkbox checkbox-primary checkbox-sm" v-model="SnapshotNote" />
                                {{ t("Main.Setting/Other.Accessibility.GameUpdateNote.Snapshot") }}
                            </label>
                        </div>

                        <span class="text-sm ml-4">
                            {{ t("Main.Setting/Other.Accessibility.GameLanguage.__Name__") }}
                        </span>
                        <label class="label text-base-content">
                            <input type="checkbox" class="checkbox checkbox-primary checkbox-sm" v-model="AutoChinese" />
                            {{ t("Main.Setting/Other.Accessibility.GameLanguage.AutoChinese") }}
                        </label>
                    </section>
                </div>
            </div>
            <div class="card bg-base-100 outline outline-base-content/25 w-full mt-4">
                <div class="card-body px-4 py-3 pb-4">
                    <h1 class="card-title">{{ t("Main.Setting/Other.Launcher.__Title__") }}</h1>

                    <section class="grid grid-cols-[144px_4fr] grid-rows-4 gap-x-8 gap-y-2 items-center">
                        <span class="text-sm ml-4" ref="refUpdateMethodName">{{ t("Main.Setting/Other.Launcher.UpdateMethod.__Name__") }}</span>
                        <select class="select select-bordered select-sm outline-none w-full" v-model="UpdateMethod">
                            <option value="auto">{{ t("Main.Setting/Other.Launcher.UpdateMethod.Auto") }}</option>
                            <option value="notice">{{ t("Main.Setting/Other.Launcher.UpdateMethod.Notice") }}</option>
                            <option value="major_notice">
                                {{ t("Main.Setting/Other.Launcher.UpdateMethod.MajorNotice") }}
                            </option>
                            <option value="disable">{{ t("Main.Setting/Other.Launcher.UpdateMethod.Disable") }}</option>
                        </select>

                        <span class="text-sm ml-4">{{ t("Main.Setting/Other.Launcher.Channel.__Name__") }}</span>
                        <select class="select select-bordered select-sm outline-none w-full" v-model="Channel">
                            <option value="overworld" :disabled="['Ender', 'Nether'].includes($meta.NOVA_CHANNEL)">
                                {{ t("Main.More/About.UpdateChannel.Channels.Overworld") }}
                                <span v-if="['Ender', 'Nether'].includes($meta.NOVA_CHANNEL)" class="text-error/50">
                                    {{ t("Main.Setting/Other.Launcher.Channel.__NotAllowed__") }}
                                </span>
                            </option>
                            <option value="nether" :disabled="['Ender'].includes($meta.NOVA_CHANNEL)">
                                {{ t("Main.More/About.UpdateChannel.Channels.Nether") }}
                                <span v-if="['Ender'].includes($meta.NOVA_CHANNEL)" class="text-error/50">
                                    {{ t("Main.Setting/Other.Launcher.Channel.__NotAllowed__") }}
                                </span>
                            </option>
                            <option value="ender">{{ t("Main.More/About.UpdateChannel.Channels.Ender") }}</option>
                        </select>

                        <span class="text-sm ml-4">{{ t("Main.Setting/Other.Launcher.Notification.__Name__") }}</span>
                        <select class="select select-bordered select-sm outline-none w-full" v-model="Notification">
                            <option value="all">{{ t("Main.Setting/Other.Launcher.Notification.Auto") }}</option>
                            <option value="major">{{ t("Main.Setting/Other.Launcher.Notification.Major") }}</option>
                            <option value="disable">{{ t("Main.Setting/Other.Launcher.Notification.Disable") }}</option>
                        </select>

                        <span class="text-sm ml-4">{{ t("Main.Setting/Other.Launcher.CacheDir.__Name__") }}</span>
                        <input
                            type="text"
                            class="input input-bordered input-sm outline-none w-full"
                            :placeholder="t('Main.Setting/Other.Launcher.CacheDir.Placeholder')"
                            v-model="CacheDir" />
                    </section>

                    <div class="flex gap-4 mt-2 ml-4">
                        <button class="btn btn-outline w-1/5 btn-primary">
                            {{ t("Main.Setting/Other.Launcher.Buttons.CheckUpdate") }}
                        </button>
                        <button class="btn btn-outline w-1/5 btn-primary">
                            {{ t("Main.Setting/Other.Launcher.Buttons.ExportConfig") }}
                        </button>
                        <button class="btn btn-outline w-1/5 btn-primary">
                            {{ t("Main.Setting/Other.Launcher.Buttons.ImportConfig") }}
                        </button>
                    </div>
                </div>
            </div>
            -->
            <div class="card bg-base-100 outline outline-base-content/25 w-full mt-4">
                <div class="card-body px-4 py-3 pb-4">
                    <h1 class="card-title">{{ t("Main.Setting/Other.Network.__Title__") }}</h1>

                    <label class="label text-base-content ml-4 my-2">
                        <input type="checkbox" class="checkbox checkbox-primary checkbox-sm" v-model="UseDoH" />
                        {{ t("Main.Setting/Other.Network.UseDoH") }}
                    </label>

                    <section class="grid grid-cols-[144px_4fr] grid-rows-1 gap-x-8 gap-y-2 items-start">
                        <span class="text-sm ml-4">{{ t("Main.Setting/Other.Network.Proxy.__Name__") }}</span>

                        <section class="flex flex-col gap-2">
                            <div class="flex gap-16">
                                <div class="flex gap-2 items-center">
                                    <input
                                        type="radio"
                                        name="radio-3"
                                        id="radio-3-1"
                                        class="radio radio-sm radio-primary"
                                        v-model="_ProxyType"
                                        value="disable" />
                                    <label class="label text-base-content" for="radio-3-1">
                                        {{ t("Main.Setting/Other.Network.Proxy.Disable") }}
                                    </label>
                                </div>
                                <div class="flex gap-2 items-center">
                                    <input
                                        type="radio"
                                        name="radio-3"
                                        id="radio-3-2"
                                        class="radio radio-sm radio-primary"
                                        v-model="_ProxyType"
                                        value="system" />
                                    <label class="label text-base-content" for="radio-3-2">
                                        {{ t("Main.Setting/Other.Network.Proxy.System") }}
                                    </label>
                                </div>
                                <div class="flex gap-2 items-center">
                                    <input
                                        type="radio"
                                        name="radio-3"
                                        id="radio-3-3"
                                        class="radio radio-sm radio-primary"
                                        v-model="_ProxyType"
                                        value="custom" />
                                    <label class="label text-base-content" for="radio-3-3">
                                        {{ t("Main.Setting/Other.Network.Proxy.Custom.__Name__") }}
                                    </label>
                                </div>
                            </div>
                            <input
                                type="text"
                                class="input input-bordered input-sm outline-none w-full"
                                :placeholder="t('Main.Setting/Other.Network.Proxy.Custom.Placeholder')"
                                v-model="CustomProxyUri"
                                v-if="_ProxyType === 'custom'" />
                            <div class="grid grid-cols-[64px_1fr_64px_1fr] items-center gap-x-2" v-if="_ProxyType === 'custom'">
                                <span class="text-sm text-right">
                                    {{ t("Main.Setting/Other.Network.Proxy.Custom.Username.__Name__") }}
                                </span>
                                <input
                                    type="text"
                                    class="input input-bordered input-sm outline-none w-full"
                                    :placeholder="t('Main.Setting/Other.Network.Proxy.Custom.Username.Placeholder')"
                                    v-model="CustomProxyUsername" />
                                <span class="text-sm text-right">
                                    {{ t("Main.Setting/Other.Network.Proxy.Custom.Password.__Name__") }}
                                </span>
                                <input
                                    type="password"
                                    class="input input-bordered input-sm outline-none w-full"
                                    :placeholder="t('Main.Setting/Other.Network.Proxy.Custom.Password.Placeholder')"
                                    v-model="CustomProxyPassword" />
                            </div>
                        </section>
                    </section>
                </div>
            </div>
            <div class="collapse collapse-arrow bg-base-100 outline outline-base-content/25 outline-offset-2 w-full mt-4">
                <input type="checkbox" />
                <div class="collapse-title font-semibold">{{ t("Main.Setting/Other.Debug.__Title__") }}</div>
                <div class="collapse-content">
                    <section class="grid grid-cols-[144px_4fr] grid-rows-1 gap-x-8 gap-y-2 items-start">
                        <label class="label text-base-content ml-4">
                            <input type="checkbox" class="checkbox checkbox-primary checkbox-sm" v-model="DebugMode" />
                            <span class="text-sm">{{ t("Main.Setting/Other.Debug.DebugMode") }}</span>
                        </label>
                    </section>
                </div>
            </div>
        </main>
    </div>
</template>
