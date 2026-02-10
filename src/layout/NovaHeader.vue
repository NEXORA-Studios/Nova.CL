<script setup lang="ts">
    import { onMounted, ref } from "vue";
    import { useTheme, useTimer } from "@/composables";
    import { getCurrentWindow } from "@tauri-apps/api/window";

    const $app = getCurrentWindow();
    const { matchTheme } = useTheme();

    const timer = useTimer();
    const isMaxiMized = ref(false);

    async function update() {
        isMaxiMized.value = await $app.isMaximized();
    }

    onMounted(() => {
        timer.onShortInterval(update);
    });
</script>

<template>
    <header style="grid-area: header" class="flex justify-end items-center rounded-none" data-tauri-drag-region @dblclick="$app.toggleMaximize">
        <div class="flex gap-4 items-center ml-4">
            <figure class="size-12 -translate-x-2">
                <img src="/logo.webp" alt="logo" v-if="matchTheme('light')" />
                <img src="/logodark.webp" alt="logo" v-if="matchTheme('dark')" />
            </figure>
            <span class="text-xl ml-5 -translate-x-9 -translate-y-0.75">Nova.CL</span>
        </div>
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
