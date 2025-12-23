<script setup lang="ts">
    import { onMounted, ref, watch } from "vue";
    import { useI18n } from "vue-i18n";
    import { ITauriTypes } from "@/types";
    import { TauriTOML } from "@/modules";
    const { t } = useI18n();

    const VersionIndie = ref<ITauriTypes.TOML.BasicLaunchConfig["version_indie_type"]>("disabled");
    const LauncherVisibility = ref<ITauriTypes.TOML.BasicLaunchConfig["launcher_visibility"]>("immediately_quit");
    const PreferIPStack = ref<ITauriTypes.TOML.BasicLaunchConfig["prefer_ip_stack"]>(4);

    const AutoRam = ref<ITauriTypes.TOML.RamConfig["auto_ram"]>(true);
    const CustomRam = ref<ITauriTypes.TOML.RamConfig["custom_ram"]>(1024);
    const PreSwap = ref<ITauriTypes.TOML.RamConfig["pre_swap"]>(false);

    const Renderer = ref<ITauriTypes.TOML.AdvancedLaunchConfig["renderer"]>("default");
    const JvmArgs = ref<ITauriTypes.TOML.AdvancedLaunchConfig["jvm_args"]>("");
    const GameArgs = ref<ITauriTypes.TOML.AdvancedLaunchConfig["game_args"]>("");
    const PreCommand = ref<ITauriTypes.TOML.AdvancedLaunchConfig["pre_command"]>("");
    const DisableRetroWrapper = ref<ITauriTypes.TOML.AdvancedLaunchConfig["disable_retrowrapper"]>(false);
    const UseDiscreteGpu = ref<ITauriTypes.TOML.AdvancedLaunchConfig["use_discrete_gpu"]>(false);
    const UseJavaExe = ref<ITauriTypes.TOML.AdvancedLaunchConfig["use_java_exe"]>(false);

    onMounted(async () => {
        const config = await TauriTOML.getGlobalConfig();
        const launch_config = config.launch;
        VersionIndie.value = launch_config.basic.version_indie_type;
        LauncherVisibility.value = launch_config.basic.launcher_visibility;
        PreferIPStack.value = launch_config.basic.prefer_ip_stack;
        AutoRam.value = launch_config.rams.auto_ram;
        CustomRam.value = launch_config.rams.custom_ram;
        PreSwap.value = launch_config.rams.pre_swap;
        Renderer.value = launch_config.advanced.renderer;
        JvmArgs.value = launch_config.advanced.jvm_args;
        GameArgs.value = launch_config.advanced.game_args;
        PreCommand.value = launch_config.advanced.pre_command;
        DisableRetroWrapper.value = launch_config.advanced.disable_retrowrapper;
        UseDiscreteGpu.value = launch_config.advanced.use_discrete_gpu;
        UseJavaExe.value = launch_config.advanced.use_java_exe;
    });

    watch(
        [
            VersionIndie,
            LauncherVisibility,
            PreferIPStack,
            AutoRam,
            CustomRam,
            PreSwap,
            Renderer,
            JvmArgs,
            GameArgs,
            PreCommand,
            DisableRetroWrapper,
            UseDiscreteGpu,
            UseJavaExe,
        ],
        async ([
            VersionIndie,
            LauncherVisibility,
            PreferIPStack,
            AutoRam,
            CustomRam,
            PreSwap,
            Renderer,
            JvmArgs,
            GameArgs,
            PreCommand,
            DisableRetroWrapper,
            UseDiscreteGpu,
            UseJavaExe,
        ]) => {
            const config = await TauriTOML.getGlobalConfig();
            config.launch.basic.version_indie_type = VersionIndie;
            config.launch.basic.launcher_visibility = LauncherVisibility;
            config.launch.basic.prefer_ip_stack = PreferIPStack;
            config.launch.rams.auto_ram = AutoRam;
            config.launch.rams.custom_ram = CustomRam;
            config.launch.rams.pre_swap = PreSwap;
            config.launch.advanced.renderer = Renderer;
            config.launch.advanced.jvm_args = JvmArgs;
            config.launch.advanced.game_args = GameArgs;
            config.launch.advanced.pre_command = PreCommand;
            config.launch.advanced.disable_retrowrapper = DisableRetroWrapper;
            config.launch.advanced.use_discrete_gpu = UseDiscreteGpu;
            config.launch.advanced.use_java_exe = UseJavaExe;
            await TauriTOML.saveGlobalConfig(config);
        }
    );
