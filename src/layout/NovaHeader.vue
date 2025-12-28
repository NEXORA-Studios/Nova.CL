<script setup lang="ts">
    import { onMounted, ref } from "vue";
    import { useTimer } from "@/composables";
    import { getCurrentWindow } from "@tauri-apps/api/window";

    const $app = getCurrentWindow();

    const timer = useTimer();
    const isMaxiMized = ref(false);

    interface SubpageMenuItemA {
        type: "go";
        display: string;
        go: number;
        current: string;
    }
    interface SubpageMenuItemB {
        type: "to";
        display: string;
        to: string;
    }
    interface SubpageMenuItemC {
        type: "divider";
    }
    type SubpageMenuItem = SubpageMenuItemA | SubpageMenuItemB | SubpageMenuItemC;
    const subpageMenu: Record<string, SubpageMenuItem[]> = {
        profile: [{ type: "go", display: "Header.Back", go: -1, current: "/account" }],
        download: [
            { type: "to", display: "Header.Download.Minecraft", to: "/download/minecraft" },
            // { type: "to", display: "Header.Download.Installer", to: "/download/installer" },
            { type: "divider" },
            { type: "to", display: "Header.Download.Mod", to: "/download/mod" },
            { type: "to", display: "Header.Download.Modpack", to: "/download/modpack" },
            { type: "to", display: "Header.Download.Datapack", to: "/download/datapack" },
            { type: "to", display: "Header.Download.Resourcepack", to: "/download/resourcepack" },
            { type: "to", display: "Header.Download.Shader", to: "/download/shader" },
            // { type: "to", display: "Header.Download.Map", to: "/download/map" },
            { type: "divider" },
            { type: "to", display: "Header.Download.Stared", to: "/download/stared" },
        ],
        setting: [
            { type: "to", display: "Header.Setting.Launch", to: "/setting/launch" },
            { type: "to", display: "Header.Setting.Customize", to: "/setting/customize" },
            { type: "to", display: "Header.Setting.Other", to: "/setting/other" },
        ],
        more: [
            { type: "to", display: "Header.More.About", to: "/more/about" },
            { type: "to", display: "Header.More.Tools", to: "/more/tools" },
        ],
    };

    async function update() {
        isMaxiMized.value = await $app.isMaximized();
    }

    onMounted(() => {
        timer.onShortInterval(update);
    });
</script>

<template>
    <header style="grid-area: header" class="flex justify-end items-center rounded-none" data-tauri-drag-region @dblclick="$app.toggleMaximize">
        <section class="ml-6">
            <TransitionGroup name="fade-slide" tag="div" v-for="(items, key) in subpageMenu" :key="key">
                <div v-if="($route.path as string).startsWith(`/${key}`)" :id="key" class="flex gap-2 absolute -translate-y-1/2">
                    <template v-for="(item, index) in items" :key="index">
                        <button
                            class="btn btn-sm btn-primary rounded-full"
                            :class="{
                                'btn-soft': (item.type === 'to' && $route.path !== item.to) || (item.type === 'go' && item.go),
                            }"
                            v-if="(item.type === 'to' && item.to) || item.type === 'go'"
                            @click="item.type === 'to' ? $router.push(item.to) : item.type === 'go' ? $router.go(item.go) : null"
                            @dblclick="null">
                            <i
                                class="icon-[material-symbols--arrow-back-2-outline-rounded] size-4 -mx-0.5"
                                v-if="item.type === 'go' && item.go === -1"></i>
                            {{ $t(item.display) }}
                        </button>
                        <div v-else class="divider divider-horizontal mx-0"></div>
                    </template>
                </div>
            </TransitionGroup>
        </section>
        <section class="ml-auto join h-full no-drag">
            <button class="join-item btn h-full btn-ghost text-base-content aspect-square" @click="$app.minimize">
                <i class="icon-[mynaui--minus-solid] size-6 -m-2"></i>
            </button>
            <button class="join-item btn h-full btn-ghost text-base-content aspect-square" @click="$app.toggleMaximize">
                <i class="icon-[material-symbols--stack-outline-rounded] -scale-x-100 size-4.5 -m-2" v-if="isMaxiMized"></i>
                <i class="icon-[material-symbols--square-outline-rounded] size-4.5 -m-2" v-else></i>
            </button>
            <button class="join-item btn h-full btn-ghost btn-error text-base-content aspect-square" @click="$app.close">
                <i class="icon-[material-symbols--close-rounded] size-6 -m-2"></i>
            </button>
        </section>
    </header>
</template>

<style scoped>
    .fade-slide-enter-active,
    .fade-slide-leave-active {
        transition: all 0.3s ease;
    }

    .fade-slide-enter-from {
        opacity: 0;
        transform: translateX(-10px);
    }

    .fade-slide-leave-to {
        opacity: 0;
        transform: translateX(10px);
    }

    .fade-slide-move {
        transition: transform 0.3s ease;
    }
</style>
