<script setup lang="ts">
    import { onMounted, ref } from "vue";
    import { useI18n } from "vue-i18n";
    import { ITauriTypes } from "@/types";
    import { TauriSystem, TauriConfig } from "@/modules";
    import { useTimer } from "@/composables";
    import { DoubleStateProgress } from "@/components/NovaUI";
    import { watchAndSet } from "@/utils";
    const { t } = useI18n();

    const { onMediumInterval } = useTimer();
    const MaxRam = ref<number>();
    const CurrentRam = ref<number>();
    const AvailableRam = ref<number>();

    const VersionIndie = ref<ITauriTypes.Config.LaunchConfig["Basic"]["VersionIndieType"]>("disabled");
    const LauncherVisibility = ref<ITauriTypes.Config.LaunchConfig["Basic"]["LauncherVisibility"]>("immediately_quit");
    const PreferIPStack = ref<ITauriTypes.Config.LaunchConfig["Basic"]["PreferIpStack"]>(4);

    const AutoRam = ref<ITauriTypes.Config.LaunchConfig["Ram"]["AutoRam"]>(true);
    const CustomRam = ref<ITauriTypes.Config.LaunchConfig["Ram"]["CustomRam"]>(1024);
    const PreSwap = ref<ITauriTypes.Config.LaunchConfig["Ram"]["PreSwap"]>(false);

    const Renderer = ref<ITauriTypes.Config.LaunchConfig["Advanced"]["Renderer"]>("default");
    const JvmArgs = ref<ITauriTypes.Config.LaunchConfig["Advanced"]["JvmArgs"]>("");
    const GameArgs = ref<ITauriTypes.Config.LaunchConfig["Advanced"]["GameArgs"]>("");
    const PreCommand = ref<ITauriTypes.Config.LaunchConfig["Advanced"]["PreLaunchCommand"]>("");
    const DisableRetroWrapper = ref<ITauriTypes.Config.LaunchConfig["Advanced"]["DisableRetroWrapper"]>(false);
    const UseDiscreteGpu = ref<ITauriTypes.Config.LaunchConfig["Advanced"]["UseDiscreteGpu"]>(false);
    const UseJavaExe = ref<ITauriTypes.Config.LaunchConfig["Advanced"]["UseJavaExe"]>(false);

    async function updateRamInfo() {
        const ram_info = await TauriSystem.getRamInfo();
        const { total, used, available } = ram_info;
        MaxRam.value = Math.round((total / 1024 / 1024 / 1024 / 1024) * 100) / 100;
        CurrentRam.value = Math.round((used / 1024 / 1024 / 1024 / 1024) * 100) / 100;
        AvailableRam.value = Math.round((available / 1024 / 1024 / 1024 / 1024) * 100) / 100;
    }

    onMounted(async () => {
        await updateRamInfo();
        VersionIndie.value = await TauriConfig.get("Launch.Basic.VersionIndieType");
        LauncherVisibility.value = await TauriConfig.get("Launch.Basic.LauncherVisibility");
        PreferIPStack.value = await TauriConfig.get("Launch.Basic.PreferIpStack");
        AutoRam.value = await TauriConfig.get("Launch.Ram.AutoRam");
        CustomRam.value = await TauriConfig.get("Launch.Ram.CustomRam");
        PreSwap.value = await TauriConfig.get("Launch.Ram.PreSwap");
        Renderer.value = await TauriConfig.get("Launch.Advanced.Renderer");
        JvmArgs.value = await TauriConfig.get("Launch.Advanced.JvmArgs");
        GameArgs.value = await TauriConfig.get("Launch.Advanced.GameArgs");
        PreCommand.value = await TauriConfig.get("Launch.Advanced.PreLaunchCommand");
        DisableRetroWrapper.value = await TauriConfig.get("Launch.Advanced.DisableRetroWrapper");
        UseDiscreteGpu.value = await TauriConfig.get("Launch.Advanced.UseDiscreteGpu");
        UseJavaExe.value = await TauriConfig.get("Launch.Advanced.UseJavaExe");
        console.debug("JvmArgs", JvmArgs.value);
    });

    onMediumInterval(updateRamInfo);

    watchAndSet(VersionIndie, "Launch.Basic.VersionIndieType");
    watchAndSet(LauncherVisibility, "Launch.Basic.LauncherVisibility");
    watchAndSet(PreferIPStack, "Launch.Basic.PreferIpStack");
    watchAndSet(AutoRam, "Launch.Ram.AutoRam");
    watchAndSet(CustomRam, "Launch.Ram.CustomRam");
    watchAndSet(PreSwap, "Launch.Ram.PreSwap");
    watchAndSet(Renderer, "Launch.Advanced.Renderer");
    watchAndSet(JvmArgs, "Launch.Advanced.JvmArgs");
    watchAndSet(GameArgs, "Launch.Advanced.GameArgs");
    watchAndSet(PreCommand, "Launch.Advanced.PreLaunchCommand");
    watchAndSet(DisableRetroWrapper, "Launch.Advanced.DisableRetroWrapper");
    watchAndSet(UseDiscreteGpu, "Launch.Advanced.UseDiscreteGpu");
    watchAndSet(UseJavaExe, "Launch.Advanced.UseJavaExe");