</script>

<template>
    <main class="p-6 pr-8 max-h-[calc(100vh-128px-var(--spacing)*4)] rounded-box overflow-auto beautiful-scrollbar">
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

        <div class="card bg-base-100 outline outline-base-content/25 w-full mt-4">
            <div class="card-body px-4 py-3 pb-5">
                <h1 class="card-title">{{ t("Main.Setting/Launch.RAM.__Title__") }}</h1>

                <div class="flex gap-2 items-center pl-4 mt-2">
                    <input
                        type="radio"
                        name="radio-1"
                        id="radio-1-1"
                        class="radio radio-primary radio-sm"
                        v-model="AutoRam"
                        :value="true" />
                    <label for="radio-1-1" class="text-sm -translate-y-px">
                        {{ t("Main.Setting/Launch.RAM.Auto") }}
                    </label>
                </div>
                <div class="grid grid-cols-[144px_1fr] gap-x-8 items-center pl-4">
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
                    <label class="input input-sm outline-none w-full">
                        <input
                            type="number"
                            :disabled="AutoRam"
                            :max="30 * 1024"
                            :min="1024"
                            :step="512"
                            v-model="CustomRam" />
                        <span class="badge badge-neutral badge-sm">MB</span>
                    </label>
                </div>
                <div class="flex gap-2 items-center pl-4 mt-1">
                    <input
                        type="checkbox"
                        id="pre-swap"
                        class="checkbox checkbox-primary checkbox-sm"
                        v-model="PreSwap" />
                    <label for="pre-swap" class="text-sm -translate-y-px">
                        {{ t("Main.Setting/Launch.RAM.PreSwap") }}
                    </label>
                </div>
            </div>
        </div>

        <div class="collapse collapse-arrow bg-base-100 outline outline-base-content/25 outline-offset-2 w-full mt-4">
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
                    <textarea
                        class="textarea w-full h-full outline-none resize-none overflow-y-auto"
                        v-model="JvmArgs"></textarea>
                    <span class="text-sm ml-4">{{ t("Main.Setting/Launch.Advanced.GameArgs.__Name__") }}</span>
                    <input type="text" class="input input-sm w-full outline-none" v-model="GameArgs" />
                    <span class="text-sm ml-4">{{ t("Main.Setting/Launch.Advanced.PreCommand.__Name__") }}</span>
                    <input type="text" class="input input-sm w-full outline-none" v-model="PreCommand" />
                </section>
                <div class="flex gap-2 items-center pl-4 mt-4">
                    <input
                        type="checkbox"
                        id="disable_retrowrapper"
                        class="checkbox checkbox-primary checkbox-sm"
                        v-model="DisableRetroWrapper" />
                    <label for="disable_retrowrapper" class="text-sm -translate-y-px">
                        {{ t("Main.Setting/Launch.Advanced.DisableRetrowrapper") }}
                    </label>
                </div>
                <div class="flex gap-2 items-center pl-4 mt-2">
                    <input
                        type="checkbox"
                        id="use_discrete_gpu"
                        class="checkbox checkbox-primary checkbox-sm"
                        v-model="UseDiscreteGpu" />
                    <label for="use_discrete_gpu" class="text-sm -translate-y-px">
                        {{ t("Main.Setting/Launch.Advanced.UseDiscreteGPU") }}
                    </label>
                </div>
                <div class="flex gap-2 items-center pl-4 mt-2">
                    <input
                        type="checkbox"
                        id="use_java_exe"
                        class="checkbox checkbox-primary checkbox-sm"
                        v-model="UseJavaExe" />
                    <label for="use_java_exe" class="text-sm -translate-y-px">
                        {{ t("Main.Setting/Launch.Advanced.UseJavaExe") }}
                    </label>
                </div>
            </div>
        </div>
    </main>
</template>
