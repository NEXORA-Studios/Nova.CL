<script setup lang="ts">
    import { ScrollArea } from "@/components";
    import Customize from "./Customize.vue";
    import Launch from "./Launch.vue";
    import Other from "./Other.vue";
</script>

<template>
    <main class="p-6 w-full h-full rounded-box beautiful-scrollbar">
        <ScrollArea maxHeight="100%">
            <section class="flex flex-col p-4 pb-22">
                <h1 class="text-2xl font-bold mb-4 -mt-2 px-2">个性化</h1>
                <div class="px-8 flex flex-col gap-4">
                    <Customize />
                </div>
                <div class="divider"></div>
                <h1 class="text-2xl font-bold mb-4 -mt-2 px-2">启动选项</h1>
                <div class="px-8 flex flex-col gap-4">
                    <Launch />
                </div>
                <div class="divider"></div>
                <h1 class="text-2xl font-bold mb-4 -mt-2 px-2">更多</h1>
                <div class="px-8 flex flex-col gap-4">
                    <Other />
                </div>
            </section>
        </ScrollArea>
    </main>
</template>