</script>

<template>
    <div class="card bg-base-100 outline outline-base-content/25 w-full">
        <div class="card-body px-4 py-3 pb-4">
            <h1 class="card-title">{{ t("Main.Setting/Launch.Basic.__Title__") }}</h1>

            <section class="grid grid-cols-[144px_4fr] grid-rows-4 gap-x-8 gap-y-2 items-center">
                <span class="text-sm ml-4">{{ t("Main.Setting/Launch.Basic.VersionIndie.__Name__") }}</span>
                <select class="select select-sm w-full outline-none" v-model="VersionIndie">
                    <option value="disabled">{{ t("Main.Setting/Launch.Basic.VersionIndie.Disabled") }}</option>
                    <option value="modded">{{ t("Main.Setting/Launch.Basic.VersionIndie.Modded") }}</option>
                    <option value="snapshot">{{ t("Main.Setting/Launch.Basic.VersionIndie.Snapshot") }}</option>
                    <option value="modded_or_snapshot">
                        {{ t("Main.Setting/Launch.Basic.VersionIndie.ModdedOrSnapshot") }}
                    </option>
                    <option value="all">{{ t("Main.Setting/Launch.Basic.VersionIndie.All") }}</option>
                </select>

                <span class="text-sm ml-4">
                    {{ t("Main.Setting/Launch.Basic.LauncherVisibility.__Name__") }}
                </span>
                <select class="select select-sm w-full outline-none" v-model="LauncherVisibility">
                    <option value="immediately_quit">
                        {{ t("Main.Setting/Launch.Basic.LauncherVisibility.ImmediatelyQuit") }}
                    </option>
                    <option value="hide_then_quit">
                        {{ t("Main.Setting/Launch.Basic.LauncherVisibility.HideThenQuit") }}
                    </option>
                    <option value="hide_then_show">
                        {{ t("Main.Setting/Launch.Basic.LauncherVisibility.HideThenShow") }}
                    </option>
                    <option value="minimize">
                        {{ t("Main.Setting/Launch.Basic.LauncherVisibility.Minimize") }}
                    </option>
                    <option value="constant">
                        {{ t("Main.Setting/Launch.Basic.LauncherVisibility.Constant") }}
                    </option>
                </select>

                <span class="text-sm ml-4">
                    {{ t("Main.Setting/Launch.Basic.PreferIPStack.__Name__") }}
                </span>
                <select class="select select-sm w-full outline-none" v-model="PreferIPStack">
                    <option :value="4">{{ t("Main.Setting/Launch.Basic.PreferIPStack.IPv4") }}</option>
                    <option :value="0">{{ t("Main.Setting/Launch.Basic.PreferIPStack.Auto") }}</option>
                    <option :value="6">{{ t("Main.Setting/Launch.Basic.PreferIPStack.IPv6") }}</option>
                </select>

                <span class="text-sm ml-4">
                    {{ t("Main.Setting/Launch.Basic.Java.__Name__") }}
                </span>
                <button class="btn btn-soft btn-primary w-fit rounded-full btn-sm">
                    <i class="icon-[material-symbols--exit-to-app-rounded] size-5"></i>
                    {{ t("Main.Setting/Launch.Basic.Java.GotoJavaManager") }}
                </button>
            </section>
        </div>
    </div>

    <div class="card bg-base-100 outline outline-base-content/25 w-full">
        <div class="card-body px-4 py-3 pb-5">
            <h1 class="card-title">{{ t("Main.Setting/Launch.RAM.__Title__") }}</h1>

            <section class="pl-4 flex flex-col gap-2">
                <div class="flex gap-2 items-center mt-2">
                    <input type="radio" name="radio-1" id="radio-1-1" class="radio radio-primary radio-sm" v-model="AutoRam" :value="true" />
                    <label for="radio-1-1" class="text-sm -translate-y-px">
                        {{ t("Main.Setting/Launch.RAM.Auto") }}
                    </label>
                </div>
                <div class="grid grid-cols-[144px_1fr] gap-x-8 items-center">
                    <div class="flex gap-2">
                        <input
                            type="radio"
                            name="radio-1"
                            id="radio-1-2"
                            class="radio radio-primary radio-sm"
                            v-model="AutoRam"
                            :value="false" />
                        <label for="radio-1-2" class="text-sm -translate-y-px">
                            {{ t("Main.Setting/Launch.RAM.Custom") }}
                        </label>
                    </div>
                    <input
                        type="range"
                        :min="1024"
                        :max="(MaxRam || 8) * 1024"
                        :step="512"
                        :disabled="AutoRam"
                        v-model.number="CustomRam"
                        v-if="MaxRam"
                        class="range range-xs range-primary w-full" />
                </div>
                <div class="flex gap-2 items-center mt-1">
                    <input type="checkbox" id="pre-swap" class="checkbox checkbox-primary checkbox-sm" v-model="PreSwap" />
                    <label for="pre-swap" class="text-sm -translate-y-px">
                        {{ t("Main.Setting/Launch.RAM.PreSwap") }}
                    </label>
                </div>
                <DoubleStateProgress
                    v-if="CurrentRam"
                    :current="CurrentRam || 0"
                    :use="AutoRam ? 1024 * 6 : CustomRam || 0"
                    :max="MaxRam || 0"
                    class="mt-2" />
            </section>
        </div>
    </div>

    <div class="collapse collapse-arrow bg-base-100 outline outline-base-content/25 outline-offset-2 w-full">
        <input type="checkbox" />
        <div class="collapse-title font-semibold">{{ t("Main.Setting/Launch.Advanced.__Title__") }}</div>
        <div class="collapse-content">
            <section class="grid grid-cols-[192px_5fr] grid-rows-[1fr_3fr_1fr_1fr] gap-x-8 gap-y-2 items-center">
                <span class="text-sm ml-4">{{ t("Main.Setting/Launch.Advanced.Renderer.__Name__") }}</span>
                <select class="select select-sm w-full outline-none" v-model="Renderer">
                    <option value="default">{{ t("Main.Setting/Launch.Advanced.Renderer.Default") }}</option>
                    <option value="llvmpipe">{{ t("Main.Setting/Launch.Advanced.Renderer.LLVMPipe") }}</option>
                    <option value="d3d12">{{ t("Main.Setting/Launch.Advanced.Renderer.D3D12") }}</option>
                    <option value="zink">{{ t("Main.Setting/Launch.Advanced.Renderer.Zink") }}</option>
                </select>
                <span class="text-sm ml-4">{{ t("Main.Setting/Launch.Advanced.JvmArgs.__Name__") }}</span>
                <textarea class="textarea w-full h-full outline-none resize-none overflow-y-auto" v-model="JvmArgs"></textarea>
                <span class="text-sm ml-4">{{ t("Main.Setting/Launch.Advanced.GameArgs.__Name__") }}</span>
                <input type="text" class="input input-sm w-full outline-none" v-model="GameArgs" />
                <span class="text-sm ml-4">{{ t("Main.Setting/Launch.Advanced.PreCommand.__Name__") }}</span>
                <input type="text" class="input input-sm w-full outline-none" v-model="PreCommand" />
            </section>
            <div class="flex gap-2 items-center pl-4 mt-4">
                <input type="checkbox" id="disable_retrowrapper" class="checkbox checkbox-primary checkbox-sm" v-model="DisableRetroWrapper" />
                <label for="disable_retrowrapper" class="text-sm -translate-y-px">
                    {{ t("Main.Setting/Launch.Advanced.DisableRetrowrapper") }}
                </label>
            </div>
            <div class="flex gap-2 items-center pl-4 mt-2">
                <input type="checkbox" id="use_discrete_gpu" class="checkbox checkbox-primary checkbox-sm" v-model="UseDiscreteGpu" />
                <label for="use_discrete_gpu" class="text-sm -translate-y-px">
                    {{ t("Main.Setting/Launch.Advanced.UseDiscreteGPU") }}
                </label>
            </div>
            <div class="flex gap-2 items-center pl-4 mt-2">
                <input type="checkbox" id="use_java_exe" class="checkbox checkbox-primary checkbox-sm" v-model="UseJavaExe" />
                <label for="use_java_exe" class="text-sm -translate-y-px">
                    {{ t("Main.Setting/Launch.Advanced.UseJavaExe") }}
                </label>
            </div>
        </div>
    </div>
</template>
