<script setup lang="ts">
    import { ref } from "vue";
    import { Avatar, GlassSurface } from "@/components";
    import { useProfileStore } from "@/modules";
    // import { ELaunchStatus } from "@/utils";
    // import { useI18n } from "vue-i18n";

    // const { locale } = useI18n();
    // const status = ref<ELaunchStatus>(ELaunchStatus.Downloaded);
    // const progress = ref<number>(0);

    const profileStore = useProfileStore();
    const showMenu = ref<boolean>(false);

    function closeMenu(fn?: Function) {
        showMenu.value = false;
        fn?.();
    }
</script>

<template>
    <footer class="absolute bottom-4 w-full flex justify-center items-center">
        <div class="relative w-5/6 flex justify-center items-center">
            <Transition name="fade">
                <GlassSurface
                    className="absolute! left-0! bottom-24! rounded-[39px]! w-96! h-90! z-1001"
                    :backgroundOpacity="0.75"
                    v-if="showMenu">
                    <div class="w-full px-6 py-4 flex flex-col gap-2">
                        <h1 class="text-lg font-bold opacity-90">下载</h1>
                        <section class="join w-full gap-1 pl-[0.5px]">
                            <button class="btn join-item h-full flex gap-2 py-2">
                                <i class="icon-[vscode-icons--file-type-minecraft] size-6 -ml-2"></i>
                                <span class="-mr-2">Minecraft</span>
                            </button>
                            <button class="btn join-item h-full flex gap-2 py-2">
                                <i class="icon-[catppuccin--folder-assets] size-6 -ml-2 filter brightness-0 dark:invert -translate-y-px"></i>
                                <span class="-mr-2">社区资源</span>
                            </button>
                            <button class="btn join-item h-full flex gap-2 py-2">
                                <i class="icon-[mynaui--heart-solid] text-error size-6 -ml-2"></i>
                                <span class="-mr-2">收藏夹</span>
                            </button>
                        </section>
                        <i class="divider my-0"></i>
                        <h1 class="text-lg font-bold opacity-90">设置</h1>
                        <section class="join w-full grid grid-cols-2 gap-1">
                            <button class="btn join-item h-full flex gap-2 py-2" @click="closeMenu(() => $router.push('/setting/global'))">
                                <i class="icon-[material-symbols--settings-applications-outline-rounded] size-6 -ml-2 mr-2"></i>
                                <span class="-mr-2">全局设置</span>
                            </button>
                            <button class="btn join-item h-full flex gap-2 py-2">
                                <i class="icon-[material-symbols--stadia-controller-outline] size-6 -ml-2 mr-2"></i>
                                <span class="-mr-2">实例设置</span>
                            </button>
                        </section>
                        <i class="divider my-0"></i>
                        <h1 class="text-lg font-bold opacity-90">更多</h1>
                        <section class="join w-full grid grid-cols-3 gap-1">
                            <button class="btn join-item h-full flex gap-2 py-2">
                                <i class="icon-[material-symbols--info-outline-rounded] size-6 -ml-2"></i>
                                <span class="-mr-2">关于</span>
                            </button>
                            <button class="btn join-item h-full flex gap-2 py-2">
                                <i class="icon-[material-symbols--feedback-outline-rounded] size-6 -ml-2"></i>
                                <span class="-mr-2">反馈</span>
                            </button>
                            <button class="btn join-item h-full flex gap-2 py-2">
                                <i class="icon-[material-symbols--service-toolbox-outline-rounded] size-6 -ml-2"></i>
                                <span class="-mr-2">百宝箱</span>
                            </button>
                        </section>
                    </div>
                </GlassSurface>
            </Transition>
            <GlassSurface className="w-5/6! h-18! rounded-full! fixed! bottom-6!" :backgroundOpacity="0.75" style="custom-style">
                <div class="w-full h-full pl-px flex">
                    <button
                        class="btn h-full rounded-full aspect-square mr-2"
                        @click="showMenu = !showMenu"
                        @contextmenu.prevent="closeMenu(() => $router.push('/'))">
                        <i class="icon-[material-symbols--page-menu-ios-outline-rounded] size-6"></i>
                    </button>
                    <button class="btn h-full relative px-6 rounded-full" @click="closeMenu(() => $router.push('/profile'))">
                        <section class="size-8 aspect-square mr-2 translate-y-px">
                            <img src="/images/barrier.png" class="size-8 -ml-4" v-if="!profileStore.currentProfile" />
                            <Avatar
                                :name="profileStore.currentProfile?.Name"
                                extra-class="size-8"
                                v-else-if="profileStore.currentProfile?.Type === 'msa'" />
                            <Avatar name="MHF_Steve" extra-class="size-8" v-else />
                        </section>
                        <div class="flex flex-col items-start" v-if="!profileStore.currentProfile">
                            <span class="opacity-90">{{ $t("Aside.NoProfile.__Title__") }}</span>
                            <span class="text-xs opacity-50">{{ $t("Aside.NoProfile.__Hint__") }}</span>
                        </div>
                        <div class="w-full flex flex-col items-start" v-else>
                            <span class="opacity-90">{{ profileStore.currentProfile?.Name }}</span>
                            <span class="text-xs opacity-50 font-normal">
                                {{
                                    profileStore.currentProfile?.Type !== undefined
                                        ? $t(`Aside.AccountType.${profileStore.currentProfile?.Type}`)
                                        : ""
                                }}
                            </span>
                        </div>
                    </button>
                    <button class="h-full btn text-success rounded-full ml-auto" @click="closeMenu()" @contextmenu.prevent="closeMenu()">
                        <i class="icon-[material-symbols--rocket-launch-outline-rounded] size-8"></i>
                        <div class="w-full flex flex-col items-start ml-2">
                            <span class="opacity-90">{{ $t("Footer.LaunchGame") }}</span>
                            <span class="text-xs opacity-50 font-normal">Create: Above and Beyond</span>
                        </div>
                    </button>
                </div>
            </GlassSurface>
        </div>
    </footer>
</template>

<style lang="css" scoped>
    .fade-enter-from,
    .fade-leave-to {
        opacity: 0;
        transform: translateY(6px);
    }

    .fade-enter-active,
    .fade-leave-active {
        transition:
            opacity 0.25s ease,
            transform 0.25s ease;
    }
</style>
