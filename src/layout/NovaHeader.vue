<script setup lang="ts">
    import { useTimer } from "@/composables";
    import { getCurrentWindow } from "@tauri-apps/api/window";
    import { onMounted, ref } from "vue";

    const $app = getCurrentWindow();

    const timer = useTimer();
    const isMaxiMized = ref(false);
    const subpageMenu = {
        download: [
            { display: "Header.Download.Minecraft", to: "/download/minecraft" },
            // { display: "Header.Download.Installer", to: "/download/installer" },
            {},
            { display: "Header.Download.Mod", to: "/download/mod" },
            { display: "Header.Download.Modpack", to: "/download/modpack" },
            { display: "Header.Download.Datapack", to: "/download/datapack" },
            { display: "Header.Download.Resourcepack", to: "/download/resourcepack" },
            { display: "Header.Download.Shader", to: "/download/shader" },
            // { display: "Header.Download.Map", to: "/download/map" },
            {},
            { display: "Header.Download.Stared", to: "/download/stared" },
        ],
        more: [
            { display: "Header.More.About", to: "/more/about" },
            { display: "Header.More.Tools", to: "/more/tools" },
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
    <header
        style="grid-area: header"
        class="flex justify-end items-center rounded-none"
        data-tauri-drag-region
        @dblclick="$app.toggleMaximize">
        <section class="ml-6">
            <TransitionGroup name="fade-slide" tag="div" v-for="(value, key) in subpageMenu" :key="key">
                <div
                    v-if="($route.path as string).startsWith(`/${key}`)"
                    :id="key"
                    class="flex gap-2 absolute -translate-y-1/2">
                    <template v-for="(item, index) in value" :key="index">
                        <button
                            class="btn btn-sm btn-primary rounded-full"
                            :class="{
                                'btn-soft': $route.path !== item.to,
                            }"
                            v-if="item.to && item.display"
                            @click="$router.push(item.to)"
                            @dblclick="null">
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
                <i
                    class="icon-[material-symbols--stack-outline-rounded] -scale-x-100 size-4.5 -m-2"
                    v-if="isMaxiMized"></i>
                <i class="icon-[material-symbols--square-outline-rounded] size-4.5 -m-2" v-else></i>
            </button>
            <button
                class="join-item btn h-full btn-ghost btn-error text-base-content aspect-square"
                @click="$app.close">
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
